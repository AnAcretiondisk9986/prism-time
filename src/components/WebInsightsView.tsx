import React, { useMemo, useState } from "react";
import type { DashboardAnalyticsDto, RangeOption, WebCategory } from "../types";
import { WEB_CATEGORY_LIST, WEB_CATEGORY_MAP } from "../types";
import { formatDuration, formatMinutesAxis, formatTimestampHm } from "../lib/format";
import { SearchIcon, WebCategorySvgIcon } from "./Icons";

interface WebInsightsViewProps {
  analytics: DashboardAnalyticsDto;
  range: RangeOption;
  onChangeRange: (r: RangeOption) => void;
  onEnableDemo: () => void;
}

const RANGE_OPTIONS: Array<{ id: RangeOption; label: string }> = [
  { id: "today", label: "今日" },
  { id: "yesterday", label: "昨日" },
  { id: "7d", label: "近 7 日" },
  { id: "30d", label: "近 30 日" },
];

export const WebInsightsView: React.FC<WebInsightsViewProps> = ({
  analytics,
  range,
  onChangeRange,
  onEnableDemo,
}) => {
  const web = analytics.web_analytics;
  const [activeWebCat, setActiveWebCat] = useState<WebCategory | null>(null);
  const [hoveredHour, setHoveredHour] = useState<number | null>(null);
  const [hoveredDonutId, setHoveredDonutId] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [filterCat, setFilterCat] = useState<WebCategory | "all">("all");

  const isSingleDay = range === "today" || range === "yesterday" || range === "custom";
  const showEmptyHint = web.total_secs < 10 && !analytics.demo_mode;

  const usedWebCategories = useMemo(() => {
    const present = new Set<WebCategory>();
    for (const h of web.hourly) {
      for (const [k, v] of Object.entries(h.by_category)) {
        if ((v || 0) > 0) present.add(k as WebCategory);
      }
    }
    return WEB_CATEGORY_LIST.filter((c) => present.has(c.id));
  }, [web.hourly]);

  const maxMinutes = useMemo(() => {
    if (isSingleDay) return 60;
    const rawMax = Math.max(30, ...web.hourly.map((h) => Math.ceil(h.total_secs / 60)));
    const step = rawMax <= 120 ? 30 : rawMax <= 360 ? 60 : 120;
    return Math.ceil(rawMax / step) * step;
  }, [web.hourly, isSingleDay]);

  const filteredSites = useMemo(() => {
    const q = search.trim().toLowerCase();
    return web.sites.filter((site) => {
      if (filterCat !== "all" && site.category !== filterCat) return false;
      if (!q) return true;
      return (
        site.display_name.toLowerCase().includes(q) ||
        site.domain.toLowerCase().includes(q) ||
        site.browsers.some((b) => b.toLowerCase().includes(q)) ||
        WEB_CATEGORY_MAP[site.category].label.toLowerCase().includes(q)
      );
    });
  }, [web.sites, filterCat, search]);

  const chartW = 740;
  const chartH = 184;
  const padLeft = 40;
  const padRight = 12;
  const padTop = 14;
  const padBottom = 26;
  const plotW = chartW - padLeft - padRight;
  const plotH = chartH - padTop - padBottom;
  const slotW = plotW / 24;
  const barW = Math.max(12, slotW * 0.56);
  const yTicks = [0, 0.25, 0.5, 0.75, 1];
  const hoveredBucket = hoveredHour !== null ? web.hourly[hoveredHour] : null;

  const donutSize = 164;
  const cx = donutSize / 2;
  const cy = donutSize / 2;
  const radius = 58;
  const circumference = 2 * Math.PI * radius;
  let offsetAcc = 0;
  const donutArcs = web.categories.map((cat) => {
    const frac = web.total_secs > 0 ? cat.total_secs / web.total_secs : 0;
    const rawLen = frac * circumference;
    const gap = web.categories.length > 1 ? Math.min(3, rawLen * 0.25) : 0;
    const dashLen = Math.max(0, rawLen - gap);
    const dashOffset = -offsetAcc;
    offsetAcc += rawLen;
    return { cat, dashArray: `${dashLen} ${circumference - dashLen}`, dashOffset };
  });
  const activeDonutCat = web.categories.find((c) => c.category === hoveredDonutId) || null;

  return (
    <div className="flex flex-col gap-5 pb-6">
      {/* Header & Range Switcher */}
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h1 className="text-[20px] font-semibold tracking-tight text-[var(--text-primary)]">
            网页浏览洞察
          </h1>
          <p className="text-[12.5px] text-[var(--text-secondary)] mt-0.5 tabular-nums">
            {analytics.date_label} · 自动解析主流浏览器访问站点与内容分布（仅记录纯净域名，剥离查询参数与路径）
            {analytics.demo_mode && " · 图表预览模式"}
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

      {showEmptyHint && (
        <div className="card-surface px-4 py-3 flex flex-wrap items-center justify-between gap-3 border-l-4 border-l-[var(--accent)]">
          <div>
            <p className="text-[13px] font-medium text-[var(--text-primary)]">
              暂未采集到浏览器内网页访问记录
            </p>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              切换使用 Edge、Chrome、Firefox、Ego 等桌面浏览器几秒后将自动汇总网站时长与时间段；您也可以开启演示预览。
            </p>
          </div>
          <button
            type="button"
            onClick={onEnableDemo}
            className="px-3.5 py-1.5 rounded-xl bg-[var(--accent)] text-white text-[12px] font-medium hover:opacity-90 transition-opacity cursor-pointer shrink-0"
          >
            预览网页洞察演示
          </button>
        </div>
      )}

      {/* 4 Web KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-3.5">
        <div className="card-surface p-4 flex flex-col justify-between gap-2">
          <span className="text-[12px] font-medium text-[var(--text-secondary)]">
            浏览器网页总时长
          </span>
          <div className="text-[26px] font-semibold tracking-tight tabular-nums text-[var(--text-primary)] leading-tight">
            {formatDuration(web.total_secs)}
          </div>
          <div className="text-[11.5px] text-[var(--text-muted)] tabular-nums">
            占总屏幕 {web.share_of_screen_pct.toFixed(1)}% · {web.active_site_count} 个站点
          </div>
        </div>

        <div className="card-surface p-4 flex flex-col justify-between gap-2">
          <span className="text-[12px] font-medium text-[var(--text-secondary)]">
            最常访问网站
          </span>
          <div className="text-[22px] font-semibold tracking-tight text-[var(--text-primary)] leading-tight truncate">
            {web.top_site_name || "待采集"}
          </div>
          <div className="text-[11.5px] text-[var(--text-muted)] tabular-nums truncate">
            {web.top_site_domain
              ? `${web.top_site_domain} · ${formatDuration(web.top_site_secs, true)}`
              : "打开主流浏览器即可自动识别"}
          </div>
        </div>

        <div className="card-surface p-4 flex flex-col justify-between gap-2">
          <div className="flex items-center justify-between">
            <span className="text-[12px] font-medium text-[var(--text-secondary)]">
              主导网页类别
            </span>
            {web.dominant_category && (
              <span
                className="w-2.5 h-2.5 rounded-full"
                style={{ backgroundColor: web.dominant_category.color }}
              />
            )}
          </div>
          <div className="text-[22px] font-semibold tracking-tight text-[var(--text-primary)] leading-tight truncate">
            {web.dominant_category ? web.dominant_category.label : "待采集"}
          </div>
          <div className="text-[11.5px] text-[var(--text-muted)] tabular-nums">
            {web.dominant_category
              ? `占网页浏览 ${web.dominant_category.percentage.toFixed(1)}% · ${formatDuration(web.dominant_category.total_secs, true)}`
              : "等待网站访问分布"}
          </div>
        </div>

        <div className="card-surface p-4 flex flex-col justify-between gap-2">
          <span className="text-[12px] font-medium text-[var(--text-secondary)]">
            浏览器载体分流
          </span>
          <div className="text-[22px] font-semibold tracking-tight text-[var(--text-primary)] leading-tight truncate">
            {web.browsers[0]?.display_name || "待采集"}
          </div>
          <div className="text-[11.5px] text-[var(--text-muted)] tabular-nums">
            {web.browsers.length > 0
              ? `${web.browsers.length} 款浏览器 · 主力占 ${web.browsers[0].percentage.toFixed(1)}%`
              : "支持主流桌面浏览器"}
          </div>
        </div>
      </div>

      {/* Hourly Web Distribution + Category Ring & Browser Split */}
      <div className="grid grid-cols-1 xl:grid-cols-12 gap-4">
        {/* Left: 24h Hourly Web Category Chart */}
        <div className="xl:col-span-7 card-surface p-5 flex flex-col gap-4 relative">
          <div className="flex flex-wrap items-start justify-between gap-3">
            <div>
              <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
                24 小时网页内容分布
              </h3>
              <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
                按整点小时统计各类站点停留时长，点击图例可单独筛选一类网页
              </p>
            </div>

            <div className="flex flex-wrap items-center gap-1.5">
              {usedWebCategories.map((cat) => {
                const selected = activeWebCat === cat.id;
                const dimmed = activeWebCat !== null && !selected;
                return (
                  <button
                    key={cat.id}
                    type="button"
                    onClick={() => setActiveWebCat(selected ? null : cat.id)}
                    className={`inline-flex items-center gap-1.5 px-2 py-1 rounded-lg text-[11px] font-medium border transition-all cursor-pointer ${
                      selected
                        ? "bg-[var(--bg-subtle)] border-[var(--border-strong)] text-[var(--text-primary)]"
                        : "border-transparent hover:bg-[var(--bg-subtle)] text-[var(--text-secondary)]"
                    } ${dimmed ? "opacity-45" : "opacity-100"}`}
                  >
                    <span className="w-2 h-2 rounded-full" style={{ backgroundColor: cat.color }} />
                    <span>{cat.shortLabel}</span>
                  </button>
                );
              })}
            </div>
          </div>

          <div className="relative w-full">
            <svg viewBox={`0 0 ${chartW} ${chartH}`} className="w-full h-auto overflow-visible">
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

              {web.hourly.map((bucket, idx) => {
                const cxBar = padLeft + idx * slotW + slotW / 2;
                const bx = cxBar - barW / 2;
                const isHovered = hoveredHour === idx;

                let yCursor = padTop + plotH;
                const segments: Array<{ cat: WebCategory; y: number; h: number; color: string }> = [];
                for (const meta of WEB_CATEGORY_LIST) {
                  const secs = bucket.by_category[meta.id] || 0;
                  if (secs <= 0) continue;
                  const mins = secs / 60;
                  const segH = Math.max(1.5, (mins / maxMinutes) * plotH);
                  const segY = yCursor - segH;
                  segments.push({ cat: meta.id, y: segY, h: segH, color: meta.color });
                  yCursor = segY;
                }

                return (
                  <g
                    key={bucket.hour}
                    onMouseEnter={() => setHoveredHour(idx)}
                    onMouseLeave={() => setHoveredHour(null)}
                    className="cursor-pointer"
                  >
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
                    <rect
                      x={padLeft + idx * slotW}
                      y={padTop}
                      width={slotW}
                      height={plotH + padBottom}
                      fill="transparent"
                    />
                    {segments.map((seg, sIdx) => {
                      const dimmed = activeWebCat !== null && activeWebCat !== seg.cat;
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
                    {(idx % 3 === 0 || idx === 23) && (
                      <text
                        x={cxBar}
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

            {hoveredBucket && (
              <div
                className="pointer-events-none absolute top-2 z-20 px-3 py-2.5 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-strong)] min-w-[205px]"
                style={{
                  left:
                    hoveredBucket.hour > 15
                      ? `calc(${((hoveredBucket.hour + 0.5) / 24) * 100}% - 215px)`
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
                  <p className="text-[11px] text-[var(--text-muted)] pt-1.5">该时段无网页浏览记录</p>
                ) : (
                  <div className="flex flex-col gap-1 pt-1.5">
                    {hoveredBucket.top_sites.map((site) => (
                      <div
                        key={site.site_id}
                        className="flex items-center justify-between gap-3 text-[11.5px]"
                      >
                        <div className="flex items-center gap-1.5 min-w-0">
                          <span
                            className="w-1.5 h-1.5 rounded-full shrink-0"
                            style={{ backgroundColor: WEB_CATEGORY_MAP[site.category].color }}
                          />
                          <span className="text-[var(--text-primary)] truncate">
                            {site.display_name}
                          </span>
                        </div>
                        <span className="text-[var(--text-secondary)] tabular-nums shrink-0">
                          {formatDuration(site.duration_secs, true)}
                        </span>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>
        </div>

        {/* Right: Web Category Ring & Browser Split */}
        <div className="xl:col-span-5 card-surface p-5 flex flex-col gap-4">
          <div>
            <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
              网页类别构成与浏览器载体
            </h3>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              内容题材比重及各桌面浏览器的载体分布
            </p>
          </div>

          {web.categories.length === 0 ? (
            <div className="flex-1 min-h-[210px] flex items-center justify-center rounded-xl bg-[var(--bg-subtle)]/50 border border-dashed border-[var(--border-hairline)]">
              <span className="text-[12.5px] text-[var(--text-muted)]">暂无网页类别数据</span>
            </div>
          ) : (
            <div className="flex flex-col gap-4 flex-1">
              <div className="flex flex-col sm:flex-row items-center gap-4">
                <div className="relative shrink-0" style={{ width: donutSize, height: donutSize }}>
                  <svg
                    width={donutSize}
                    height={donutSize}
                    viewBox={`0 0 ${donutSize} ${donutSize}`}
                    className="-rotate-90"
                  >
                    <circle
                      cx={cx}
                      cy={cy}
                      r={radius}
                      fill="none"
                      stroke="var(--bg-subtle)"
                      strokeWidth={16}
                    />
                    {donutArcs.map(({ cat, dashArray, dashOffset }) => {
                      const isHovered = hoveredDonutId === cat.category;
                      const dimmed = hoveredDonutId !== null && !isHovered;
                      return (
                        <circle
                          key={cat.category}
                          cx={cx}
                          cy={cy}
                          r={radius}
                          fill="none"
                          stroke={cat.color}
                          strokeWidth={isHovered ? 20 : 16}
                          strokeDasharray={dashArray}
                          strokeDashoffset={dashOffset}
                          strokeLinecap="butt"
                          opacity={dimmed ? 0.28 : 1}
                          className="transition-all duration-150 cursor-pointer"
                          onMouseEnter={() => setHoveredDonutId(cat.category)}
                          onMouseLeave={() => setHoveredDonutId(null)}
                        />
                      );
                    })}
                  </svg>
                  <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none px-3 text-center">
                    <span className="text-[10.5px] font-medium text-[var(--text-muted)] truncate max-w-[96px]">
                      {activeDonutCat ? activeDonutCat.label : "网页总时长"}
                    </span>
                    <span className="text-[16px] font-semibold tabular-nums text-[var(--text-primary)] mt-0.5">
                      {activeDonutCat
                        ? `${activeDonutCat.percentage.toFixed(1)}%`
                        : formatDuration(web.total_secs, true)}
                    </span>
                  </div>
                </div>

                <div className="flex-1 w-full flex flex-col gap-1 max-h-[168px] scroll-area pr-1">
                  {web.categories.map((cat) => {
                    const isHovered = hoveredDonutId === cat.category;
                    return (
                      <div
                        key={cat.category}
                        onMouseEnter={() => setHoveredDonutId(cat.category)}
                        onMouseLeave={() => setHoveredDonutId(null)}
                        className={`flex items-center justify-between gap-2 px-2 py-1 rounded-lg transition-colors ${
                          isHovered ? "bg-[var(--bg-subtle)]" : "hover:bg-[var(--bg-subtle)]/60"
                        }`}
                      >
                        <div className="flex items-center gap-2 min-w-0">
                          <span
                            className="w-5 h-5 rounded-md flex items-center justify-center shrink-0"
                            style={{ backgroundColor: `${cat.color}18`, color: cat.color }}
                          >
                            <WebCategorySvgIcon category={cat.category} size={12} />
                          </span>
                          <span className="text-[12px] font-medium text-[var(--text-primary)] truncate">
                            {cat.label}
                          </span>
                        </div>
                        <div className="text-right text-[11.5px] tabular-nums shrink-0">
                          <span className="font-semibold text-[var(--text-primary)]">
                            {formatDuration(cat.total_secs, true)}
                          </span>
                          <span className="text-[var(--text-muted)] ml-1.5">
                            {cat.percentage.toFixed(0)}%
                          </span>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>

              {web.browsers.length > 0 && (
                <div className="pt-3 border-t border-[var(--border-hairline)] flex flex-col gap-2">
                  <span className="text-[11.5px] font-medium text-[var(--text-secondary)]">
                    宿主浏览器载体分布
                  </span>
                  {web.browsers.map((b) => (
                    <div key={b.app_id} className="flex items-center gap-3 text-[11.5px]">
                      <span className="w-24 text-[var(--text-primary)] truncate">{b.display_name}</span>
                      <div className="flex-1 h-1.5 rounded-full bg-[var(--bg-subtle)] overflow-hidden">
                        <div
                          className="h-full rounded-full bg-[#0E8AC8]"
                          style={{ width: `${Math.min(100, Math.max(4, b.percentage))}%` }}
                        />
                      </div>
                      <span className="tabular-nums text-[var(--text-secondary)] shrink-0">
                        {formatDuration(b.total_secs, true)} · {b.percentage.toFixed(1)}%
                      </span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      {/* Site Leaderboard */}
      <div className="card-surface p-5 flex flex-col gap-4">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
              网站访问时长排行与活跃时段
            </h3>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              内置主流网站精准归类；其余已访问网站自动按干净根域名或主机名独立累计
            </p>
          </div>

          <div className="relative w-64">
            <SearchIcon
              size={14}
              className="absolute left-3 top-1/2 -translate-y-1/2 text-[var(--text-muted)]"
            />
            <input
              type="text"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="搜索网站名称、域名或宿主浏览器..."
              className="w-full pl-8 pr-3 py-1.5 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12px] text-[var(--text-primary)] placeholder:text-[var(--text-muted)] focus:outline-none"
            />
          </div>
        </div>

        {web.sites.length > 0 && (
          <div className="flex flex-wrap items-center gap-1.5">
            <button
              type="button"
              onClick={() => setFilterCat("all")}
              className={`px-3 py-1 rounded-xl text-[11.5px] font-medium border transition-colors cursor-pointer ${
                filterCat === "all"
                  ? "bg-[var(--text-primary)] text-[var(--bg-surface)] border-transparent"
                  : "bg-[var(--bg-subtle)] text-[var(--text-secondary)] border-[var(--border-hairline)] hover:text-[var(--text-primary)]"
              }`}
            >
              全部站点 ({web.sites.length})
            </button>
            {WEB_CATEGORY_LIST.map((cat) => {
              const cnt = web.sites.filter((s) => s.category === cat.id).length;
              if (cnt === 0) return null;
              const active = filterCat === cat.id;
              return (
                <button
                  key={cat.id}
                  type="button"
                  onClick={() => setFilterCat(cat.id)}
                  className={`inline-flex items-center gap-1.5 px-3 py-1 rounded-xl text-[11.5px] font-medium border transition-colors cursor-pointer ${
                    active
                      ? "bg-[var(--text-primary)] text-[var(--bg-surface)] border-transparent"
                      : "bg-[var(--bg-subtle)] text-[var(--text-secondary)] border-[var(--border-hairline)] hover:text-[var(--text-primary)]"
                  }`}
                >
                  <span
                    className="w-2 h-2 rounded-full"
                    style={{ backgroundColor: active ? "currentColor" : cat.color }}
                  />
                  <span>
                    {cat.shortLabel} ({cnt})
                  </span>
                </button>
              );
            })}
          </div>
        )}

        {filteredSites.length === 0 ? (
          <div className="py-10 text-center text-[12.5px] text-[var(--text-muted)]">
            {web.sites.length === 0 ? "暂无网页访问记录" : "当前筛选条件下未匹配到网站记录"}
          </div>
        ) : (
          <div className="flex flex-col divide-y divide-[var(--border-hairline)]/70">
            {filteredSites.map((site, idx) => {
              const cat = WEB_CATEGORY_MAP[site.category];
              return (
                <div
                  key={site.site_id}
                  className="py-3 px-2 rounded-xl hover:bg-[var(--bg-subtle)]/55 transition-colors flex flex-col sm:flex-row sm:items-center justify-between gap-3"
                >
                  <div className="flex items-center gap-3 min-w-0 flex-1">
                    <span className="w-5 text-[11.5px] font-medium tabular-nums text-[var(--text-muted)] shrink-0">
                      {String(idx + 1).padStart(2, "0")}
                    </span>
                    <span
                      className="w-9 h-9 rounded-xl flex items-center justify-center shrink-0 border border-[var(--border-hairline)]"
                      style={{ backgroundColor: `${cat.color}14`, color: cat.color }}
                    >
                      <WebCategorySvgIcon category={site.category} size={17} />
                    </span>
                    <div className="min-w-0 flex-1">
                      <div className="flex items-center gap-2 flex-wrap">
                        <span className="text-[13.5px] font-semibold text-[var(--text-primary)] truncate">
                          {site.display_name}
                        </span>
                        <span className="px-2 py-0.5 rounded-md bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[10.5px] font-mono text-[var(--text-secondary)]">
                          {site.domain}
                        </span>
                        <span
                          className="px-2 py-0.5 rounded-full text-[10.5px] font-medium"
                          style={{ backgroundColor: `${cat.color}16`, color: cat.color }}
                        >
                          {cat.shortLabel}
                        </span>
                      </div>

                      <div className="flex flex-wrap items-center gap-1.5 mt-1.5">
                        {site.browsers.map((b) => (
                          <span
                            key={b}
                            className="text-[10.5px] text-[var(--text-muted)] pr-1.5"
                          >
                            载体：{b}
                          </span>
                        ))}
                        {site.active_ranges_preview.map((rng) => (
                          <span
                            key={rng}
                            className="px-2 py-0.5 rounded-md bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[10.5px] tabular-nums text-[var(--text-secondary)]"
                          >
                            {rng}
                          </span>
                        ))}
                        {site.active_ranges_preview.length === 0 && (
                          <span className="text-[11px] text-[var(--text-muted)] tabular-nums">
                            最近访问 {formatTimestampHm(site.last_seen_ts)}
                          </span>
                        )}
                      </div>
                    </div>
                  </div>

                  <div className="flex items-center gap-4 shrink-0 pl-8 sm:pl-0">
                    <div className="w-28 hidden md:block">
                      <div className="h-1.5 w-full rounded-full bg-[var(--bg-subtle)] overflow-hidden">
                        <div
                          className="h-full rounded-full"
                          style={{
                            width: `${Math.min(100, Math.max(4, site.percentage))}%`,
                            backgroundColor: cat.color,
                          }}
                        />
                      </div>
                    </div>
                    <div className="text-right min-w-[78px]">
                      <div className="text-[13.5px] font-semibold tabular-nums text-[var(--text-primary)]">
                        {formatDuration(site.total_secs, true)}
                      </div>
                      <div className="text-[11px] tabular-nums text-[var(--text-muted)]">
                        {site.percentage.toFixed(1)}% · {site.session_count} 次
                      </div>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>

      {/* Chronological Web Session Stream */}
      <div className="card-surface p-5 flex flex-col gap-3">
        <div>
          <h3 className="text-[14px] font-semibold text-[var(--text-primary)] tracking-tight">
            网页浏览时段明细流
          </h3>
          <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
            当在同一浏览器内切换站点时自动切分独立时段，完整保留访问起止时刻与页面标题摘要
          </p>
        </div>

        {web.recent_web_sessions.length === 0 ? (
          <div className="py-8 text-center text-[12.5px] text-[var(--text-muted)]">
            暂无网页浏览时段明细
          </div>
        ) : (
          <div className="flex flex-col divide-y divide-[var(--border-hairline)]/70">
            {web.recent_web_sessions.map((ev) => {
              const cat = ev.web_category ? WEB_CATEGORY_MAP[ev.web_category] : WEB_CATEGORY_MAP.search_portal;
              return (
                <div
                  key={ev.id}
                  className="py-2.5 px-2 grid grid-cols-1 md:grid-cols-[170px_1fr_92px] items-center gap-3"
                >
                  <div className="flex items-center gap-2 text-[12px] tabular-nums text-[var(--text-secondary)]">
                    <span className="text-[11px] text-[var(--text-muted)]">{ev.date.slice(5)}</span>
                    <span className="font-medium text-[var(--text-primary)]">
                      {ev.start_time_label.slice(0, 5)} - {ev.end_time_label.slice(0, 5)}
                    </span>
                  </div>

                  <div className="flex items-center gap-2.5 min-w-0">
                    <span
                      className="w-7 h-7 rounded-lg flex items-center justify-center shrink-0"
                      style={{ backgroundColor: `${cat.color}16`, color: cat.color }}
                    >
                      <WebCategorySvgIcon
                        category={ev.web_category || "search_portal"}
                        size={14}
                      />
                    </span>
                    <div className="min-w-0 flex-1">
                      <div className="flex items-center gap-2 flex-wrap">
                        <span className="text-[13px] font-medium text-[var(--text-primary)] truncate">
                          {ev.web_site_name || "未知网站"}
                        </span>
                        {ev.web_domain && (
                          <span className="text-[11px] font-mono text-[var(--text-muted)]">
                            {ev.web_domain}
                          </span>
                        )}
                        <span className="px-1.5 py-0.5 rounded-md bg-[var(--bg-subtle)] text-[10.5px] text-[var(--text-secondary)]">
                          {ev.display_name}
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
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
};
