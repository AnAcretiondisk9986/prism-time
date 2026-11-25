import React, { useState } from "react";
import type { AppSettings, LiveStatusDto } from "../types";
import { exportUsageReport } from "../lib/ipc";
import { ExportIcon } from "./Icons";

interface SettingsViewProps {
  settings: AppSettings;
  liveStatus: LiveStatusDto | null;
  demoMode: boolean;
  onUpdateSettings: (next: AppSettings) => Promise<void>;
}

const IDLE_OPTIONS = [
  { secs: 60, label: "1 分钟" },
  { secs: 180, label: "3 分钟" },
  { secs: 300, label: "5 分钟" },
  { secs: 600, label: "10 分钟" },
];

export const SettingsView: React.FC<SettingsViewProps> = ({
  settings,
  liveStatus,
  demoMode,
  onUpdateSettings,
}) => {
  const [exportStatus, setExportStatus] = useState<string | null>(null);

  const patch = (partial: Partial<AppSettings>) => {
    void onUpdateSettings({ ...settings, ...partial });
  };

  const handleExport = async (format: "csv" | "json") => {
    try {
      const res = await exportUsageReport("today", format, demoMode);
      setExportStatus(`已导出 ${res.record_count} 条明细至：${res.file_path}`);
    } catch (err) {
      setExportStatus(`导出失败：${String(err)}`);
    }
  };

  return (
    <div className="flex flex-col gap-5 pb-6 max-w-4xl">
      <div>
        <h1 className="text-[20px] font-semibold tracking-tight text-[var(--text-primary)]">
          监控与隐私设置
        </h1>
        <p className="text-[12.5px] text-[var(--text-secondary)] mt-0.5">
          调整 Windows 前台窗口采样策略、空闲离开检测及本地纯离线数据管理
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Card 1: Tracking & Idle */}
        <div className="card-surface p-5 flex flex-col gap-4">
          <h3 className="text-[14px] font-semibold text-[var(--text-primary)]">
            采集与空闲检测策略
          </h3>

          <ToggleRow
            title="启用后台实时监控"
            desc="每秒探测 Windows 前台活跃窗口并归属到对应桌面软件"
            checked={settings.tracking_enabled}
            onChange={(v) => patch({ tracking_enabled: v })}
          />

          <div className="flex flex-col gap-2 py-2 border-t border-[var(--border-hairline)]">
            <div className="flex items-center justify-between">
              <span className="text-[13px] font-medium text-[var(--text-primary)]">
                无操作空闲离开判定
              </span>
              <span className="text-[11.5px] text-[var(--text-muted)]">
                影音播放自动放宽 3 倍
              </span>
            </div>
            <div className="grid grid-cols-4 gap-1.5">
              {IDLE_OPTIONS.map((opt) => (
                <button
                  key={opt.secs}
                  type="button"
                  onClick={() => patch({ idle_threshold_secs: opt.secs })}
                  className={`py-1.5 rounded-xl text-[12px] font-medium border transition-colors cursor-pointer ${
                    settings.idle_threshold_secs === opt.secs
                      ? "bg-[var(--text-primary)] text-[var(--bg-surface)] border-transparent"
                      : "bg-[var(--bg-subtle)] text-[var(--text-secondary)] border-[var(--border-hairline)] hover:text-[var(--text-primary)]"
                  }`}
                >
                  {opt.label}
                </button>
              ))}
            </div>
          </div>

          <div className="border-t border-[var(--border-hairline)] pt-3">
            <ToggleRow
              title="启用浏览器内网页识别统计"
              desc="自动识别主流浏览器当前访问的网站与域名分类，仅在本地统计且不记录 URL 查询参数"
              checked={settings.web_tracking_enabled}
              onChange={(v) => patch({ web_tracking_enabled: v })}
            />
          </div>

          <div className="border-t border-[var(--border-hairline)] pt-3">
            <ToggleRow
              title="锁屏或进入安全桌面时暂停"
              desc="检测到 Win+L 锁屏或 UAC 安全桌面后立即封存当前时段"
              checked={settings.pause_on_lock}
              onChange={(v) => patch({ pause_on_lock: v })}
            />
          </div>

          <div className="border-t border-[var(--border-hairline)] pt-3">
            <ToggleRow
              title="统计澄镜自身面板浏览时长"
              desc="默认关闭，避免查看统计报表本身影响实际工作时长分布"
              checked={settings.track_self}
              onChange={(v) => patch({ track_self: v })}
            />
          </div>
        </div>

        {/* Card 2: System Integration & Theme */}
        <div className="card-surface p-5 flex flex-col gap-4">
          <h3 className="text-[14px] font-semibold text-[var(--text-primary)]">
            系统驻留与视觉主题
          </h3>

          <ToggleRow
            title="关闭窗口时最小化至系统托盘"
            desc="点击右上角关闭按钮后保持后台静默统计，可从托盘重新呼出"
            checked={settings.minimize_to_tray_on_close}
            onChange={(v) => patch({ minimize_to_tray_on_close: v })}
          />

          <div className="border-t border-[var(--border-hairline)] pt-3">
            <ToggleRow
              title="随 Windows 登录自动启动"
              desc="写入当前用户注册表 Run 启动项，开机自动开始记录时长"
              checked={settings.launch_at_startup}
              onChange={(v) => patch({ launch_at_startup: v })}
            />
          </div>

          <div className="border-t border-[var(--border-hairline)] pt-3 flex flex-col gap-2">
            <span className="text-[13px] font-medium text-[var(--text-primary)]">
              界面色彩主题
            </span>
            <div className="grid grid-cols-2 gap-2">
              <button
                type="button"
                onClick={() => patch({ theme: "light" })}
                className={`py-2.5 px-3 rounded-xl border text-left transition-all cursor-pointer ${
                  settings.theme === "light"
                    ? "border-[var(--accent)] bg-[var(--accent-soft)]"
                    : "border-[var(--border-hairline)] bg-[var(--bg-subtle)]"
                }`}
              >
                <div className="text-[12.5px] font-semibold text-[var(--text-primary)]">
                  澄白 · Alabaster
                </div>
                <div className="text-[11px] text-[var(--text-muted)] mt-0.5">
                  日间阅读高通透纸感界面
                </div>
              </button>

              <button
                type="button"
                onClick={() => patch({ theme: "dark" })}
                className={`py-2.5 px-3 rounded-xl border text-left transition-all cursor-pointer ${
                  settings.theme === "dark"
                    ? "border-[var(--accent)] bg-[var(--accent-soft)]"
                    : "border-[var(--border-hairline)] bg-[var(--bg-subtle)]"
                }`}
              >
                <div className="text-[12.5px] font-semibold text-[var(--text-primary)]">
                  曜石 · Obsidian
                </div>
                <div className="text-[11px] text-[var(--text-muted)] mt-0.5">
                  夜间低眩光沉静工作空间
                </div>
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Card 3: Local Storage & Data Export */}
      <div className="card-surface p-5 flex flex-col gap-4">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h3 className="text-[14px] font-semibold text-[var(--text-primary)]">
              本地沙盒存储与报告导出
            </h3>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              所有记录均以按日分片 JSONL 文件保存在本地磁盘，不进行任何云端遥测上传
            </p>
          </div>
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={() => void handleExport("csv")}
              className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] hover:border-[var(--border-strong)] text-[12px] font-medium text-[var(--text-primary)] cursor-pointer"
            >
              <ExportIcon size={14} />
              <span>导出今日 CSV</span>
            </button>
            <button
              type="button"
              onClick={() => void handleExport("json")}
              className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] hover:border-[var(--border-strong)] text-[12px] font-medium text-[var(--text-primary)] cursor-pointer"
            >
              <ExportIcon size={14} />
              <span>导出完整 JSON</span>
            </button>
          </div>
        </div>

        <div className="px-3.5 py-2.5 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12px] text-[var(--text-secondary)] select-text">
          数据目录：{liveStatus?.data_dir || "%LOCALAPPDATA%\\PrismTime"}
        </div>

        {exportStatus && (
          <div className="text-[12px] text-[var(--status-ok)] select-text">{exportStatus}</div>
        )}
      </div>
    </div>
  );
};

interface ToggleRowProps {
  title: string;
  desc: string;
  checked: boolean;
  onChange: (next: boolean) => void;
}

const ToggleRow: React.FC<ToggleRowProps> = ({ title, desc, checked, onChange }) => (
  <div className="flex items-center justify-between gap-4">
    <div className="min-w-0">
      <div className="text-[13px] font-medium text-[var(--text-primary)]">{title}</div>
      <div className="text-[11.5px] text-[var(--text-muted)] mt-0.5">{desc}</div>
    </div>
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      onClick={() => onChange(!checked)}
      className={`w-10 h-6 rounded-full p-0.5 transition-colors cursor-pointer shrink-0 ${
        checked ? "bg-[var(--accent)]" : "bg-[var(--border-strong)]"
      }`}
    >
      <span
        className={`block w-5 h-5 rounded-full bg-white shadow-xs transition-transform ${
          checked ? "translate-x-4" : "translate-x-0"
        }`}
      />
    </button>
  </div>
);
