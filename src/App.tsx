import React, { useCallback, useEffect, useState } from "react";
import type {
  AppSettings,
  CatalogAppDto,
  DashboardAnalyticsDto,
  LiveStatusDto,
  NavTab,
  RangeOption,
} from "./types";
import { formatDateISO, formatDuration } from "./lib/format";
import {
  fetchDashboardAnalytics,
  fetchLiveStatus,
  fetchSettings,
  fetchSoftwareCatalog,
  saveSettings,
  toggleTracking,
} from "./lib/ipc";
import { AppDetailDrawer } from "./components/AppDetailDrawer";
import { CatalogView } from "./components/CatalogView";
import {
  CatalogNavIcon,
  OverviewNavIcon,
  SettingsNavIcon,
  TimelineNavIcon,
  WebNavIcon,
} from "./components/Icons";
import { OverviewView } from "./components/OverviewView";
import { SettingsView } from "./components/SettingsView";
import { TimelineView } from "./components/TimelineView";
import { Titlebar } from "./components/Titlebar";
import { WebInsightsView } from "./components/WebInsightsView";

export function App() {
  const [activeTab, setActiveTab] = useState<NavTab>("overview");
  const [range, setRange] = useState<RangeOption>("today");
  const [selectedDate, setSelectedDate] = useState<string>(() => formatDateISO(new Date()));
  const [demoMode, setDemoMode] = useState<boolean>(false);

  const [settings, setSettings] = useState<AppSettings>({
    tracking_enabled: true,
    idle_threshold_secs: 180,
    pause_on_lock: true,
    minimize_to_tray_on_close: true,
    launch_at_startup: false,
    track_self: false,
    web_tracking_enabled: true,
    theme: "light",
  });
  const [liveStatus, setLiveStatus] = useState<LiveStatusDto | null>(null);
  const [analytics, setAnalytics] = useState<DashboardAnalyticsDto | null>(null);
  const [catalog, setCatalog] = useState<CatalogAppDto[]>([]);
  const [detailAppId, setDetailAppId] = useState<string | null>(null);

  // Sync HTML data-theme attribute
  useEffect(() => {
    document.documentElement.setAttribute("data-theme", settings.theme);
  }, [settings.theme]);

  const loadAnalytics = useCallback(async () => {
    const dateParam = range === "custom" || range === "7d" || range === "30d" ? selectedDate : null;
    const data = await fetchDashboardAnalytics(range, dateParam, demoMode);
    setAnalytics(data);
  }, [range, selectedDate, demoMode]);

  const loadAll = useCallback(async () => {
    const [st, cat, set] = await Promise.all([
      fetchLiveStatus(),
      fetchSoftwareCatalog(),
      fetchSettings(),
    ]);
    setLiveStatus(st);
    setCatalog(cat);
    setSettings(set);
  }, []);

  useEffect(() => {
    void loadAll();
  }, [loadAll]);

  useEffect(() => {
    void loadAnalytics();
  }, [loadAnalytics]);

  // 1-second live pulse for stopwatch & today's total; refresh analytics every 5 seconds
  useEffect(() => {
    let tick = 0;
    const timer = window.setInterval(() => {
      tick += 1;
      void fetchLiveStatus().then(setLiveStatus);
      if (tick % 5 === 0 && !demoMode) {
        void loadAnalytics();
      }
    }, 1000);
    return () => window.clearInterval(timer);
  }, [demoMode, loadAnalytics]);

  const handleUpdateSettings = async (next: AppSettings) => {
    setSettings(next);
    const saved = await saveSettings(next);
    setSettings(saved);
  };

  const handleToggleTheme = () => {
    const nextTheme = settings.theme === "light" ? "dark" : "light";
    void handleUpdateSettings({ ...settings, theme: nextTheme });
  };

  const handleToggleTracking = async () => {
    const nextEnabled = !(liveStatus?.tracking_enabled ?? settings.tracking_enabled);
    const st = await toggleTracking(nextEnabled);
    setLiveStatus(st);
    setSettings((prev) => ({ ...prev, tracking_enabled: nextEnabled }));
  };

  const handleRangeChange = (nextRange: RangeOption) => {
    setRange(nextRange);
    if (nextRange === "today") {
      setSelectedDate(formatDateISO(new Date()));
    }
  };

  const handleSelectDate = (dateStr: string) => {
    setSelectedDate(dateStr);
    if (range !== "7d" && range !== "30d") {
      setRange(dateStr === formatDateISO(new Date()) ? "today" : "custom");
    }
  };

  const navItems: Array<{
    id: NavTab;
    label: string;
    icon: React.ReactNode;
  }> = [
    { id: "overview", label: "今日概览", icon: <OverviewNavIcon size={17} /> },
    { id: "timeline", label: "时段趋势", icon: <TimelineNavIcon size={17} /> },
    { id: "web", label: "网页洞察", icon: <WebNavIcon size={17} /> },
    { id: "catalog", label: "软件识别库", icon: <CatalogNavIcon size={17} /> },
    { id: "settings", label: "监控设置", icon: <SettingsNavIcon size={17} /> },
  ];

  return (
    <div className="h-screen w-screen flex flex-col overflow-hidden bg-[var(--bg-canvas)] text-[var(--text-primary)]">
      <Titlebar
        liveStatus={liveStatus}
        theme={settings.theme}
        demoMode={demoMode}
        onToggleTheme={handleToggleTheme}
        onToggleDemo={() => setDemoMode((v) => !v)}
        onToggleTracking={() => void handleToggleTracking()}
      />

      <div className="flex-1 flex min-h-0">
        {/* Minimalist Left Sidebar */}
        <aside className="w-[210px] shrink-0 bg-[var(--bg-sidebar)] border-r border-[var(--border-hairline)] p-3 flex flex-col justify-between">
          <nav className="flex flex-col gap-1">
            {navItems.map((item) => {
              const active = activeTab === item.id;
              return (
                <button
                  key={item.id}
                  type="button"
                  onClick={() => setActiveTab(item.id)}
                  className={`flex items-center gap-2.5 px-3 py-2.5 rounded-xl text-[13px] font-medium transition-all cursor-pointer ${
                    active
                      ? "bg-[var(--bg-surface)] text-[var(--text-primary)] shadow-xs border border-[var(--border-hairline)]"
                      : "text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-hover)]/65 border border-transparent"
                  }`}
                >
                  <span className={active ? "text-[var(--accent)]" : ""}>{item.icon}</span>
                  <span>{item.label}</span>
                </button>
              );
            })}
          </nav>

          {/* Bottom Real-time Summary Card */}
          <div className="p-3.5 rounded-2xl bg-[var(--bg-surface)] border border-[var(--border-hairline)] flex flex-col gap-1.5">
            <span className="text-[11px] font-medium text-[var(--text-muted)]">
              今日真实屏幕累计
            </span>
            <span className="text-[19px] font-semibold tabular-nums text-[var(--text-primary)] tracking-tight">
              {liveStatus ? formatDuration(liveStatus.today_total_secs) : "--"}
            </span>
            <span className="text-[11px] text-[var(--text-secondary)] tabular-nums">
              已录入 {liveStatus?.catalog_count ?? catalog.length} 款桌面软件
            </span>
          </div>
        </aside>

        {/* Main Workspace Canvas */}
        <main className="flex-1 min-w-0 scroll-area px-7 py-6">
          <div className="max-w-[1200px] mx-auto">
            {activeTab === "overview" && analytics && (
              <OverviewView
                analytics={analytics}
                range={range}
                onChangeRange={handleRangeChange}
                onSelectApp={(aid) => setDetailAppId(aid)}
                onEnableDemo={() => setDemoMode(true)}
                onGoToTimeline={() => setActiveTab("timeline")}
                onGoToWeb={() => setActiveTab("web")}
              />
            )}

            {activeTab === "timeline" && analytics && (
              <TimelineView
                analytics={analytics}
                range={range}
                selectedDate={analytics.selected_date}
                demoMode={demoMode}
                onChangeRange={handleRangeChange}
                onSelectDate={handleSelectDate}
                onSelectApp={(aid) => setDetailAppId(aid)}
              />
            )}

            {activeTab === "web" && analytics && (
              <WebInsightsView
                analytics={analytics}
                range={range}
                onChangeRange={handleRangeChange}
                onEnableDemo={() => setDemoMode(true)}
              />
            )}

            {activeTab === "catalog" && (
              <CatalogView
                catalog={catalog}
                onRefreshCatalog={(updated) => {
                  setCatalog(updated);
                  void loadAnalytics();
                }}
                onSelectApp={(aid) => setDetailAppId(aid)}
              />
            )}

            {activeTab === "settings" && (
              <SettingsView
                settings={settings}
                liveStatus={liveStatus}
                demoMode={demoMode}
                onUpdateSettings={handleUpdateSettings}
              />
            )}
          </div>
        </main>
      </div>

      {/* App Deep Dive Drawer */}
      <AppDetailDrawer
        appId={detailAppId}
        date={analytics?.selected_date || selectedDate}
        demoMode={demoMode}
        onClose={() => setDetailAppId(null)}
        onAppUpdated={() => {
          void fetchSoftwareCatalog().then(setCatalog);
          void loadAnalytics();
        }}
      />
    </div>
  );
}

export default App;
