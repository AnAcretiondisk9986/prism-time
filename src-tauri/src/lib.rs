mod analytics;
mod catalog;
mod commands;
mod demo;
mod models;
mod scanner;
mod storage;
mod win_probe;

use crate::catalog::{
    apply_override, extract_process_stem, is_ignored_process, is_lock_screen_process,
    resolve_observed_app, SELF_APP_ID,
};
use crate::commands::{RuntimeTrackerState, SharedAppState};
use crate::models::{AppCategory, UsageSession};
use crate::scanner::scan_installed_software;
use crate::storage::Storage;
use crate::win_probe::{
    extract_icon_png_data_url, get_idle_seconds, is_workstation_locked, probe_foreground_window,
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

fn spawn_monitor_thread(shared: Arc<SharedAppState>) {
    thread::spawn(move || {
        let mut checkpoint_ticks: u64 = 0;

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

            // Check lock screen
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
            let current_cat = {
                let rt = shared.runtime.lock().unwrap();
                let catalog = shared.catalog.lock().unwrap();
                rt.active_session
                    .as_ref()
                    .and_then(|s| catalog.get(&s.app_id).map(|m| m.category))
            };
            // Relax idle detection 3x while watching media so full-screen videos aren't clipped prematurely
            let effective_idle_limit = if current_cat == Some(AppCategory::Media) {
                settings.idle_threshold_secs.saturating_mul(3)
            } else {
                settings.idle_threshold_secs
            };

            if idle_secs >= effective_idle_limit {
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

            if is_ignored_process(
                &sample.exe_stem_lower,
                &sample.exe_path.to_lowercase(),
                Some(&sample.window_class),
            ) {
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

            let resolved = {
                let mut catalog = shared.catalog.lock().unwrap();
                let overrides = shared.overrides.lock().unwrap();
                let mut meta = resolve_observed_app(
                    &sample.exe_path,
                    sample.pe_product.as_deref(),
                    sample.pe_description.as_deref(),
                    sample.pe_company.as_deref(),
                    Some(&sample.window_title),
                    &catalog,
                );

                if let Some(ov) = overrides.get(&meta.app_id) {
                    apply_override(&mut meta, ov);
                }
                if meta.icon_data_url.is_none() && !sample.exe_path.is_empty() {
                    meta.icon_data_url = extract_icon_png_data_url(&sample.exe_path);
                }
                meta.last_seen_ts = Some(now_ts);
                meta.installed = true;
                if !sample.exe_path.is_empty()
                    && !meta
                        .exe_paths
                        .iter()
                        .any(|p| p.eq_ignore_ascii_case(&sample.exe_path))
                {
                    meta.exe_paths.push(sample.exe_path.clone());
                }
                if !proc_stem.is_empty()
                    && !meta
                        .process_names
                        .iter()
                        .any(|p| p.eq_ignore_ascii_case(&proc_stem))
                {
                    meta.process_names.push(proc_stem.clone());
                }

                let is_new_entry = !catalog.contains_key(&meta.app_id)
                    || catalog.get(&meta.app_id).is_some_and(|old| {
                        (old.icon_data_url.is_none() && meta.icon_data_url.is_some())
                            || old.source != meta.source
                            || old.display_name != meta.display_name
                    });
                catalog.insert(meta.app_id.clone(), meta.clone());
                if is_new_entry {
                    let _ = shared.storage.save_catalog(&catalog);
                }
                meta
            };

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

            let sample_title = if sample.window_title.trim().is_empty() {
                None
            } else {
                let t = sample.window_title.trim();
                let truncated: String = t.chars().take(72).collect();
                Some(truncated)
            };

            let mut to_flush: Option<UsageSession> = None;
            let mut checkpoint_copy: Option<UsageSession> = None;

            {
                let mut rt = shared.runtime.lock().unwrap();
                rt.state_code = "active".to_string();
                rt.idle_secs = idle_secs;

                let same_app_and_day = rt.active_session.as_ref().is_some_and(|cur| {
                    let cur_day = Local
                        .timestamp_opt(cur.start_ts, 0)
                        .single()
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or_default();
                    cur.app_id == resolved.app_id && cur_day == today_str
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
    let mut catalog = storage.load_catalog();

    // Perform initial local software scan (extract icons for uninstalled/uncached items)
    scan_installed_software(&mut catalog, true);
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
        runtime: Mutex::new(RuntimeTrackerState {
            state_code: initial_state_code,
            active_session: None,
            idle_secs: 0,
        }),
        allow_exit: Arc::new(Mutex::new(false)),
    });

    spawn_monitor_thread(Arc::clone(&shared));

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
