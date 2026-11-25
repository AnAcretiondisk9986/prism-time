import React, { useMemo, useState } from "react";
import type { DashboardAnalyticsDto, RangeOption } from "../types";
import { CATEGORY_MAP } from "../types";
import { formatDuration, shiftDateISO } from "../lib/format";
import { exportUsageReport } from "../lib/ipc";
import { AppAvatar } from "./AppAvatar";
import { PeriodRhythmChart } from "./charts/PeriodRhythmChart";
import { SessionSwimlaneChart } from "./charts/SessionSwimlaneChart";
import { ChevronLeftIcon, ChevronRightIcon, ExportIcon, SearchIcon } from "./Icons";

interface TimelineViewProps {
  analytics: DashboardAnalyticsDto;
  range: RangeOption;
  selectedDate: string;
  demoMode: boolean;
  onChangeRange: (r: RangeOption) => void;
  onSelectDate: (date: string) => void;
  onSelectApp: (appId: string) => void;
}

export const TimelineView: React.FC<TimelineViewProps> = ({
  analytics,
  range,
  selectedDate,
  demoMode,
  onChangeRange,
  onSelectDate,
  onSelectApp,
}) => {
  const [sessionFilter, setSessionFilter] = useState("");
  const [exportNotice, setExportNotice] = useState<string | null>(null);

  const filteredSessions = useMemo(() => {
    const q = sessionFilter.trim().toLowerCase();
    if (!q) return analytics.recent_sessions;
    return analytics.recent_sessions.filter(
      (s) =>
        s.display_name.toLowerCase().includes(q) ||
        (s.process_name || "").toLowerCase().includes(q) ||
        CATEGORY_MAP[s.category].label.toLowerCase().includes(q) ||
        (s.sample_title || "").toLowerCase().includes(q),
    );
  }, [analytics.recent_sessions, sessionFilter]);

  const handleExport = async (format: "csv" | "json") => {
    try {
      const res = await exportUsageReport(range, format, demoMode);
      setExportNotice(`已导出 ${res.record_count} 条时段记录至 ${res.file_path}`);
      setTimeout(() => setExportNotice(null), 5000);
    } catch (err) {
      setExportNotice(`导出失败：${String(err)}`);
    }
  };

  return (
    <div className="flex flex-col gap-5 pb-6">
      {/* Header & Controls */}
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h1 className="text-[20px] font-semibold tracking-tight text-[var(--text-primary)]">
            时段轨迹与多日趋势
          </h1>
          <p className="text-[12.5px] text-[var(--text-secondary)] mt-0.5 tabular-nums">
            {analytics.date_label} · 精确追踪每次软件切换的时间区间与每日作息演化
          </p>
        </div>

        <div className="flex flex-wrap items-center gap-2">
          {/* Date Step Navigation */}
          <div className="inline-flex items-center rounded-xl bg-[var(--bg-surface)] border border-[var(--border-hairline)] p-0.5">
            <button
              type="button"
              onClick={() => onSelectDate(shiftDateISO(selectedDate, -1))}
              className="w-7 h-7 rounded-lg flex items-center justify-center text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-subtle)] cursor-pointer"
              title="前一天"
            >
              <ChevronLeftIcon />
            </button>
            <input
              type="date"
              value={selectedDate}
              onChange={(e) => e.target.value && onSelectDate(e.target.value)}
              className="px-2 text-[12px] font-medium tabular-nums bg-transparent text-[var(--text-primary)] focus:outline-none cursor-pointer"
            />
            <button
              type="button"
              onClick={() => onSelectDate(shiftDateISO(selectedDate, 1))}
              className="w-7 h-7 rounded-lg flex items-center justify-center text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-subtle)] cursor-pointer"
              title="后一天"
            >
              <ChevronRightIcon />
            </button>
          </div>

          {/* Range Chips */}
          <div className="inline-flex p-1 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-hairline)]">
            {(
              [
                ["today", "今日"],
                ["7d", "近 7 日"],
                ["30d", "近 30 日"],
              ] as const
            ).map(([id, label]) => (
              <button
                key={id}
                type="button"
                onClick={() => onChangeRange(id)}
                className={`px-3 py-1 rounded-[8px] text-[12px] font-medium transition-all cursor-pointer ${
                  range === id
                    ? "bg-[var(--text-primary)] text-[var(--bg-surface)]"
                    : "text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
                }`}
              >
                {label}
              </button>
            ))}
          </div>

          <button
            type="button"
            onClick={() => void handleExport("csv")}
            className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-hairline)] hover:border-[var(--border-strong)] text-[12px] font-medium text-[var(--text-primary)] transition-colors cursor-pointer"
          >
            <ExportIcon size={14} />
            <span>导出 CSV</span>
          </button>
        </div>
      </div>

      {exportNotice && (
        <div className="card-surface px-4 py-2.5 text-[12px] text-[var(--text-primary)] border-l-4 border-l-[var(--status-ok)] select-text">
          {exportNotice}
        </div>
      )}

      {/* 24h Swimlane */}
      <SessionSwimlaneChart
        swimlanes={analytics.swimlanes}
        daySegments={analytics.day_segments}
        isToday={range === "today"}
        onSelectApp={onSelectApp}
      />

      {/* Period Rhythm + Multi-day Trend */}
      <PeriodRhythmChart
        periods={analytics.periods}
        dailyTrend={analytics.daily_trend}
        selectedDate={analytics.selected_date}
        onSelectDate={onSelectDate}
      />

      {/* Chronological Session Ledger */}
      <div className="card-surface p-5 flex flex-col gap-4">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
              软件使用时间段明细流
            </h3>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              按发生时间倒序排列，完整呈现每段屏幕专注的开始时间、结束时间与窗口上下文
            </p>
          </div>

          <div className="relative w-64">
            <SearchIcon
              size={14}
              className="absolute left-3 top-1/2 -translate-y-1/2 text-[var(--text-muted)]"
            />
            <input
              type="text"
              value={sessionFilter}
              onChange={(e) => setSessionFilter(e.target.value)}
              placeholder="筛选软件名称或窗口摘要..."
              className="w-full pl-8 pr-3 py-1.5 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12px] text-[var(--text-primary)] placeholder:text-[var(--text-muted)] focus:outline-none"
            />
          </div>
        </div>

        {filteredSessions.length === 0 ? (
          <div className="py-10 text-center text-[12.5px] text-[var(--text-muted)]">
            未找到符合条件的时段记录
          </div>
        ) : (
          <div className="flex flex-col divide-y divide-[var(--border-hairline)]/70">
            {filteredSessions.map((ev) => {
              const cat = CATEGORY_MAP[ev.category];
              return (
                <button
                  key={ev.id}
                  type="button"
                  onClick={() => onSelectApp(ev.app_id)}
                  className="w-full text-left py-2.5 px-2 rounded-xl hover:bg-[var(--bg-subtle)]/65 transition-colors grid grid-cols-1 md:grid-cols-[170px_1fr_92px] items-center gap-3 cursor-pointer group"
                >
                  <div className="flex items-center gap-2 text-[12px] tabular-nums text-[var(--text-secondary)]">
                    <span className="text-[11px] text-[var(--text-muted)]">{ev.date.slice(5)}</span>
                    <span className="font-medium text-[var(--text-primary)]">
                      {ev.start_time_label.slice(0, 5)} - {ev.end_time_label.slice(0, 5)}
                    </span>
                  </div>

                  <div className="flex items-center gap-2.5 min-w-0">
                    <AppAvatar
                      name={ev.display_name}
                      category={ev.category}
                      iconDataUrl={ev.icon_data_url}
                      size={28}
                    />
                    <div className="min-w-0 flex-1">
                      <div className="flex items-center gap-2">
                        <span className="text-[13px] font-medium text-[var(--text-primary)] group-hover:text-[var(--accent)] truncate">
                          {ev.display_name}
                        </span>
                        <span
                          className="px-2 py-0.5 rounded-full text-[10.5px] font-medium shrink-0"
                          style={{
                            backgroundColor: `${cat.color}16`,
                            color: cat.color,
                          }}
                        >
                          {cat.shortLabel}
                        </span>
                      </div>
                      {ev.sample_title && (
                        <p className="text-[11px] text-[var(--text-muted)] truncate mt-0.5">
                          {ev.sample_title}
                        </p>
                      )}
                    </div>
                  </div>

                  <div className="text-right text-[12.5px] font-semibold tabular-nums text-[var(--text-primary)]">
                    {formatDuration(ev.duration_secs)}
                  </div>
                </button>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
};
