import React, { useEffect, useState } from "react";
import type { AppSettings, LiveStatusDto, SyncConfig, SyncStatusDto } from "../types";
import { formatTimestampHm } from "../lib/format";
import {
  exportUsageReport,
  fetchSyncStatus,
  runSyncNow,
  testSyncConnection,
} from "../lib/ipc";
import { CloudSyncIcon, ExportIcon, GitRepoIcon, RefreshIcon } from "./Icons";

interface SettingsViewProps {
  settings: AppSettings;
  liveStatus: LiveStatusDto | null;
  demoMode: boolean;
  onUpdateSettings: (next: AppSettings) => Promise<void>;
  onSyncCompleted?: () => void;
}

const IDLE_OPTIONS = [
  { secs: 60, label: "1 分钟" },
  { secs: 180, label: "3 分钟" },
  { secs: 300, label: "5 分钟" },
  { secs: 600, label: "10 分钟" },
];

const SYNC_INTERVAL_OPTIONS = [
  { mins: 15, label: "15 分钟" },
  { mins: 30, label: "30 分钟" },
  { mins: 60, label: "1 小时" },
  { mins: 180, label: "3 小时" },
];

const WEBDAV_PRESETS = [
  { label: "坚果云", url: "https://dav.jianguoyun.com/dav/" },
  { label: "Nextcloud", url: "https://your-domain.com/remote.php/dav/files/username/" },
  { label: "Alist / 本地", url: "http://127.0.0.1:5244/dav/" },
];

export const SettingsView: React.FC<SettingsViewProps> = ({
  settings,
  liveStatus,
  demoMode,
  onUpdateSettings,
  onSyncCompleted,
}) => {
  const [exportStatus, setExportStatus] = useState<string | null>(null);
  const [syncDraft, setSyncDraft] = useState<SyncConfig>(settings.sync);
  const [syncStatus, setSyncStatus] = useState<SyncStatusDto | null>(null);
  const [showSecret, setShowSecret] = useState(false);
  const [testing, setTesting] = useState(false);
  const [syncing, setSyncing] = useState(false);
  const [syncNotice, setSyncNotice] = useState<{ ok: boolean; text: string } | null>(null);

  useEffect(() => {
    setSyncDraft(settings.sync);
  }, [settings.sync]);

  useEffect(() => {
    void fetchSyncStatus().then(setSyncStatus);
  }, []);

  const patch = (partial: Partial<AppSettings>) => {
    void onUpdateSettings({ ...settings, ...partial });
  };

  const patchSync = (partial: Partial<SyncConfig>) => {
    const nextSync = { ...syncDraft, ...partial };
    setSyncDraft(nextSync);
  };

  const handleSaveSync = async () => {
    await onUpdateSettings({ ...settings, sync: syncDraft });
    setSyncNotice({ ok: true, text: "同步配置已保存至本地 settings.json" });
  };

  const handleTestSync = async () => {
    if (syncDraft.provider === "none") return;
    setTesting(true);
    setSyncNotice(null);
    try {
      const nextSettings: AppSettings = { ...settings, sync: syncDraft };
      await onUpdateSettings(nextSettings);
      const msg = await testSyncConnection(syncDraft.provider, nextSettings);
      setSyncNotice({ ok: true, text: msg });
    } catch (err) {
      setSyncNotice({ ok: false, text: String(err) });
    } finally {
      setTesting(false);
    }
  };

  const handleRunSync = async () => {
    if (syncDraft.provider === "none") return;
    setSyncing(true);
    setSyncNotice(null);
    try {
      const nextSettings: AppSettings = { ...settings, sync: syncDraft };
      await onUpdateSettings(nextSettings);
      const res = await runSyncNow(syncDraft.provider, nextSettings);
      setSyncStatus(res);
      setSyncNotice({ ok: true, text: res.message });
      onSyncCompleted?.();
    } catch (err) {
      setSyncNotice({ ok: false, text: String(err) });
      void fetchSyncStatus().then(setSyncStatus);
    } finally {
      setSyncing(false);
    }
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
    <div className="flex flex-col gap-5 pb-8 max-w-4xl">
      <div>
        <h1 className="text-[20px] font-semibold tracking-tight text-[var(--text-primary)]">
          监控、同步与隐私设置
        </h1>
        <p className="text-[12.5px] text-[var(--text-secondary)] mt-0.5">
          调整 Windows 前台采样策略、WebDAV / GitHub 仓库跨设备同步及本地纯离线数据管理
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

      {/* Card 3: WebDAV & GitHub Repo Sync */}
      <div className="card-surface p-5 flex flex-col gap-4">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h3 className="text-[14px] font-semibold text-[var(--text-primary)]">
              跨设备双向同步（WebDAV / GitHub 仓库）
            </h3>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              默认纯离线运行；按需绑定您自有 WebDAV 网盘或私有 GitHub 仓库，按日无损合并多台设备会话与自定义规则
            </p>
          </div>
          {syncStatus?.last_sync_ts && (
            <span className="px-2.5 py-1 rounded-full text-[11px] font-medium bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[var(--text-secondary)] tabular-nums">
              上次同步：{formatTimestampHm(syncStatus.last_sync_ts)}
            </span>
          )}
        </div>

        {/* Provider Switcher */}
        <div className="grid grid-cols-3 gap-2">
          {[
            { id: "none" as const, title: "仅本地离线", desc: "不连接任何远端服务" },
            { id: "webdav" as const, title: "WebDAV 同步", desc: "坚果云 / Nextcloud / Alist" },
            { id: "github" as const, title: "GitHub 仓库", desc: "通过私有仓库与 PAT 同步" },
          ].map((opt) => {
            const active = syncDraft.provider === opt.id;
            return (
              <button
                key={opt.id}
                type="button"
                onClick={() => {
                  const nextSync = { ...syncDraft, provider: opt.id };
                  setSyncDraft(nextSync);
                  if (opt.id === "none") {
                    void onUpdateSettings({ ...settings, sync: nextSync });
                  }
                }}
                className={`p-3 rounded-xl border text-left transition-all cursor-pointer ${
                  active
                    ? "border-[var(--accent)] bg-[var(--accent-soft)]"
                    : "border-[var(--border-hairline)] bg-[var(--bg-subtle)] hover:border-[var(--border-strong)]"
                }`}
              >
                <div className="flex items-center gap-1.5 text-[12.5px] font-semibold text-[var(--text-primary)]">
                  {opt.id === "webdav" && <CloudSyncIcon size={14} />}
                  {opt.id === "github" && <GitRepoIcon size={14} />}
                  <span>{opt.title}</span>
                </div>
                <div className="text-[11px] text-[var(--text-muted)] mt-0.5">{opt.desc}</div>
              </button>
            );
          })}
        </div>

        {/* WebDAV Configuration Form */}
        {syncDraft.provider === "webdav" && (
          <div className="flex flex-col gap-3 pt-2 border-t border-[var(--border-hairline)]">
            <div className="flex flex-wrap items-center gap-1.5">
              <span className="text-[11.5px] text-[var(--text-muted)] mr-1">快捷填充地址：</span>
              {WEBDAV_PRESETS.map((p) => (
                <button
                  key={p.label}
                  type="button"
                  onClick={() =>
                    patchSync({ webdav: { ...syncDraft.webdav, server_url: p.url } })
                  }
                  className="px-2.5 py-1 rounded-lg text-[11px] font-medium bg-[var(--bg-subtle)] border border-[var(--border-hairline)] hover:border-[var(--border-strong)] text-[var(--text-secondary)] hover:text-[var(--text-primary)] cursor-pointer"
                >
                  {p.label}
                </button>
              ))}
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <label className="flex flex-col gap-1 md:col-span-2">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  WebDAV 服务器地址
                </span>
                <input
                  type="text"
                  value={syncDraft.webdav.server_url}
                  onChange={(e) =>
                    patchSync({ webdav: { ...syncDraft.webdav, server_url: e.target.value } })
                  }
                  placeholder="https://dav.jianguoyun.com/dav/"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  账号 / 用户名
                </span>
                <input
                  type="text"
                  value={syncDraft.webdav.username}
                  onChange={(e) =>
                    patchSync({ webdav: { ...syncDraft.webdav, username: e.target.value } })
                  }
                  placeholder="your-email@example.com"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>

              <label className="flex flex-col gap-1">
                <div className="flex items-center justify-between">
                  <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                    应用专用密码
                  </span>
                  <button
                    type="button"
                    onClick={() => setShowSecret((v) => !v)}
                    className="text-[11px] text-[var(--accent)] cursor-pointer"
                  >
                    {showSecret ? "隐藏" : "显示"}
                  </button>
                </div>
                <input
                  type={showSecret ? "text" : "password"}
                  value={syncDraft.webdav.password}
                  onChange={(e) =>
                    patchSync({ webdav: { ...syncDraft.webdav, password: e.target.value } })
                  }
                  placeholder="建议使用网盘应用专用密码"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  远端同步子目录
                </span>
                <input
                  type="text"
                  value={syncDraft.webdav.remote_dir}
                  onChange={(e) =>
                    patchSync({ webdav: { ...syncDraft.webdav, remote_dir: e.target.value } })
                  }
                  placeholder="PrismTime"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  本机设备标识名
                </span>
                <input
                  type="text"
                  value={syncDraft.device_name}
                  onChange={(e) => patchSync({ device_name: e.target.value })}
                  placeholder="Windows-PC"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>
            </div>
          </div>
        )}

        {/* GitHub Repository Configuration Form */}
        {syncDraft.provider === "github" && (
          <div className="flex flex-col gap-3 pt-2 border-t border-[var(--border-hairline)]">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  GitHub 用户名 / 组织（Owner）
                </span>
                <input
                  type="text"
                  value={syncDraft.github.owner}
                  onChange={(e) =>
                    patchSync({ github: { ...syncDraft.github, owner: e.target.value } })
                  }
                  placeholder="your-github-username"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  目标仓库名（强烈建议使用 Private 私有仓库）
                </span>
                <input
                  type="text"
                  value={syncDraft.github.repo}
                  onChange={(e) =>
                    patchSync({ github: { ...syncDraft.github, repo: e.target.value } })
                  }
                  placeholder="prismtime-sync-data"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  分支名称（Branch）
                </span>
                <input
                  type="text"
                  value={syncDraft.github.branch}
                  onChange={(e) =>
                    patchSync({ github: { ...syncDraft.github, branch: e.target.value } })
                  }
                  placeholder="main"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  仓库内数据目录前缀
                </span>
                <input
                  type="text"
                  value={syncDraft.github.path_prefix}
                  onChange={(e) =>
                    patchSync({ github: { ...syncDraft.github, path_prefix: e.target.value } })
                  }
                  placeholder="prismtime-sync"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>

              <label className="flex flex-col gap-1 md:col-span-2">
                <div className="flex items-center justify-between">
                  <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                    Personal Access Token（需具备目标仓库 Contents Read &amp; Write 权限）
                  </span>
                  <button
                    type="button"
                    onClick={() => setShowSecret((v) => !v)}
                    className="text-[11px] text-[var(--accent)] cursor-pointer"
                  >
                    {showSecret ? "隐藏" : "显示"}
                  </button>
                </div>
                <input
                  type={showSecret ? "text" : "password"}
                  value={syncDraft.github.token}
                  onChange={(e) =>
                    patchSync({ github: { ...syncDraft.github, token: e.target.value } })
                  }
                  placeholder="github_pat_... 或 ghp_..."
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>

              <label className="flex flex-col gap-1 md:col-span-2">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  本机设备标识名（写入 Commit 提交备注）
                </span>
                <input
                  type="text"
                  value={syncDraft.device_name}
                  onChange={(e) => patchSync({ device_name: e.target.value })}
                  placeholder="Windows-PC"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[12.5px] text-[var(--text-primary)] focus:outline-none select-text"
                />
              </label>
            </div>
          </div>
        )}

        {/* Shared Sync Toggles & Action Buttons */}
        {syncDraft.provider !== "none" && (
          <div className="flex flex-col gap-3.5 pt-3 border-t border-[var(--border-hairline)]">
            <ToggleRow
              title="同时同步自定义软件/进程规则"
              desc="多设备共享自定义软件名称、归类、进程别名与忽略规则"
              checked={syncDraft.sync_overrides}
              onChange={(v) => patchSync({ sync_overrides: v })}
            />

            <ToggleRow
              title="开启后台定时自动同步"
              desc="按设定周期在后台静默合并会话记录（遵循 Windows 系统代理配置）"
              checked={syncDraft.auto_sync_enabled}
              onChange={(v) => patchSync({ auto_sync_enabled: v })}
            />

            {syncDraft.auto_sync_enabled && (
              <div className="grid grid-cols-4 gap-1.5">
                {SYNC_INTERVAL_OPTIONS.map((opt) => (
                  <button
                    key={opt.mins}
                    type="button"
                    onClick={() => patchSync({ auto_sync_interval_mins: opt.mins })}
                    className={`py-1.5 rounded-xl text-[12px] font-medium border transition-colors cursor-pointer ${
                      syncDraft.auto_sync_interval_mins === opt.mins
                        ? "bg-[var(--text-primary)] text-[var(--bg-surface)] border-transparent"
                        : "bg-[var(--bg-subtle)] text-[var(--text-secondary)] border-[var(--border-hairline)] hover:text-[var(--text-primary)]"
                    }`}
                  >
                    每 {opt.label}
                  </button>
                ))}
              </div>
            )}

            <div className="flex flex-wrap items-center gap-2 pt-1">
              <button
                type="button"
                onClick={() => void handleSaveSync()}
                className="px-3.5 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] hover:border-[var(--border-strong)] text-[12.5px] font-medium text-[var(--text-primary)] cursor-pointer"
              >
                保存配置
              </button>
              <button
                type="button"
                disabled={testing || syncing}
                onClick={() => void handleTestSync()}
                className="px-3.5 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] hover:border-[var(--border-strong)] text-[12.5px] font-medium text-[var(--text-primary)] cursor-pointer disabled:opacity-60"
              >
                {testing ? "正在测试连通性..." : "测试连接"}
              </button>
              <button
                type="button"
                disabled={syncing || testing}
                onClick={() => void handleRunSync()}
                className="inline-flex items-center gap-1.5 px-4 py-2 rounded-xl bg-[var(--accent)] text-white text-[12.5px] font-medium hover:opacity-90 cursor-pointer disabled:opacity-60"
              >
                <RefreshIcon size={13} className={syncing ? "animate-spin" : ""} />
                <span>{syncing ? "正在双向同步..." : "立即双向同步"}</span>
              </button>
            </div>

            {syncNotice && (
              <div
                className={`px-3.5 py-2.5 rounded-xl text-[12px] border select-text ${
                  syncNotice.ok
                    ? "bg-[#109e70]/10 border-[#109e70]/30 text-[var(--status-ok)]"
                    : "bg-[#dc2626]/10 border-[#dc2626]/30 text-[#dc2626]"
                }`}
              >
                {syncNotice.text}
              </div>
            )}
          </div>
        )}
      </div>

      {/* Card 4: Local Storage & Data Export */}
      <div className="card-surface p-5 flex flex-col gap-4">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h3 className="text-[14px] font-semibold text-[var(--text-primary)]">
              本地沙盒存储与报告导出
            </h3>
            <p className="text-[12px] text-[var(--text-secondary)] mt-0.5">
              所有记录均以按日分片 JSONL 文件保存在本地磁盘，未经您手动配置同步绝不向外传输数据
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
