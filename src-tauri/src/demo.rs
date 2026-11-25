use crate::analytics::local_day_bounds;
use crate::catalog::KNOWN_APPS;
use crate::models::{AppCategory, AppMetadata, UsageSession, WebCategory};
use chrono::{Duration, NaiveDate};
use std::collections::HashMap;

pub struct DemoDataset {
    pub catalog: HashMap<String, AppMetadata>,
    pub sessions_by_date: HashMap<NaiveDate, Vec<UsageSession>>,
}

/// Generates a non-destructive demo dataset anchored to real apps detected on this machine.
/// Never persisted to user session files.
pub fn generate_demo_dataset(
    real_catalog: &HashMap<String, AppMetadata>,
    anchor_date: NaiveDate,
) -> DemoDataset {
    let mut catalog = real_catalog.clone();

    let mut pick = |candidates: &[&str], fallback_rule_id: &str| -> String {
        for &cid in candidates {
            if let Some(m) = catalog.get(cid) {
                if !m.ignored {
                    return m.app_id.clone();
                }
            }
        }
        if !catalog.contains_key(fallback_rule_id) {
            if let Some(rule) = KNOWN_APPS.iter().find(|r| r.app_id == fallback_rule_id) {
                catalog.insert(
                    rule.app_id.to_string(),
                    AppMetadata {
                        app_id: rule.app_id.to_string(),
                        display_name: rule.display_name.to_string(),
                        category: rule.category,
                        description: rule.description.to_string(),
                        publisher: rule.publisher.to_string(),
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
        }
        fallback_rule_id.to_string()
    };

    let dev_main = pick(&["google.antigravity", "microsoft.vscode", "zai.zcode", "qoder.ide"], "google.antigravity");
    let dev_term = pick(&["waveterm.app", "wezfurlong.wezterm", "microsoft.windowsterminal"], "microsoft.windowsterminal");
    let ai_main = pick(&["openai.codex", "google.gemini", "tencent.workbuddy", "bytedance.doubao"], "openai.codex");
    let browser_main = pick(&["citrolabs.ego", "microsoft.edge", "google.chrome", "mozilla.firefox"], "microsoft.edge");
    let social_main = pick(&["tencent.weixin", "tencent.qq", "alibaba.dingtalk"], "tencent.weixin");
    let design_main = pick(&["adobe.photoshop", "bytedance.jianying", "artcraft.designcraft"], "adobe.photoshop");
    let office_main = pick(&["kingsoft.wps", "obsidian.notes", "marktext.editor"], "kingsoft.wps");
    let media_main = pick(&["folia.music", "bilibili.desktop", "netease.cloudmusic", "bytedance.douyin"], "bilibili.desktop");
    let game_main = pick(&["valve.steam", "game.gtav", "game.pubg", "tencent.wegame"], "valve.steam");

    // Ensure all selected demo apps have realistic fallback descriptions if needed
    let mut sessions_by_date: HashMap<NaiveDate, Vec<UsageSession>> = HashMap::new();

    for offset in 0..30i64 {
        let day = anchor_date - Duration::days(offset);
        let (day_start, _) = local_day_bounds(day);
        let mut list = Vec::new();
        let mut seq = 0usize;

        //Deterministic variation per day
        let wobble = (offset % 5) * 6;
        let is_weekend = offset % 7 == 5 || offset % 7 == 6;

        let push_block = |list: &mut Vec<UsageSession>,
                          seq: &mut usize,
                          app_id: &str,
                          start_hm: (i64, i64),
                          dur_mins: i64,
                          title: &str,
                          web_info: Option<(&str, &str, &str, WebCategory)>| {
            let st = day_start + start_hm.0 * 3600 + start_hm.1 * 60;
            let dur = dur_mins * 60;
            *seq += 1;
            let (ws_id, ws_name, ws_dom, ws_cat) = match web_info {
                Some((sid, sname, sdom, scat)) => (
                    Some(sid.to_string()),
                    Some(sname.to_string()),
                    Some(sdom.to_string()),
                    Some(scat),
                ),
                None => (None, None, None, None),
            };
            list.push(UsageSession {
                id: format!("demo-{}-{}", day.format("%Y%m%d"), *seq),
                app_id: app_id.to_string(),
                process_name: None,
                start_ts: st,
                end_ts: st + dur,
                duration_secs: dur,
                sample_title: Some(title.to_string()),
                web_site_id: ws_id,
                web_site_name: ws_name,
                web_domain: ws_dom,
                web_category: ws_cat,
            });
        };

        if !is_weekend {
            // Early morning / morning work rhythm
            push_block(&mut list, &mut seq, &social_main, (8, 52), 14 + (offset % 4) * 2, "工作群与消息确认", None);
            push_block(
                &mut list,
                &mut seq,
                &browser_main,
                (9, 8),
                16 + wobble / 3,
                "tauri-apps/tauri: Build desktop apps with Rust",
                Some(("web.github", "GitHub", "github.com", WebCategory::DevTech)),
            );
            push_block(
                &mut list,
                &mut seq,
                &browser_main,
                (9, 25),
                10,
                "Rust Win32 UI Automation 文档检索",
                Some(("web.juejin", "稀土掘金", "juejin.cn", WebCategory::DevTech)),
            );
            push_block(&mut list, &mut seq, &dev_main, (9, 36), 68 + wobble, "prism-time — 核心架构开发", None);
            push_block(&mut list, &mut seq, &ai_main, (10, 46), 24 + (offset % 3) * 4, "方案评审与代码辅助", None);
            push_block(&mut list, &mut seq, &dev_term, (11, 12), 18, "cargo test & 构建验证", None);
            push_block(&mut list, &mut seq, &social_main, (11, 32), 16, "午间沟通同步", None);

            // Afternoon peak
            push_block(&mut list, &mut seq, &office_main, (13, 45), 34 + (offset % 3) * 5, "项目规格文档整理", None);
            push_block(&mut list, &mut seq, &dev_main, (14, 22), 76 + wobble, "可视化组件与状态同步", None);
            push_block(
                &mut list,
                &mut seq,
                &browser_main,
                (15, 42),
                15,
                "澄镜本地预览调试",
                Some(("web.localhost", "本地开发预览", "localhost:1420", WebCategory::DevTech)),
            );
            push_block(
                &mut list,
                &mut seq,
                &browser_main,
                (15, 58),
                14,
                "Chart Layout Spec — UI 图标设计系统",
                Some(("web.figma", "Figma", "figma.com", WebCategory::DocsWork)),
            );
            push_block(&mut list, &mut seq, &design_main, (16, 15), 42 + (offset % 4) * 4, "界面视觉切图与图标细节", None);
            push_block(&mut list, &mut seq, &ai_main, (17, 02), 29, "自动化测试脚本生成", None);
            push_block(&mut list, &mut seq, &dev_term, (17, 34), 19, "发布检查与本地打包", None);

            // Evening rhythm
            push_block(&mut list, &mut seq, &social_main, (19, 25), 22, "晚间消息与好友互动", None);
            push_block(&mut list, &mut seq, &media_main, (19, 52), 48 + wobble, "沉浸影音与音乐播放", None);
            if offset % 2 == 0 {
                push_block(&mut list, &mut seq, &game_main, (20, 48), 54 + (offset % 3) * 10, "晚间休闲游戏时段", None);
            } else {
                push_block(&mut list, &mut seq, &office_main, (20, 45), 36, "个人知识库整理与复盘", None);
            }
            push_block(
                &mut list,
                &mut seq,
                &browser_main,
                (21, 50),
                17,
                "Rust 桌面应用工程实践视频",
                Some(("web.bilibili", "哔哩哔哩", "bilibili.com", WebCategory::VideoStream)),
            );
            push_block(
                &mut list,
                &mut seq,
                &browser_main,
                (22, 8),
                12,
                "效率工具工作流讨论",
                Some(("web.zhihu", "知乎", "zhihu.com", WebCategory::SocialForum)),
            );
            if offset % 3 == 0 {
                push_block(&mut list, &mut seq, &dev_main, (22, 22), 34, "开源项目优化", None);
            }
        } else {
            // Weekend / relaxed rhythm
            push_block(
                &mut list,
                &mut seq,
                &browser_main,
                (10, 15),
                20 + wobble / 2,
                "开源周刊与社区热点讨论",
                Some(("web.zhihu", "知乎", "zhihu.com", WebCategory::SocialForum)),
            );
            push_block(
                &mut list,
                &mut seq,
                &browser_main,
                (10, 38),
                16,
                "周末论文阅读 · arXiv cs.HC",
                Some(("web.arxiv", "arXiv", "arxiv.org", WebCategory::DocsWork)),
            );
            push_block(&mut list, &mut seq, &media_main, (11, 0), 55, "高清视频与音乐欣赏", None);
            push_block(&mut list, &mut seq, &social_main, (13, 30), 24, "聊天与日程安排", None);
            push_block(&mut list, &mut seq, &game_main, (14, 10), 86 + wobble, "沉浸游戏时段", None);
            push_block(&mut list, &mut seq, &design_main, (16, 5), 45, "摄影后期与创意实验", None);
            push_block(&mut list, &mut seq, &ai_main, (17, 10), 28, "AI 创意对话与探索", None);
            push_block(&mut list, &mut seq, &media_main, (20, 15), 72, "晚间影音娱乐", None);
            push_block(
                &mut list,
                &mut seq,
                &browser_main,
                (20, 50),
                22,
                "创意剪辑与音乐现场回顾",
                Some(("web.bilibili", "哔哩哔哩", "bilibili.com", WebCategory::VideoStream)),
            );
            push_block(&mut list, &mut seq, &social_main, (21, 35), 25, "晚间沟通", None);
        }

        // Keep today's demo sessions clipped up to current time? Actually full-day preview lets users immediately inspect all 24h periods!
        sessions_by_date.insert(day, list);
    }

    // Ensure every referenced app exists in catalog
    for app_id in [
        dev_main, dev_term, ai_main, browser_main, social_main, design_main, office_main,
        media_main, game_main,
    ] {
        catalog.entry(app_id.clone()).or_insert_with(|| AppMetadata {
            app_id: app_id.clone(),
            display_name: app_id.clone(),
            category: AppCategory::Utility,
            description: "本机常用桌面应用".to_string(),
            publisher: "本地应用".to_string(),
            icon_data_url: None,
            exe_paths: vec![],
            process_names: vec![],
            source: "known".to_string(),
            installed: true,
            ignored: false,
            last_seen_ts: None,
        });
    }

    DemoDataset {
        catalog,
        sessions_by_date,
    }
}
