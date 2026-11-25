use crate::analytics::{build_app_detail, build_dashboard_analytics};
use crate::catalog::apply_override;
use crate::demo::generate_demo_dataset;
use crate::models::{
    AppDetailDto, AppMetadata, AppOverride, AppOverrideInput, AppSettings, CatalogAppDto,
    DashboardAnalyticsDto, ExportResultDto, LiveStatusDto, UsageSession,
};
use crate::scanner::scan_installed_software;
use crate::storage::Storage;
use chrono::{Duration, Local, NaiveDate};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};

#[derive(Debug, Clone)]
pub struct RuntimeTrackerState {
    pub state_code: String, // "active" | "idle" | "locked" | "paused" | "self" | "desktop"
    pub active_session: Option<UsageSession>,
    pub idle_secs: u64,
}

impl Default for RuntimeTrackerState {
    fn default() -> Self {
        Self {
            state_code: "active".to_string(),
            active_session: None,
            idle_secs: 0,
        }
    }
}

pub struct SharedAppState {
    pub storage: Storage,
    pub settings: Mutex<AppSettings>,
    pub catalog: Mutex<HashMap<String, AppMetadata>>,
    pub overrides: Mutex<HashMap<String, AppOverride>>,
    pub runtime: Mutex<RuntimeTrackerState>,
    pub allow_exit: Arc<Mutex<bool>>,
}

impl SharedAppState {
    pub fn build_sessions_map(
        &self,
        dates: &[NaiveDate],
        include_active: bool,
    ) -> HashMap<NaiveDate, Vec<UsageSession>> {
        let mut map = HashMap::new();
        for &d in dates {
            map.insert(d, self.storage.load_sessions_for_date(d));
        }

        if include_active {
            let active = self.runtime.lock().unwrap().active_session.clone();
            if let Some(s) = active {
                if s.duration_secs >= 1 {
                    let today = Local::now().date_naive();
                    if let Some(vec) = map.get_mut(&today) {
                        vec.push(s);
                    }
                }
            }
        }
        map
    }
}

#[cfg(windows)]
pub fn apply_startup_registry(enabled: bool) -> Result<(), String> {
    use windows_registry::CURRENT_USER;
    let run_key = CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Run")
        .map_err(|e| format!("无法访问开机自启注册表项: {e}"))?;
    if enabled {
        let exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let val = format!("\"{}\"", exe.to_string_lossy());
        run_key
            .set_string("PrismTime", val)
            .map_err(|e| format!("写入开机自启失败: {e}"))?;
    } else {
        let _ = run_key.remove_value("PrismTime");
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn apply_startup_registry(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_live_status(state: State<'_, Arc<SharedAppState>>) -> LiveStatusDto {
    let settings = state.settings.lock().unwrap().clone();
    let runtime = state.runtime.lock().unwrap().clone();
    let catalog = state.catalog.lock().unwrap();
    let today = Local::now().date_naive();
    let mut today_sessions = state.storage.load_sessions_for_date(today);
    if let Some(ref active) = runtime.active_session {
        today_sessions.push(active.clone());
    }
    let today_total_secs: i64 = today_sessions
        .iter()
        .filter(|s| catalog.get(&s.app_id).map(|m| !m.ignored).unwrap_or(true))
        .map(|s| s.duration_secs.max(0))
        .sum();

    let (app_id, app_name, cat, cat_label, icon, web_site, web_domain, sess_secs) =
        if let Some(ref active) = runtime.active_session {
            let meta = catalog.get(&active.app_id);
            (
                Some(active.app_id.clone()),
                meta.map(|m| m.display_name.clone()),
                meta.map(|m| m.category),
                meta.map(|m| m.category.label().to_string()),
                meta.and_then(|m| m.icon_data_url.clone()),
                active.web_site_name.clone(),
                active.web_domain.clone(),
                active.duration_secs,
            )
        } else {
            (None, None, None, None, None, None, None, 0)
        };

    LiveStatusDto {
        tracking_enabled: settings.tracking_enabled,
        state: runtime.state_code,
        current_app_id: app_id,
        current_app_name: app_name,
        current_category: cat,
        current_category_label: cat_label,
        current_icon_data_url: icon,
        current_web_site: web_site,
        current_web_domain: web_domain,
        current_session_secs: sess_secs,
        today_total_secs,
        idle_secs: runtime.idle_secs,
        catalog_count: catalog.values().filter(|m| !m.ignored).count(),
        data_dir: state.storage.root_dir.to_string_lossy().to_string(),
    }
}

#[tauri::command]
pub fn get_dashboard_analytics(
    range: String,
    date: Option<String>,
    demo_mode: bool,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<DashboardAnalyticsDto, String> {
    let today = Local::now().date_naive();
    let selected_date = date
        .as_deref()
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap_or(today);

    let real_catalog = state.catalog.lock().unwrap().clone();
    let has_real_data = state.storage.has_any_real_sessions()
        || state
            .runtime
            .lock()
            .unwrap()
            .active_session
            .as_ref()
            .is_some_and(|s| s.duration_secs >= 2);

    if demo_mode {
        let demo = generate_demo_dataset(&real_catalog, today);
        return Ok(build_dashboard_analytics(
            &range,
            selected_date,
            today,
            &demo.sessions_by_date,
            &demo.catalog,
            true,
            has_real_data,
        ));
    }

    let lookback_days = if range == "30d" { 60 } else { 15 };
    let dates: Vec<NaiveDate> = (0..lookback_days)
        .map(|i| selected_date - Duration::days(i))
        .collect();
    let sessions_by_date = state.build_sessions_map(&dates, true);

    Ok(build_dashboard_analytics(
        &range,
        selected_date,
        today,
        &sessions_by_date,
        &real_catalog,
        false,
        has_real_data,
    ))
}

#[tauri::command]
pub fn get_app_detail(
    app_id: String,
    date: Option<String>,
    demo_mode: bool,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<AppDetailDto, String> {
    let today = Local::now().date_naive();
    let selected_date = date
        .as_deref()
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap_or(today);
    let real_catalog = state.catalog.lock().unwrap().clone();

    if demo_mode {
        let demo = generate_demo_dataset(&real_catalog, today);
        return Ok(build_app_detail(
            &app_id,
            selected_date,
            today,
            &demo.sessions_by_date,
            &demo.catalog,
            true,
        ));
    }

    let dates: Vec<NaiveDate> = (0..7).map(|i| selected_date - Duration::days(i)).collect();
    let sessions_by_date = state.build_sessions_map(&dates, true);
    Ok(build_app_detail(
        &app_id,
        selected_date,
        today,
        &sessions_by_date,
        &real_catalog,
        false,
    ))
}

#[tauri::command]
pub fn get_software_catalog(state: State<'_, Arc<SharedAppState>>) -> Vec<CatalogAppDto> {
    build_catalog_dtos(&state)
}

#[tauri::command]
pub fn rescan_local_software(state: State<'_, Arc<SharedAppState>>) -> Vec<CatalogAppDto> {
    {
        let mut catalog = state.catalog.lock().unwrap();
        let overrides = state.overrides.lock().unwrap();
        scan_installed_software(&mut catalog, true);
        for (aid, ov) in overrides.iter() {
            if let Some(meta) = catalog.get_mut(aid) {
                apply_override(meta, ov);
            }
        }
        let _ = state.storage.save_catalog(&catalog);
    }
    build_catalog_dtos(&state)
}

fn build_catalog_dtos(state: &Arc<SharedAppState>) -> Vec<CatalogAppDto> {
    let today = Local::now().date_naive();
    let mut today_sessions = state.storage.load_sessions_for_date(today);
    if let Some(ref active) = state.runtime.lock().unwrap().active_session {
        today_sessions.push(active.clone());
    }
    let mut today_map: HashMap<String, i64> = HashMap::new();
    for s in today_sessions {
        *today_map.entry(s.app_id).or_insert(0) += s.duration_secs.max(0);
    }

    let catalog = state.catalog.lock().unwrap();
    let mut items: Vec<CatalogAppDto> = catalog
        .values()
        .map(|m| CatalogAppDto {
            app_id: m.app_id.clone(),
            display_name: m.display_name.clone(),
            category: m.category,
            description: m.description.clone(),
            publisher: m.publisher.clone(),
            icon_data_url: m.icon_data_url.clone(),
            process_names: m.process_names.clone(),
            source: m.source.clone(),
            installed: m.installed,
            ignored: m.ignored,
            today_secs: *today_map.get(&m.app_id).unwrap_or(&0),
            last_seen_ts: m.last_seen_ts,
            install_hint: m.exe_paths.first().cloned(),
        })
        .collect();

    items.sort_by(|a, b| {
        b.today_secs
            .cmp(&a.today_secs)
            .then_with(|| a.ignored.cmp(&b.ignored))
            .then_with(|| a.display_name.cmp(&b.display_name))
    });
    items
}

#[tauri::command]
pub fn update_app_override(
    input: AppOverrideInput,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<CatalogAppDto, String> {
    let ov = AppOverride {
        app_id: input.app_id.clone(),
        display_name: input.display_name,
        category: input.category,
        description: input.description,
        ignored: input.ignored,
    };

    {
        let mut overrides = state.overrides.lock().unwrap();
        overrides.insert(input.app_id.clone(), ov.clone());
        state.storage.save_overrides(&overrides)?;
    }

    let updated_meta = {
        let mut catalog = state.catalog.lock().unwrap();
        let meta = catalog
            .get_mut(&input.app_id)
            .ok_or_else(|| "未找到该软件记录".to_string())?;
        apply_override(meta, &ov);
        let cloned = meta.clone();
        state.storage.save_catalog(&catalog)?;
        cloned
    };

    Ok(CatalogAppDto {
        app_id: updated_meta.app_id,
        display_name: updated_meta.display_name,
        category: updated_meta.category,
        description: updated_meta.description,
        publisher: updated_meta.publisher,
        icon_data_url: updated_meta.icon_data_url,
        process_names: updated_meta.process_names,
        source: updated_meta.source,
        installed: updated_meta.installed,
        ignored: updated_meta.ignored,
        today_secs: 0,
        last_seen_ts: updated_meta.last_seen_ts,
        install_hint: updated_meta.exe_paths.first().cloned(),
    })
}

#[tauri::command]
pub fn get_settings(state: State<'_, Arc<SharedAppState>>) -> AppSettings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
pub fn update_settings(
    settings: AppSettings,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<AppSettings, String> {
    let mut sanitized = settings;
    sanitized.idle_threshold_secs = sanitized.idle_threshold_secs.clamp(30, 3600);
    if sanitized.theme != "light" && sanitized.theme != "dark" {
        sanitized.theme = "light".to_string();
    }
    let _ = apply_startup_registry(sanitized.launch_at_startup);
    state.storage.save_settings(&sanitized)?;
    *state.settings.lock().unwrap() = sanitized.clone();
    Ok(sanitized)
}

#[tauri::command]
pub fn toggle_tracking(
    enabled: bool,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<LiveStatusDto, String> {
    {
        let mut settings = state.settings.lock().unwrap();
        settings.tracking_enabled = enabled;
        state.storage.save_settings(&settings)?;
    }
    if !enabled {
        let mut rt = state.runtime.lock().unwrap();
        if let Some(session) = rt.active_session.take() {
            let _ = state.storage.append_session(&session);
        }
        state.storage.save_active_checkpoint(None);
        rt.state_code = "paused".to_string();
    } else {
        let mut rt = state.runtime.lock().unwrap();
        rt.state_code = "active".to_string();
    }
    Ok(get_live_status(state))
}

#[tauri::command]
pub fn export_usage_report(
    range: String,
    format: String,
    demo_mode: bool,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<ExportResultDto, String> {
    let analytics = get_dashboard_analytics(range.clone(), None, demo_mode, state.clone())?;
    let stamp = Local::now().format("%Y%m%d-%H%M%S");
    let ext = if format == "json" { "json" } else { "csv" };
    let filename = format!("prismtime-report-{range}-{stamp}.{ext}");

    let (content, count) = if ext == "json" {
        let json = serde_json::to_string_pretty(&analytics).map_err(|e| e.to_string())?;
        (json, analytics.recent_sessions.len())
    } else {
        let mut csv = String::from("\u{FEFF}日期,开始时间,结束时间,持续秒数,软件名称,进程名,类别,访问站点,站点域名,窗口摘要\n");
        for ev in &analytics.recent_sessions {
            let title = ev
                .sample_title
                .as_deref()
                .unwrap_or("")
                .replace('"', "\"\"");
            let name = ev.display_name.replace('"', "\"\"");
            let proc_name = ev
                .process_name
                .as_deref()
                .unwrap_or("")
                .replace('"', "\"\"");
            let web_name = ev
                .web_site_name
                .as_deref()
                .unwrap_or("")
                .replace('"', "\"\"");
            let web_domain = ev
                .web_domain
                .as_deref()
                .unwrap_or("")
                .replace('"', "\"\"");
            csv.push_str(&format!(
                "{},{},{},{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                ev.date,
                ev.start_time_label,
                ev.end_time_label,
                ev.duration_secs,
                name,
                proc_name,
                ev.category.label(),
                web_name,
                web_domain,
                title
            ));
        }
        (csv, analytics.recent_sessions.len())
    };

    let path = state.storage.export_file(&filename, &content)?;
    Ok(ExportResultDto {
        file_path: path.to_string_lossy().to_string(),
        record_count: count,
        format: ext.to_string(),
    })
}

#[tauri::command]
pub fn window_minimize(app: AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.minimize();
    }
}

#[tauri::command]
pub fn window_toggle_maximize(app: AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let is_max = win.is_maximized().unwrap_or(false);
        if is_max {
            let _ = win.unmaximize();
        } else {
            let _ = win.maximize();
        }
    }
}

#[tauri::command]
pub fn window_close_or_hide(app: AppHandle, state: State<'_, Arc<SharedAppState>>) {
    let hide_to_tray = state.settings.lock().unwrap().minimize_to_tray_on_close;
    if let Some(win) = app.get_webview_window("main") {
        if hide_to_tray {
            let _ = win.hide();
        } else {
            *state.allow_exit.lock().unwrap() = true;
            app.exit(0);
        }
    }
}
