use crate::analytics::{build_app_detail, build_dashboard_analytics};
use crate::catalog::{apply_override, normalize_process_token, slugify_app_id};
use crate::demo::generate_demo_dataset;
use crate::models::{
    AppDetailDto, AppMetadata, AppOverride, AppOverrideInput, AppSettings, CatalogAppDto,
    CustomAppRule, CustomAppRuleInput, DashboardAnalyticsDto, ExportResultDto, LiveStatusDto,
    SyncStatusDto, UsageSession,
};
use crate::scanner::scan_installed_software;
use crate::storage::Storage;
use crate::sync;
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
    pub custom_rules: Mutex<Vec<CustomAppRule>>,
    pub sync_status: Mutex<SyncStatusDto>,
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

pub fn sync_custom_rule_into_catalog(
    catalog: &mut HashMap<String, AppMetadata>,
    rule: &CustomAppRule,
) {
    let procs: Vec<String> = rule
        .process_names
        .iter()
        .map(|p| normalize_process_token(p))
        .filter(|p| !p.is_empty())
        .collect();
    let entry = catalog
        .entry(rule.app_id.clone())
        .or_insert_with(|| AppMetadata {
            app_id: rule.app_id.clone(),
            display_name: rule.display_name.clone(),
            category: rule.category,
            description: if rule.description.trim().is_empty() {
                format!("用户自定义规则 · {}", rule.category.label())
            } else {
                rule.description.clone()
            },
            publisher: if rule.publisher.trim().is_empty() {
                "用户自定义".to_string()
            } else {
                rule.publisher.clone()
            },
            icon_data_url: rule.custom_icon_data_url.clone(),
            exe_paths: Vec::new(),
            process_names: procs.clone(),
            source: "custom".to_string(),
            installed: true,
            ignored: rule.ignored,
            last_seen_ts: None,
        });
    entry.display_name = rule.display_name.clone();
    entry.category = rule.category;
    if !rule.description.trim().is_empty() {
        entry.description = rule.description.clone();
    }
    if !rule.publisher.trim().is_empty() {
        entry.publisher = rule.publisher.clone();
    }
    if let Some(ref ic) = rule.custom_icon_data_url {
        if ic.starts_with("data:image/") {
            entry.icon_data_url = Some(ic.clone());
        }
    }
    entry.process_names = procs;
    entry.ignored = rule.ignored;
    entry.source = "custom".to_string();
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
        let custom_rules = state.custom_rules.lock().unwrap();
        scan_installed_software(&mut catalog, true);
        for rule in custom_rules.iter() {
            sync_custom_rule_into_catalog(&mut catalog, rule);
        }
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
    let overrides = state.overrides.lock().unwrap();
    let mut items: Vec<CatalogAppDto> = catalog
        .values()
        .map(|m| {
            let ov = overrides.get(&m.app_id);
            CatalogAppDto {
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
                has_override: ov.is_some(),
                extra_process_names: ov.map(|o| o.extra_process_names.clone()).unwrap_or_default(),
                idle_multiplier: ov.and_then(|o| o.idle_multiplier),
                treat_as_browser: ov.and_then(|o| o.treat_as_browser),
                merge_into_app_id: ov.and_then(|o| o.merge_into_app_id.clone()),
                custom_tag: ov.and_then(|o| o.custom_tag.clone()),
            }
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
    let extra_procs: Vec<String> = input
        .extra_process_names
        .iter()
        .map(|p| normalize_process_token(p))
        .filter(|p| !p.is_empty())
        .collect();

    let ov = AppOverride {
        app_id: input.app_id.clone(),
        display_name: input.display_name,
        category: input.category,
        description: input.description,
        ignored: input.ignored,
        extra_process_names: extra_procs.clone(),
        idle_multiplier: input.idle_multiplier,
        treat_as_browser: input.treat_as_browser,
        merge_into_app_id: input.merge_into_app_id.filter(|s| !s.trim().is_empty()),
        custom_tag: input.custom_tag.filter(|s| !s.trim().is_empty()),
        custom_icon_data_url: input.custom_icon_data_url.filter(|s| !s.trim().is_empty()),
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
        has_override: true,
        extra_process_names: ov.extra_process_names,
        idle_multiplier: ov.idle_multiplier,
        treat_as_browser: ov.treat_as_browser,
        merge_into_app_id: ov.merge_into_app_id,
        custom_tag: ov.custom_tag,
    })
}

#[tauri::command]
pub fn reset_app_override(
    app_id: String,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<Vec<CatalogAppDto>, String> {
    {
        let mut overrides = state.overrides.lock().unwrap();
        overrides.remove(&app_id);
        state.storage.save_overrides(&overrides)?;
    }
    Ok(rescan_local_software(state))
}

#[tauri::command]
pub fn get_custom_app_rules(state: State<'_, Arc<SharedAppState>>) -> Vec<CustomAppRule> {
    state.custom_rules.lock().unwrap().clone()
}

#[tauri::command]
pub fn save_custom_app_rule(
    input: CustomAppRuleInput,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<Vec<CustomAppRule>, String> {
    let display_name = input.display_name.trim().to_string();
    if display_name.is_empty() {
        return Err("请填写自定义软件名称".to_string());
    }
    let process_names: Vec<String> = input
        .process_names
        .iter()
        .map(|p| normalize_process_token(p))
        .filter(|p| !p.is_empty())
        .collect();
    let path_contains: Vec<String> = input
        .path_contains
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let title_contains: Vec<String> = input
        .title_contains
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if process_names.is_empty() && path_contains.is_empty() && title_contains.is_empty() {
        return Err("请至少填写一个进程名、路径关键词或窗口标题关键词".to_string());
    }

    let now_ts = Local::now().timestamp();
    let rule_id = input
        .rule_id
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| format!("rule-{now_ts}"));
    let slug = slugify_app_id(&display_name).replace("scanned.", "");
    let app_id = format!("custom.{slug}");

    let rule = CustomAppRule {
        rule_id: rule_id.clone(),
        app_id,
        display_name,
        category: input.category,
        description: input.description.unwrap_or_default().trim().to_string(),
        publisher: input
            .publisher
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "用户自定义".to_string()),
        process_names,
        path_contains,
        title_contains,
        ignored: input.ignored,
        treat_as_browser: input.treat_as_browser,
        idle_multiplier: input.idle_multiplier.min(10),
        merge_into_app_id: input.merge_into_app_id.filter(|s| !s.trim().is_empty()),
        custom_tag: input.custom_tag.filter(|s| !s.trim().is_empty()),
        custom_icon_data_url: input.custom_icon_data_url.filter(|s| !s.trim().is_empty()),
    };

    let rules_snapshot = {
        let mut rules = state.custom_rules.lock().unwrap();
        if let Some(pos) = rules.iter().position(|r| r.rule_id == rule_id) {
            rules[pos] = rule.clone();
        } else {
            rules.push(rule.clone());
        }
        state.storage.save_custom_rules(&rules)?;
        rules.clone()
    };

    {
        let mut catalog = state.catalog.lock().unwrap();
        sync_custom_rule_into_catalog(&mut catalog, &rule);
        let _ = state.storage.save_catalog(&catalog);
    }

    Ok(rules_snapshot)
}

#[tauri::command]
pub fn delete_custom_app_rule(
    rule_id: String,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<Vec<CustomAppRule>, String> {
    let mut removed_app_id: Option<String> = None;
    let rules_snapshot = {
        let mut rules = state.custom_rules.lock().unwrap();
        if let Some(pos) = rules.iter().position(|r| r.rule_id == rule_id) {
            removed_app_id = Some(rules[pos].app_id.clone());
            rules.remove(pos);
        }
        state.storage.save_custom_rules(&rules)?;
        rules.clone()
    };

    if let Some(aid) = removed_app_id {
        let mut catalog = state.catalog.lock().unwrap();
        if catalog.get(&aid).is_some_and(|m| m.source == "custom") {
            catalog.remove(&aid);
            let _ = state.storage.save_catalog(&catalog);
        }
    }

    Ok(rules_snapshot)
}

#[tauri::command]
pub fn get_sync_status(state: State<'_, Arc<SharedAppState>>) -> SyncStatusDto {
    state.sync_status.lock().unwrap().clone()
}

#[tauri::command]
pub async fn test_sync_connection(
    provider: String,
    settings: AppSettings,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || sync::test_connection(&provider, &settings.sync))
        .await
        .map_err(|e| format!("测试同步任务异常: {e}"))?
}

#[tauri::command]
pub async fn run_sync_now(
    provider: Option<String>,
    settings: Option<AppSettings>,
    state: State<'_, Arc<SharedAppState>>,
) -> Result<SyncStatusDto, String> {
    let shared = Arc::clone(state.inner());
    let sync_cfg = if let Some(s) = settings {
        let mut current = shared.settings.lock().unwrap();
        *current = s.clone();
        let _ = shared.storage.save_settings(&s);
        s.sync
    } else {
        shared.settings.lock().unwrap().sync.clone()
    };
    let active_provider = provider
        .filter(|p| !p.trim().is_empty())
        .unwrap_or_else(|| sync_cfg.provider.clone());

    {
        let mut st = shared.sync_status.lock().unwrap();
        st.status = "syncing".to_string();
        st.provider = Some(active_provider.clone());
        st.message = "正在执行双向同步...".to_string();
    }

    // Flush active session snapshot so today's latest usage is included
    let pending_copy = shared.runtime.lock().unwrap().active_session.clone();
    if let Some(ref s) = pending_copy {
        if s.duration_secs >= 2 && s.end_ts > s.start_ts {
            let _ = shared.storage.append_session(s);
        }
    }

    let result = tauri::async_runtime::spawn_blocking(move || {
        let mut ov = shared.overrides.lock().unwrap().clone();
        let mut rules = shared.custom_rules.lock().unwrap().clone();
        let res = sync::run_bidirectional_sync(
            &active_provider,
            &sync_cfg,
            &shared.storage,
            &mut ov,
            &mut rules,
        );

        match res {
            Ok(status_dto) => {
                *shared.overrides.lock().unwrap() = ov.clone();
                *shared.custom_rules.lock().unwrap() = rules.clone();
                {
                    let mut catalog = shared.catalog.lock().unwrap();
                    for rule in &rules {
                        sync_custom_rule_into_catalog(&mut catalog, rule);
                    }
                    for (aid, o) in &ov {
                        if let Some(meta) = catalog.get_mut(aid) {
                            apply_override(meta, o);
                        }
                    }
                    let _ = shared.storage.save_catalog(&catalog);
                }
                *shared.sync_status.lock().unwrap() = status_dto.clone();
                Ok(status_dto)
            }
            Err(err_msg) => {
                let err_dto = SyncStatusDto {
                    status: "error".to_string(),
                    provider: Some(active_provider),
                    last_sync_ts: Some(Local::now().timestamp()),
                    message: err_msg.clone(),
                    uploaded_files: 0,
                    downloaded_files: 0,
                    merged_sessions: 0,
                };
                *shared.sync_status.lock().unwrap() = err_dto;
                Err(err_msg)
            }
        }
    })
    .await
    .map_err(|e| format!("同步线程执行异常: {e}"))?;

    result
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
    sanitized.sync.auto_sync_interval_mins = sanitized.sync.auto_sync_interval_mins.clamp(5, 1440);
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
