import React, { useState } from "react";
import type { CategoryStatDto } from "../../types";
import { formatDuration } from "../../lib/format";
import { CategorySvgIcon } from "../Icons";

interface CategoryDonutChartProps {
  categories: CategoryStatDto[];
  totalSecs: number;
}

export const CategoryDonutChart: React.FC<CategoryDonutChartProps> = ({
  categories,
  totalSecs,
}) => {
  const [hoveredId, setHoveredId] = useState<string | null>(null);

  const size = 180;
  const cx = size / 2;
  const cy = size / 2;
  const radius = 64;
  const circumference = 2 * Math.PI * radius;

  const activeCat = categories.find((c) => c.category === hoveredId) || null;

  let offsetAcc = 0;
  const arcs = categories.map((cat) => {
    const frac = totalSecs > 0 ? cat.total_secs / totalSecs : 0;
    const rawLen = frac * circumference;
    const gap = categories.length > 1 ? Math.min(3, rawLen * 0.25) : 0;
    const dashLen = Math.max(0, rawLen - gap);
    const dashOffset = -offsetAcc;
    offsetAcc += rawLen;
    return {
      cat,
      dashArray: `${dashLen} ${circumference - dashLen}`,
      dashOffset,
    };
  });

  return (
    <div className="card-surface p-5 flex flex-col gap-4 h-full">
      <div>
        <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
          软件类别占比与高峰时段
        </h3>
        <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
          各类应用耗时结构及其全天最活跃时间窗口
        </p>
      </div>

      {categories.length === 0 ? (
        <div className="flex-1 min-h-[210px] flex items-center justify-center rounded-xl bg-[var(--bg-subtle)]/50 border border-dashed border-[var(--border-hairline)]">
          <span className="text-[12.5px] text-[var(--text-muted)]">暂无类别分布数据</span>
        </div>
      ) : (
        <div className="flex flex-col sm:flex-row items-center gap-5 flex-1">
          {/* Precision SVG Donut */}
          <div className="relative shrink-0" style={{ width: size, height: size }}>
            <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} className="-rotate-90">
              <circle
                cx={cx}
                cy={cy}
                r={radius}
                fill="none"
                stroke="var(--bg-subtle)"
                strokeWidth={18}
              />
              {arcs.map(({ cat, dashArray, dashOffset }) => {
                const isHovered = hoveredId === cat.category;
                const dimmed = hoveredId !== null && !isHovered;
                return (
                  <circle
                    key={cat.category}
                    cx={cx}
                    cy={cy}
                    r={radius}
                    fill="none"
                    stroke={cat.color}
                    strokeWidth={isHovered ? 22 : 18}
                    strokeDasharray={dashArray}
                    strokeDashoffset={dashOffset}
                    strokeLinecap="butt"
                    opacity={dimmed ? 0.28 : 1}
                    className="transition-all duration-150 cursor-pointer"
                    onMouseEnter={() => setHoveredId(cat.category)}
                    onMouseLeave={() => setHoveredId(null)}
                  />
                );
              })}
            </svg>

            <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none px-4 text-center">
              <span className="text-[11px] font-medium text-[var(--text-muted)] truncate max-w-[105px]">
                {activeCat ? activeCat.label : "总屏幕时长"}
              </span>
              <span className="text-[18px] font-semibold tabular-nums text-[var(--text-primary)] tracking-tight mt-0.5">
                {activeCat ? `${activeCat.percentage}%` : formatDuration(totalSecs, true)}
              </span>
              {activeCat && (
                <span className="text-[10.5px] tabular-nums text-[var(--text-secondary)]">
                  {formatDuration(activeCat.total_secs, true)}
                </span>
              )}
            </div>
          </div>

          {/* Ordered Breakdown List */}
          <div className="flex-1 w-full flex flex-col gap-1.5 max-h-[230px] scroll-area pr-1">
            {categories.map((cat) => {
              const isHovered = hoveredId === cat.category;
              return (
                <div
                  key={cat.category}
                  onMouseEnter={() => setHoveredId(cat.category)}
                  onMouseLeave={() => setHoveredId(null)}
                  className={`flex items-center justify-between gap-2.5 px-2.5 py-1.5 rounded-xl transition-colors cursor-default ${
                    isHovered ? "bg-[var(--bg-subtle)]" : "hover:bg-[var(--bg-subtle)]/60"
                  }`}
                >
                  <div className="flex items-center gap-2 min-w-0">
                    <span
                      className="w-6 h-6 rounded-lg flex items-center justify-center shrink-0"
                      style={{
                        backgroundColor: `${cat.color}16`,
                        color: cat.color,
                      }}
                    >
                      <CategorySvgIcon category={cat.category} size={13} />
                    </span>
                    <div className="min-w-0">
                      <div className="text-[12.5px] font-medium text-[var(--text-primary)] truncate">
                        {cat.label}
                      </div>
                      <div className="text-[10.5px] text-[var(--text-muted)] tabular-nums">
                        {cat.app_count} 款应用 · 高峰 {cat.peak_hour_range}
                      </div>
                    </div>
                  </div>

                  <div className="text-right shrink-0">
                    <div className="text-[12.5px] font-semibold tabular-nums text-[var(--text-primary)]">
                      {formatDuration(cat.total_secs, true)}
                    </div>
                    <div className="text-[10.5px] tabular-nums text-[var(--text-muted)]">
                      {cat.percentage.toFixed(1)}%
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
};
