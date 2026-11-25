import React, { useMemo, useState } from "react";
import type { AppCategory, HourlyCategoryBucketDto } from "../../types";
import { CATEGORY_LIST, CATEGORY_MAP } from "../../types";
import { formatDuration, formatMinutesAxis } from "../../lib/format";

interface HourlyStackedChartProps {
  hourly: HourlyCategoryBucketDto[];
  isSingleDay: boolean;
}

export const HourlyStackedChart: React.FC<HourlyStackedChartProps> = ({
  hourly,
  isSingleDay,
}) => {
  const [activeCategory, setActiveCategory] = useState<AppCategory | null>(null);
  const [hoveredHour, setHoveredHour] = useState<number | null>(null);

  const usedCategories = useMemo(() => {
    const present = new Set<AppCategory>();
    for (const h of hourly) {
      for (const [k, v] of Object.entries(h.by_category)) {
        if ((v || 0) > 0) present.add(k as AppCategory);
      }
    }
    return CATEGORY_LIST.filter((c) => present.has(c.id));
  }, [hourly]);

  const maxMinutes = useMemo(() => {
    if (isSingleDay) return 60;
    const rawMax = Math.max(
      30,
      ...hourly.map((h) => Math.ceil(h.total_secs / 60)),
    );
    const step = rawMax <= 120 ? 30 : rawMax <= 360 ? 60 : 120;
    return Math.ceil(rawMax / step) * step;
  }, [hourly, isSingleDay]);

  const chartW = 740;
  const chartH = 188;
  const padLeft = 40;
  const padRight = 12;
  const padTop = 14;
  const padBottom = 26;
  const plotW = chartW - padLeft - padRight;
  const plotH = chartH - padTop - padBottom;
  const slotW = plotW / 24;
  const barW = Math.max(12, slotW * 0.56);

  const yTicks = [0, 0.25, 0.5, 0.75, 1];
  const hoveredBucket = hoveredHour !== null ? hourly[hoveredHour] : null;

  return (
    <div className="card-surface p-5 flex flex-col gap-4 relative">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div>
          <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
            24 小时分时活跃分布
          </h3>
          <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
            {isSingleDay
              ? "按整点小时统计各类软件分钟占比，点击图例可聚焦单一类别"
              : "统计区间内每个小时段累计活跃时长，揭示全天作息与精力高峰"}
          </p>
        </div>

        {/* Category Filter Legend */}
        <div className="flex flex-wrap items-center gap-1.5">
          {usedCategories.map((cat) => {
            const selected = activeCategory === cat.id;
            const dimmed = activeCategory !== null && !selected;
            return (
              <button
                key={cat.id}
                type="button"
                onClick={() => setActiveCategory(selected ? null : cat.id)}
                className={`inline-flex items-center gap-1.5 px-2 py-1 rounded-lg text-[11px] font-medium border transition-all cursor-pointer ${
                  selected
                    ? "bg-[var(--bg-subtle)] border-[var(--border-strong)] text-[var(--text-primary)]"
                    : "border-transparent hover:bg-[var(--bg-subtle)] text-[var(--text-secondary)]"
                } ${dimmed ? "opacity-45" : "opacity-100"}`}
              >
                <span
                  className="w-2 h-2 rounded-full"
                  style={{ backgroundColor: cat.color }}
                />
                <span>{cat.shortLabel}</span>
              </button>
            );
          })}
        </div>
      </div>

      <div className="relative w-full">
        <svg viewBox={`0 0 ${chartW} ${chartH}`} className="w-full h-auto overflow-visible">
          {/* Y-axis gridlines */}
          {yTicks.map((ratio) => {
            const y = padTop + plotH * (1 - ratio);
            const valMin = maxMinutes * ratio;
            return (
              <g key={ratio}>
                <line
                  x1={padLeft}
                  y1={y}
                  x2={chartW - padRight}
                  y2={y}
                  stroke="var(--border-hairline)"
                  strokeDasharray={ratio === 0 ? undefined : "3 3"}
                  strokeWidth={1}
                />
                <text
                  x={padLeft - 8}
                  y={y + 3.5}
                  textAnchor="end"
                  fill="var(--text-muted)"
                  fontSize="10"
                  className="tabular-nums"
                >
                  {ratio === 0 ? "0" : formatMinutesAxis(valMin)}
                </text>
              </g>
            );
          })}

          {/* 24 Hour Stacked Columns */}
          {hourly.map((bucket, idx) => {
            const cx = padLeft + idx * slotW + slotW / 2;
            const bx = cx - barW / 2;
            const isHovered = hoveredHour === idx;

            let yCursor = padTop + plotH;
            const segments: Array<{
              cat: AppCategory;
              y: number;
              h: number;
              color: string;
            }> = [];

            for (const meta of CATEGORY_LIST) {
              const secs = bucket.by_category[meta.id] || 0;
              if (secs <= 0) continue;
              const mins = secs / 60;
              const segH = Math.max(1.5, (mins / maxMinutes) * plotH);
              const segY = yCursor - segH;
              segments.push({
                cat: meta.id,
                y: segY,
                h: segH,
                color: meta.color,
              });
              yCursor = segY;
            }

            return (
              <g
                key={bucket.hour}
                onMouseEnter={() => setHoveredHour(idx)}
                onMouseLeave={() => setHoveredHour(null)}
                className="cursor-pointer"
              >
                {/* Hover Column Highlight */}
                {isHovered && (
                  <rect
                    x={padLeft + idx * slotW + 2}
                    y={padTop}
                    width={slotW - 4}
                    height={plotH}
                    rx={5}
                    fill="var(--bg-subtle)"
                    opacity={0.75}
                  />
                )}

                {/* Invisible full-height hit box */}
                <rect
                  x={padLeft + idx * slotW}
                  y={padTop}
                  width={slotW}
                  height={plotH + padBottom}
                  fill="transparent"
                />

                {segments.map((seg, sIdx) => {
                  const dimmed = activeCategory !== null && activeCategory !== seg.cat;
                  const isTop = sIdx === segments.length - 1;
                  return (
                    <rect
                      key={seg.cat}
                      x={bx}
                      y={seg.y}
                      width={barW}
                      height={seg.h}
                      rx={isTop ? 3 : 0.5}
                      fill={seg.color}
                      opacity={dimmed ? 0.16 : isHovered ? 1 : 0.9}
                    />
                  );
                })}

                {/* X-axis tick labels every 3 hours */}
                {(idx % 3 === 0 || idx === 23) && (
                  <text
                    x={cx}
                    y={chartH - 7}
                    textAnchor="middle"
                    fill={isHovered ? "var(--text-primary)" : "var(--text-muted)"}
                    fontSize="10"
                    fontWeight={isHovered ? 600 : 400}
                    className="tabular-nums"
                  >
                    {String(idx).padStart(2, "0")}
                  </text>
                )}
              </g>
            );
          })}
        </svg>

        {/* Floating Tooltip */}
        {hoveredBucket && (
          <div
            className="pointer-events-none absolute top-2 z-20 px-3 py-2.5 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-strong)] min-w-[200px]"
            style={{
              left:
                hoveredBucket.hour > 15
                  ? `calc(${((hoveredBucket.hour + 0.5) / 24) * 100}% - 210px)`
                  : `calc(${((hoveredBucket.hour + 0.5) / 24) * 100}% + 12px)`,
              boxShadow: "var(--shadow-popover)",
            }}
          >
            <div className="flex items-center justify-between gap-4 pb-1.5 border-b border-[var(--border-hairline)]">
              <span className="text-[12px] font-semibold tabular-nums text-[var(--text-primary)]">
                {String(hoveredBucket.hour).padStart(2, "0")}:00 -{" "}
                {String((hoveredBucket.hour + 1) % 24).padStart(2, "0")}:00
              </span>
              <span className="text-[12px] font-semibold tabular-nums text-[var(--accent)]">
                {formatDuration(hoveredBucket.total_secs)}
              </span>
            </div>

            {hoveredBucket.total_secs === 0 ? (
              <p className="text-[11px] text-[var(--text-muted)] pt-1.5">该时段无活跃屏幕记录</p>
            ) : (
              <div className="flex flex-col gap-1 pt-1.5">
                {hoveredBucket.top_apps.map((app) => (
                  <div
                    key={app.app_id}
                    className="flex items-center justify-between gap-3 text-[11.5px]"
                  >
                    <div className="flex items-center gap-1.5 min-w-0">
                      <span
                        className="w-1.5 h-1.5 rounded-full shrink-0"
                        style={{ backgroundColor: CATEGORY_MAP[app.category].color }}
                      />
                      <span className="text-[var(--text-primary)] truncate">
                        {app.display_name}
                      </span>
                    </div>
                    <span className="text-[var(--text-secondary)] tabular-nums shrink-0">
                      {formatDuration(app.duration_secs, true)}
                    </span>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};
