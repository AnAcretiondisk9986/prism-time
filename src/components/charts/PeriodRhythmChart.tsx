import React, { useMemo, useState } from "react";
import type { AppCategory, DailyTrendPointDto, PeriodBucketDto } from "../../types";
import { CATEGORY_LIST, CATEGORY_MAP } from "../../types";
import { formatDuration } from "../../lib/format";

interface PeriodRhythmChartProps {
  periods: PeriodBucketDto[];
  dailyTrend: DailyTrendPointDto[];
  selectedDate: string;
  onSelectDate?: (date: string) => void;
}

export const PeriodRhythmChart: React.FC<PeriodRhythmChartProps> = ({
  periods,
  dailyTrend,
  selectedDate,
  onSelectDate,
}) => {
  const [hoveredTrendIdx, setHoveredTrendIdx] = useState<number | null>(null);

  const maxPeriodSecs = useMemo(
    () => Math.max(60, ...periods.map((p) => p.total_secs)),
    [periods],
  );

  const maxTrendSecs = useMemo(
    () => Math.max(1800, ...dailyTrend.map((d) => d.total_secs)),
    [dailyTrend],
  );

  const avgTrendSecs = useMemo(() => {
    const active = dailyTrend.filter((d) => d.total_secs > 0);
    if (active.length === 0) return 0;
    return Math.round(active.reduce((a, b) => a + b.total_secs, 0) / active.length);
  }, [dailyTrend]);

  const chartW = 540;
  const chartH = 185;
  const padL = 36;
  const padR = 10;
  const padT = 18;
  const padB = 26;
  const plotW = chartW - padL - padR;
  const plotH = chartH - padT - padB;
  const n = Math.max(1, dailyTrend.length);
  const slotW = plotW / n;
  const barW = Math.min(26, Math.max(7, slotW * 0.56));
  const avgY = padT + plotH * (1 - avgTrendSecs / maxTrendSecs);

  const hoveredDay = hoveredTrendIdx !== null ? dailyTrend[hoveredTrendIdx] : null;

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
      {/* Left: 5 Time-of-Day Rhythm Matrix */}
      <div className="card-surface p-5 flex flex-col gap-4">
        <div>
          <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
            五时段用屏节律分布
          </h3>
          <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
            深夜至夜晚各作息窗口的使用强度与主导软件类别
          </p>
        </div>

        <div className="flex flex-col gap-3">
          {periods.map((p) => {
            const widthRatio = Math.min(100, (p.total_secs / maxPeriodSecs) * 100);
            const domMeta = p.dominant_category ? CATEGORY_MAP[p.dominant_category] : null;
            return (
              <div key={p.period_id} className="flex flex-col gap-1.5">
                <div className="flex items-center justify-between text-[12px]">
                  <div className="flex items-center gap-2">
                    <span className="font-semibold text-[var(--text-primary)]">{p.label}</span>
                    <span className="text-[11px] tabular-nums text-[var(--text-muted)]">
                      {p.time_range}
                    </span>
                    {domMeta && (
                      <span
                        className="px-2 py-0.5 rounded-full text-[10.5px] font-medium"
                        style={{
                          backgroundColor: `${domMeta.color}16`,
                          color: domMeta.color,
                        }}
                      >
                        主导 · {domMeta.shortLabel}
                      </span>
                    )}
                  </div>
                  <span className="font-semibold tabular-nums text-[var(--text-primary)]">
                    {p.total_secs > 0 ? formatDuration(p.total_secs, true) : "--"}
                  </span>
                </div>

                {/* Segmented Progress Bar */}
                <div className="h-2.5 w-full rounded-full bg-[var(--bg-subtle)] overflow-hidden flex">
                  {p.total_secs > 0 && (
                    <div
                      className="h-full flex rounded-full overflow-hidden transition-all duration-300"
                      style={{ width: `${Math.max(2, widthRatio)}%` }}
                    >
                      {CATEGORY_LIST.map((cat) => {
                        const val = p.by_category[cat.id] || 0;
                        if (val <= 0) return null;
                        const innerPct = (val / p.total_secs) * 100;
                        return (
                          <div
                            key={cat.id}
                            style={{
                              width: `${innerPct}%`,
                              backgroundColor: cat.color,
                            }}
                            title={`${cat.label}: ${formatDuration(val, true)}`}
                          />
                        );
                      })}
                    </div>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* Right: Multi-Day Stacked Trend */}
      <div className="card-surface p-5 flex flex-col gap-4 relative">
        <div className="flex items-start justify-between gap-3">
          <div>
            <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
              连续多日屏幕时长趋势
            </h3>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              每日各类软件使用堆叠对比，点击柱条切换至当日详情
            </p>
          </div>
          {avgTrendSecs > 0 && (
            <span className="px-2.5 py-1 rounded-lg bg-[var(--bg-subtle)] text-[11px] font-medium tabular-nums text-[var(--text-secondary)] shrink-0">
              日均 {formatDuration(avgTrendSecs, true)}
            </span>
          )}
        </div>

        <div className="relative w-full">
          <svg viewBox={`0 0 ${chartW} ${chartH}`} className="w-full h-auto overflow-visible">
            {[0, 0.5, 1].map((r) => {
              const y = padT + plotH * (1 - r);
              const hours = (maxTrendSecs * r) / 3600;
              return (
                <g key={r}>
                  <line
                    x1={padL}
                    y1={y}
                    x2={chartW - padR}
                    y2={y}
                    stroke="var(--border-hairline)"
                    strokeDasharray={r === 0 ? undefined : "3 3"}
                  />
                  <text
                    x={padL - 6}
                    y={y + 3.5}
                    textAnchor="end"
                    fill="var(--text-muted)"
                    fontSize="10"
                    className="tabular-nums"
                  >
                    {r === 0 ? "0" : `${hours.toFixed(1)}h`}
                  </text>
                </g>
              );
            })}

            {avgTrendSecs > 0 && (
              <line
                x1={padL}
                y1={avgY}
                x2={chartW - padR}
                y2={avgY}
                stroke="var(--accent)"
                strokeWidth={1}
                strokeDasharray="4 4"
                opacity={0.75}
              />
            )}

            {dailyTrend.map((point, idx) => {
              const cx = padL + idx * slotW + slotW / 2;
              const bx = cx - barW / 2;
              const isSelected = point.date === selectedDate;
              const isHovered = hoveredTrendIdx === idx;
              const showLabel =
                dailyTrend.length <= 10 || idx % 5 === 0 || idx === dailyTrend.length - 1;

              let yCursor = padT + plotH;
              const stacks: Array<{ cat: AppCategory; y: number; h: number; color: string }> = [];
              for (const meta of CATEGORY_LIST) {
                const secs = point.by_category[meta.id] || 0;
                if (secs <= 0) continue;
                const h = Math.max(1.5, (secs / maxTrendSecs) * plotH);
                const y = yCursor - h;
                stacks.push({ cat: meta.id, y, h, color: meta.color });
                yCursor = y;
              }

              return (
                <g
                  key={point.date}
                  onMouseEnter={() => setHoveredTrendIdx(idx)}
                  onMouseLeave={() => setHoveredTrendIdx(null)}
                  onClick={() => onSelectDate?.(point.date)}
                  className="cursor-pointer"
                >
                  {(isSelected || isHovered) && (
                    <rect
                      x={padLeft(padL, idx, slotW) + 1}
                      y={padT}
                      width={slotW - 2}
                      height={plotH}
                      rx={4}
                      fill="var(--bg-subtle)"
                      opacity={isSelected ? 0.85 : 0.55}
                    />
                  )}
                  <rect
                    x={padL + idx * slotW}
                    y={padT}
                    width={slotW}
                    height={plotH + padB}
                    fill="transparent"
                  />
                  {stacks.map((s, sIdx) => (
                    <rect
                      key={s.cat}
                      x={bx}
                      y={s.y}
                      width={barW}
                      height={s.h}
                      rx={sIdx === stacks.length - 1 ? 3 : 0.5}
                      fill={s.color}
                      opacity={isSelected || isHovered ? 1 : 0.86}
                    />
                  ))}

                  {showLabel && (
                    <text
                      x={cx}
                      y={chartH - 7}
                      textAnchor="middle"
                      fill={isSelected ? "var(--accent)" : "var(--text-muted)"}
                      fontSize="10"
                      fontWeight={isSelected ? 600 : 400}
                      className="tabular-nums"
                    >
                      {point.short_label}
                    </text>
                  )}
                </g>
              );
            })}
          </svg>

          {hoveredDay && (
            <div
              className="pointer-events-none absolute top-1 z-20 px-3 py-2 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-strong)] text-[11.5px]"
              style={{
                left:
                  hoveredTrendIdx! > dailyTrend.length * 0.6
                    ? "18%"
                    : "55%",
                boxShadow: "var(--shadow-popover)",
              }}
            >
              <div className="font-semibold text-[var(--text-primary)] tabular-nums">
                {hoveredDay.date} · {formatDuration(hoveredDay.total_secs)}
              </div>
              <div className="flex flex-wrap gap-2 mt-1">
                {CATEGORY_LIST.filter((c) => (hoveredDay.by_category[c.id] || 0) > 0)
                  .slice(0, 3)
                  .map((c) => (
                    <span key={c.id} className="inline-flex items-center gap-1 text-[var(--text-secondary)] tabular-nums">
                      <span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: c.color }} />
                      {c.shortLabel} {formatDuration(hoveredDay.by_category[c.id] || 0, true)}
                    </span>
                  ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

function padLeft(padL: number, idx: number, slotW: number) {
  return padL + idx * slotW;
}
