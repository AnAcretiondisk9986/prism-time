import React from "react";
import type { DashboardAnalyticsDto, RangeOption } from "../types";
import { CATEGORY_MAP, WEB_CATEGORY_MAP } from "../types";
import { formatDuration } from "../lib/format";
import { AppAvatar } from "./AppAvatar";
import { CategoryDonutChart } from "./charts/CategoryDonutChart";
import { HourlyStackedChart } from "./charts/HourlyStackedChart";
import { SessionSwimlaneChart } from "./charts/SessionSwimlaneChart";
import { ChevronRightIcon, WebCategorySvgIcon } from "./Icons";

interface OverviewViewProps {
  analytics: DashboardAnalyticsDto;
  range: RangeOption;
  onChangeRange: (r: RangeOption) => void;
  onSelectApp: (appId: string) => void;
  onEnableDemo: () => void;
  onGoToTimeline: () => void;
  onGoToWeb: () => void;
}

const RANGE_OPTIONS: Array<{ id: RangeOption; label: string }> = [
  { id: "today", label: "今日" },
  { id: "yesterday", label: "昨日" },
  { id: "7d", label: "近 7 日" },
  { id: "30d", label: "近 30 日" },
];

export const OverviewView: React.FC<OverviewViewProps> = ({
  analytics,
  range,
  onChangeRange,
  onSelectApp,
  onEnableDemo,
  onGoToTimeline,
  onGoToWeb,
}) => {
  const showEmptyHint = !analytics.has_real_data && !analytics.demo_mode && analytics.total_secs < 10;

  return (
    <div className="flex flex-col gap-5 pb-6">
      {/* Page Header & Range Switcher */}
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h1 className="text-[20px] font-semibold tracking-tight text-[var(--text-primary)]">
            屏幕使用概览
          </h1>
          <p className="text-[12.5px] text-[var(--text-secondary)] mt-0.5 tabular-nums">
            {analytics.date_label}
            {analytics.demo_mode && " · 当前处于非破坏性图表预览状态"}
          </p>
        </div>

        <div className="inline-flex p-1 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-hairline)]">
          {RANGE_OPTIONS.map((opt) => (
            <button
              key={opt.id}
              type="button"
              onClick={() => onChangeRange(opt.id)}
              className={`px-3.5 py-1.5 rounded-[9px] text-[12px] font-medium transition-all cursor-pointer ${
                range === opt.id
                  ? "bg-[var(--text-primary)] text-[var(--bg-surface)]"
                  : "text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
              }`}
            >
              {opt.label}
            </button>
          ))}
        </div>
      </div>

      {/* Gentle Empty Hint Banner when newly launched with <10s real data */}
      {showEmptyHint && (
        <div className="card-surface px-4 py-3 flex flex-wrap items-center justify-between gap-3 border-l-4 border-l-[var(--accent)]">
          <div>
            <p className="text-[13px] font-medium text-[var(--text-primary)]">
              澄镜已在后台开始实时采集您的屏幕使用时长
            </p>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              切换使用其他桌面软件几秒后数据将自动累积；您也可以先预览基于本机真实安装软件生成的全天时段图表。
            </p>
          </div>
          <button
            type="button"
            onClick={onEnableDemo}
            className="px-3.5 py-1.5 rounded-xl bg-[var(--accent)] text-white text-[12px] font-medium hover:opacity-90 transition-opacity cursor-pointer shrink-0"
          >
            预览图表演示
          </button>
        </div>
      )}

      {/* 4 Architectural Metric Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-3.5">
        <div className="card-surface p-4 flex flex-col justify-between gap-2">
          <div className="flex items-center justify-between">
            <span className="text-[12px] font-medium text-[var(--text-secondary)]">
              总屏幕使用时长
            </span>
            {analytics.delta_percentage !== null && (
              <span
                className={`px-2 py-0.5 rounded-full text-[11px] font-medium tabular-nums ${
                  analytics.delta_percentage >= 0
                    ? "bg-[#3D5AFE]/10 text-[var(--accent)]"
                    : "bg-[#1A9988]/12 text-[#1A9988]"
                }`}
              >
                {analytics.delta_percentage >= 0 ? "+" : ""}
                {analytics.delta_percentage.toFixed(1)}%
              </span>
            )}
          </div>
          <div className="text-[26px] font-semibold tracking-tight tabular-nums text-[var(--text-primary)] leading-tight">
            {formatDuration(analytics.total_secs)}
          </div>
          <div className="text-[11.5px] text-[var(--text-muted)] tabular-nums">
            活跃软件 {analytics.active_app_count} 款 · 共 {analytics.session_count} 个时段
          </div>
        </div>

        <div className="card-surface p-4 flex flex-col justify-between gap-2">
          <div className="flex items-center justify-between">
            <span className="text-[12px] font-medium text-[var(--text-secondary)]">
              主导软件类别
            </span>
            {analytics.dominant_category && (
              <span
                className="w-2.5 h-2.5 rounded-full"
                style={{ backgroundColor: analytics.dominant_category.color }}
              />
            )}
          </div>
          <div className="text-[22px] font-semibold tracking-tight text-[var(--text-primary)] leading-tight truncate">
            {analytics.dominant_category ? analytics.dominant_category.label : "待采集"}
          </div>
          <div className="text-[11.5px] text-[var(--text-muted)] tabular-nums">
            {analytics.dominant_category
              ? `占总时长 ${analytics.dominant_category.percentage.toFixed(1)}% · ${formatDuration(analytics.dominant_category.total_secs, true)}`
              : "等待识别软件使用分布"}
          </div>
        </div>

        <div className="card-surface p-4 flex flex-col justify-between gap-2">
          <span className="text-[12px] font-medium text-[var(--text-secondary)]">
            全天高峰时段
          </span>
          <div className="text-[22px] font-semibold tracking-tight tabular-nums text-[var(--text-primary)] leading-tight">
            {analytics.peak_hour_label}
          </div>
          <div className="text-[11.5px] text-[var(--text-muted)]">
            屏幕集中度最高的一小时窗口
          </div>
        </div>

        <div className="card-surface p-4 flex flex-col justify-between gap-2">
          <span className="text-[12px] font-medium text-[var(--text-secondary)]">
            最长单次连续使用
          </span>
          <div className="text-[22px] font-semibold tracking-tight tabular-nums text-[var(--text-primary)] leading-tight">
            {analytics.longest_focus_secs > 0
              ? formatDuration(analytics.longest_focus_secs, true)
              : "--"}
          </div>
          <div className="text-[11.5px] text-[var(--text-muted)] truncate">
            {analytics.longest_focus_app_name
              ? `来自 ${analytics.longest_focus_app_name}`
              : "尚无连续专注记录"}
          </div>
        </div>
      </div>

      {/* 24-hour Swimlane / Period Band */}
      <SessionSwimlaneChart
        swimlanes={analytics.swimlanes}
        daySegments={analytics.day_segments}
        isToday={range === "today"}
        onSelectApp={onSelectApp}
      />

      {/* Hourly Distribution + Category Ring */}
      <div className="grid grid-cols-1 xl:grid-cols-12 gap-4">
        <div className="xl:col-span-7">
          <HourlyStackedChart
            hourly={analytics.hourly}
            isSingleDay={range === "today" || range === "yesterday" || range === "custom"}
          />
        </div>
        <div className="xl:col-span-5">
          <CategoryDonutChart
            categories={analytics.categories}
            totalSecs={analytics.total_secs}
          />
        </div>
      </div>

      {/* Top Software Usage Ranking */}
      <div className="card-surface p-5 flex flex-col gap-4">
        <div className="flex items-center justify-between gap-3">
          <div>
            <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
              软件使用时长排行与时段摘要
            </h3>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              已自动合并多进程子组件与后台窗体，点击任意软件查看分时曲线与明细
            </p>
          </div>
          <button
            type="button"
            onClick={onGoToTimeline}
            className="inline-flex items-center gap-1 px-3 py-1.5 rounded-xl text-[12px] font-medium text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-subtle)] transition-colors cursor-pointer"
          >
            <span>查看完整时段趋势</span>
            <ChevronRightIcon size={14} />
          </button>
        </div>

        {analytics.top_apps.length === 0 ? (
          <div className="py-10 text-center text-[12.5px] text-[var(--text-muted)]">
            暂无应用使用记录
          </div>
        ) : (
          <div className="flex flex-col divide-y divide-[var(--border-hairline)]/70">
            {analytics.top_apps.map((app, index) => {
              const cat = CATEGORY_MAP[app.category];
              return (
                <button
                  key={app.app_id}
                  type="button"
                  onClick={() => onSelectApp(app.app_id)}
                  className="w-full text-left py-3 px-2 rounded-xl hover:bg-[var(--bg-subtle)]/65 transition-colors flex flex-col sm:flex-row sm:items-center justify-between gap-3 cursor-pointer group"
                >
                  <div className="flex items-center gap-3 min-w-0 flex-1">
                    <span className="w-5 text-[11.5px] font-medium tabular-nums text-[var(--text-muted)] shrink-0">
                      {String(index + 1).padStart(2, "0")}
                    </span>
                    <AppAvatar
                      name={app.display_name}
                      category={app.category}
                      iconDataUrl={app.icon_data_url}
                      size={36}
                    />
                    <div className="min-w-0 flex-1">
                      <div className="flex items-center gap-2 flex-wrap">
                        <span className="text-[13.5px] font-semibold text-[var(--text-primary)] group-hover:text-[var(--accent)] transition-colors truncate">
                          {app.display_name}
                        </span>
                        <span
                          className="px-2 py-0.5 rounded-full text-[10.5px] font-medium"
                          style={{
                            backgroundColor: `${cat.color}16`,
                            color: cat.color,
                          }}
                        >
                          {cat.shortLabel}
                        </span>
                        <span className="text-[11px] text-[var(--text-muted)]">
                          高峰 · {app.peak_period_label}
                        </span>
                      </div>
                      <p className="text-[11.5px] text-[var(--text-secondary)] truncate mt-0.5">
                        {app.description}
                      </p>
                      {app.active_ranges_preview.length > 0 && (
                        <div className="flex flex-wrap items-center gap-1.5 mt-1.5">
                          {app.active_ranges_preview.map((rng) => (
                            <span
                              key={rng}
                              className="px-2 py-0.5 rounded-md bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[10.5px] tabular-nums text-[var(--text-secondary)]"
                            >
                              {rng}
                            </span>
                          ))}
                        </div>
                      )}
                    </div>
                  </div>

                  {/* Right Duration & Ratio */}
                  <div className="flex items-center gap-4 shrink-0 pl-8 sm:pl-0">
                    <div className="w-28 hidden md:block">
                      <div className="h-1.5 w-full rounded-full bg-[var(--bg-subtle)] overflow-hidden">
                        <div
                          className="h-full rounded-full"
                          style={{
                            width: `${Math.min(100, Math.max(4, app.percentage))}%`,
                            backgroundColor: cat.color,
                          }}
                        />
                      </div>
                    </div>
                    <div className="text-right min-w-[76px]">
                      <div className="text-[13.5px] font-semibold tabular-nums text-[var(--text-primary)]">
                        {formatDuration(app.total_secs, true)}
                      </div>
                      <div className="text-[11px] tabular-nums text-[var(--text-muted)]">
                        {app.percentage.toFixed(1)}% · {app.session_count} 次
                      </div>
                    </div>
                  </div>
                </button>
              );
            })}
          </div>
        )}
      </div>

      {/* Browser Inner Web Highlights Quick Card */}
      {analytics.web_analytics.sites.length > 0 && (
        <div className="card-surface p-5 flex flex-col gap-3.5">
          <div className="flex items-center justify-between gap-3">
            <div>
              <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
                浏览器内热门网站速览
              </h3>
              <p className="text-[12px] text-[var(--text-secondary)] mt-0.5 tabular-nums">
                网页浏览累计 {formatDuration(analytics.web_analytics.total_secs, true)} · 占总屏幕{" "}
                {analytics.web_analytics.share_of_screen_pct.toFixed(1)}% · 识别{" "}
                {analytics.web_analytics.active_site_count} 个站点
              </p>
            </div>
            <button
              type="button"
              onClick={onGoToWeb}
              className="inline-flex items-center gap-1 px-3 py-1.5 rounded-xl text-[12px] font-medium text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-subtle)] transition-colors cursor-pointer"
            >
              <span>查看完整网页洞察</span>
              <ChevronRightIcon size={14} />
            </button>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-3">
            {analytics.web_analytics.sites.slice(0, 4).map((site) => {
              const cat = WEB_CATEGORY_MAP[site.category];
              return (
                <button
                  key={site.site_id}
                  type="button"
                  onClick={onGoToWeb}
                  className="text-left p-3.5 rounded-xl bg-[var(--bg-subtle)]/60 border border-[var(--border-hairline)] hover:border-[var(--border-strong)] transition-colors flex flex-col justify-between gap-2 cursor-pointer"
                >
                  <div className="flex items-center gap-2.5 min-w-0">
                    <span
                      className="w-7 h-7 rounded-lg flex items-center justify-center shrink-0"
                      style={{ backgroundColor: `${cat.color}18`, color: cat.color }}
                    >
                      <WebCategorySvgIcon category={site.category} size={14} />
                    </span>
                    <div className="min-w-0">
                      <div className="text-[13px] font-semibold text-[var(--text-primary)] truncate">
                        {site.display_name}
                      </div>
                      <div className="text-[10.5px] font-mono text-[var(--text-muted)] truncate">
                        {site.domain}
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center justify-between text-[11.5px] tabular-nums">
                    <span className="font-semibold text-[var(--text-primary)]">
                      {formatDuration(site.total_secs, true)}
                    </span>
                    <span className="text-[var(--text-muted)]">{site.percentage.toFixed(1)}%</span>
                  </div>
                </button>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
};
