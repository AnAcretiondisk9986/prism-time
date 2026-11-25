use crate::catalog::{humanize_fallback_name, infer_category_from_hints};
use crate::models::{
    AppCategory, AppDetailDto, AppMetadata, AppUsageSummaryDto, BrowserCarrierDto, CategoryStatDto,
    DailyTrendPointDto, DashboardAnalyticsDto, HourTopAppDto, HourlyCategoryBucketDto,
    HourlyWebBucketDto, PeriodBucketDto, SessionEventDto, SwimlaneRowDto, SwimlaneSegmentDto,
    UsageSession, WebAnalyticsDto, WebCategory, WebCategoryStatDto, WebHourTopSiteDto,
    WebSiteSummaryDto,
};
use crate::web_catalog::resolve_browser_page;
use chrono::{Duration, Local, NaiveDate, TimeZone, Timelike};
use std::collections::{BTreeMap, HashMap, HashSet};

const PERIOD_DEFS: &[(&str, &str, &str, usize, usize)] = &[
    ("late_night", "深夜", "00:00 - 06:00", 0, 6),
    ("early_morning", "清晨", "06:00 - 09:00", 6, 9),
    ("morning", "上午", "09:00 - 12:00", 9, 12),
    ("afternoon", "下午", "12:00 - 18:00", 12, 18),
    ("evening", "夜晚", "18:00 - 24:00", 18, 24),
];

pub fn local_day_bounds(date: NaiveDate) -> (i64, i64) {
    let start_naive = date.and_hms_opt(0, 0, 0).unwrap();
    let next_naive = date
        .succ_opt()
        .unwrap_or(date)
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let start_ts = Local
        .from_local_datetime(&start_naive)
        .earliest()
        .map(|d| d.timestamp())
        .unwrap_or(0);
    let end_ts = Local
        .from_local_datetime(&next_naive)
        .earliest()
        .map(|d| d.timestamp())
        .unwrap_or(start_ts + 86400);
    (start_ts, end_ts)
}

fn resolve_meta(app_id: &str, catalog: &HashMap<String, AppMetadata>) -> AppMetadata {
    if let Some(meta) = catalog.get(app_id) {
        return meta.clone();
    }
    let (fallback_name, fallback_processes, is_proc) =
        if let Some(proc_part) = app_id.strip_prefix("proc.").filter(|s| !s.is_empty()) {
            (proc_part.to_string(), vec![proc_part.to_string()], true)
        } else {
            let name = app_id
                .split('.')
                .last()
                .map(humanize_fallback_name)
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "桌面应用".to_string());
            (name, Vec::new(), false)
        };
    let category = infer_category_from_hints(&fallback_name.to_lowercase(), "", "");
    AppMetadata {
        app_id: app_id.to_string(),
        display_name: fallback_name,
        category,
        description: if is_proc {
            format!("未内置识别进程 · {}", category.label())
        } else {
            format!("{} · 已记录应用", category.label())
        },
        publisher: if is_proc {
            "本地未识别程序".to_string()
        } else {
            "本地应用".to_string()
        },
        icon_data_url: None,
        exe_paths: Vec::new(),
        process_names: fallback_processes,
        source: "observed".to_string(),
        installed: true,
        ignored: false,
        last_seen_ts: None,
    }
}

fn resolve_session_web_info(
    session: &UsageSession,
    meta: &AppMetadata,
) -> Option<(String, String, String, WebCategory)> {
    if let (Some(sid), Some(sname), Some(dom), Some(cat)) = (
        &session.web_site_id,
        &session.web_site_name,
        &session.web_domain,
        session.web_category,
    ) {
        return Some((sid.clone(), sname.clone(), dom.clone(), cat));
    }
    if meta.category == AppCategory::Browser {
        if let Some(ref title) = session.sample_title {
            let page = resolve_browser_page(None, title);
            return Some((page.site_id, page.display_name, page.domain, page.category));
        }
    }
    None
}

fn format_hms(ts: i64) -> String {
    Local
        .timestamp_opt(ts, 0)
        .single()
        .map(|d| d.format("%H:%M:%S").to_string())
        .unwrap_or_else(|| "00:00:00".to_string())
}

fn format_hm(ts: i64) -> String {
    Local
        .timestamp_opt(ts, 0)
        .single()
        .map(|d| d.format("%H:%M").to_string())
        .unwrap_or_else(|| "00:00".to_string())
}

fn format_date_str(ts: i64) -> String {
    Local
        .timestamp_opt(ts, 0)
        .single()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "1970-01-01".to_string())
}

pub fn build_dashboard_analytics(
    range: &str,
    selected_date: NaiveDate,
    today: NaiveDate,
    sessions_by_date: &HashMap<NaiveDate, Vec<UsageSession>>,
    catalog: &HashMap<String, AppMetadata>,
    demo_mode: bool,
    has_real_data: bool,
) -> DashboardAnalyticsDto {
    let (target_dates, prev_dates, date_label) = match range {
        "yesterday" => {
            let y = today - Duration::days(1);
            let prev = y - Duration::days(1);
            (vec![y], vec![prev], format!("昨日 · {}", y.format("%Y-%m-%d")))
        }
        "7d" => {
            let cur: Vec<NaiveDate> = (0..7)
                .rev()
                .map(|i| selected_date - Duration::days(i))
                .collect();
            let prev: Vec<NaiveDate> = (7..14)
                .rev()
                .map(|i| selected_date - Duration::days(i))
                .collect();
            let start = cur.first().copied().unwrap_or(selected_date);
            (
                cur,
                prev,
                format!("近 7 日 · {} 至 {}", start.format("%m/%d"), selected_date.format("%m/%d")),
            )
        }
        "30d" => {
            let cur: Vec<NaiveDate> = (0..30)
                .rev()
                .map(|i| selected_date - Duration::days(i))
                .collect();
            let prev: Vec<NaiveDate> = (30..60)
                .rev()
                .map(|i| selected_date - Duration::days(i))
                .collect();
            let start = cur.first().copied().unwrap_or(selected_date);
            (
                cur,
                prev,
                format!("近 30 日 · {} 至 {}", start.format("%m/%d"), selected_date.format("%m/%d")),
            )
        }
        "custom" => {
            let prev = selected_date - Duration::days(1);
            (
                vec![selected_date],
                vec![prev],
                format!("单日详情 · {}", selected_date.format("%Y-%m-%d")),
            )
        }
        _ => {
            let prev = today - Duration::days(1);
            (vec![today], vec![prev], format!("今日 · {}", today.format("%Y-%m-%d")))
        }
    };

    let focus_date = target_dates.last().copied().unwrap_or(selected_date);

    // Collect filtered sessions across target dates
    let mut total_secs: i64 = 0;
    let mut session_count: usize = 0;
    let mut longest_focus_secs: i64 = 0;
    let mut longest_focus_app_name: Option<String> = None;

    let mut hourly_cat_secs: Vec<BTreeMap<AppCategory, i64>> = vec![BTreeMap::new(); 24];
    let mut hourly_app_secs: Vec<HashMap<String, i64>> = vec![HashMap::new(); 24];
    let mut hourly_totals: Vec<i64> = vec![0; 24];

    let mut cat_totals: HashMap<AppCategory, i64> = HashMap::new();
    let mut cat_apps: HashMap<AppCategory, HashSet<String>> = HashMap::new();
    let mut cat_hourly: HashMap<AppCategory, [i64; 24]> = HashMap::new();

    struct AppAcc {
        meta: AppMetadata,
        total_secs: i64,
        session_count: usize,
        longest_secs: i64,
        hour_secs: [i64; 24],
        ranges: Vec<(i64, i64)>,
        last_seen: i64,
    }
    let mut app_accs: HashMap<String, AppAcc> = HashMap::new();
    let mut all_events: Vec<SessionEventDto> = Vec::new();
    let mut day_segments_raw: Vec<SwimlaneSegmentDto> = Vec::new();

    // Web browsing accumulators
    let mut web_total_secs: i64 = 0;
    let mut web_session_count: usize = 0;
    let mut web_hourly_cat_secs: Vec<BTreeMap<WebCategory, i64>> = vec![BTreeMap::new(); 24];
    let mut web_hourly_site_secs: Vec<HashMap<String, i64>> = vec![HashMap::new(); 24];
    let mut web_hourly_totals: Vec<i64> = vec![0; 24];
    let mut web_cat_totals: HashMap<WebCategory, i64> = HashMap::new();
    let mut web_cat_sites: HashMap<WebCategory, HashSet<String>> = HashMap::new();
    let mut browser_carriers: HashMap<String, (AppMetadata, i64)> = HashMap::new();
    struct WebSiteAcc {
        site_id: String,
        display_name: String,
        domain: String,
        category: WebCategory,
        total_secs: i64,
        session_count: usize,
        browsers: HashSet<String>,
        ranges: Vec<(i64, i64)>,
        last_seen: i64,
    }
    let mut web_site_accs: HashMap<String, WebSiteAcc> = HashMap::new();
    let mut recent_web_sessions: Vec<SessionEventDto> = Vec::new();

    for &d in &target_dates {
        let (day_start, day_end) = local_day_bounds(d);
        let Some(day_sessions) = sessions_by_date.get(&d) else {
            continue;
        };
        for s in day_sessions {
            let meta = resolve_meta(&s.app_id, catalog);
            if meta.ignored {
                continue;
            }
            let cs = s.start_ts.max(day_start);
            let ce = s.end_ts.min(day_end);
            if ce <= cs {
                continue;
            }
            let dur = ce - cs;
            total_secs += dur;
            session_count += 1;
            if dur > longest_focus_secs {
                longest_focus_secs = dur;
                longest_focus_app_name = Some(meta.display_name.clone());
            }

            *cat_totals.entry(meta.category).or_insert(0) += dur;
            cat_apps
                .entry(meta.category)
                .or_default()
                .insert(meta.app_id.clone());

            let acc = app_accs.entry(meta.app_id.clone()).or_insert_with(|| AppAcc {
                meta: meta.clone(),
                total_secs: 0,
                session_count: 0,
                longest_secs: 0,
                hour_secs: [0; 24],
                ranges: Vec::new(),
                last_seen: 0,
            });
            acc.total_secs += dur;
            acc.session_count += 1;
            acc.longest_secs = acc.longest_secs.max(dur);
            acc.last_seen = acc.last_seen.max(ce);
            if d == focus_date && dur >= 10 {
                acc.ranges.push((cs, ce));
            }

            let web_info = resolve_session_web_info(s, &meta);
            if let Some((ref w_sid, ref w_name, ref w_dom, w_cat)) = web_info {
                web_total_secs += dur;
                web_session_count += 1;
                *web_cat_totals.entry(w_cat).or_insert(0) += dur;
                web_cat_sites.entry(w_cat).or_default().insert(w_sid.clone());
                let carrier = browser_carriers
                    .entry(meta.app_id.clone())
                    .or_insert_with(|| (meta.clone(), 0));
                carrier.1 += dur;

                let w_acc = web_site_accs
                    .entry(w_sid.clone())
                    .or_insert_with(|| WebSiteAcc {
                        site_id: w_sid.clone(),
                        display_name: w_name.clone(),
                        domain: w_dom.clone(),
                        category: w_cat,
                        total_secs: 0,
                        session_count: 0,
                        browsers: HashSet::new(),
                        ranges: Vec::new(),
                        last_seen: 0,
                    });
                w_acc.total_secs += dur;
                w_acc.session_count += 1;
                w_acc.browsers.insert(meta.display_name.clone());
                w_acc.last_seen = w_acc.last_seen.max(ce);
                if d == focus_date && dur >= 10 {
                    w_acc.ranges.push((cs, ce));
                }
            }

            // Slice across 24 hours of this day cleanly
            let cat_arr = cat_hourly.entry(meta.category).or_insert([0; 24]);
            for h in 0..24usize {
                let hs = day_start + (h as i64) * 3600;
                let he = hs + 3600;
                let ov = ce.min(he) - cs.max(hs);
                if ov > 0 {
                    hourly_totals[h] += ov;
                    *hourly_cat_secs[h].entry(meta.category).or_insert(0) += ov;
                    *hourly_app_secs[h].entry(meta.app_id.clone()).or_insert(0) += ov;
                    cat_arr[h] += ov;
                    acc.hour_secs[h] += ov;

                    if let Some((ref w_sid, _, _, w_cat)) = web_info {
                        web_hourly_totals[h] += ov;
                        *web_hourly_cat_secs[h].entry(w_cat).or_insert(0) += ov;
                        *web_hourly_site_secs[h].entry(w_sid.clone()).or_insert(0) += ov;
                    }
                }
            }

            if d == focus_date {
                let seg = SwimlaneSegmentDto {
                    app_id: meta.app_id.clone(),
                    display_name: meta.display_name.clone(),
                    category: meta.category,
                    start_ts: cs,
                    end_ts: ce,
                    start_minute_of_day: (cs - day_start) as f64 / 60.0,
                    end_minute_of_day: (ce - day_start) as f64 / 60.0,
                    duration_secs: dur,
                    start_label: format_hm(cs),
                    end_label: format_hm(ce),
                    sample_title: s.sample_title.clone(),
                };
                day_segments_raw.push(seg);
            }

            let ev = SessionEventDto {
                id: s.id.clone(),
                app_id: meta.app_id.clone(),
                display_name: meta.display_name.clone(),
                process_name: s
                    .process_name
                    .clone()
                    .or_else(|| meta.process_names.first().cloned()),
                category: meta.category,
                icon_data_url: meta.icon_data_url.clone(),
                date: d.format("%Y-%m-%d").to_string(),
                start_ts: cs,
                end_ts: ce,
                start_time_label: format_hms(cs),
                end_time_label: format_hms(ce),
                duration_secs: dur,
                sample_title: s.sample_title.clone(),
                web_site_id: web_info.as_ref().map(|w| w.0.clone()),
                web_site_name: web_info.as_ref().map(|w| w.1.clone()),
                web_domain: web_info.as_ref().map(|w| w.2.clone()),
                web_category: web_info.as_ref().map(|w| w.3),
            };
            if web_info.is_some() {
                recent_web_sessions.push(ev.clone());
            }
            all_events.push(ev);
        }
    }

    // Previous period total for delta comparison
    let mut previous_total_secs: i64 = 0;
    for &d in &prev_dates {
        let (day_start, day_end) = local_day_bounds(d);
        if let Some(day_sessions) = sessions_by_date.get(&d) {
            for s in day_sessions {
                let meta = resolve_meta(&s.app_id, catalog);
                if meta.ignored {
                    continue;
                }
                let cs = s.start_ts.max(day_start);
                let ce = s.end_ts.min(day_end);
                if ce > cs {
                    previous_total_secs += ce - cs;
                }
            }
        }
    }
    let delta_percentage = if previous_total_secs > 60 {
        Some(((total_secs - previous_total_secs) as f64 / previous_total_secs as f64) * 100.0)
    } else {
        None
    };

    // Peak hour label
    let peak_hour = hourly_totals
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .and_then(|(idx, &val)| if val > 0 { Some(idx) } else { None });
    let peak_hour_label = match peak_hour {
        Some(h) => format!("{:02}:00 - {:02}:00", h, (h + 1) % 24),
        None => "--".to_string(),
    };

    // Categories sorted descending
    let mut categories: Vec<CategoryStatDto> = AppCategory::all()
        .iter()
        .filter_map(|&cat| {
            let secs = *cat_totals.get(&cat).unwrap_or(&0);
            if secs <= 0 {
                return None;
            }
            let pct = if total_secs > 0 {
                (secs as f64 / total_secs as f64) * 100.0
            } else {
                0.0
            };
            let arr = cat_hourly.get(&cat).copied().unwrap_or([0; 24]);
            let peak_range = best_two_hour_window(&arr);
            Some(CategoryStatDto {
                category: cat,
                label: cat.label().to_string(),
                color: cat.color().to_string(),
                total_secs: secs,
                percentage: (pct * 10.0).round() / 10.0,
                app_count: cat_apps.get(&cat).map(|s| s.len()).unwrap_or(0),
                peak_hour_range: peak_range,
            })
        })
        .collect();
    categories.sort_by(|a, b| b.total_secs.cmp(&a.total_secs));
    let dominant_category = categories.first().cloned();

    // Hourly buckets
    let hourly: Vec<HourlyCategoryBucketDto> = (0..24)
        .map(|h| {
            let mut apps: Vec<HourTopAppDto> = hourly_app_secs[h]
                .iter()
                .map(|(aid, &secs)| {
                    let m = resolve_meta(aid, catalog);
                    HourTopAppDto {
                        app_id: aid.clone(),
                        display_name: m.display_name,
                        category: m.category,
                        duration_secs: secs,
                    }
                })
                .collect();
            apps.sort_by(|a, b| b.duration_secs.cmp(&a.duration_secs));
            apps.truncate(3);

            HourlyCategoryBucketDto {
                hour: h as u8,
                label: format!("{h:02}:00"),
                total_secs: hourly_totals[h],
                by_category: hourly_cat_secs[h].clone(),
                top_apps: apps,
            }
        })
        .collect();

    // 5 Periods
    let periods: Vec<PeriodBucketDto> = PERIOD_DEFS
        .iter()
        .map(|&(pid, label, time_range, h_start, h_end)| {
            let mut by_cat: BTreeMap<AppCategory, i64> = BTreeMap::new();
            let mut p_total: i64 = 0;
            for h in h_start..h_end {
                p_total += hourly_totals[h];
                for (&cat, &secs) in &hourly_cat_secs[h] {
                    *by_cat.entry(cat).or_insert(0) += secs;
                }
            }
            let dom_cat = by_cat
                .iter()
                .max_by_key(|&(_, v)| v)
                .and_then(|(&c, &v)| if v > 0 { Some(c) } else { None });
            PeriodBucketDto {
                period_id: pid.to_string(),
                label: label.to_string(),
                time_range: time_range.to_string(),
                total_secs: p_total,
                dominant_category: dom_cat,
                by_category: by_cat,
            }
        })
        .collect();

    // Top apps
    let mut top_apps: Vec<AppUsageSummaryDto> = app_accs
        .into_values()
        .map(|mut acc| {
            let pct = if total_secs > 0 {
                (acc.total_secs as f64 / total_secs as f64) * 100.0
            } else {
                0.0
            };
            acc.ranges.sort_by_key(|r| r.0);
            let merged_ranges = merge_time_ranges(&acc.ranges, 90);
            let preview: Vec<String> = merged_ranges
                .into_iter()
                .take(4)
                .map(|(s, e)| format!("{}-{}", format_hm(s), format_hm(e)))
                .collect();

            AppUsageSummaryDto {
                app_id: acc.meta.app_id,
                display_name: acc.meta.display_name,
                category: acc.meta.category,
                description: acc.meta.description,
                publisher: acc.meta.publisher,
                icon_data_url: acc.meta.icon_data_url,
                total_secs: acc.total_secs,
                percentage: (pct * 10.0).round() / 10.0,
                session_count: acc.session_count,
                longest_session_secs: acc.longest_secs,
                peak_period_label: best_period_for_hours(&acc.hour_secs),
                active_ranges_preview: preview,
                last_seen_ts: Some(acc.last_seen),
            }
        })
        .collect();
    top_apps.sort_by(|a, b| b.total_secs.cmp(&a.total_secs));

    // Swimlane segments (coalesce gaps <= 45s for clean continuous bands on focus_date)
    day_segments_raw.sort_by_key(|s| s.start_ts);
    let day_segments = coalesce_segments(day_segments_raw, 45);

    // Top 8 apps swimlanes for focus_date
    let mut lane_map: HashMap<String, Vec<SwimlaneSegmentDto>> = HashMap::new();
    for seg in &day_segments {
        lane_map
            .entry(seg.app_id.clone())
            .or_default()
            .push(seg.clone());
    }
    let mut swimlanes: Vec<SwimlaneRowDto> = lane_map
        .into_iter()
        .map(|(aid, segs)| {
            let m = resolve_meta(&aid, catalog);
            let lane_total: i64 = segs.iter().map(|s| s.duration_secs).sum();
            SwimlaneRowDto {
                app_id: aid,
                display_name: m.display_name,
                category: m.category,
                icon_data_url: m.icon_data_url,
                total_secs: lane_total,
                segments: segs,
            }
        })
        .collect();
    swimlanes.sort_by(|a, b| b.total_secs.cmp(&a.total_secs));
    swimlanes.truncate(8);

    // Daily trend (at least 7 days for single-day ranges, or 30 days for 30d)
    let trend_len = if range == "30d" { 30 } else { 7 };
    let trend_dates: Vec<NaiveDate> = (0..trend_len)
        .rev()
        .map(|i| focus_date - Duration::days(i))
        .collect();
    let daily_trend = build_daily_trend(&trend_dates, today, sessions_by_date, catalog);

    // Recent sessions descending
    all_events.sort_by(|a, b| b.start_ts.cmp(&a.start_ts));
    all_events.truncate(80);
    recent_web_sessions.sort_by(|a, b| b.start_ts.cmp(&a.start_ts));
    recent_web_sessions.truncate(60);

    // Build WebAnalyticsDto
    let mut web_categories: Vec<WebCategoryStatDto> = WebCategory::all()
        .iter()
        .filter_map(|&wcat| {
            let secs = *web_cat_totals.get(&wcat).unwrap_or(&0);
            if secs <= 0 {
                return None;
            }
            let pct = if web_total_secs > 0 {
                ((secs as f64 / web_total_secs as f64) * 1000.0).round() / 10.0
            } else {
                0.0
            };
            Some(WebCategoryStatDto {
                category: wcat,
                label: wcat.label().to_string(),
                color: wcat.color().to_string(),
                total_secs: secs,
                percentage: pct,
                site_count: web_cat_sites.get(&wcat).map(|s| s.len()).unwrap_or(0),
            })
        })
        .collect();
    web_categories.sort_by(|a, b| b.total_secs.cmp(&a.total_secs));
    let web_dominant_category = web_categories.first().cloned();

    let mut web_browsers: Vec<BrowserCarrierDto> = browser_carriers
        .into_values()
        .map(|(m, secs)| {
            let pct = if web_total_secs > 0 {
                ((secs as f64 / web_total_secs as f64) * 1000.0).round() / 10.0
            } else {
                0.0
            };
            BrowserCarrierDto {
                app_id: m.app_id,
                display_name: m.display_name,
                icon_data_url: m.icon_data_url,
                total_secs: secs,
                percentage: pct,
            }
        })
        .collect();
    web_browsers.sort_by(|a, b| b.total_secs.cmp(&a.total_secs));

    let web_hourly: Vec<HourlyWebBucketDto> = (0..24)
        .map(|h| {
            let mut top_sites: Vec<WebHourTopSiteDto> = web_hourly_site_secs[h]
                .iter()
                .filter_map(|(sid, &secs)| {
                    let acc = web_site_accs.get(sid)?;
                    Some(WebHourTopSiteDto {
                        site_id: sid.clone(),
                        display_name: acc.display_name.clone(),
                        category: acc.category,
                        duration_secs: secs,
                    })
                })
                .collect();
            top_sites.sort_by(|a, b| b.duration_secs.cmp(&a.duration_secs));
            top_sites.truncate(3);

            HourlyWebBucketDto {
                hour: h as u8,
                label: format!("{h:02}:00"),
                total_secs: web_hourly_totals[h],
                by_category: web_hourly_cat_secs[h].clone(),
                top_sites,
            }
        })
        .collect();

    let mut web_sites: Vec<WebSiteSummaryDto> = web_site_accs
        .into_values()
        .map(|mut w_acc| {
            let pct = if web_total_secs > 0 {
                ((w_acc.total_secs as f64 / web_total_secs as f64) * 1000.0).round() / 10.0
            } else {
                0.0
            };
            w_acc.ranges.sort_by_key(|r| r.0);
            let merged_ranges = merge_time_ranges(&w_acc.ranges, 90);
            let preview: Vec<String> = merged_ranges
                .into_iter()
                .take(4)
                .map(|(s, e)| format!("{}-{}", format_hm(s), format_hm(e)))
                .collect();
            let mut browsers_list: Vec<String> = w_acc.browsers.into_iter().collect();
            browsers_list.sort();

            WebSiteSummaryDto {
                site_id: w_acc.site_id,
                display_name: w_acc.display_name,
                domain: w_acc.domain,
                category: w_acc.category,
                total_secs: w_acc.total_secs,
                percentage: pct,
                session_count: w_acc.session_count,
                browsers: browsers_list,
                active_ranges_preview: preview,
                last_seen_ts: Some(w_acc.last_seen),
            }
        })
        .collect();
    web_sites.sort_by(|a, b| b.total_secs.cmp(&a.total_secs));

    let web_share_pct = if total_secs > 0 {
        ((web_total_secs as f64 / total_secs as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };
    let top_site = web_sites.first();

    let web_analytics = WebAnalyticsDto {
        total_secs: web_total_secs,
        share_of_screen_pct: web_share_pct,
        active_site_count: web_sites.len(),
        session_count: web_session_count,
        top_site_name: top_site.map(|s| s.display_name.clone()),
        top_site_domain: top_site.map(|s| s.domain.clone()),
        top_site_secs: top_site.map(|s| s.total_secs).unwrap_or(0),
        dominant_category: web_dominant_category,
        categories: web_categories,
        browsers: web_browsers,
        hourly: web_hourly,
        sites: web_sites,
        recent_web_sessions,
    };

    DashboardAnalyticsDto {
        range: range.to_string(),
        date_label,
        selected_date: focus_date.format("%Y-%m-%d").to_string(),
        demo_mode,
        has_real_data,
        total_secs,
        previous_total_secs,
        delta_percentage: delta_percentage.map(|v| (v * 10.0).round() / 10.0),
        active_app_count: top_apps.len(),
        session_count,
        longest_focus_secs,
        longest_focus_app_name,
        peak_hour_label,
        dominant_category,
        categories,
        hourly,
        swimlanes,
        day_segments,
        periods,
        daily_trend,
        top_apps,
        recent_sessions: all_events,
        web_analytics,
    }
}

pub fn build_app_detail(
    app_id: &str,
    date: NaiveDate,
    today: NaiveDate,
    sessions_by_date: &HashMap<NaiveDate, Vec<UsageSession>>,
    catalog: &HashMap<String, AppMetadata>,
    demo_mode: bool,
) -> AppDetailDto {
    let meta = resolve_meta(app_id, catalog);
    let (day_start, day_end) = local_day_bounds(date);
    let mut day_total_all_apps: i64 = 0;
    let mut app_total: i64 = 0;
    let mut hourly_secs = vec![0i64; 24];
    let mut longest_secs: i64 = 0;
    let mut events = Vec::new();

    if let Some(day_sessions) = sessions_by_date.get(&date) {
        for s in day_sessions {
            let m = resolve_meta(&s.app_id, catalog);
            if m.ignored {
                continue;
            }
            let cs = s.start_ts.max(day_start);
            let ce = s.end_ts.min(day_end);
            if ce <= cs {
                continue;
            }
            let dur = ce - cs;
            day_total_all_apps += dur;

            if s.app_id == app_id {
                app_total += dur;
                longest_secs = longest_secs.max(dur);
                for h in 0..24usize {
                    let hs = day_start + (h as i64) * 3600;
                    let he = hs + 3600;
                    let ov = ce.min(he) - cs.max(hs);
                    if ov > 0 {
                        hourly_secs[h] += ov;
                    }
                }
                let web_info = resolve_session_web_info(s, &meta);
                events.push(SessionEventDto {
                    id: s.id.clone(),
                    app_id: meta.app_id.clone(),
                    display_name: meta.display_name.clone(),
                    process_name: s
                        .process_name
                        .clone()
                        .or_else(|| meta.process_names.first().cloned()),
                    category: meta.category,
                    icon_data_url: meta.icon_data_url.clone(),
                    date: format_date_str(cs),
                    start_ts: cs,
                    end_ts: ce,
                    start_time_label: format_hms(cs),
                    end_time_label: format_hms(ce),
                    duration_secs: dur,
                    sample_title: s.sample_title.clone(),
                    web_site_id: web_info.as_ref().map(|w| w.0.clone()),
                    web_site_name: web_info.as_ref().map(|w| w.1.clone()),
                    web_domain: web_info.as_ref().map(|w| w.2.clone()),
                    web_category: web_info.as_ref().map(|w| w.3),
                });
            }
        }
    }

    events.sort_by(|a, b| b.start_ts.cmp(&a.start_ts));
    let session_count = events.len();
    let average_session_secs = if session_count > 0 {
        app_total / (session_count as i64)
    } else {
        0
    };
    let share_percentage = if day_total_all_apps > 0 {
        ((app_total as f64 / day_total_all_apps as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };

    let peak_hour = hourly_secs
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .and_then(|(h, &v)| if v > 0 { Some(h) } else { None });
    let peak_hour_label = match peak_hour {
        Some(h) => format!("{:02}:00 - {:02}:00", h, (h + 1) % 24),
        None => "--".to_string(),
    };

    let trend_dates: Vec<NaiveDate> = (0..7).rev().map(|i| date - Duration::days(i)).collect();
    let seven_day_trend = trend_dates
        .iter()
        .map(|&d| {
            let (ds, de) = local_day_bounds(d);
            let mut total = 0i64;
            let mut by_cat = BTreeMap::new();
            if let Some(list) = sessions_by_date.get(&d) {
                for s in list {
                    if s.app_id == app_id {
                        let cs = s.start_ts.max(ds);
                        let ce = s.end_ts.min(de);
                        if ce > cs {
                            total += ce - cs;
                        }
                    }
                }
            }
            if total > 0 {
                by_cat.insert(meta.category, total);
            }
            DailyTrendPointDto {
                date: d.format("%Y-%m-%d").to_string(),
                short_label: if d == today {
                    "今日".to_string()
                } else {
                    d.format("%m/%d").to_string()
                },
                total_secs: total,
                by_category: by_cat,
            }
        })
        .collect();

    AppDetailDto {
        app: meta,
        date: date.format("%Y-%m-%d").to_string(),
        demo_mode,
        total_secs: app_total,
        share_percentage,
        session_count,
        longest_session_secs: longest_secs,
        average_session_secs,
        peak_hour_label,
        hourly_secs,
        sessions: events,
        seven_day_trend,
    }
}

fn build_daily_trend(
    dates: &[NaiveDate],
    today: NaiveDate,
    sessions_by_date: &HashMap<NaiveDate, Vec<UsageSession>>,
    catalog: &HashMap<String, AppMetadata>,
) -> Vec<DailyTrendPointDto> {
    dates
        .iter()
        .map(|&d| {
            let (ds, de) = local_day_bounds(d);
            let mut by_cat: BTreeMap<AppCategory, i64> = BTreeMap::new();
            let mut total = 0i64;
            if let Some(day_sessions) = sessions_by_date.get(&d) {
                for s in day_sessions {
                    let meta = resolve_meta(&s.app_id, catalog);
                    if meta.ignored {
                        continue;
                    }
                    let cs = s.start_ts.max(ds);
                    let ce = s.end_ts.min(de);
                    if ce > cs {
                        let dur = ce - cs;
                        total += dur;
                        *by_cat.entry(meta.category).or_insert(0) += dur;
                    }
                }
            }
            DailyTrendPointDto {
                date: d.format("%Y-%m-%d").to_string(),
                short_label: if d == today {
                    "今日".to_string()
                } else {
                    d.format("%m/%d").to_string()
                },
                total_secs: total,
                by_category: by_cat,
            }
        })
        .collect()
}

fn best_two_hour_window(hour_secs: &[i64; 24]) -> String {
    let mut best_idx = None;
    let mut best_sum = 0i64;
    for h in 0..23 {
        let sum = hour_secs[h] + hour_secs[h + 1];
        if sum > best_sum {
            best_sum = sum;
            best_idx = Some(h);
        }
    }
    match best_idx {
        Some(h) if best_sum > 0 => format!("{:02}:00-{:02}:00", h, h + 2),
        _ => "--".to_string(),
    }
}

fn best_period_for_hours(hour_secs: &[i64; 24]) -> String {
    let mut best_label = "--";
    let mut best_sum = 0i64;
    for &(_, label, _, h_start, h_end) in PERIOD_DEFS {
        let sum: i64 = (h_start..h_end).map(|h| hour_secs[h]).sum();
        if sum > best_sum {
            best_sum = sum;
            best_label = label;
        }
    }
    best_label.to_string()
}

fn merge_time_ranges(sorted: &[(i64, i64)], max_gap_secs: i64) -> Vec<(i64, i64)> {
    let mut out: Vec<(i64, i64)> = Vec::new();
    for &(s, e) in sorted {
        if let Some(last) = out.last_mut() {
            if s - last.1 <= max_gap_secs {
                last.1 = last.1.max(e);
                continue;
            }
        }
        out.push((s, e));
    }
    out
}

fn coalesce_segments(sorted: Vec<SwimlaneSegmentDto>, max_gap_secs: i64) -> Vec<SwimlaneSegmentDto> {
    let mut merged: Vec<SwimlaneSegmentDto> = Vec::new();
    for seg in sorted {
        // Check if the same app had a segment ending within max_gap_secs
        if let Some(prev) = merged
            .iter_mut()
            .rev()
            .take(4)
            .find(|p| p.app_id == seg.app_id && seg.start_ts - p.end_ts <= max_gap_secs)
        {
            prev.end_ts = prev.end_ts.max(seg.end_ts);
            prev.end_minute_of_day = prev.end_minute_of_day.max(seg.end_minute_of_day);
            prev.duration_secs += seg.duration_secs;
            prev.end_label = seg.end_label;
            if prev.sample_title.is_none() {
                prev.sample_title = seg.sample_title;
            }
            continue;
        }
        merged.push(seg);
    }
    merged
}

#[allow(dead_code)]
pub fn hour_of_timestamp(ts: i64) -> usize {
    Local
        .timestamp_opt(ts, 0)
        .single()
        .map(|d| d.hour() as usize)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hourly_slicing_conserves_exact_seconds_across_boundaries() {
        let day = NaiveDate::from_ymd_opt(2026, 11, 25).unwrap();
        let start_ts = Local
            .from_local_datetime(&day.and_hms_opt(9, 45, 0).unwrap())
            .single()
            .unwrap()
            .timestamp();
        let end_ts = Local
            .from_local_datetime(&day.and_hms_opt(11, 15, 0).unwrap())
            .single()
            .unwrap()
            .timestamp();

        let mut sessions_by_date = HashMap::new();
        sessions_by_date.insert(
            day,
            vec![UsageSession {
                id: "s-1".to_string(),
                app_id: "google.antigravity".to_string(),
                process_name: Some("Antigravity".to_string()),
                start_ts,
                end_ts,
                duration_secs: end_ts - start_ts,
                sample_title: Some("prism-time".to_string()),
                web_site_id: None,
                web_site_name: None,
                web_domain: None,
                web_category: None,
            }],
        );

        let mut catalog = HashMap::new();
        catalog.insert(
            "google.antigravity".to_string(),
            AppMetadata {
                app_id: "google.antigravity".to_string(),
                display_name: "Antigravity".to_string(),
                category: AppCategory::Dev,
                description: "IDE".to_string(),
                publisher: "Google".to_string(),
                icon_data_url: None,
                exe_paths: vec![],
                process_names: vec![],
                source: "known".to_string(),
                installed: true,
                ignored: false,
                last_seen_ts: None,
            },
        );

        let dto = build_dashboard_analytics("custom", day, day, &sessions_by_date, &catalog, false, true);
        assert_eq!(dto.total_secs, 90 * 60);
        assert_eq!(dto.hourly[9].total_secs, 15 * 60);
        assert_eq!(dto.hourly[10].total_secs, 60 * 60);
        assert_eq!(dto.hourly[11].total_secs, 15 * 60);
        let hourly_sum: i64 = dto.hourly.iter().map(|h| h.total_secs).sum();
        let period_sum: i64 = dto.periods.iter().map(|p| p.total_secs).sum();
        assert_eq!(hourly_sum, dto.total_secs);
        assert_eq!(period_sum, dto.total_secs);
    }

    #[test]
    fn web_analytics_aggregates_browser_sites_and_conserves_seconds() {
        let day = NaiveDate::from_ymd_opt(2026, 11, 25).unwrap();
        let t1_s = Local.from_local_datetime(&day.and_hms_opt(9, 50, 0).unwrap()).single().unwrap().timestamp();
        let t1_e = Local.from_local_datetime(&day.and_hms_opt(10, 20, 0).unwrap()).single().unwrap().timestamp();
        let t2_s = Local.from_local_datetime(&day.and_hms_opt(20, 0, 0).unwrap()).single().unwrap().timestamp();
        let t2_e = Local.from_local_datetime(&day.and_hms_opt(20, 45, 0).unwrap()).single().unwrap().timestamp();

        let mut sessions_by_date = HashMap::new();
        sessions_by_date.insert(
            day,
            vec![
                UsageSession {
                    id: "w-1".to_string(),
                    app_id: "microsoft.edge".to_string(),
                    process_name: Some("msedge".to_string()),
                    start_ts: t1_s,
                    end_ts: t1_e,
                    duration_secs: t1_e - t1_s,
                    sample_title: Some("tauri-apps/tauri".to_string()),
                    web_site_id: Some("web.github".to_string()),
                    web_site_name: Some("GitHub".to_string()),
                    web_domain: Some("github.com".to_string()),
                    web_category: Some(WebCategory::DevTech),
                },
                UsageSession {
                    id: "w-2".to_string(),
                    app_id: "google.chrome".to_string(),
                    process_name: Some("chrome".to_string()),
                    start_ts: t2_s,
                    end_ts: t2_e,
                    duration_secs: t2_e - t2_s,
                    sample_title: Some("Rust 工程实践 - 哔哩哔哩 - Google Chrome".to_string()),
                    web_site_id: None,
                    web_site_name: None,
                    web_domain: None,
                    web_category: None,
                },
            ],
        );

        let mut catalog = HashMap::new();
        for (aid, name) in [("microsoft.edge", "Microsoft Edge"), ("google.chrome", "Google Chrome")] {
            catalog.insert(
                aid.to_string(),
                AppMetadata {
                    app_id: aid.to_string(),
                    display_name: name.to_string(),
                    category: AppCategory::Browser,
                    description: "Browser".to_string(),
                    publisher: "Vendor".to_string(),
                    icon_data_url: None,
                    exe_paths: vec![],
                    process_names: vec![],
                    source: "known".to_string(),
                    installed: true,
                    ignored: false,
                    last_seen_ts: None,
                },
            );
        }

        let dto = build_dashboard_analytics("custom", day, day, &sessions_by_date, &catalog, false, true);
        let web = &dto.web_analytics;
        assert_eq!(web.total_secs, 75 * 60);
        assert_eq!(web.active_site_count, 2);
        assert_eq!(web.top_site_name.as_deref(), Some("哔哩哔哩"));
        assert_eq!(web.browsers.len(), 2);
        assert_eq!(web.hourly[9].total_secs, 10 * 60);
        assert_eq!(web.hourly[10].total_secs, 20 * 60);
        assert_eq!(web.hourly[20].total_secs, 45 * 60);
        let web_hourly_sum: i64 = web.hourly.iter().map(|h| h.total_secs).sum();
        assert_eq!(web_hourly_sum, web.total_secs);
    }
}
