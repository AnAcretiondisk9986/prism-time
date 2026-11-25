import React, { useMemo, useState } from "react";
import type { AppCategory, SwimlaneRowDto, SwimlaneSegmentDto } from "../../types";
import { CATEGORY_LIST, CATEGORY_MAP } from "../../types";
import { formatDuration } from "../../lib/format";
import { AppAvatar } from "../AppAvatar";
import { CategorySvgIcon } from "../Icons";

interface SessionSwimlaneChartProps {
  swimlanes: SwimlaneRowDto[];
  daySegments: SwimlaneSegmentDto[];
  isToday: boolean;
  onSelectApp: (appId: string) => void;
}

type LaneMode = "apps" | "categories";

export const SessionSwimlaneChart: React.FC<SessionSwimlaneChartProps> = ({
  swimlanes,
  daySegments,
  isToday,
  onSelectApp,
}) => {
  const [mode, setMode] = useState<LaneMode>("apps");
  const [hoveredSeg, setHoveredSeg] = useState<{
    seg: SwimlaneSegmentDto;
    xPct: number;
  } | null>(null);

  const nowMinute = useMemo(() => {
    const d = new Date();
    return d.getHours() * 60 + d.getMinutes();
  }, []);

  const categoryLanes = useMemo(() => {
    const map = new Map<AppCategory, SwimlaneSegmentDto[]>();
    for (const seg of daySegments) {
      const arr = map.get(seg.category) || [];
      arr.push(seg);
      map.set(seg.category, arr);
    }
    return CATEGORY_LIST.map((meta) => {
      const segs = map.get(meta.id) || [];
      const total = segs.reduce((acc, s) => acc + s.duration_secs, 0);
      return { meta, segs, total };
    })
      .filter((row) => row.total > 0)
      .sort((a, b) => b.total - a.total);
  }, [daySegments]);

  const hourTicks = [0, 4, 8, 12, 16, 20, 24];

  const hasContent = daySegments.length > 0;

  return (
    <div className="card-surface p-5 flex flex-col gap-4 relative">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
            24 小时使用时间段图谱
          </h3>
          <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
            精准呈现软件与类别在全天各时间段的起止分布，悬浮查看详情，点击色块深入分析
          </p>
        </div>

        <div className="inline-flex p-0.5 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)]">
          <button
            type="button"
            onClick={() => setMode("apps")}
            className={`px-3 py-1 rounded-[10px] text-[12px] font-medium transition-all cursor-pointer ${
              mode === "apps"
                ? "bg-[var(--bg-surface)] text-[var(--text-primary)] shadow-xs"
                : "text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
            }`}
          >
            按软件分泳道
          </button>
          <button
            type="button"
            onClick={() => setMode("categories")}
            className={`px-3 py-1 rounded-[10px] text-[12px] font-medium transition-all cursor-pointer ${
              mode === "categories"
                ? "bg-[var(--bg-surface)] text-[var(--text-primary)] shadow-xs"
                : "text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
            }`}
          >
            按类别时间带
          </button>
        </div>
      </div>

      {!hasContent ? (
        <div className="h-[190px] flex flex-col items-center justify-center rounded-xl bg-[var(--bg-subtle)]/50 border border-dashed border-[var(--border-hairline)] text-center px-4">
          <p className="text-[13px] font-medium text-[var(--text-secondary)]">
            当日暂无足够长度的使用时间段记录
          </p>
          <p className="text-[11.5px] text-[var(--text-muted)] mt-1">
            澄镜正在后台实时采样，切换并停留使用任意桌面软件即可生成连续时段光带；也可点击顶部「图表演示」预览效果
          </p>
        </div>
      ) : (
        <div className="relative">
          {/* Header Time Axis */}
          <div className="grid grid-cols-[156px_1fr_64px] items-center gap-3 pb-2 border-b border-[var(--border-hairline)]">
            <span className="text-[11px] font-medium text-[var(--text-muted)] pl-1">
              {mode === "apps" ? "高频应用泳道" : "软件类别泳道"}
            </span>
            <div className="relative h-4">
              {hourTicks.map((h) => {
                const pct = (h / 24) * 100;
                const align =
                  h === 0 ? "translate-x-0" : h === 24 ? "-translate-x-full" : "-translate-x-1/2";
                return (
                  <span
                    key={h}
                    className={`absolute top-0 text-[10.5px] font-medium tabular-nums whitespace-nowrap text-[var(--text-muted)] ${align}`}
                    style={{ left: `${pct}%` }}
                  >
                    {String(h).padStart(2, "0")}:00
                  </span>
                );
              })}
            </div>
            <span className="text-[11px] font-medium text-[var(--text-muted)] text-right pr-1">
              当日累计
            </span>
          </div>

          {/* Rows */}
          <div className="divide-y divide-[var(--border-hairline)]/70">
            {mode === "apps"
              ? swimlanes.map((row) => (
                  <div
                    key={row.app_id}
                    className="grid grid-cols-[156px_1fr_64px] items-center gap-3 py-2 hover:bg-[var(--bg-subtle)]/45 transition-colors rounded-lg px-1"
                  >
                    <button
                      type="button"
                      onClick={() => onSelectApp(row.app_id)}
                      className="flex items-center gap-2 min-w-0 text-left cursor-pointer group"
                    >
                      <AppAvatar
                        name={row.display_name}
                        category={row.category}
                        iconDataUrl={row.icon_data_url}
                        size={24}
                      />
                      <span className="text-[12.5px] font-medium text-[var(--text-primary)] truncate group-hover:text-[var(--accent)] transition-colors">
                        {row.display_name}
                      </span>
                    </button>

                    <TrackSvg
                      segments={row.segments}
                      isToday={isToday}
                      nowMinute={nowMinute}
                      onHover={setHoveredSeg}
                      onClickSeg={(seg) => onSelectApp(seg.app_id)}
                    />

                    <span className="text-[12px] font-medium tabular-nums text-[var(--text-secondary)] text-right pr-1">
                      {formatDuration(row.total_secs, true)}
                    </span>
                  </div>
                ))
              : categoryLanes.map(({ meta, segs, total }) => (
                  <div
                    key={meta.id}
                    className="grid grid-cols-[156px_1fr_64px] items-center gap-3 py-2 hover:bg-[var(--bg-subtle)]/45 transition-colors rounded-lg px-1"
                  >
                    <div className="flex items-center gap-2 min-w-0">
                      <div
                        className="w-6 h-6 rounded-lg flex items-center justify-center shrink-0"
                        style={{
                          backgroundColor: `${meta.color}18`,
                          color: meta.color,
                        }}
                      >
                        <CategorySvgIcon category={meta.id} size={13} />
                      </div>
                      <span className="text-[12.5px] font-medium text-[var(--text-primary)] truncate">
                        {meta.label}
                      </span>
                    </div>

                    <TrackSvg
                      segments={segs}
                      isToday={isToday}
                      nowMinute={nowMinute}
                      onHover={setHoveredSeg}
                      onClickSeg={(seg) => onSelectApp(seg.app_id)}
                    />

                    <span className="text-[12px] font-medium tabular-nums text-[var(--text-secondary)] text-right pr-1">
                      {formatDuration(total, true)}
                    </span>
                  </div>
                ))}
          </div>

          {/* Floating Tooltip */}
          {hoveredSeg && (
            <div
              className="pointer-events-none absolute -top-2 z-30 px-3 py-2 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-strong)] text-[12px] min-w-[205px] max-w-[280px]"
              style={{
                left: `clamp(170px, ${hoveredSeg.xPct * 0.72 + 18}%, calc(100% - 260px))`,
                boxShadow: "var(--shadow-popover)",
              }}
            >
              <div className="flex items-center justify-between gap-2">
                <div className="flex items-center gap-1.5 min-w-0">
                  <span
                    className="w-2 h-2 rounded-full shrink-0"
                    style={{ backgroundColor: CATEGORY_MAP[hoveredSeg.seg.category].color }}
                  />
                  <span className="font-semibold text-[var(--text-primary)] truncate">
                    {hoveredSeg.seg.display_name}
                  </span>
                </div>
                <span className="text-[11px] tabular-nums font-semibold text-[var(--accent)] shrink-0">
                  {formatDuration(hoveredSeg.seg.duration_secs)}
                </span>
              </div>
              <div className="flex items-center justify-between text-[11px] text-[var(--text-secondary)] mt-1 tabular-nums">
                <span>{CATEGORY_MAP[hoveredSeg.seg.category].label}</span>
                <span>
                  {hoveredSeg.seg.start_label} - {hoveredSeg.seg.end_label}
                </span>
              </div>
              {hoveredSeg.seg.sample_title && (
                <p className="text-[10.5px] text-[var(--text-muted)] mt-1 truncate">
                  {hoveredSeg.seg.sample_title}
                </p>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
};

interface TrackSvgProps {
  segments: SwimlaneSegmentDto[];
  isToday: boolean;
  nowMinute: number;
  onHover: (val: { seg: SwimlaneSegmentDto; xPct: number } | null) => void;
  onClickSeg: (seg: SwimlaneSegmentDto) => void;
}

const TrackSvg: React.FC<TrackSvgProps> = ({
  segments,
  isToday,
  nowMinute,
  onHover,
  onClickSeg,
}) => {
  return (
    <div className="relative h-6 w-full">
      <svg
        viewBox="0 0 1440 24"
        preserveAspectRatio="none"
        className="w-full h-full overflow-visible rounded-md"
      >
        {/* Subtle night & evening backdrop zones */}
        <rect x={0} y={4} width={360} height={16} rx={3} fill="var(--bg-subtle)" opacity={0.7} />
        <rect x={360} y={4} width={720} height={16} fill="var(--bg-subtle)" opacity={0.38} />
        <rect x={1080} y={4} width={360} height={16} rx={3} fill="var(--bg-subtle)" opacity={0.7} />

        {/* Vertical 4h guide hairlines matching axis ticks */}
        {[240, 480, 720, 960, 1200].map((m) => (
          <line
            key={m}
            x1={m}
            y1={2}
            x2={m}
            y2={22}
            stroke="var(--border-hairline)"
            strokeWidth={1}
          />
        ))}

        {/* Segments */}
        {segments.map((seg, idx) => {
          const rawW = Math.max(4, seg.end_minute_of_day - seg.start_minute_of_day);
          const color = CATEGORY_MAP[seg.category].color;
          const midPct = ((seg.start_minute_of_day + rawW / 2) / 1440) * 100;
          return (
            <rect
              key={`${seg.app_id}-${seg.start_ts}-${idx}`}
              x={seg.start_minute_of_day}
              y={4}
              width={rawW}
              height={16}
              rx={3}
              fill={color}
              className="cursor-pointer transition-opacity hover:opacity-85"
              onMouseEnter={() => onHover({ seg, xPct: midPct })}
              onMouseLeave={() => onHover(null)}
              onClick={() => onClickSeg(seg)}
            />
          );
        })}

        {/* Current time needle */}
        {isToday && (
          <line
            x1={nowMinute}
            y1={1}
            x2={nowMinute}
            y2={23}
            stroke="var(--accent)"
            strokeWidth={1.6}
          />
        )}
      </svg>
    </div>
  );
};
