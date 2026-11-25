import React, { useEffect, useState } from "react";
import type { AppCategory, AppDetailDto } from "../types";
import { CATEGORY_LIST, CATEGORY_MAP } from "../types";
import { formatDuration } from "../lib/format";
import { fetchAppDetail, updateAppOverride } from "../lib/ipc";
import { AppAvatar } from "./AppAvatar";
import { CloseWinIcon } from "./Icons";

interface AppDetailDrawerProps {
  appId: string | null;
  date: string;
  demoMode: boolean;
  onClose: () => void;
  onAppUpdated: () => void;
}

export const AppDetailDrawer: React.FC<AppDetailDrawerProps> = ({
  appId,
  date,
  demoMode,
  onClose,
  onAppUpdated,
}) => {
  const [detail, setDetail] = useState<AppDetailDto | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!appId) {
      setDetail(null);
      return;
    }
    let cancelled = false;
    setLoading(true);
    fetchAppDetail(appId, date, demoMode)
      .then((res) => {
        if (!cancelled) setDetail(res);
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [appId, date, demoMode]);

  if (!appId) return null;

  const handleCategoryChange = async (nextCategory: AppCategory) => {
    if (!detail) return;
    await updateAppOverride({
      app_id: detail.app.app_id,
      category: nextCategory,
    });
    const refreshed = await fetchAppDetail(detail.app.app_id, date, demoMode);
    setDetail(refreshed);
    onAppUpdated();
  };

  const maxHourSecs = detail ? Math.max(600, ...detail.hourly_secs) : 3600;
  const catColor = detail ? CATEGORY_MAP[detail.app.category].color : "var(--accent)";

  return (
    <div
      className="fixed inset-0 z-40 bg-black/25 backdrop-blur-[1.5px] flex justify-end"
      onClick={onClose}
    >
      <aside
        onClick={(e) => e.stopPropagation()}
        className="w-full max-w-[460px] h-full bg-[var(--bg-surface)] border-l border-[var(--border-hairline)] flex flex-col"
        style={{ boxShadow: "var(--shadow-popover)" }}
      >
        {/* Drawer Header */}
        <div className="px-5 py-4 border-b border-[var(--border-hairline)] flex items-center justify-between gap-3">
          <span className="text-[12px] font-medium text-[var(--text-muted)] tabular-nums">
            单软件时段洞察 · {date}
          </span>
          <button
            type="button"
            onClick={onClose}
            className="w-7 h-7 rounded-lg flex items-center justify-center text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-subtle)] cursor-pointer"
            aria-label="关闭详情"
          >
            <CloseWinIcon />
          </button>
        </div>

        {/* Drawer Body */}
        <div className="flex-1 scroll-area p-5 flex flex-col gap-5">
          {loading || !detail ? (
            <div className="py-16 text-center text-[13px] text-[var(--text-muted)]">
              正在解析软件使用轨迹...
            </div>
          ) : (
            <>
              {/* Identity Block */}
              <div className="flex items-start gap-3.5">
                <AppAvatar
                  name={detail.app.display_name}
                  category={detail.app.category}
                  iconDataUrl={detail.app.icon_data_url}
                  size={48}
                />
                <div className="min-w-0 flex-1">
                  <h2 className="text-[18px] font-semibold text-[var(--text-primary)] tracking-tight truncate">
                    {detail.app.display_name}
                  </h2>
                  <p className="text-[12px] text-[var(--text-secondary)] mt-0.5 leading-relaxed">
                    {detail.app.description}
                  </p>
                  <div className="flex flex-wrap items-center gap-2 mt-2.5">
                    <select
                      value={detail.app.category}
                      onChange={(e) => void handleCategoryChange(e.target.value as AppCategory)}
                      className="px-2.5 py-1 rounded-lg text-[11.5px] font-medium bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[var(--text-primary)] focus:outline-none cursor-pointer"
                    >
                      {CATEGORY_LIST.map((cat) => (
                        <option key={cat.id} value={cat.id}>
                          {cat.label}
                        </option>
                      ))}
                    </select>
                    <span className="text-[11px] text-[var(--text-muted)]">
                      {detail.app.publisher}
                    </span>
                    {detail.app.process_names.length > 0 && (
                      <span className="px-2 py-0.5 rounded-md bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[10.5px] text-[var(--text-secondary)]">
                        进程：{detail.app.process_names.join(", ")}
                      </span>
                    )}
                  </div>
                </div>
              </div>

              {/* 4 Compact KPI Tiles */}
              <div className="grid grid-cols-2 gap-2.5">
                <div className="p-3 rounded-xl bg-[var(--bg-subtle)]/75 border border-[var(--border-hairline)]">
                  <div className="text-[11px] text-[var(--text-muted)]">当日总使用时长</div>
                  <div className="text-[18px] font-semibold tabular-nums text-[var(--text-primary)] mt-1">
                    {formatDuration(detail.total_secs)}
                  </div>
                  <div className="text-[10.5px] text-[var(--text-secondary)] tabular-nums mt-0.5">
                    占当日屏幕 {detail.share_percentage.toFixed(1)}%
                  </div>
                </div>

                <div className="p-3 rounded-xl bg-[var(--bg-subtle)]/75 border border-[var(--border-hairline)]">
                  <div className="text-[11px] text-[var(--text-muted)]">活跃高峰小时</div>
                  <div className="text-[16px] font-semibold tabular-nums text-[var(--text-primary)] mt-1">
                    {detail.peak_hour_label}
                  </div>
                  <div className="text-[10.5px] text-[var(--text-secondary)] tabular-nums mt-0.5">
                    共启动 {detail.session_count} 个时段
                  </div>
                </div>

                <div className="p-3 rounded-xl bg-[var(--bg-subtle)]/75 border border-[var(--border-hairline)]">
                  <div className="text-[11px] text-[var(--text-muted)]">最长单次连续</div>
                  <div className="text-[16px] font-semibold tabular-nums text-[var(--text-primary)] mt-1">
                    {detail.longest_session_secs > 0
                      ? formatDuration(detail.longest_session_secs)
                      : "--"}
                  </div>
                </div>

                <div className="p-3 rounded-xl bg-[var(--bg-subtle)]/75 border border-[var(--border-hairline)]">
                  <div className="text-[11px] text-[var(--text-muted)]">平均单次停留</div>
                  <div className="text-[16px] font-semibold tabular-nums text-[var(--text-primary)] mt-1">
                    {detail.average_session_secs > 0
                      ? formatDuration(detail.average_session_secs)
                      : "--"}
                  </div>
                </div>
              </div>

              {/* 24-Hour Mini Curve for this app */}
              <div className="p-4 rounded-xl bg-[var(--bg-subtle)]/55 border border-[var(--border-hairline)] flex flex-col gap-2.5">
                <div className="flex items-center justify-between text-[12px]">
                  <span className="font-semibold text-[var(--text-primary)]">当日 24 小时分布</span>
                  <span className="text-[11px] text-[var(--text-muted)] tabular-nums">
                    00:00 - 24:00
                  </span>
                </div>
                <svg viewBox="0 0 384 86" className="w-full h-auto">
                  {detail.hourly_secs.map((secs, h) => {
                    const barH = secs > 0 ? Math.max(4, (secs / maxHourSecs) * 62) : 2;
                    return (
                      <g key={h}>
                        <rect
                          x={h * 16 + 2}
                          y={66 - barH}
                          width={11}
                          height={barH}
                          rx={2.5}
                          fill={secs > 0 ? catColor : "var(--border-hairline)"}
                          opacity={secs > 0 ? 0.9 : 0.7}
                        >
                          <title>{`${String(h).padStart(2, "0")}:00 - ${formatDuration(secs)}`}</title>
                        </rect>
                        {h % 6 === 0 && (
                          <text
                            x={h * 16 + 7}
                            y={81}
                            textAnchor="middle"
                            fontSize="9"
                            fill="var(--text-muted)"
                            className="tabular-nums"
                          >
                            {String(h).padStart(2, "0")}
                          </text>
                        )}
                      </g>
                    );
                  })}
                </svg>
              </div>

              {/* Exact Session Intervals */}
              <div className="flex flex-col gap-2.5">
                <h4 className="text-[13px] font-semibold text-[var(--text-primary)]">
                  当日精确使用时间段 ({detail.sessions.length})
                </h4>
                {detail.sessions.length === 0 ? (
                  <p className="text-[12px] text-[var(--text-muted)] py-4 text-center">
                    当日暂无该软件的前台使用会话
                  </p>
                ) : (
                  <div className="flex flex-col gap-1.5">
                    {detail.sessions.map((s) => (
                      <div
                        key={s.id}
                        className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)]/65 border border-[var(--border-hairline)] flex items-center justify-between gap-3"
                      >
                        <div className="min-w-0">
                          <div className="text-[12px] font-medium tabular-nums text-[var(--text-primary)]">
                            {s.start_time_label} - {s.end_time_label}
                          </div>
                          {s.sample_title && (
                            <div className="text-[11px] text-[var(--text-muted)] truncate mt-0.5">
                              {s.sample_title}
                            </div>
                          )}
                        </div>
                        <span className="text-[12px] font-semibold tabular-nums text-[var(--accent)] shrink-0">
                          {formatDuration(s.duration_secs)}
                        </span>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </>
          )}
        </div>
      </aside>
    </div>
  );
};
