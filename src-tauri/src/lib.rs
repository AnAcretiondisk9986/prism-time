mod analytics;
mod catalog;
mod commands;
mod demo;
mod models;
mod scanner;
mod storage;
mod sync;
mod web_catalog;
mod win_http;
mod win_probe;

use crate::catalog::{
    apply_override, extract_process_stem, is_browser_stem, is_ignored_process,
    is_lock_screen_process, match_custom_rule, resolve_observed_app_with_custom, SELF_APP_ID,
};
use crate::commands::{sync_custom_rule_into_catalog, RuntimeTrackerState, SharedAppState};
use crate::models::{AppCategory, SyncStatusDto, UsageSession};
use crate::scanner::scan_installed_software;
use crate::storage::Storage;
use crate::web_catalog::resolve_browser_page;
use crate::win_probe::{
    extract_icon_png_data_url, get_idle_seconds, is_workstation_locked, probe_browser_omnibox_url,
    probe_foreground_window,
};
use chrono::{Local, TimeZone};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent};

fn flush_active_session(state: &SharedAppState, session: Option<UsageSession>) {
    if let Some(s) = session {
        if s.duration_secs >= 2 && s.end_ts > s.start_ts {
            let _ = state.storage.append_session(&s);
        }
    }
    state.storage.save_active_checkpoint(None);
}

fn spawn_auto_sync_thread(shared: Arc<SharedAppState>) {
    thread::spawn(move || {
        let mut elapsed_secs: u64 = 0;
        loop {
            thread::sleep(Duration::from_secs(30));
            elapsed_secs += 30;

            let sync_cfg = shared.settings.lock().unwrap().sync.clone();
            if !sync_cfg.auto_sync_enabled || sync_cfg.provider == "none" {
                elapsed_secs = 0;
                continue;
            }
            let interval_secs = sync_cfg.auto_sync_interval_mins.clamp(5, 1440) * 60;
            if elapsed_secs < interval_secs {
                continue;
            }
            elapsed_secs = 0;

            let provider = sync_cfg.provider.clone();
            let mut ov = shared.overrides.lock().unwrap().clone();
            let mut rules = shared.custom_rules.lock().unwrap().clone();
            match sync::run_bidirectional_sync(
                &provider,
                &sync_cfg,
                &shared.storage,
                &mut ov,
                &mut rules,
            ) {
                Ok(status) => {
                    *shared.overrides.lock().unwrap() = ov.clone();
                    *shared.custom_rules.lock().unwrap() = rules.clone();
                    {
                        let mut catalog = shared.catalog.lock().unwrap();
                        for r in &rules {
                            sync_custom_rule_into_catalog(&mut catalog, r);
                        }
                        for (aid, o) in &ov {
                            if let Some(meta) = catalog.get_mut(aid) {
                                apply_override(meta, o);
                            }
                        }
                        let _ = shared.storage.save_catalog(&catalog);
                    }
                    *shared.sync_status.lock().unwrap() = status;
                }
                Err(msg) => {
                    *shared.sync_status.lock().unwrap() = SyncStatusDto {
                        status: "error".to_string(),
                        provider: Some(provider),
                        last_sync_ts: Some(Local::now().timestamp()),
                        message: msg,
                        uploaded_files: 0,
                        downloaded_files: 0,
                        merged_sessions: 0,
                    };
                }
            }
        }
    });
}

fn spawn_monitor_thread(shared: Arc<SharedAppState>) {
    thread::spawn(move || {
        let mut checkpoint_ticks: u64 = 0;
        let mut browser_url_cache: Option<(isize, String, u8, Option<String>)> = None;

        loop {
            thread::sleep(Duration::from_secs(1));

            let settings = shared.settings.lock().unwrap().clone();
            if !settings.tracking_enabled {
                let pending = {
                    let mut rt = shared.runtime.lock().unwrap();
                    rt.state_code = "paused".to_string();
                    rt.idle_secs = 0;
                    rt.active_session.take()
                };
                if pending.is_some() {
                    flush_active_session(&shared, pending);
                }
                continue;
            }

            if settings.pause_on_lock && is_workstation_locked() {
                let pending = {
                    let mut rt = shared.runtime.lock().unwrap();
                    rt.state_code = "locked".to_string();
                    rt.active_session.take()
                };
                if pending.is_some() {
                    flush_active_session(&shared, pending);
                }
                continue;
            }

            let idle_secs = get_idle_seconds();
            let (current_cat, custom_idle_mult) = {
                let rt = shared.runtime.lock().unwrap();
                let catalog = shared.catalog.lock().unwrap();
                let overrides = shared.overrides.lock().unwrap();
                let custom_rules = shared.custom_rules.lock().unwrap();
                if let Some(ref s) = rt.active_session {
                    let cat = catalog.get(&s.app_id).map(|m| m.category);
                    let mult = overrides
                        .get(&s.app_id)
                        .and_then(|o| o.idle_multiplier)
                        .or_else(|| {
                            custom_rules
                                .iter()
                                .find(|r| r.app_id == s.app_id)
                                .map(|r| r.idle_multiplier)
                        });
                    (cat, mult)
                } else {
                    (None, None)
                }
            };

            // idle_multiplier == Some(0) means continuous tracking (never idle for this app)
            let is_idle = match custom_idle_mult {
                Some(0) => false,
                Some(mult) => idle_secs >= settings.idle_threshold_secs.saturating_mul(mult as u64),
                None => {
                    let limit = if current_cat == Some(AppCategory::Media) {
                        settings.idle_threshold_secs.saturating_mul(3)
                    } else {
                        settings.idle_threshold_secs
                    };
                    idle_secs >= limit
                }
            };

            if is_idle {
                let pending = {
                    let mut rt = shared.runtime.lock().unwrap();
                    rt.state_code = "idle".to_string();
                    rt.idle_secs = idle_secs;
                    rt.active_session.take()
                };
                if pending.is_some() {
                    flush_active_session(&shared, pending);
                }
                continue;
            }

            let Some(sample) = probe_foreground_window() else {
                continue;
            };

            if is_lock_screen_process(&sample.exe_stem_lower) && settings.pause_on_lock {
                let pending = {
                    let mut rt = shared.runtime.lock().unwrap();
                    rt.state_code = "locked".to_string();
                    rt.active_session.take()
                };
                if pending.is_some() {
                    flush_active_session(&shared, pending);
                }
                continue;
            }

            // Priority #0 check: if user defined an explicit custom rule matching this process,
            // allow it to bypass default system noise filtering!
            let has_explicit_custom_rule = {
                let custom_rules = shared.custom_rules.lock().unwrap();
                match_custom_rule(
                    &sample.exe_stem_lower,
                    &sample.exe_path.to_lowercase(),
                    &sample.window_title.to_lowercase(),
                    &custom_rules,
                )
                .is_some()
            };

            if !has_explicit_custom_rule
                && is_ignored_process(
                    &sample.exe_stem_lower,
                    &sample.exe_path.to_lowercase(),
                    Some(&sample.window_class),
                )
            {
                let pending = {
                    let mut rt = shared.runtime.lock().unwrap();
                    rt.state_code = "desktop".to_string();
                    rt.idle_secs = idle_secs;
                    rt.active_session.take()
                };
                if pending.is_some() {
                    flush_active_session(&shared, pending);
                }
                continue;
            }

            let now_dt = Local::now();
            let now_ts = now_dt.timestamp();
            let today_str = now_dt.format("%Y-%m-%d").to_string();
            let proc_stem = extract_process_stem(&sample.exe_path);

            let resolved_ctx = {
                let mut catalog = shared.catalog.lock().unwrap();
                let overrides = shared.overrides.lock().unwrap();
                let custom_rules = shared.custom_rules.lock().unwrap();
                let mut ctx = resolve_observed_app_with_custom(
                    &sample.exe_path,
                    sample.pe_product.as_deref(),
                    sample.pe_description.as_deref(),
                    sample.pe_company.as_deref(),
                    Some(&sample.window_title),
                    &catalog,
                    &overrides,
                    &custom_rules,
                );

                if ctx.meta.icon_data_url.is_none() && !sample.exe_path.is_empty() {
                    ctx.meta.icon_data_url = extract_icon_png_data_url(&sample.exe_path);
                }
                ctx.meta.last_seen_ts = Some(now_ts);
                ctx.meta.installed = true;
                if !sample.exe_path.is_empty()
                    && !ctx
                        .meta
                        .exe_paths
                        .iter()
                        .any(|p| p.eq_ignore_ascii_case(&sample.exe_path))
                {
                    ctx.meta.exe_paths.push(sample.exe_path.clone());
                }
                if !proc_stem.is_empty()
                    && !ctx
                        .meta
                        .process_names
                        .iter()
                        .any(|p| p.eq_ignore_ascii_case(&proc_stem))
                {
                    ctx.meta.process_names.push(proc_stem.clone());
                }

                let is_new_entry = !catalog.contains_key(&ctx.meta.app_id)
                    || catalog.get(&ctx.meta.app_id).is_some_and(|old| {
                        (old.icon_data_url.is_none() && ctx.meta.icon_data_url.is_some())
                            || old.source != ctx.meta.source
                            || old.display_name != ctx.meta.display_name
                    });
                catalog.insert(ctx.meta.app_id.clone(), ctx.meta.clone());
                if is_new_entry {
                    let _ = shared.storage.save_catalog(&catalog);
                }
                ctx
            };

            let resolved = resolved_ctx.meta;

            if resolved.ignored {
                let pending = {
                    let mut rt = shared.runtime.lock().unwrap();
                    rt.idle_secs = idle_secs;
                    rt.active_session.take()
                };
                if pending.is_some() {
                    flush_active_session(&shared, pending);
                }
                continue;
            }

            if resolved.app_id == SELF_APP_ID && !settings.track_self {
                let pending = {
                    let mut rt = shared.runtime.lock().unwrap();
                    rt.state_code = "self".to_string();
                    rt.idle_secs = idle_secs;
                    rt.active_session.take()
                };
                if pending.is_some() {
                    flush_active_session(&shared, pending);
                }
                continue;
            }

            let is_browser = match resolved_ctx.treat_as_browser {
                Some(explicit) => explicit,
                None => {
                    resolved.category == AppCategory::Browser
                        || is_browser_stem(&sample.exe_stem_lower)
                }
            };
            let web_page = if is_browser && settings.web_tracking_enabled {
                let cached_url = match &browser_url_cache {
                    Some((cached_hwnd, cached_title, age, url))
                        if *cached_hwnd == sample.hwnd_key
                            && cached_title == &sample.window_title
                            && *age < 4 =>
                    {
                        let next_url = url.clone();
                        browser_url_cache =
                            Some((*cached_hwnd, cached_title.clone(), age + 1, next_url.clone()));
                        next_url
                    }
                    _ => {
                        let fresh = probe_browser_omnibox_url(sample.hwnd_key);
                        browser_url_cache = Some((
                            sample.hwnd_key,
                            sample.window_title.clone(),
                            0,
                            fresh.clone(),
                        ));
                        fresh
                    }
                };
                Some(resolve_browser_page(cached_url.as_deref(), &sample.window_title))
            } else {
                None
            };

            let raw_title_source = web_page
                .as_ref()
                .and_then(|w| w.clean_page_title.as_deref())
                .unwrap_or(sample.window_title.as_str());
            let sample_title = if raw_title_source.trim().is_empty() {
                None
            } else {
                let truncated: String = raw_title_source.trim().chars().take(72).collect();
                Some(truncated)
            };

            let mut to_flush: Option<UsageSession> = None;
            let mut checkpoint_copy: Option<UsageSession> = None;

            {
                let mut rt = shared.runtime.lock().unwrap();
                rt.state_code = "active".to_string();
                rt.idle_secs = idle_secs;

                let next_web_site_id = web_page.as_ref().map(|w| w.site_id.clone());
                let same_app_and_day = rt.active_session.as_ref().is_some_and(|cur| {
                    let cur_day = Local
                        .timestamp_opt(cur.start_ts, 0)
                        .single()
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or_default();
                    cur.app_id == resolved.app_id
                        && cur.web_site_id == next_web_site_id
                        && cur_day == today_str
                });

                if same_app_and_day {
                    if let Some(cur) = rt.active_session.as_mut() {
                        cur.end_ts = (now_ts + 1).max(cur.start_ts + 1);
                        cur.duration_secs = cur.end_ts - cur.start_ts;
                        if cur.sample_title.is_none() {
                            cur.sample_title = sample_title;
                        }
                        checkpoint_copy = Some(cur.clone());
                    }
                } else {
                    to_flush = rt.active_session.take();
                    let new_session = UsageSession {
                        id: format!("s-{now_ts}-{}", resolved.app_id),
                        app_id: resolved.app_id.clone(),
                        process_name: if proc_stem.is_empty() {
                            None
                        } else {
                            Some(proc_stem.clone())
                        },
                        start_ts: now_ts,
                        end_ts: now_ts + 1,
                        duration_secs: 1,
                        sample_title,
                        web_site_id: web_page.as_ref().map(|w| w.site_id.clone()),
                        web_site_name: web_page.as_ref().map(|w| w.display_name.clone()),
                        web_domain: web_page.as_ref().map(|w| w.domain.clone()),
                        web_category: web_page.as_ref().map(|w| w.category),
                    };
                    checkpoint_copy = Some(new_session.clone());
                    rt.active_session = Some(new_session);
                    checkpoint_ticks = 0;
                }
            }

            if to_flush.is_some() {
                flush_active_session(&shared, to_flush);
            }

            checkpoint_ticks += 1;
            if checkpoint_ticks >= 15 {
                checkpoint_ticks = 0;
                shared
                    .storage
                    .save_active_checkpoint(checkpoint_copy.as_ref());
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let storage = Storage::new(None);
    let _ = storage.recover_active_checkpoint();
    let settings = storage.load_settings();
    let overrides = storage.load_overrides();
    let custom_rules = storage.load_custom_rules();
    let mut catalog = storage.load_catalog();

    scan_installed_software(&mut catalog, true);
    for rule in &custom_rules {
        sync_custom_rule_into_catalog(&mut catalog, rule);
    }
    for (aid, ov) in &overrides {
        if let Some(meta) = catalog.get_mut(aid) {
            apply_override(meta, ov);
        }
    }
    let _ = storage.save_catalog(&catalog);

    let initial_state_code = if settings.tracking_enabled {
        "active".to_string()
    } else {
        "paused".to_string()
    };

    let shared = Arc::new(SharedAppState {
        storage,
        settings: Mutex::new(settings),
        catalog: Mutex::new(catalog),
        overrides: Mutex::new(overrides),
        custom_rules: Mutex::new(custom_rules),
        sync_status: Mutex::new(SyncStatusDto::default()),
        runtime: Mutex::new(RuntimeTrackerState {
            state_code: initial_state_code,
            active_session: None,
            idle_secs: 0,
        }),
        allow_exit: Arc::new(Mutex::new(false)),
    });

    spawn_monitor_thread(Arc::clone(&shared));
    spawn_auto_sync_thread(Arc::clone(&shared));

    let shared_for_tray = Arc::clone(&shared);
    let shared_for_win = Arc::clone(&shared);

    tauri::Builder::default()
        .manage(Arc::clone(&shared))
        .setup(move |app| {
            let show_item = MenuItem::with_id(app, "show", "打开澄镜主面板", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出监控", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let icon = app.default_window_icon().cloned();
            let mut tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("澄镜 · 屏幕使用洞察")
                .on_menu_event(move |app_handle, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(win) = app_handle.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.unminimize();
                            let _ = win.set_focus();
                        }
                    }
                    "quit" => {
                        let pending = shared_for_tray.runtime.lock().unwrap().active_session.take();
                        flush_active_session(&shared_for_tray, pending);
                        *shared_for_tray.allow_exit.lock().unwrap() = true;
                        app_handle.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app_handle = tray.app_handle();
                        if let Some(win) = app_handle.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.unminimize();
                            let _ = win.set_focus();
                        }
                    }
                });

            if let Some(ic) = icon {
                tray_builder = tray_builder.icon(ic);
            }
            let _ = tray_builder.build(app)?;
            Ok(())
        })
        .on_window_event(move |window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let allow_exit = *shared_for_win.allow_exit.lock().unwrap();
                let minimize_to_tray = shared_for_win
                    .settings
                    .lock()
                    .unwrap()
                    .minimize_to_tray_on_close;
                if !allow_exit && minimize_to_tray {
                    api.prevent_close();
                    let _ = window.hide();
                } else {
                    let pending = shared_for_win.runtime.lock().unwrap().active_session.take();
                    flush_active_session(&shared_for_win, pending);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_live_status,
            commands::get_dashboard_analytics,
            commands::get_app_detail,
            commands::get_software_catalog,
            commands::rescan_local_software,
            commands::update_app_override,
            commands::reset_app_override,
            commands::get_custom_app_rules,
            commands::save_custom_app_rule,
            commands::delete_custom_app_rule,
            commands::get_sync_status,
            commands::test_sync_connection,
            commands::run_sync_now,
            commands::get_settings,
            commands::update_settings,
            commands::toggle_tracking,
            commands::export_usage_report,
            commands::window_minimize,
            commands::window_toggle_maximize,
            commands::window_close_or_hide,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
