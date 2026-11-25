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

export interface AppSettings {
  tracking_enabled: boolean;
  idle_threshold_secs: number;
  pause_on_lock: boolean;
  minimize_to_tray_on_close: boolean;
  launch_at_startup: boolean;
  track_self: boolean;
  theme: "light" | "dark";
}

export interface LiveStatusDto {
  tracking_enabled: boolean;
  state: "active" | "idle" | "locked" | "paused" | "self" | "desktop";
  current_app_id: string | null;
  current_app_name: string | null;
  current_category: AppCategory | null;
  current_category_label: string | null;
  current_icon_data_url: string | null;
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
}

export interface AppOverrideInput {
  app_id: string;
  display_name?: string | null;
  category?: AppCategory | null;
  description?: string | null;
  ignored?: boolean | null;
}

export interface ExportResultDto {
  file_path: string;
  record_count: number;
  format: string;
}

export type NavTab = "overview" | "timeline" | "catalog" | "settings";
export type RangeOption = "today" | "yesterday" | "7d" | "30d" | "custom";
