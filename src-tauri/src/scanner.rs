use crate::catalog::{
    clean_software_display_name, extract_process_stem, infer_category_from_hints,
    match_known_by_registry_name, slugify_app_id,
};
use crate::models::AppMetadata;
use crate::win_probe::extract_icon_png_data_url;
use std::collections::HashMap;
use std::path::Path;

#[cfg(windows)]
use windows_registry::{Key, LOCAL_MACHINE, CURRENT_USER};

const NOISE_REG_TOKENS: &[&str] = &[
    "redistributable",
    "visual c++",
    "runtime",
    "sdk addon",
    "driver",
    "驱动程序",
    "hotfix",
    "service pack",
    "help viewer",
    "help 查看器",
    "language pack",
    "语言包",
    "sql server",
    "iis ",
    "silverlight",
    "clr types",
    "native client",
    "mobile device support",
    "coreeditorfonts",
    "microsoft .net sdk",
    "windows application compatibility",
    "anticheatexpert",
    "安全组件",
    "maintenance service",
    "webview support",
];

fn should_skip_registry_entry(display_name_lower: &str, publisher_lower: &str) -> bool {
    if display_name_lower.trim().is_empty() {
        return true;
    }
    if NOISE_REG_TOKENS
        .iter()
        .any(|tok| display_name_lower.contains(tok))
    {
        return true;
    }
    if display_name_lower.contains("kb")
        && display_name_lower.contains("hotfix")
    {
        return true;
    }
    if publisher_lower.contains("microsoft")
        && (display_name_lower.contains("update")
            || display_name_lower.contains("tools")
            || display_name_lower.contains("prerequisites")
            || display_name_lower.contains("vs_"))
    {
        return true;
    }
    false
}

fn clean_display_icon_path(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Strip trailing ",0" or ",-101" icon resource index
    let path_part = if trimmed.starts_with('"') {
        if let Some(end_quote) = trimmed[1..].find('"') {
            &trimmed[1..1 + end_quote]
        } else {
            trimmed.trim_matches('"')
        }
    } else if let Some(comma) = trimmed.rfind(',') {
        let after = trimmed[comma + 1..].trim();
        if after.trim_start_matches('-').chars().all(|c| c.is_ascii_digit()) {
            trimmed[..comma].trim()
        } else {
            trimmed
        }
    } else {
        trimmed
    };

    let candidate = path_part.trim().trim_matches('"');
    if candidate.is_empty() {
        return None;
    }
    let lower = candidate.to_lowercase();
    if lower.ends_with("msiexec.exe")
        || lower.contains("uninstall")
        || lower.contains("unins000")
        || lower.ends_with(".dll")
    {
        return None;
    }
    if Path::new(candidate).exists() {
        Some(candidate.to_string())
    } else {
        None
    }
}

fn find_exe_in_install_location(install_loc: &str) -> Option<String> {
    let clean = install_loc.trim().trim_matches('"');
    if clean.is_empty() {
        return None;
    }
    let dir = Path::new(clean);
    if !dir.is_dir() {
        return None;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return None;
    };
    let mut best: Option<String> = None;
    for entry in entries.flatten() {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("exe")) {
            let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
            if stem.contains("unins") || stem.contains("crash") || stem.contains("update") {
                continue;
            }
            best = Some(p.to_string_lossy().to_string());
            break;
        }
    }
    best
}

pub fn scan_installed_software(existing: &mut HashMap<String, AppMetadata>, extract_icons: bool) -> usize {
    let mut added_or_updated = 0usize;

    #[cfg(windows)]
    {
        let roots: &[(&Key, &str)] = &[
            (LOCAL_MACHINE, r"Software\Microsoft\Windows\CurrentVersion\Uninstall"),
            (LOCAL_MACHINE, r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall"),
            (CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Uninstall"),
        ];

        for &(hive, subkey) in roots {
            let Ok(uninstall_key) = hive.open(subkey) else {
                continue;
            };
            let Ok(keys_iter) = uninstall_key.keys() else {
                continue;
            };
            for sub_name in keys_iter {
                let Ok(app_key) = uninstall_key.open(&sub_name) else {
                    continue;
                };
                if app_key.get_u32("SystemComponent").unwrap_or(0) == 1 {
                    continue;
                }
                if app_key
                    .get_string("ParentKeyName")
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false)
                {
                    continue;
                }

                let Ok(raw_name) = app_key.get_string("DisplayName") else {
                    continue;
                };
                let publisher = app_key
                    .get_string("Publisher")
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                let raw_name_lower = raw_name.to_lowercase();
                let publisher_lower = publisher.to_lowercase();
                if should_skip_registry_entry(&raw_name_lower, &publisher_lower) {
                    continue;
                }

                let install_loc = app_key
                    .get_string("InstallLocation")
                    .unwrap_or_default()
                    .trim()
                    .trim_matches('"')
                    .to_string();
                let install_loc_lower = install_loc.to_lowercase();

                let display_icon_raw = app_key.get_string("DisplayIcon").unwrap_or_default();
                let icon_file = clean_display_icon_path(&display_icon_raw)
                    .or_else(|| find_exe_in_install_location(&install_loc));

                let cleaned_name = clean_software_display_name(&raw_name);
                if cleaned_name.is_empty() {
                    continue;
                }

                let (app_id, display_name, category, description, final_publisher, source) =
                    if let Some(rule) = match_known_by_registry_name(&raw_name_lower, &install_loc_lower) {
                        (
                            rule.app_id.to_string(),
                            rule.display_name.to_string(),
                            rule.category,
                            rule.description.to_string(),
                            if publisher.is_empty() {
                                rule.publisher.to_string()
                            } else {
                                publisher.clone()
                            },
                            "known".to_string(),
                        )
                    } else {
                        let cat = infer_category_from_hints(
                            &cleaned_name.to_lowercase(),
                            &publisher_lower,
                            &install_loc_lower,
                        );
                        let pub_clean = if publisher.is_empty() {
                            "第三方应用".to_string()
                        } else {
                            publisher.clone()
                        };
                        let desc = format!("{} · {}", cat.label(), pub_clean);
                        let aid = slugify_app_id(&cleaned_name);
                        (aid, cleaned_name, cat, desc, pub_clean, "scanned".to_string())
                    };

                let entry = existing.entry(app_id.clone()).or_insert_with(|| AppMetadata {
                    app_id: app_id.clone(),
                    display_name: display_name.clone(),
                    category,
                    description: description.clone(),
                    publisher: final_publisher.clone(),
                    icon_data_url: None,
                    exe_paths: Vec::new(),
                    process_names: Vec::new(),
                    source: source.clone(),
                    installed: true,
                    ignored: false,
                    last_seen_ts: None,
                });

                entry.installed = true;
                if entry.publisher.is_empty() {
                    entry.publisher = final_publisher;
                }
                if !install_loc.is_empty() && !entry.exe_paths.iter().any(|p| p.eq_ignore_ascii_case(&install_loc)) {
                    entry.exe_paths.push(install_loc.clone());
                }
                if let Some(ref ic) = icon_file {
                    if ic.to_lowercase().ends_with(".exe") {
                        let stem = extract_process_stem(ic);
                        if !stem.is_empty()
                            && !entry
                                .process_names
                                .iter()
                                .any(|p| p.eq_ignore_ascii_case(&stem))
                        {
                            entry.process_names.push(stem);
                        }
                    }
                    if !entry.exe_paths.iter().any(|p| p.eq_ignore_ascii_case(ic)) {
                        entry.exe_paths.push(ic.clone());
                    }
                    if extract_icons && entry.icon_data_url.is_none() {
                        entry.icon_data_url = extract_icon_png_data_url(ic);
                    }
                }
                added_or_updated += 1;
            }
        }
    }

    added_or_updated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_registry_scan_discovers_installed_software_cleanly() {
        let mut catalog = HashMap::new();
        let count = scan_installed_software(&mut catalog, false);
        println!("Scanned installed registry entries matched: {count}, unique apps: {}", catalog.len());
        assert!(
            catalog.len() >= 25,
            "expected at least 25 distinct installed apps on this Windows machine, got {}",
            catalog.len()
        );
        for app in catalog.values() {
            assert!(
                !app.display_name.to_lowercase().ends_with(".exe"),
                "display name should never be raw exe: {}",
                app.display_name
            );
            assert!(
                !app.display_name.to_lowercase().contains("redistributable"),
                "runtime redistributables must be filtered out: {}",
                app.display_name
            );
        }
    }
}
