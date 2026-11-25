use crate::models::{
    AppOverride, CustomAppRule, GitHubSyncConfig, SyncConfig, SyncStatusDto, UsageSession,
    WebDavConfig,
};
use crate::storage::Storage;
use crate::win_http::http_request;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncManifest {
    pub schema_version: u32,
    pub updated_at: i64,
    pub last_device: String,
    #[serde(default)]
    pub session_dates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncOverridesBundle {
    pub updated_at: i64,
    #[serde(default)]
    pub overrides: HashMap<String, AppOverride>,
    #[serde(default)]
    pub custom_rules: Vec<CustomAppRule>,
}

/// Lossless merge of local and remote sessions for a single day.
/// Deduplicates by session `id`, and also avoids exact `(app_id, start_ts, end_ts, web_site_id)` duplicates.
pub fn merge_session_lists(
    local: &[UsageSession],
    remote: &[UsageSession],
) -> (Vec<UsageSession>, usize) {
    let mut by_id: BTreeMap<String, UsageSession> = BTreeMap::new();
    let mut seen_keys: BTreeSet<(String, i64, i64, Option<String>)> = BTreeSet::new();

    for s in local {
        if s.duration_secs <= 0 || s.end_ts <= s.start_ts {
            continue;
        }
        seen_keys.insert((
            s.app_id.clone(),
            s.start_ts,
            s.end_ts,
            s.web_site_id.clone(),
        ));
        by_id.insert(s.id.clone(), s.clone());
    }

    let mut added_from_remote = 0usize;
    for s in remote {
        if s.duration_secs <= 0 || s.end_ts <= s.start_ts {
            continue;
        }
        let key = (
            s.app_id.clone(),
            s.start_ts,
            s.end_ts,
            s.web_site_id.clone(),
        );
        if let Some(existing) = by_id.get(&s.id) {
            if s.duration_secs > existing.duration_secs {
                by_id.insert(s.id.clone(), s.clone());
            }
            continue;
        }
        if seen_keys.contains(&key) {
            continue;
        }
        seen_keys.insert(key);
        by_id.insert(s.id.clone(), s.clone());
        added_from_remote += 1;
    }

    let mut merged: Vec<UsageSession> = by_id.into_values().collect();
    merged.sort_by(|a, b| a.start_ts.cmp(&b.start_ts).then_with(|| a.id.cmp(&b.id)));
    (merged, added_from_remote)
}

pub fn serialize_sessions_jsonl(sessions: &[UsageSession]) -> String {
    let mut out = String::new();
    for s in sessions {
        if let Ok(line) = serde_json::to_string(s) {
            out.push_str(&line);
            out.push('\n');
        }
    }
    out
}

pub fn parse_sessions_jsonl(content: &str) -> Vec<UsageSession> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }
            serde_json::from_str::<UsageSession>(trimmed).ok()
        })
        .filter(|s| s.duration_secs > 0 && s.end_ts > s.start_ts)
        .collect()
}

pub fn test_connection(provider: &str, config: &SyncConfig) -> Result<String, String> {
    match provider {
        "webdav" => test_webdav_connection(&config.webdav),
        "github" => test_github_connection(&config.github),
        _ => Err("请先选择 WebDAV 或 GitHub 同步方式".to_string()),
    }
}

pub fn run_bidirectional_sync(
    provider: &str,
    config: &SyncConfig,
    storage: &Storage,
    local_overrides: &mut HashMap<String, AppOverride>,
    local_custom_rules: &mut Vec<CustomAppRule>,
) -> Result<SyncStatusDto, String> {
    match provider {
        "webdav" => sync_webdav(config, storage, local_overrides, local_custom_rules),
        "github" => sync_github(config, storage, local_overrides, local_custom_rules),
        _ => Err("尚未启用任何同步服务，请在设置中选择 WebDAV 或 GitHub".to_string()),
    }
}

// ============================================================================
// WebDAV Provider
// ============================================================================

fn webdav_auth_header(cfg: &WebDavConfig) -> String {
    let raw = format!("{}:{}", cfg.username.trim(), cfg.password);
    format!("Basic {}", BASE64.encode(raw.as_bytes()))
}

fn webdav_join(base: &str, parts: &[&str]) -> String {
    let mut url = base.trim().trim_end_matches('/').to_string();
    for part in parts {
        let p = part.trim().trim_matches('/');
        if !p.is_empty() {
            url.push('/');
            url.push_str(p);
        }
    }
    url
}

fn ensure_webdav_dir(cfg: &WebDavConfig, auth: &str, rel_parts: &[&str]) -> Result<(), String> {
    let mut acc: Vec<&str> = Vec::new();
    for p in rel_parts {
        if p.trim().is_empty() {
            continue;
        }
        acc.push(p);
        let dir_url = format!("{}/", webdav_join(&cfg.server_url, &acc));
        let headers = [("Authorization", auth.to_string())];
        let resp = http_request("MKCOL", &dir_url, &headers, None)?;
        // 201 Created, 405 Already Exists, 301/302/200/204 OK
        if !matches!(resp.status, 200 | 201 | 204 | 301 | 302 | 405 | 409) {
            return Err(format!(
                "创建 WebDAV 目录失败 ({dir_url})：HTTP {}",
                resp.status
            ));
        }
    }
    Ok(())
}

fn test_webdav_connection(cfg: &WebDavConfig) -> Result<String, String> {
    if cfg.server_url.trim().is_empty() {
        return Err("请填写 WebDAV 服务器地址（如 https://dav.jianguoyun.com/dav/）".to_string());
    }
    let auth = webdav_auth_header(cfg);
    let url = format!("{}/", cfg.server_url.trim().trim_end_matches('/'));
    let headers = [
        ("Authorization", auth.clone()),
        ("Depth", "0".to_string()),
    ];
    let resp = http_request("PROPFIND", &url, &headers, None)?;
    match resp.status {
        200 | 207 | 204 => {
            if !cfg.remote_dir.trim().is_empty() {
                let _ = ensure_webdav_dir(cfg, &auth, &[cfg.remote_dir.trim()]);
            }
            Ok(format!(
                "WebDAV 连接成功（HTTP {}），远端目录 /{} 已就绪",
                resp.status,
                cfg.remote_dir.trim().trim_matches('/')
            ))
        }
        401 | 403 => Err("WebDAV 认证失败：请检查用户名与应用专用密码".to_string()),
        404 => Err("WebDAV 路径不存在（HTTP 404），请确认基础 URL 是否正确".to_string()),
        other => Err(format!("WebDAV 服务器返回异常状态码 HTTP {other}")),
    }
}

fn webdav_get_text(url: &str, auth: &str) -> Result<Option<String>, String> {
    let headers = [("Authorization", auth.to_string())];
    let resp = http_request("GET", url, &headers, None)?;
    match resp.status {
        200..=299 => Ok(Some(resp.text())),
        404 => Ok(None),
        401 | 403 => Err("WebDAV 认证失败，请检查账号或应用密码".to_string()),
        other => Err(format!("下载 WebDAV 文件失败（HTTP {other}）")),
    }
}

fn webdav_put_text(url: &str, auth: &str, content: &str) -> Result<(), String> {
    let headers = [
        ("Authorization", auth.to_string()),
        ("Content-Type", "application/json; charset=utf-8".to_string()),
    ];
    let resp = http_request("PUT", url, &headers, Some(content.as_bytes()))?;
    if matches!(resp.status, 200 | 201 | 204) {
        Ok(())
    } else {
        Err(format!("上传 WebDAV 文件失败（HTTP {}）", resp.status))
    }
}

fn sync_webdav(
    config: &SyncConfig,
    storage: &Storage,
    local_overrides: &mut HashMap<String, AppOverride>,
    local_custom_rules: &mut Vec<CustomAppRule>,
) -> Result<SyncStatusDto, String> {
    let cfg = &config.webdav;
    if cfg.server_url.trim().is_empty() {
        return Err("请先填写 WebDAV 服务器地址".to_string());
    }
    let auth = webdav_auth_header(cfg);
    let root_dir = cfg.remote_dir.trim().trim_matches('/');
    if root_dir.is_empty() {
        ensure_webdav_dir(cfg, &auth, &["sessions"])?;
    } else {
        ensure_webdav_dir(cfg, &auth, &[root_dir, "sessions"])?;
    }

    let manifest_url = webdav_join(&cfg.server_url, &[root_dir, "sync_manifest.json"]);
    let remote_manifest: SyncManifest = webdav_get_text(&manifest_url, &auth)?
        .and_then(|txt| serde_json::from_str(&txt).ok())
        .unwrap_or_default();

    let local_dates = storage.list_session_dates();
    let mut all_dates: BTreeSet<String> = local_dates.into_iter().collect();
    for d in &remote_manifest.session_dates {
        all_dates.insert(d.clone());
    }

    let mut uploaded_files = 0usize;
    let mut downloaded_files = 0usize;
    let mut merged_sessions = 0usize;

    for date_str in &all_dates {
        let local_sessions = storage.load_sessions_by_date_str(date_str);
        let remote_url = webdav_join(
            &cfg.server_url,
            &[root_dir, "sessions", &format!("{date_str}.jsonl")],
        );
        let remote_text = webdav_get_text(&remote_url, &auth)?;
        let remote_sessions = remote_text
            .as_deref()
            .map(parse_sessions_jsonl)
            .unwrap_or_default();

        if remote_text.is_some() {
            downloaded_files += 1;
        }

        let (merged, added_from_remote) = merge_session_lists(&local_sessions, &remote_sessions);
        merged_sessions += added_from_remote;

        if added_from_remote > 0 {
            storage.write_sessions_for_date_str(date_str, &merged)?;
        }

        let merged_jsonl = serialize_sessions_jsonl(&merged);
        let remote_normalized = serialize_sessions_jsonl(&remote_sessions);
        if !merged.is_empty() && (remote_text.is_none() || merged_jsonl != remote_normalized) {
            webdav_put_text(&remote_url, &auth, &merged_jsonl)?;
            uploaded_files += 1;
        }
    }

    if config.sync_overrides {
        let ov_url = webdav_join(&cfg.server_url, &[root_dir, "custom_profile.json"]);
        if let Some(txt) = webdav_get_text(&ov_url, &auth)? {
            if let Ok(remote_bundle) = serde_json::from_str::<SyncOverridesBundle>(&txt) {
                downloaded_files += 1;
                merge_override_bundles(local_overrides, local_custom_rules, &remote_bundle);
                storage.save_overrides(local_overrides)?;
                storage.save_custom_rules(local_custom_rules)?;
            }
        }
        let next_bundle = SyncOverridesBundle {
            updated_at: Local::now().timestamp(),
            overrides: local_overrides.clone(),
            custom_rules: local_custom_rules.clone(),
        };
        let body = serde_json::to_string_pretty(&next_bundle).map_err(|e| e.to_string())?;
        webdav_put_text(&ov_url, &auth, &body)?;
        uploaded_files += 1;
    }

    let now_ts = Local::now().timestamp();
    let next_manifest = SyncManifest {
        schema_version: 1,
        updated_at: now_ts,
        last_device: config.device_name.clone(),
        session_dates: all_dates.into_iter().collect(),
    };
    let manifest_json = serde_json::to_string_pretty(&next_manifest).map_err(|e| e.to_string())?;
    webdav_put_text(&manifest_url, &auth, &manifest_json)?;

    Ok(SyncStatusDto {
        status: "success".to_string(),
        provider: Some("webdav".to_string()),
        last_sync_ts: Some(now_ts),
        message: format!(
            "WebDAV 双向同步完成：上传 {uploaded_files} 项，检查/拉取 {downloaded_files} 项，合并新增 {merged_sessions} 条时段"
        ),
        uploaded_files,
        downloaded_files,
        merged_sessions,
    })
}

// ============================================================================
// GitHub Repo Contents REST API Provider
// ============================================================================

#[derive(Debug, Deserialize)]
struct GitHubContentResp {
    sha: String,
    content: Option<String>,
}

fn github_headers(token: &str) -> Vec<(&'static str, String)> {
    vec![
        ("Authorization", format!("Bearer {}", token.trim())),
        ("Accept", "application/vnd.github+json".to_string()),
        ("X-GitHub-Api-Version", "2022-11-28".to_string()),
        ("Content-Type", "application/json; charset=utf-8".to_string()),
    ]
}

fn github_path(prefix: &str, rel: &str) -> String {
    let p = prefix.trim().trim_matches('/');
    let r = rel.trim().trim_matches('/');
    if p.is_empty() {
        r.to_string()
    } else {
        format!("{p}/{r}")
    }
}

fn test_github_connection(cfg: &GitHubSyncConfig) -> Result<String, String> {
    if cfg.owner.trim().is_empty() || cfg.repo.trim().is_empty() {
        return Err("请填写 GitHub 仓库拥有者（Owner）与仓库名（Repo）".to_string());
    }
    if cfg.token.trim().is_empty() {
        return Err("请填写具有仓库 Contents 读写权限的 GitHub Personal Access Token".to_string());
    }
    let url = format!(
        "https://api.github.com/repos/{}/{}",
        cfg.owner.trim(),
        cfg.repo.trim()
    );
    let headers = github_headers(&cfg.token);
    let resp = http_request("GET", &url, &headers, None)?;
    match resp.status {
        200 => {
            let v: serde_json::Value = serde_json::from_slice(&resp.body).unwrap_or_default();
            let private = v.get("private").and_then(|x| x.as_bool()).unwrap_or(false);
            let vis = if private { "私有仓库" } else { "公开仓库（建议改用私有仓库）" };
            Ok(format!(
                "GitHub 仓库 {}/{} 连接成功（{vis}，分支 {}）",
                cfg.owner.trim(),
                cfg.repo.trim(),
                cfg.branch.trim()
            ))
        }
        401 => Err("GitHub Token 无效或已过期（HTTP 401）".to_string()),
        403 => Err("GitHub 权限不足或触发速率限制（HTTP 403）".to_string()),
        404 => Err("未找到目标仓库或 Token 无权访问（HTTP 404）".to_string()),
        other => Err(format!("GitHub API 返回异常状态码 HTTP {other}")),
    }
}

fn github_get_file(cfg: &GitHubSyncConfig, rel_path: &str) -> Result<Option<(String, String)>, String> {
    let full_path = github_path(&cfg.path_prefix, rel_path);
    let branch = if cfg.branch.trim().is_empty() {
        "main"
    } else {
        cfg.branch.trim()
    };
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}?ref={}",
        cfg.owner.trim(),
        cfg.repo.trim(),
        full_path,
        branch
    );
    let headers = github_headers(&cfg.token);
    let resp = http_request("GET", &url, &headers, None)?;
    match resp.status {
        200 => {
            let parsed: GitHubContentResp = serde_json::from_slice(&resp.body)
                .map_err(|e| format!("解析 GitHub 文件响应失败: {e}"))?;
            let raw_b64 = parsed.content.unwrap_or_default().replace(['\n', '\r'], "");
            let bytes = BASE64
                .decode(raw_b64.as_bytes())
                .map_err(|e| format!("解码 GitHub 文件内容失败: {e}"))?;
            let text = String::from_utf8_lossy(&bytes).to_string();
            Ok(Some((text, parsed.sha)))
        }
        404 => Ok(None),
        401 | 403 => Err("GitHub 鉴权失败，请检查 PAT 权限（需 Contents Read & Write）".to_string()),
        other => Err(format!("读取 GitHub 文件 {full_path} 失败（HTTP {other}）")),
    }
}

fn github_put_file(
    cfg: &GitHubSyncConfig,
    rel_path: &str,
    content: &str,
    sha: Option<&str>,
    commit_msg: &str,
) -> Result<(), String> {
    let full_path = github_path(&cfg.path_prefix, rel_path);
    let branch = if cfg.branch.trim().is_empty() {
        "main"
    } else {
        cfg.branch.trim()
    };
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        cfg.owner.trim(),
        cfg.repo.trim(),
        full_path
    );

    let mut payload = serde_json::json!({
        "message": commit_msg,
        "content": BASE64.encode(content.as_bytes()),
        "branch": branch,
    });
    if let Some(s) = sha {
        payload["sha"] = serde_json::Value::String(s.to_string());
    }

    let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let headers = github_headers(&cfg.token);
    let resp = http_request("PUT", &url, &headers, Some(body.as_bytes()))?;
    if matches!(resp.status, 200 | 201) {
        Ok(())
    } else {
        Err(format!(
            "提交 GitHub 文件 {full_path} 失败（HTTP {}）：{}",
            resp.status,
            resp.text().chars().take(160).collect::<String>()
        ))
    }
}

fn sync_github(
    config: &SyncConfig,
    storage: &Storage,
    local_overrides: &mut HashMap<String, AppOverride>,
    local_custom_rules: &mut Vec<CustomAppRule>,
) -> Result<SyncStatusDto, String> {
    let cfg = &config.github;
    if cfg.owner.trim().is_empty() || cfg.repo.trim().is_empty() || cfg.token.trim().is_empty() {
        return Err("请先完整填写 GitHub Owner、仓库名与 Personal Access Token".to_string());
    }

    let manifest_remote = github_get_file(cfg, "sync_manifest.json")?;
    let (remote_manifest, manifest_sha): (SyncManifest, Option<String>) = match manifest_remote {
        Some((txt, sha)) => (serde_json::from_str(&txt).unwrap_or_default(), Some(sha)),
        None => (SyncManifest::default(), None),
    };

    let local_dates = storage.list_session_dates();
    let mut all_dates: BTreeSet<String> = local_dates.into_iter().collect();
    for d in &remote_manifest.session_dates {
        all_dates.insert(d.clone());
    }

    let mut uploaded_files = 0usize;
    let mut downloaded_files = 0usize;
    let mut merged_sessions = 0usize;
    let device = config.device_name.trim();

    for date_str in &all_dates {
        let local_sessions = storage.load_sessions_by_date_str(date_str);
        let rel = format!("sessions/{date_str}.jsonl");
        let remote_file = github_get_file(cfg, &rel)?;

        let (remote_sessions, remote_sha) = match remote_file {
            Some((txt, sha)) => {
                downloaded_files += 1;
                (parse_sessions_jsonl(&txt), Some(sha))
            }
            None => (Vec::new(), None),
        };

        let (merged, added_from_remote) = merge_session_lists(&local_sessions, &remote_sessions);
        merged_sessions += added_from_remote;

        if added_from_remote > 0 {
            storage.write_sessions_for_date_str(date_str, &merged)?;
        }

        let merged_jsonl = serialize_sessions_jsonl(&merged);
        let remote_normalized = serialize_sessions_jsonl(&remote_sessions);
        if !merged.is_empty() && (remote_sha.is_none() || merged_jsonl != remote_normalized) {
            github_put_file(
                cfg,
                &rel,
                &merged_jsonl,
                remote_sha.as_deref(),
                &format!("chore(prismtime): sync sessions {date_str} from {device}"),
            )?;
            uploaded_files += 1;
        }
    }

    if config.sync_overrides {
        let remote_ov = github_get_file(cfg, "custom_profile.json")?;
        let mut ov_sha: Option<String> = None;
        if let Some((txt, sha)) = remote_ov {
            ov_sha = Some(sha);
            if let Ok(bundle) = serde_json::from_str::<SyncOverridesBundle>(&txt) {
                downloaded_files += 1;
                merge_override_bundles(local_overrides, local_custom_rules, &bundle);
                storage.save_overrides(local_overrides)?;
                storage.save_custom_rules(local_custom_rules)?;
            }
        }
        let next_bundle = SyncOverridesBundle {
            updated_at: Local::now().timestamp(),
            overrides: local_overrides.clone(),
            custom_rules: local_custom_rules.clone(),
        };
        let body = serde_json::to_string_pretty(&next_bundle).map_err(|e| e.to_string())?;
        github_put_file(
            cfg,
            "custom_profile.json",
            &body,
            ov_sha.as_deref(),
            &format!("chore(prismtime): sync custom rules from {device}"),
        )?;
        uploaded_files += 1;
    }

    let now_ts = Local::now().timestamp();
    let next_manifest = SyncManifest {
        schema_version: 1,
        updated_at: now_ts,
        last_device: config.device_name.clone(),
        session_dates: all_dates.into_iter().collect(),
    };
    let manifest_json = serde_json::to_string_pretty(&next_manifest).map_err(|e| e.to_string())?;
    github_put_file(
        cfg,
        "sync_manifest.json",
        &manifest_json,
        manifest_sha.as_deref(),
        &format!("chore(prismtime): update sync manifest from {device}"),
    )?;

    Ok(SyncStatusDto {
        status: "success".to_string(),
        provider: Some("github".to_string()),
        last_sync_ts: Some(now_ts),
        message: format!(
            "GitHub 仓库同步完成：提交 {uploaded_files} 项，检查/拉取 {downloaded_files} 项，合并新增 {merged_sessions} 条时段"
        ),
        uploaded_files,
        downloaded_files,
        merged_sessions,
    })
}

fn merge_override_bundles(
    local_overrides: &mut HashMap<String, AppOverride>,
    local_custom_rules: &mut Vec<CustomAppRule>,
    remote: &SyncOverridesBundle,
) {
    for (aid, ov) in &remote.overrides {
        local_overrides
            .entry(aid.clone())
            .or_insert_with(|| ov.clone());
    }
    for rule in &remote.custom_rules {
        if !local_custom_rules.iter().any(|r| r.rule_id == rule.rule_id) {
            local_custom_rules.push(rule.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_sess(id: &str, app: &str, start: i64, end: i64) -> UsageSession {
        UsageSession {
            id: id.to_string(),
            app_id: app.to_string(),
            process_name: Some("Antigravity".to_string()),
            start_ts: start,
            end_ts: end,
            duration_secs: end - start,
            sample_title: None,
            web_site_id: None,
            web_site_name: None,
            web_domain: None,
            web_category: None,
        }
    }

    #[test]
    fn merge_sessions_deduplicates_and_preserves_both_devices() {
        let local = vec![
            mk_sess("s-1", "google.antigravity", 1000, 1600),
            mk_sess("s-2", "tencent.weixin", 1700, 1900),
        ];
        let remote = vec![
            mk_sess("s-1", "google.antigravity", 1000, 1600), // duplicate id
            mk_sess("s-remote-dup", "tencent.weixin", 1700, 1900), // duplicate tuple
            mk_sess("s-3", "adobe.photoshop", 2000, 2600), // brand new from device B
        ];

        let (merged, added) = merge_session_lists(&local, &remote);
        assert_eq!(added, 1);
        assert_eq!(merged.len(), 3);
        assert_eq!(merged[0].id, "s-1");
        assert_eq!(merged[1].id, "s-2");
        assert_eq!(merged[2].id, "s-3");
    }
}
