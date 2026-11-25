use crate::models::{AppMetadata, AppOverride, AppSettings, CustomAppRule, UsageSession};
use chrono::{Local, NaiveDate, TimeZone};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub struct Storage {
    pub root_dir: PathBuf,
    sessions_dir: PathBuf,
    exports_dir: PathBuf,
    catalog_path: PathBuf,
    overrides_path: PathBuf,
    custom_rules_path: PathBuf,
    settings_path: PathBuf,
    active_session_path: PathBuf,
}

impl Storage {
    pub fn new(custom_root: Option<PathBuf>) -> Self {
        let root_dir = custom_root.unwrap_or_else(|| {
            std::env::var_os("LOCALAPPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(std::env::temp_dir)
                .join("PrismTime")
        });

        let sessions_dir = root_dir.join("sessions");
        let exports_dir = root_dir.join("exports");
        let catalog_path = root_dir.join("catalog.json");
        let overrides_path = root_dir.join("overrides.json");
        let custom_rules_path = root_dir.join("custom_rules.json");
        let settings_path = root_dir.join("settings.json");
        let active_session_path = root_dir.join("active_session.json");

        let _ = fs::create_dir_all(&sessions_dir);
        let _ = fs::create_dir_all(&exports_dir);

        Self {
            root_dir,
            sessions_dir,
            exports_dir,
            catalog_path,
            overrides_path,
            custom_rules_path,
            settings_path,
            active_session_path,
        }
    }

    pub fn load_settings(&self) -> AppSettings {
        read_json_or_default(&self.settings_path)
    }

    pub fn save_settings(&self, settings: &AppSettings) -> Result<(), String> {
        atomic_write_json(&self.settings_path, settings)
    }

    pub fn load_catalog(&self) -> HashMap<String, AppMetadata> {
        read_json_or_default(&self.catalog_path)
    }

    pub fn save_catalog(&self, catalog: &HashMap<String, AppMetadata>) -> Result<(), String> {
        atomic_write_json(&self.catalog_path, catalog)
    }

    pub fn load_overrides(&self) -> HashMap<String, AppOverride> {
        read_json_or_default(&self.overrides_path)
    }

    pub fn save_overrides(&self, overrides: &HashMap<String, AppOverride>) -> Result<(), String> {
        atomic_write_json(&self.overrides_path, overrides)
    }

    pub fn load_custom_rules(&self) -> Vec<CustomAppRule> {
        read_json_or_default(&self.custom_rules_path)
    }

    pub fn save_custom_rules(&self, rules: &[CustomAppRule]) -> Result<(), String> {
        atomic_write_json(&self.custom_rules_path, &rules)
    }

    pub fn list_session_dates(&self) -> Vec<String> {
        let Ok(entries) = fs::read_dir(&self.sessions_dir) else {
            return Vec::new();
        };
        let mut dates: Vec<String> = entries
            .flatten()
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                let stem = name.strip_suffix(".jsonl")?;
                if NaiveDate::parse_from_str(stem, "%Y-%m-%d").is_ok() {
                    Some(stem.to_string())
                } else {
                    None
                }
            })
            .collect();
        dates.sort();
        dates
    }

    pub fn load_sessions_by_date_str(&self, date_str: &str) -> Vec<UsageSession> {
        if let Ok(d) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            self.load_sessions_for_date(d)
        } else {
            Vec::new()
        }
    }

    pub fn write_sessions_for_date_str(
        &self,
        date_str: &str,
        sessions: &[UsageSession],
    ) -> Result<(), String> {
        let path = self.sessions_dir.join(format!("{date_str}.jsonl"));
        let tmp = self.sessions_dir.join(format!("{date_str}.jsonl.tmp"));
        let mut buf = String::new();
        for s in sessions {
            if s.duration_secs > 0 && s.end_ts > s.start_ts {
                let line = serde_json::to_string(s).map_err(|e| e.to_string())?;
                buf.push_str(&line);
                buf.push('\n');
            }
        }
        fs::write(&tmp, buf.as_bytes()).map_err(|e| format!("写入同步会话临时文件失败: {e}"))?;
        if path.exists() {
            let _ = fs::remove_file(&path);
        }
        fs::rename(&tmp, &path).map_err(|e| format!("替换同步会话文件失败: {e}"))?;
        Ok(())
    }

    pub fn save_active_checkpoint(&self, session: Option<&UsageSession>) {
        match session {
            Some(s) if s.end_ts > s.start_ts => {
                let _ = atomic_write_json(&self.active_session_path, s);
            }
            _ => {
                let _ = fs::remove_file(&self.active_session_path);
            }
        }
    }

    pub fn recover_active_checkpoint(&self) -> Option<UsageSession> {
        let data = fs::read_to_string(&self.active_session_path).ok()?;
        let session: UsageSession = serde_json::from_str(&data).ok()?;
        let _ = fs::remove_file(&self.active_session_path);
        if session.duration_secs >= 2 && session.end_ts > session.start_ts {
            let _ = self.append_session(&session);
            Some(session)
        } else {
            None
        }
    }

    pub fn append_session(&self, session: &UsageSession) -> Result<(), String> {
        if session.duration_secs < 2 || session.end_ts <= session.start_ts {
            return Ok(());
        }
        // Split across local midnight boundaries if needed so every session stays in its local date shard
        let parts = split_session_by_local_day(session);
        for (date_str, part) in parts {
            let path = self.sessions_dir.join(format!("{date_str}.jsonl"));
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .map_err(|e| format!("无法写入会话日志: {e}"))?;
            let line = serde_json::to_string(&part).map_err(|e| e.to_string())?;
            writeln!(file, "{line}").map_err(|e| format!("写入会话行失败: {e}"))?;
        }
        Ok(())
    }

    pub fn load_sessions_for_date(&self, date: NaiveDate) -> Vec<UsageSession> {
        let path = self
            .sessions_dir
            .join(format!("{}.jsonl", date.format("%Y-%m-%d")));
        let Ok(file) = File::open(path) else {
            return Vec::new();
        };
        let reader = BufReader::new(file);
        let mut sessions = Vec::new();
        for line in reader.lines().map_while(Result::ok) {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(s) = serde_json::from_str::<UsageSession>(trimmed) {
                if s.duration_secs > 0 && s.end_ts > s.start_ts {
                    sessions.push(s);
                }
            }
        }
        sessions
    }

    #[allow(dead_code)]
    pub fn load_sessions_for_dates(&self, dates: &[NaiveDate]) -> Vec<UsageSession> {
        let mut all = Vec::new();
        for d in dates {
            all.extend(self.load_sessions_for_date(*d));
        }
        all
    }

    pub fn has_any_real_sessions(&self) -> bool {
        let Ok(entries) = fs::read_dir(&self.sessions_dir) else {
            return false;
        };
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() && meta.len() > 10 {
                    return true;
                }
            }
        }
        false
    }

    pub fn export_file(&self, filename: &str, content: &str) -> Result<PathBuf, String> {
        let path = self.exports_dir.join(filename);
        fs::write(&path, content.as_bytes()).map_err(|e| format!("导出文件失败: {e}"))?;
        Ok(path)
    }
}

pub fn split_session_by_local_day(session: &UsageSession) -> Vec<(String, UsageSession)> {
    let mut result = Vec::new();
    let mut cur_start = session.start_ts;
    let end = session.end_ts;
    let mut part_idx = 0usize;

    while cur_start < end {
        let dt = Local
            .timestamp_opt(cur_start, 0)
            .single()
            .unwrap_or_else(Local::now);
        let date = dt.date_naive();
        let next_date = date.succ_opt().unwrap_or(date);
        let next_midnight_naive = next_date.and_hms_opt(0, 0, 0).unwrap();
        let next_midnight_ts = Local
            .from_local_datetime(&next_midnight_naive)
            .earliest()
            .map(|d| d.timestamp())
            .unwrap_or(cur_start + 86400);

        let cur_end = end.min(next_midnight_ts);
        if cur_end > cur_start {
            let id = if part_idx == 0 && cur_end == end {
                session.id.clone()
            } else {
                format!("{}-p{}", session.id, part_idx)
            };
            result.push((
                date.format("%Y-%m-%d").to_string(),
                UsageSession {
                    id,
                    app_id: session.app_id.clone(),
                    process_name: session.process_name.clone(),
                    start_ts: cur_start,
                    end_ts: cur_end,
                    duration_secs: cur_end - cur_start,
                    sample_title: session.sample_title.clone(),
                    web_site_id: session.web_site_id.clone(),
                    web_site_name: session.web_site_name.clone(),
                    web_domain: session.web_domain.clone(),
                    web_category: session.web_category,
                },
            ));
        }
        cur_start = next_midnight_ts;
        part_idx += 1;
        if part_idx > 30 {
            break;
        }
    }

    result
}

fn read_json_or_default<T: serde::de::DeserializeOwned + Default>(path: &Path) -> T {
    let Ok(content) = fs::read_to_string(path) else {
        return T::default();
    };
    serde_json::from_str(&content).unwrap_or_default()
}

fn atomic_write_json<T: serde::Serialize>(path: &Path, data: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let tmp = path.with_extension("json.tmp");
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&tmp, json.as_bytes()).map_err(|e| format!("写入临时文件失败: {e}"))?;
    if path.exists() {
        let _ = fs::remove_file(path);
    }
    fs::rename(&tmp, path).map_err(|e| format!("保存文件失败: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_session_across_midnight_conserves_total_seconds() {
        let d1 = NaiveDate::from_ymd_opt(2026, 11, 24).unwrap();
        let d2 = NaiveDate::from_ymd_opt(2026, 11, 25).unwrap();
        let start_ts = Local
            .from_local_datetime(&d1.and_hms_opt(23, 50, 0).unwrap())
            .single()
            .unwrap()
            .timestamp();
        let end_ts = Local
            .from_local_datetime(&d2.and_hms_opt(0, 15, 0).unwrap())
            .single()
            .unwrap()
            .timestamp();

        let session = UsageSession {
            id: "s-midnight".to_string(),
            app_id: "google.antigravity".to_string(),
            process_name: Some("Antigravity".to_string()),
            start_ts,
            end_ts,
            duration_secs: end_ts - start_ts,
            sample_title: None,
            web_site_id: None,
            web_site_name: None,
            web_domain: None,
            web_category: None,
        };

        let parts = split_session_by_local_day(&session);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].0, "2026-11-24");
        assert_eq!(parts[0].1.duration_secs, 10 * 60);
        assert_eq!(parts[1].0, "2026-11-25");
        assert_eq!(parts[1].1.duration_secs, 15 * 60);
        let total: i64 = parts.iter().map(|(_, s)| s.duration_secs).sum();
        assert_eq!(total, session.duration_secs);
    }
}
