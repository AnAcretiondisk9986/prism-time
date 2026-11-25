use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppCategory {
    Dev,
    Ai,
    Browser,
    Design,
    Office,
    Social,
    Media,
    Game,
    Utility,
}

impl AppCategory {
    #[allow(dead_code)]
    pub fn id(self) -> &'static str {
        match self {
            AppCategory::Dev => "dev",
            AppCategory::Ai => "ai",
            AppCategory::Browser => "browser",
            AppCategory::Design => "design",
            AppCategory::Office => "office",
            AppCategory::Social => "social",
            AppCategory::Media => "media",
            AppCategory::Game => "game",
            AppCategory::Utility => "utility",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            AppCategory::Dev => "研发与终端",
            AppCategory::Ai => "AI 与智能助手",
            AppCategory::Browser => "网页浏览",
            AppCategory::Design => "设计创意",
            AppCategory::Office => "办公与学习",
            AppCategory::Social => "社交通讯",
            AppCategory::Media => "影音娱乐",
            AppCategory::Game => "游戏娱乐",
            AppCategory::Utility => "效率与系统",
        }
    }

    pub fn color(self) -> &'static str {
        match self {
            AppCategory::Dev => "#4F6BED",
            AppCategory::Ai => "#7C5CFA",
            AppCategory::Browser => "#0E8AC8",
            AppCategory::Design => "#D96B43",
            AppCategory::Office => "#2D9469",
            AppCategory::Social => "#1A9988",
            AppCategory::Media => "#C88A2B",
            AppCategory::Game => "#C94A6A",
            AppCategory::Utility => "#64748B",
        }
    }

    pub fn all() -> &'static [AppCategory] {
        &[
            AppCategory::Dev,
            AppCategory::Ai,
            AppCategory::Browser,
            AppCategory::Design,
            AppCategory::Office,
            AppCategory::Social,
            AppCategory::Media,
            AppCategory::Game,
            AppCategory::Utility,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebCategory {
    DevTech,
    AiWeb,
    VideoStream,
    SocialForum,
    DocsWork,
    SearchPortal,
    ShoppingLife,
    BrowserInternal,
}

impl WebCategory {
    pub fn label(self) -> &'static str {
        match self {
            WebCategory::DevTech => "代码与技术",
            WebCategory::AiWeb => "AI 智能平台",
            WebCategory::VideoStream => "视频与音乐",
            WebCategory::SocialForum => "社交与社区",
            WebCategory::DocsWork => "文档与学术",
            WebCategory::SearchPortal => "搜索与门户",
            WebCategory::ShoppingLife => "电商与消费",
            WebCategory::BrowserInternal => "浏览器内置页",
        }
    }

    pub fn color(self) -> &'static str {
        match self {
            WebCategory::DevTech => "#4F6BED",
            WebCategory::AiWeb => "#7C5CFA",
            WebCategory::VideoStream => "#C88A2B",
            WebCategory::SocialForum => "#1A9988",
            WebCategory::DocsWork => "#2D9469",
            WebCategory::SearchPortal => "#0E8AC8",
            WebCategory::ShoppingLife => "#D96B43",
            WebCategory::BrowserInternal => "#64748B",
        }
    }

    pub fn all() -> &'static [WebCategory] {
        &[
            WebCategory::DevTech,
            WebCategory::AiWeb,
            WebCategory::VideoStream,
            WebCategory::SocialForum,
            WebCategory::DocsWork,
            WebCategory::SearchPortal,
            WebCategory::ShoppingLife,
            WebCategory::BrowserInternal,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    pub app_id: String,
    pub display_name: String,
    pub category: AppCategory,
    pub description: String,
    pub publisher: String,
    pub icon_data_url: Option<String>,
    pub exe_paths: Vec<String>,
    pub process_names: Vec<String>,
    pub source: String,
    pub installed: bool,
    pub ignored: bool,
    pub last_seen_ts: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppOverride {
    pub app_id: String,
    pub display_name: Option<String>,
    pub category: Option<AppCategory>,
    pub description: Option<String>,
    pub ignored: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSession {
    pub id: String,
    pub app_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,
    pub start_ts: i64,
    pub end_ts: i64,
    pub duration_secs: i64,
    pub sample_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_site_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_site_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_category: Option<WebCategory>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub tracking_enabled: bool,
    pub idle_threshold_secs: u64,
    pub pause_on_lock: bool,
    pub minimize_to_tray_on_close: bool,
    pub launch_at_startup: bool,
    pub track_self: bool,
    #[serde(default = "default_true")]
    pub web_tracking_enabled: bool,
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            tracking_enabled: true,
            idle_threshold_secs: 180,
            pause_on_lock: true,
            minimize_to_tray_on_close: true,
            launch_at_startup: false,
            track_self: false,
            web_tracking_enabled: true,
            theme: "light".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveStatusDto {
    pub tracking_enabled: bool,
    pub state: String,
    pub current_app_id: Option<String>,
    pub current_app_name: Option<String>,
    pub current_category: Option<AppCategory>,
    pub current_category_label: Option<String>,
    pub current_icon_data_url: Option<String>,
    pub current_web_site: Option<String>,
    pub current_web_domain: Option<String>,
    pub current_session_secs: i64,
    pub today_total_secs: i64,
    pub idle_secs: u64,
    pub catalog_count: usize,
    pub data_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStatDto {
    pub category: AppCategory,
    pub label: String,
    pub color: String,
    pub total_secs: i64,
    pub percentage: f64,
    pub app_count: usize,
    pub peak_hour_range: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourTopAppDto {
    pub app_id: String,
    pub display_name: String,
    pub category: AppCategory,
    pub duration_secs: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyCategoryBucketDto {
    pub hour: u8,
    pub label: String,
    pub total_secs: i64,
    pub by_category: BTreeMap<AppCategory, i64>,
    pub top_apps: Vec<HourTopAppDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwimlaneSegmentDto {
    pub app_id: String,
    pub display_name: String,
    pub category: AppCategory,
    pub start_ts: i64,
    pub end_ts: i64,
    pub start_minute_of_day: f64,
    pub end_minute_of_day: f64,
    pub duration_secs: i64,
    pub start_label: String,
    pub end_label: String,
    pub sample_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwimlaneRowDto {
    pub app_id: String,
    pub display_name: String,
    pub category: AppCategory,
    pub icon_data_url: Option<String>,
    pub total_secs: i64,
    pub segments: Vec<SwimlaneSegmentDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodBucketDto {
    pub period_id: String,
    pub label: String,
    pub time_range: String,
    pub total_secs: i64,
    pub dominant_category: Option<AppCategory>,
    pub by_category: BTreeMap<AppCategory, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyTrendPointDto {
    pub date: String,
    pub short_label: String,
    pub total_secs: i64,
    pub by_category: BTreeMap<AppCategory, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUsageSummaryDto {
    pub app_id: String,
    pub display_name: String,
    pub category: AppCategory,
    pub description: String,
    pub publisher: String,
    pub icon_data_url: Option<String>,
    pub total_secs: i64,
    pub percentage: f64,
    pub session_count: usize,
    pub longest_session_secs: i64,
    pub peak_period_label: String,
    pub active_ranges_preview: Vec<String>,
    pub last_seen_ts: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEventDto {
    pub id: String,
    pub app_id: String,
    pub display_name: String,
    pub process_name: Option<String>,
    pub category: AppCategory,
    pub icon_data_url: Option<String>,
    pub date: String,
    pub start_ts: i64,
    pub end_ts: i64,
    pub start_time_label: String,
    pub end_time_label: String,
    pub duration_secs: i64,
    pub sample_title: Option<String>,
    pub web_site_id: Option<String>,
    pub web_site_name: Option<String>,
    pub web_domain: Option<String>,
    pub web_category: Option<WebCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebCategoryStatDto {
    pub category: WebCategory,
    pub label: String,
    pub color: String,
    pub total_secs: i64,
    pub percentage: f64,
    pub site_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCarrierDto {
    pub app_id: String,
    pub display_name: String,
    pub icon_data_url: Option<String>,
    pub total_secs: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebHourTopSiteDto {
    pub site_id: String,
    pub display_name: String,
    pub category: WebCategory,
    pub duration_secs: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyWebBucketDto {
    pub hour: u8,
    pub label: String,
    pub total_secs: i64,
    pub by_category: BTreeMap<WebCategory, i64>,
    pub top_sites: Vec<WebHourTopSiteDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSiteSummaryDto {
    pub site_id: String,
    pub display_name: String,
    pub domain: String,
    pub category: WebCategory,
    pub total_secs: i64,
    pub percentage: f64,
    pub session_count: usize,
    pub browsers: Vec<String>,
    pub active_ranges_preview: Vec<String>,
    pub last_seen_ts: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAnalyticsDto {
    pub total_secs: i64,
    pub share_of_screen_pct: f64,
    pub active_site_count: usize,
    pub session_count: usize,
    pub top_site_name: Option<String>,
    pub top_site_domain: Option<String>,
    pub top_site_secs: i64,
    pub dominant_category: Option<WebCategoryStatDto>,
    pub categories: Vec<WebCategoryStatDto>,
    pub browsers: Vec<BrowserCarrierDto>,
    pub hourly: Vec<HourlyWebBucketDto>,
    pub sites: Vec<WebSiteSummaryDto>,
    pub recent_web_sessions: Vec<SessionEventDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardAnalyticsDto {
    pub range: String,
    pub date_label: String,
    pub selected_date: String,
    pub demo_mode: bool,
    pub has_real_data: bool,
    pub total_secs: i64,
    pub previous_total_secs: i64,
    pub delta_percentage: Option<f64>,
    pub active_app_count: usize,
    pub session_count: usize,
    pub longest_focus_secs: i64,
    pub longest_focus_app_name: Option<String>,
    pub peak_hour_label: String,
    pub dominant_category: Option<CategoryStatDto>,
    pub categories: Vec<CategoryStatDto>,
    pub hourly: Vec<HourlyCategoryBucketDto>,
    pub swimlanes: Vec<SwimlaneRowDto>,
    pub day_segments: Vec<SwimlaneSegmentDto>,
    pub periods: Vec<PeriodBucketDto>,
    pub daily_trend: Vec<DailyTrendPointDto>,
    pub top_apps: Vec<AppUsageSummaryDto>,
    pub recent_sessions: Vec<SessionEventDto>,
    pub web_analytics: WebAnalyticsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDetailDto {
    pub app: AppMetadata,
    pub date: String,
    pub demo_mode: bool,
    pub total_secs: i64,
    pub share_percentage: f64,
    pub session_count: usize,
    pub longest_session_secs: i64,
    pub average_session_secs: i64,
    pub peak_hour_label: String,
    pub hourly_secs: Vec<i64>,
    pub sessions: Vec<SessionEventDto>,
    pub seven_day_trend: Vec<DailyTrendPointDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogAppDto {
    pub app_id: String,
    pub display_name: String,
    pub category: AppCategory,
    pub description: String,
    pub publisher: String,
    pub icon_data_url: Option<String>,
    pub process_names: Vec<String>,
    pub source: String,
    pub installed: bool,
    pub ignored: bool,
    pub today_secs: i64,
    pub last_seen_ts: Option<i64>,
    pub install_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppOverrideInput {
    pub app_id: String,
    pub display_name: Option<String>,
    pub category: Option<AppCategory>,
    pub description: Option<String>,
    pub ignored: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResultDto {
    pub file_path: String,
    pub record_count: usize,
    pub format: String,
}
