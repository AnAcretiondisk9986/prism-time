import React from "react";
import type { LiveStatusDto } from "../types";
import { formatStopwatch } from "../lib/format";
import {
  closeOrHideAppWindow,
  minimizeAppWindow,
  toggleMaximizeAppWindow,
} from "../lib/ipc";
import {
  CloseWinIcon,
  MaximizeWinIcon,
  MinimizeWinIcon,
  MoonIcon,
  PrismBrandIcon,
  SunIcon,
} from "./Icons";

interface TitlebarProps {
  liveStatus: LiveStatusDto | null;
  theme: "light" | "dark";
  demoMode: boolean;
  onToggleTheme: () => void;
  onToggleDemo: () => void;
  onToggleTracking: () => void;
}

export const Titlebar: React.FC<TitlebarProps> = ({
  liveStatus,
  theme,
  demoMode,
  onToggleTheme,
  onToggleDemo,
  onToggleTracking,
}) => {
  const stateLabel = (() => {
    if (!liveStatus) return "正在初始化探针";
    if (!liveStatus.tracking_enabled || liveStatus.state === "paused") return "监控已暂停";
    if (liveStatus.state === "locked") return "已锁屏 · 自动暂停";
    if (liveStatus.state === "idle") return `离开空闲 ${liveStatus.idle_secs}s`;
    if (liveStatus.state === "self") return "当前正在浏览澄镜面板";
    if (liveStatus.state === "desktop") return "桌面待机中";
    if (liveStatus.current_app_name) {
      return `正在使用 ${liveStatus.current_app_name}`;
    }
    return "实时监听前台中";
  })();

  const isLive =
    liveStatus?.tracking_enabled &&
    (liveStatus.state === "active" || liveStatus.state === "self");

  return (
    <header
      data-tauri-drag-region
      className="h-12 shrink-0 flex items-center justify-between px-4 border-b border-[var(--border-hairline)] bg-[var(--bg-sidebar)]"
    >
      {/* Left: Brand Identity */}
      <div className="flex items-center gap-2.5 pointer-events-none">
        <div className="w-7 h-7 rounded-lg flex items-center justify-center bg-[var(--bg-surface)] border border-[var(--border-hairline)] text-[var(--text-primary)]">
          <PrismBrandIcon size={17} />
        </div>
        <div className="flex items-baseline gap-2">
          <span className="text-[13.5px] font-semibold tracking-tight text-[var(--text-primary)]">
            澄镜
          </span>
          <span className="text-[11px] font-medium tracking-wide uppercase text-[var(--text-muted)]">
            PrismTime
          </span>
        </div>
      </div>

      {/* Center: Live Telemetry Pill */}
      <div className="flex items-center gap-2">
        <button
          type="button"
          onClick={onToggleTracking}
          title={liveStatus?.tracking_enabled ? "点击暂停后台监控" : "点击恢复后台监控"}
          className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-[var(--bg-surface)] border border-[var(--border-hairline)] hover:border-[var(--border-strong)] transition-colors cursor-pointer"
        >
          <span
            className={`w-2 h-2 rounded-full ${isLive ? "live-dot" : ""}`}
            style={{
              backgroundColor: isLive
                ? "var(--status-ok)"
                : liveStatus?.state === "idle" || liveStatus?.state === "locked"
                  ? "var(--status-warn)"
                  : "var(--status-muted)",
            }}
          />
          <span className="text-[12px] font-medium text-[var(--text-primary)] max-w-[230px] truncate">
            {stateLabel}
          </span>
          {liveStatus && liveStatus.current_session_secs > 0 && liveStatus.state === "active" && (
            <span className="text-[11px] font-medium tabular-nums text-[var(--text-secondary)] pl-1.5 border-l border-[var(--border-hairline)]">
              {formatStopwatch(liveStatus.current_session_secs)}
            </span>
          )}
        </button>

        <button
          type="button"
          onClick={onToggleDemo}
          className={`px-2.5 py-1.5 rounded-full text-[11.5px] font-medium border transition-colors cursor-pointer ${
            demoMode
              ? "bg-[var(--accent-soft)] text-[var(--accent)] border-[var(--accent)]/30"
              : "bg-[var(--bg-surface)] text-[var(--text-secondary)] border-[var(--border-hairline)] hover:text-[var(--text-primary)] hover:border-[var(--border-strong)]"
          }`}
          title="切换基于本机已识别软件的非破坏性图表演示数据"
        >
          {demoMode ? "演示视图 · 点击切回真实监控" : "图表演示"}
        </button>
      </div>

      {/* Right: Theme Toggle + Window Controls */}
      <div className="flex items-center gap-1.5">
        <button
          type="button"
          onClick={onToggleTheme}
          title={theme === "light" ? "切换至曜石深色" : "切换至澄白浅色"}
          className="w-8 h-8 rounded-lg flex items-center justify-center text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-hover)] transition-colors cursor-pointer"
        >
          {theme === "light" ? <MoonIcon size={15} /> : <SunIcon size={15} />}
        </button>

        <div className="h-4 w-[1px] bg-[var(--border-hairline)] mx-1" />

        <button
          type="button"
          onClick={() => void minimizeAppWindow()}
          aria-label="最小化窗口"
          className="w-8 h-8 rounded-lg flex items-center justify-center text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-hover)] transition-colors cursor-pointer"
        >
          <MinimizeWinIcon />
        </button>
        <button
          type="button"
          onClick={() => void toggleMaximizeAppWindow()}
          aria-label="最大化或还原窗口"
          className="w-8 h-8 rounded-lg flex items-center justify-center text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-hover)] transition-colors cursor-pointer"
        >
          <MaximizeWinIcon />
        </button>
        <button
          type="button"
          onClick={() => void closeOrHideAppWindow()}
          aria-label="关闭窗口"
          className="w-8 h-8 rounded-lg flex items-center justify-center text-[var(--text-secondary)] hover:text-[#DC2626] hover:bg-[#DC2626]/10 transition-colors cursor-pointer"
        >
          <CloseWinIcon />
        </button>
      </div>
    </header>
  );
};
