export type AppCategory =
  | "dev"
  | "ai"
  | "browser"
  | "design"
  | "office"
  | "social"
  | "media"
  | "game"
  | "utility";

export interface CategoryMeta {
  id: AppCategory;
  label: string;
  shortLabel: string;
  color: string;
}

export const CATEGORY_LIST: CategoryMeta[] = [
  { id: "dev", label: "研发与终端", shortLabel: "研发", color: "#4F6BED" },
  { id: "ai", label: "AI 与智能助手", shortLabel: "AI 助手", color: "#7C5CFA" },
  { id: "browser", label: "网页浏览", shortLabel: "浏览", color: "#0E8AC8" },
  { id: "design", label: "设计创意", shortLabel: "设计", color: "#D96B43" },
  { id: "office", label: "办公与学习", shortLabel: "办公", color: "#2D9469" },
  { id: "social", label: "社交通讯", shortLabel: "社交", color: "#1A9988" },
  { id: "media", label: "影音娱乐", shortLabel: "影音", color: "#C88A2B" },
  { id: "game", label: "游戏娱乐", shortLabel: "游戏", color: "#C94A6A" },
  { id: "utility", label: "效率与系统", shortLabel: "效率", color: "#64748B" },
];

export const CATEGORY_MAP: Record<AppCategory, CategoryMeta> = CATEGORY_LIST.reduce(
  (acc, item) => {
    acc[item.id] = item;
    return acc;
  },
  {} as Record<AppCategory, CategoryMeta>,
);

export type WebCategory =
  | "dev_tech"
  | "ai_web"
  | "video_stream"
  | "social_forum"
  | "docs_work"
  | "search_portal"
  | "shopping_life"
  | "browser_internal";

export interface WebCategoryMeta {
  id: WebCategory;
  label: string;
  shortLabel: string;
  color: string;
}

export const WEB_CATEGORY_LIST: WebCategoryMeta[] = [
  { id: "dev_tech", label: "代码与技术", shortLabel: "技术", color: "#4F6BED" },
  { id: "ai_web", label: "AI 智能平台", shortLabel: "AI 平台", color: "#7C5CFA" },
  { id: "video_stream", label: "视频与音乐", shortLabel: "影音", color: "#C88A2B" },
  { id: "social_forum", label: "社交与社区", shortLabel: "社区", color: "#1A9988" },
  { id: "docs_work", label: "文档与学术", shortLabel: "文档", color: "#2D9469" },
  { id: "search_portal", label: "搜索与门户", shortLabel: "浏览", color: "#0E8AC8" },
  { id: "shopping_life", label: "电商与消费", shortLabel: "消费", color: "#D96B43" },
  { id: "browser_internal", label: "浏览器内置页", shortLabel: "内置页", color: "#64748B" },
];

export const WEB_CATEGORY_MAP: Record<WebCategory, WebCategoryMeta> = WEB_CATEGORY_LIST.reduce(
  (acc, item) => {
    acc[item.id] = item;
    return acc;
  },
  {} as Record<WebCategory, WebCategoryMeta>,
);

export interface AppMetadata {
  app_id: string;
  display_name: string;
  category: AppCategory;
  description: string;
  publisher: string;
  icon_data_url: string | null;
  exe_paths: string[];
  process_names: string[];
  source: string;
  installed: boolean;
  ignored: boolean;
  last_seen_ts: number | null;
}

export interface WebDavConfig {
  server_url: string;
  username: string;
  password: string;
  remote_dir: string;
}

export interface GitHubSyncConfig {
  owner: string;
  repo: string;
  branch: string;
  path_prefix: string;
  token: string;
}

export interface SyncConfig {
  provider: "none" | "webdav" | "github";
  auto_sync_enabled: boolean;
  auto_sync_interval_mins: number;
  device_name: string;
  sync_overrides: boolean;
  webdav: WebDavConfig;
  github: GitHubSyncConfig;
}

export interface SyncStatusDto {
  status: "idle" | "syncing" | "success" | "error";
  provider: string | null;
  last_sync_ts: number | null;
  message: string;
  uploaded_files: number;
  downloaded_files: number;
  merged_sessions: number;
}

export interface AppSettings {
  tracking_enabled: boolean;
  idle_threshold_secs: number;
  pause_on_lock: boolean;
  minimize_to_tray_on_close: boolean;
  launch_at_startup: boolean;
  track_self: boolean;
  web_tracking_enabled: boolean;
  theme: "light" | "dark";
  sync: SyncConfig;
}

export interface LiveStatusDto {
  tracking_enabled: boolean;
  state: "active" | "idle" | "locked" | "paused" | "self" | "desktop";
  current_app_id: string | null;
  current_app_name: string | null;
  current_category: AppCategory | null;
  current_category_label: string | null;
  current_icon_data_url: string | null;
  current_web_site: string | null;
  current_web_domain: string | null;
  current_session_secs: number;
  today_total_secs: number;
  idle_secs: number;
  catalog_count: number;
  data_dir: string;
}

export interface CategoryStatDto {
  category: AppCategory;
  label: string;
  color: string;
  total_secs: number;
  percentage: number;
  app_count: number;
  peak_hour_range: string;
}

export interface HourTopAppDto {
  app_id: string;
  display_name: string;
  category: AppCategory;
  duration_secs: number;
}

export interface HourlyCategoryBucketDto {
  hour: number;
  label: string;
  total_secs: number;
  by_category: Partial<Record<AppCategory, number>>;
  top_apps: HourTopAppDto[];
}

export interface SwimlaneSegmentDto {
  app_id: string;
  display_name: string;
  category: AppCategory;
  start_ts: number;
  end_ts: number;
  start_minute_of_day: number;
  end_minute_of_day: number;
  duration_secs: number;
  start_label: string;
  end_label: string;
  sample_title: string | null;
}

export interface SwimlaneRowDto {
  app_id: string;
  display_name: string;
  category: AppCategory;
  icon_data_url: string | null;
  total_secs: number;
  segments: SwimlaneSegmentDto[];
}

export interface PeriodBucketDto {
  period_id: string;
  label: string;
  time_range: string;
  total_secs: number;
  dominant_category: AppCategory | null;
  by_category: Partial<Record<AppCategory, number>>;
}

export interface DailyTrendPointDto {
  date: string;
  short_label: string;
  total_secs: number;
  by_category: Partial<Record<AppCategory, number>>;
}

export interface AppUsageSummaryDto {
  app_id: string;
  display_name: string;
  category: AppCategory;
  description: string;
  publisher: string;
  icon_data_url: string | null;
  total_secs: number;
  percentage: number;
  session_count: number;
  longest_session_secs: number;
  peak_period_label: string;
  active_ranges_preview: string[];
  last_seen_ts: number | null;
}

export interface SessionEventDto {
  id: string;
  app_id: string;
  display_name: string;
  process_name: string | null;
  category: AppCategory;
  icon_data_url: string | null;
  date: string;
  start_ts: number;
  end_ts: number;
  start_time_label: string;
  end_time_label: string;
  duration_secs: number;
  sample_title: string | null;
  web_site_id: string | null;
  web_site_name: string | null;
  web_domain: string | null;
  web_category: WebCategory | null;
}

export interface WebCategoryStatDto {
  category: WebCategory;
  label: string;
  color: string;
  total_secs: number;
  percentage: number;
  site_count: number;
}

export interface BrowserCarrierDto {
  app_id: string;
  display_name: string;
  icon_data_url: string | null;
  total_secs: number;
  percentage: number;
}

export interface WebHourTopSiteDto {
  site_id: string;
  display_name: string;
  category: WebCategory;
  duration_secs: number;
}

export interface HourlyWebBucketDto {
  hour: number;
  label: string;
  total_secs: number;
  by_category: Partial<Record<WebCategory, number>>;
  top_sites: WebHourTopSiteDto[];
}

export interface WebSiteSummaryDto {
  site_id: string;
  display_name: string;
  domain: string;
  category: WebCategory;
  total_secs: number;
  percentage: number;
  session_count: number;
  browsers: string[];
  active_ranges_preview: string[];
  last_seen_ts: number | null;
}

export interface WebAnalyticsDto {
  total_secs: number;
  share_of_screen_pct: number;
  active_site_count: number;
  session_count: number;
  top_site_name: string | null;
  top_site_domain: string | null;
  top_site_secs: number;
  dominant_category: WebCategoryStatDto | null;
  categories: WebCategoryStatDto[];
  browsers: BrowserCarrierDto[];
  hourly: HourlyWebBucketDto[];
  sites: WebSiteSummaryDto[];
  recent_web_sessions: SessionEventDto[];
}

export interface DashboardAnalyticsDto {
  range: string;
  date_label: string;
  selected_date: string;
  demo_mode: boolean;
  has_real_data: boolean;
  total_secs: number;
  previous_total_secs: number;
  delta_percentage: number | null;
  active_app_count: number;
  session_count: number;
  longest_focus_secs: number;
  longest_focus_app_name: string | null;
  peak_hour_label: string;
  dominant_category: CategoryStatDto | null;
  categories: CategoryStatDto[];
  hourly: HourlyCategoryBucketDto[];
  swimlanes: SwimlaneRowDto[];
  day_segments: SwimlaneSegmentDto[];
  periods: PeriodBucketDto[];
  daily_trend: DailyTrendPointDto[];
  top_apps: AppUsageSummaryDto[];
  recent_sessions: SessionEventDto[];
  web_analytics: WebAnalyticsDto;
}

export interface AppDetailDto {
  app: AppMetadata;
  date: string;
  demo_mode: boolean;
  total_secs: number;
  share_percentage: number;
  session_count: number;
  longest_session_secs: number;
  average_session_secs: number;
  peak_hour_label: string;
  hourly_secs: number[];
  sessions: SessionEventDto[];
  seven_day_trend: DailyTrendPointDto[];
}

export interface CatalogAppDto {
  app_id: string;
  display_name: string;
  category: AppCategory;
  description: string;
  publisher: string;
  icon_data_url: string | null;
  process_names: string[];
  source: string;
  installed: boolean;
  ignored: boolean;
  today_secs: number;
  last_seen_ts: number | null;
  install_hint: string | null;
  has_override?: boolean;
  extra_process_names?: string[];
  idle_multiplier?: number | null;
  treat_as_browser?: boolean | null;
  merge_into_app_id?: string | null;
  custom_tag?: string | null;
}

export interface AppOverrideInput {
  app_id: string;
  display_name?: string | null;
  category?: AppCategory | null;
  description?: string | null;
  ignored?: boolean | null;
  extra_process_names?: string[];
  idle_multiplier?: number | null;
  treat_as_browser?: boolean | null;
  merge_into_app_id?: string | null;
  custom_tag?: string | null;
  custom_icon_data_url?: string | null;
}

export interface CustomAppRule {
  rule_id: string;
  app_id: string;
  display_name: string;
  category: AppCategory;
  description: string;
  publisher: string;
  process_names: string[];
  path_contains: string[];
  title_contains: string[];
  ignored: boolean;
  treat_as_browser: boolean;
  idle_multiplier: number;
  merge_into_app_id?: string | null;
  custom_tag?: string | null;
  custom_icon_data_url?: string | null;
}

export interface CustomAppRuleInput {
  rule_id?: string | null;
  display_name: string;
  category: AppCategory;
  description?: string | null;
  publisher?: string | null;
  process_names: string[];
  path_contains?: string[];
  title_contains?: string[];
  ignored?: boolean;
  treat_as_browser?: boolean;
  idle_multiplier?: number;
  merge_into_app_id?: string | null;
  custom_tag?: string | null;
  custom_icon_data_url?: string | null;
}

export interface ExportResultDto {
  file_path: string;
  record_count: number;
  format: string;
}

export type NavTab = "overview" | "timeline" | "web" | "catalog" | "settings";
export type RangeOption = "today" | "yesterday" | "7d" | "30d" | "custom";
