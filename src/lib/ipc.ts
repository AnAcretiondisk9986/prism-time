import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type {
  AppCategory,
  AppDetailDto,
  AppOverrideInput,
  AppSettings,
  CatalogAppDto,
  DashboardAnalyticsDto,
  ExportResultDto,
  LiveStatusDto,
  SessionEventDto,
  SwimlaneSegmentDto,
  WebAnalyticsDto,
  WebCategory,
} from "../types";
import { CATEGORY_MAP, WEB_CATEGORY_LIST } from "../types";
import { formatDateISO, shiftDateISO } from "./format";

export function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

// --- Browser Preview Fallback Store (used only when running in pure Vite browser outside Tauri) ---
let previewSettings: AppSettings = {
  tracking_enabled: true,
  idle_threshold_secs: 180,
  pause_on_lock: true,
  minimize_to_tray_on_close: true,
  launch_at_startup: false,
  track_self: false,
  web_tracking_enabled: true,
  theme: "light",
};

const previewCatalog: CatalogAppDto[] = [
  {
    app_id: "google.antigravity",
    display_name: "Antigravity",
    category: "dev",
    description: "Google 推出的智能体协作代码编辑器与研发工作台",
    publisher: "Google",
    icon_data_url: null,
    process_names: ["Antigravity"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 8640,
    last_seen_ts: Math.floor(Date.now() / 1000),
    install_hint: "%LOCALAPPDATA%\\Programs\\antigravity\\Antigravity.exe",
  },
  {
    app_id: "openai.codex",
    display_name: "OpenAI Codex",
    category: "ai",
    description: "OpenAI 智能编程代理与自动化桌面协同工作台",
    publisher: "OpenAI",
    icon_data_url: null,
    process_names: ["Codex"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 3180,
    last_seen_ts: Math.floor(Date.now() / 1000) - 420,
    install_hint: "C:\\Program Files\\WindowsApps\\OpenAI.Codex\\app\\ChatGPT.exe",
  },
  {
    app_id: "citrolabs.ego",
    display_name: "Ego Browser",
    category: "browser",
    description: "面向人类与 AI 代理并行浏览的隔离工作空间浏览器",
    publisher: "Citro Labs",
    icon_data_url: null,
    process_names: ["ego"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 4500,
    last_seen_ts: Math.floor(Date.now() / 1000) - 960,
    install_hint: "C:\\Program Files\\Citro Labs\\ego lite\\Application\\ego.exe",
  },
  {
    app_id: "tencent.weixin",
    display_name: "微信",
    category: "social",
    description: "桌面端即时通讯、文件传输与小程序生态平台",
    publisher: "腾讯科技",
    icon_data_url: null,
    process_names: ["Weixin"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 2880,
    last_seen_ts: Math.floor(Date.now() / 1000) - 1800,
    install_hint: "D:\\TXWX\\Weixin\\Weixin.exe",
  },
  {
    app_id: "adobe.photoshop",
    display_name: "Adobe Photoshop",
    category: "design",
    description: "专业位图图像编辑、合成与视觉设计软件",
    publisher: "Adobe",
    icon_data_url: null,
    process_names: ["Photoshop"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 2520,
    last_seen_ts: Math.floor(Date.now() / 1000) - 3600,
    install_hint: "E:\\Adobe\\Adobe Photoshop 2025",
  },
  {
    app_id: "kingsoft.wps",
    display_name: "WPS Office",
    category: "office",
    description: "集文字文档、电子表格、演示文稿与 PDF 于一体的办公套件",
    publisher: "金山办公",
    icon_data_url: null,
    process_names: ["wps"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 2100,
    last_seen_ts: Math.floor(Date.now() / 1000) - 5400,
    install_hint: "%LOCALAPPDATA%\\Kingsoft\\WPS Office",
  },
  {
    app_id: "folia.music",
    display_name: "Folia",
    category: "media",
    description: "主打沉浸式全屏动态歌词动效的跨平台音乐播放器",
    publisher: "chthollyphile",
    icon_data_url: null,
    process_names: ["Folia"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 2880,
    last_seen_ts: Math.floor(Date.now() / 1000) - 7200,
    install_hint: "%LOCALAPPDATA%\\Programs\\Folia",
  },
  {
    app_id: "valve.steam",
    display_name: "Steam",
    category: "game",
    description: "PC 数字游戏发行平台、创意工坊与好友联机中枢",
    publisher: "Valve",
    icon_data_url: null,
    process_names: ["steam"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 3240,
    last_seen_ts: Math.floor(Date.now() / 1000) - 9600,
    install_hint: "C:\\Program Files (x86)\\Steam\\steam.exe",
  },
  {
    app_id: "zai.zcode",
    display_name: "ZCode",
    category: "dev",
    description: "智谱 AI 推出的原生智能体编程开发环境，支持长程目标编程与多端触发",
    publisher: "Z.ai",
    icon_data_url: null,
    process_names: ["ZCode"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 0,
    last_seen_ts: null,
    install_hint: "%LOCALAPPDATA%\\Programs\\ZCode",
  },
  {
    app_id: "qoder.ide",
    display_name: "Qoder",
    category: "dev",
    description: "面向专业工程研发的 Agentic 智能编程 IDE，支持 Editor 与 Quest 协同",
    publisher: "Qoder",
    icon_data_url: null,
    process_names: ["Qoder"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 0,
    last_seen_ts: null,
    install_hint: "D:\\Qoder Global\\Qoder",
  },
  {
    app_id: "tencent.workbuddy",
    display_name: "WorkBuddy",
    category: "ai",
    description: "腾讯全场景 AI 桌面智能体工作台，支持多智能体协作与本地文件处理",
    publisher: "腾讯科技",
    icon_data_url: null,
    process_names: ["WorkBuddy"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 0,
    last_seen_ts: null,
    install_hint: "%LOCALAPPDATA%\\Programs\\WorkBuddy",
  },
  {
    app_id: "ecopaste.app",
    display_name: "EcoPaste",
    category: "utility",
    description: "基于 Tauri 构建的本地优先跨平台剪贴板历史管理工具",
    publisher: "ayangweb",
    icon_data_url: null,
    process_names: ["EcoPaste"],
    source: "known",
    installed: true,
    ignored: false,
    today_secs: 0,
    last_seen_ts: null,
    install_hint: "C:\\Program Files\\EcoPaste\\EcoPaste.exe",
  },
];

function buildPreviewAnalytics(range: string, date: string | null, demoMode: boolean): DashboardAnalyticsDto {
  const todayStr = formatDateISO(new Date());
  const selectedDate = date || (range === "yesterday" ? shiftDateISO(todayStr, -1) : todayStr);
  const dayBase = Math.floor(new Date(`${selectedDate}T00:00:00`).getTime() / 1000);

  const blocks: Array<[string, number, number, string, { site_id: string; site_name: string; domain: string; cat: WebCategory }?]> = [
    ["tencent.weixin", 8 * 60 + 50, 18, "工作群消息与日程同步"],
    [
      "citrolabs.ego",
      9 * 60 + 10,
      16,
      "tauri-apps/tauri: Build desktop apps with Rust",
      { site_id: "web.github", site_name: "GitHub", domain: "github.com", cat: "dev_tech" },
    ],
    [
      "citrolabs.ego",
      9 * 60 + 27,
      10,
      "Rust Win32 UI Automation 工程实践",
      { site_id: "web.juejin", site_name: "稀土掘金", domain: "juejin.cn", cat: "dev_tech" },
    ],
    ["google.antigravity", 9 * 60 + 38, 72, "prism-time — Rust 后端与接口设计"],
    ["openai.codex", 10 * 60 + 52, 24, "Win32 探测与单元测试生成"],
    ["tencent.weixin", 11 * 60 + 20, 14, "午间团队消息确认"],
    ["kingsoft.wps", 13 * 60 + 45, 35, "桌面端视觉设计规格整理"],
    ["google.antigravity", 14 * 60 + 22, 72, "图表组件与时段切片联调"],
    [
      "citrolabs.ego",
      15 * 60 + 38,
      16,
      "澄镜本地预览调试",
      { site_id: "web.localhost", site_name: "本地开发预览", domain: "localhost:1420", cat: "dev_tech" },
    ],
    [
      "citrolabs.ego",
      15 * 60 + 55,
      14,
      "Chart Layout Spec — UI 图标规范",
      { site_id: "web.figma", site_name: "Figma", domain: "figma.com", cat: "docs_work" },
    ],
    ["adobe.photoshop", 16 * 60 + 12, 42, "软件图标与栅格切图导出"],
    ["openai.codex", 16 * 60 + 58, 29, "自动化编译回归检查"],
    ["tencent.weixin", 19 * 60 + 25, 16, "晚间消息互动"],
    ["folia.music", 19 * 60 + 48, 48, "沉浸式全屏歌词音乐播放"],
    ["valve.steam", 20 * 60 + 42, 54, "晚间休闲游戏时段"],
    [
      "citrolabs.ego",
      21 * 60 + 42,
      19,
      "现代桌面开发效率讨论",
      { site_id: "web.zhihu", site_name: "知乎", domain: "zhihu.com", cat: "social_forum" },
    ],
  ];

  const mult = range === "7d" ? 6.4 : range === "30d" ? 26.8 : 1;
  const segments: Array<SwimlaneSegmentDto & { web?: { site_id: string; site_name: string; domain: string; cat: WebCategory } }> = blocks.map(([aid, startMin, durMin, title, web]) => {
    const app = previewCatalog.find((a) => a.app_id === aid)!;
    const endMin = startMin + durMin;
    const sh = Math.floor(startMin / 60);
    const sm = startMin % 60;
    const eh = Math.floor(endMin / 60);
    const em = endMin % 60;
    return {
      app_id: aid,
      display_name: app.display_name,
      category: app.category,
      start_ts: dayBase + startMin * 60,
      end_ts: dayBase + endMin * 60,
      start_minute_of_day: startMin,
      end_minute_of_day: endMin,
      duration_secs: durMin * 60,
      start_label: `${String(sh).padStart(2, "0")}:${String(sm).padStart(2, "0")}`,
      end_label: `${String(eh).padStart(2, "0")}:${String(em).padStart(2, "0")}`,
      sample_title: title,
      web,
    };
  });

  const hourly = Array.from({ length: 24 }, (_, h) => {
    const hs = h * 60;
    const he = (h + 1) * 60;
    const byCat: Partial<Record<AppCategory, number>> = {};
    const appAcc: Record<string, number> = {};
    let total = 0;
    for (const seg of segments) {
      const ov = Math.min(seg.end_minute_of_day, he) - Math.max(seg.start_minute_of_day, hs);
      if (ov > 0) {
        const secs = Math.round(ov * 60 * mult);
        total += secs;
        byCat[seg.category] = (byCat[seg.category] || 0) + secs;
        appAcc[seg.app_id] = (appAcc[seg.app_id] || 0) + secs;
      }
    }
    const topApps = Object.entries(appAcc)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 3)
      .map(([aid, secs]) => {
        const app = previewCatalog.find((a) => a.app_id === aid)!;
        return {
          app_id: aid,
          display_name: app.display_name,
          category: app.category,
          duration_secs: secs,
        };
      });
    return {
      hour: h,
      label: `${String(h).padStart(2, "0")}:00`,
      total_secs: total,
      by_category: byCat,
      top_apps: topApps,
    };
  });

  const totalSecs = hourly.reduce((acc, h) => acc + h.total_secs, 0);
  const catTotals: Partial<Record<AppCategory, number>> = {};
  for (const h of hourly) {
    for (const [k, v] of Object.entries(h.by_category)) {
      const c = k as AppCategory;
      catTotals[c] = (catTotals[c] || 0) + (v || 0);
    }
  }

  const categories = Object.entries(catTotals)
    .map(([k, secs]) => {
      const cat = k as AppCategory;
      const meta = CATEGORY_MAP[cat];
      return {
        category: cat,
        label: meta.label,
        color: meta.color,
        total_secs: secs || 0,
        percentage: totalSecs > 0 ? Math.round(((secs || 0) / totalSecs) * 1000) / 10 : 0,
        app_count: previewCatalog.filter((a) => a.category === cat && a.today_secs > 0).length || 1,
        peak_hour_range: cat === "dev" ? "14:00-16:00" : cat === "media" ? "19:00-21:00" : "09:00-11:00",
      };
    })
    .sort((a, b) => b.total_secs - a.total_secs);

  const periodDefs: Array<[string, string, string, number, number]> = [
    ["late_night", "深夜", "00:00 - 06:00", 0, 6],
    ["early_morning", "清晨", "06:00 - 09:00", 6, 9],
    ["morning", "上午", "09:00 - 12:00", 9, 12],
    ["afternoon", "下午", "12:00 - 18:00", 12, 18],
    ["evening", "夜晚", "18:00 - 24:00", 18, 24],
  ];

  const periods = periodDefs.map(([pid, label, timeRange, s, e]) => {
    const byCat: Partial<Record<AppCategory, number>> = {};
    let pTotal = 0;
    for (let h = s; h < e; h++) {
      pTotal += hourly[h].total_secs;
      for (const [k, v] of Object.entries(hourly[h].by_category)) {
        const c = k as AppCategory;
        byCat[c] = (byCat[c] || 0) + (v || 0);
      }
    }
    const dom = Object.entries(byCat).sort((a, b) => (b[1] || 0) - (a[1] || 0))[0]?.[0] as
      | AppCategory
      | undefined;
    return {
      period_id: pid,
      label,
      time_range: timeRange,
      total_secs: pTotal,
      dominant_category: dom ?? null,
      by_category: byCat,
    };
  });

  const swimlaneMap = new Map<string, SwimlaneSegmentDto[]>();
  for (const s of segments) {
    const arr = swimlaneMap.get(s.app_id) || [];
    arr.push(s);
    swimlaneMap.set(s.app_id, arr);
  }
  const swimlanes = Array.from(swimlaneMap.entries())
    .map(([aid, segs]) => {
      const app = previewCatalog.find((a) => a.app_id === aid)!;
      return {
        app_id: aid,
        display_name: app.display_name,
        category: app.category,
        icon_data_url: app.icon_data_url,
        total_secs: segs.reduce((acc, s) => acc + s.duration_secs, 0),
        segments: segs,
      };
    })
    .sort((a, b) => b.total_secs - a.total_secs);

  const topApps = swimlanes.map((lane) => {
    const app = previewCatalog.find((a) => a.app_id === lane.app_id)!;
    const scaledTotal = Math.round(lane.total_secs * mult);
    const longest = Math.max(...lane.segments.map((s) => s.duration_secs));
    return {
      app_id: app.app_id,
      display_name: app.display_name,
      category: app.category,
      description: app.description,
      publisher: app.publisher,
      icon_data_url: app.icon_data_url,
      total_secs: scaledTotal,
      percentage: totalSecs > 0 ? Math.round((scaledTotal / totalSecs) * 1000) / 10 : 0,
      session_count: Math.round(lane.segments.length * (range === "7d" ? 6 : range === "30d" ? 24 : 1)),
      longest_session_secs: longest,
      peak_period_label: lane.category === "media" || lane.category === "game" ? "夜晚" : "下午",
      active_ranges_preview: lane.segments.map((s) => `${s.start_label}-${s.end_label}`),
      last_seen_ts: app.last_seen_ts,
    };
  });

  const trendDays = range === "30d" ? 30 : 7;
  const dailyTrend = Array.from({ length: trendDays }, (_, i) => {
    const offset = trendDays - 1 - i;
    const dStr = shiftDateISO(selectedDate, -offset);
    const factor = 0.72 + ((i * 19) % 45) / 100;
    const byCat: Partial<Record<AppCategory, number>> = {};
    let dTotal = 0;
    for (const c of categories) {
      const val = Math.round((c.total_secs / mult) * factor);
      byCat[c.category] = val;
      dTotal += val;
    }
    return {
      date: dStr,
      short_label: dStr === todayStr ? "今日" : dStr.slice(5).replace("-", "/"),
      total_secs: dTotal,
      by_category: byCat,
    };
  });

  const recentSessions: SessionEventDto[] = [...segments]
    .reverse()
    .map((s, idx) => {
      const app = previewCatalog.find((a) => a.app_id === s.app_id);
      return {
        id: `prev-${idx}`,
        app_id: s.app_id,
        display_name: s.display_name,
        process_name: app?.process_names[0] ?? null,
        category: s.category,
        icon_data_url: null,
        date: selectedDate,
        start_ts: s.start_ts,
        end_ts: s.end_ts,
        start_time_label: `${s.start_label}:00`,
        end_time_label: `${s.end_label}:00`,
        duration_secs: s.duration_secs,
        sample_title: s.sample_title,
        web_site_id: s.web?.site_id ?? null,
        web_site_name: s.web?.site_name ?? null,
        web_domain: s.web?.domain ?? null,
        web_category: s.web?.cat ?? null,
      };
    });

  // Preview Web Analytics
  const webSegments = segments.filter((s) => Boolean(s.web));
  const webTotalSecs = Math.round(webSegments.reduce((acc, s) => acc + s.duration_secs * mult, 0));
  const webSiteMap = new Map<string, {
    site_id: string;
    display_name: string;
    domain: string;
    category: WebCategory;
    total_secs: number;
    session_count: number;
    browsers: Set<string>;
    ranges: string[];
    last_seen_ts: number;
  }>();
  const webCatMap: Partial<Record<WebCategory, { secs: number; sites: Set<string> }>> = {};

  for (const s of webSegments) {
    const w = s.web!;
    const secs = Math.round(s.duration_secs * mult);
    const existing = webSiteMap.get(w.site_id) || {
      site_id: w.site_id,
      display_name: w.site_name,
      domain: w.domain,
      category: w.cat,
      total_secs: 0,
      session_count: 0,
      browsers: new Set<string>(),
      ranges: [],
      last_seen_ts: s.end_ts,
    };
    existing.total_secs += secs;
    existing.session_count += 1;
    existing.browsers.add(s.display_name);
    existing.ranges.push(`${s.start_label}-${s.end_label}`);
    existing.last_seen_ts = Math.max(existing.last_seen_ts, s.end_ts);
    webSiteMap.set(w.site_id, existing);

    const wc = webCatMap[w.cat] || { secs: 0, sites: new Set<string>() };
    wc.secs += secs;
    wc.sites.add(w.site_id);
    webCatMap[w.cat] = wc;
  }

  const webSites = Array.from(webSiteMap.values())
    .map((item) => ({
      site_id: item.site_id,
      display_name: item.display_name,
      domain: item.domain,
      category: item.category,
      total_secs: item.total_secs,
      percentage: webTotalSecs > 0 ? Math.round((item.total_secs / webTotalSecs) * 1000) / 10 : 0,
      session_count: item.session_count,
      browsers: Array.from(item.browsers),
      active_ranges_preview: item.ranges.slice(0, 4),
      last_seen_ts: item.last_seen_ts,
    }))
    .sort((a, b) => b.total_secs - a.total_secs);

  const webCategories = WEB_CATEGORY_LIST.filter((m) => (webCatMap[m.id]?.secs || 0) > 0)
    .map((m) => {
      const entry = webCatMap[m.id]!;
      return {
        category: m.id,
        label: m.label,
        color: m.color,
        total_secs: entry.secs,
        percentage: webTotalSecs > 0 ? Math.round((entry.secs / webTotalSecs) * 1000) / 10 : 0,
        site_count: entry.sites.size,
      };
    })
    .sort((a, b) => b.total_secs - a.total_secs);

  const webHourly = Array.from({ length: 24 }, (_, h) => {
    const hs = h * 60;
    const he = (h + 1) * 60;
    const byCat: Partial<Record<WebCategory, number>> = {};
    const siteAcc: Record<string, number> = {};
    let total = 0;
    for (const seg of webSegments) {
      const ov = Math.min(seg.end_minute_of_day, he) - Math.max(seg.start_minute_of_day, hs);
      if (ov > 0 && seg.web) {
        const secs = Math.round(ov * 60 * mult);
        total += secs;
        byCat[seg.web.cat] = (byCat[seg.web.cat] || 0) + secs;
        siteAcc[seg.web.site_id] = (siteAcc[seg.web.site_id] || 0) + secs;
      }
    }
    const topSites = Object.entries(siteAcc)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 3)
      .map(([sid, secs]) => {
        const meta = webSiteMap.get(sid)!;
        return {
          site_id: sid,
          display_name: meta.display_name,
          category: meta.category,
          duration_secs: secs,
        };
      });
    return {
      hour: h,
      label: `${String(h).padStart(2, "0")}:00`,
      total_secs: total,
      by_category: byCat,
      top_sites: topSites,
    };
  });

  const webAnalytics: WebAnalyticsDto = {
    total_secs: webTotalSecs,
    share_of_screen_pct: totalSecs > 0 ? Math.round((webTotalSecs / totalSecs) * 1000) / 10 : 0,
    active_site_count: webSites.length,
    session_count: webSegments.length,
    top_site_name: webSites[0]?.display_name ?? null,
    top_site_domain: webSites[0]?.domain ?? null,
    top_site_secs: webSites[0]?.total_secs ?? 0,
    dominant_category: webCategories[0] ?? null,
    categories: webCategories,
    browsers: webTotalSecs > 0 ? [{ app_id: "citrolabs.ego", display_name: "Ego Browser", icon_data_url: null, total_secs: webTotalSecs, percentage: 100 }] : [],
    hourly: webHourly,
    sites: webSites,
    recent_web_sessions: recentSessions.filter((ev) => Boolean(ev.web_site_id)),
  };

  return {
    range,
    date_label:
      range === "7d"
        ? `近 7 日 · ${dailyTrend[0]?.short_label} 至 ${selectedDate.slice(5).replace("-", "/")}`
        : range === "30d"
          ? `近 30 日 · ${dailyTrend[0]?.short_label} 至 ${selectedDate.slice(5).replace("-", "/")}`
          : range === "yesterday"
            ? `昨日 · ${selectedDate}`
            : `今日 · ${selectedDate}`,
    selected_date: selectedDate,
    demo_mode: demoMode,
    has_real_data: true,
    total_secs: totalSecs,
    previous_total_secs: Math.round(totalSecs * 0.91),
    delta_percentage: 9.8,
    active_app_count: topApps.length,
    session_count: recentSessions.length,
    longest_focus_secs: 72 * 60,
    longest_focus_app_name: "Antigravity",
    peak_hour_label: "14:00 - 15:00",
    dominant_category: categories[0] || null,
    categories,
    hourly,
    swimlanes,
    day_segments: segments,
    periods,
    daily_trend: dailyTrend,
    top_apps: topApps,
    recent_sessions: recentSessions,
    web_analytics: webAnalytics,
  };
}

// --- Unified IPC API ---

export async function fetchLiveStatus(): Promise<LiveStatusDto> {
  if (!isTauriRuntime()) {
    return {
      tracking_enabled: previewSettings.tracking_enabled,
      state: previewSettings.tracking_enabled ? "active" : "paused",
      current_app_id: "google.antigravity",
      current_app_name: "Antigravity",
      current_category: "dev",
      current_category_label: "研发与终端",
      current_icon_data_url: null,
      current_web_site: null,
      current_web_domain: null,
      current_session_secs: 1842,
      today_total_secs: 29700,
      idle_secs: 4,
      catalog_count: previewCatalog.filter((a) => !a.ignored).length,
      data_dir: "%LOCALAPPDATA%\\PrismTime",
    };
  }
  return invoke<LiveStatusDto>("get_live_status");
}

export async function fetchDashboardAnalytics(
  range: string,
  date: string | null,
  demoMode: boolean,
): Promise<DashboardAnalyticsDto> {
  if (!isTauriRuntime()) {
    return buildPreviewAnalytics(range, date, demoMode);
  }
  return invoke<DashboardAnalyticsDto>("get_dashboard_analytics", {
    range,
    date,
    demoMode,
  });
}

export async function fetchAppDetail(
  appId: string,
  date: string | null,
  demoMode: boolean,
): Promise<AppDetailDto> {
  if (!isTauriRuntime()) {
    const dash = buildPreviewAnalytics("today", date, demoMode);
    const app = previewCatalog.find((a) => a.app_id === appId) || previewCatalog[0];
    const appSessions = dash.recent_sessions.filter((s) => s.app_id === app.app_id);
    const total = appSessions.reduce((acc, s) => acc + s.duration_secs, 0);
    const hourlySecs = Array.from({ length: 24 }, (_, h) => {
      const hs = h * 60;
      const he = (h + 1) * 60;
      let acc = 0;
      for (const seg of dash.day_segments.filter((s) => s.app_id === app.app_id)) {
        const ov = Math.min(seg.end_minute_of_day, he) - Math.max(seg.start_minute_of_day, hs);
        if (ov > 0) acc += Math.round(ov * 60);
      }
      return acc;
    });
    const peakH = hourlySecs.reduce((best, val, idx, arr) => (val > arr[best] ? idx : best), 0);
    return {
      app: {
        app_id: app.app_id,
        display_name: app.display_name,
        category: app.category,
        description: app.description,
        publisher: app.publisher,
        icon_data_url: app.icon_data_url,
        exe_paths: app.install_hint ? [app.install_hint] : [],
        process_names: app.process_names,
        source: app.source,
        installed: app.installed,
        ignored: app.ignored,
        last_seen_ts: app.last_seen_ts,
      },
      date: dash.selected_date,
      demo_mode: demoMode,
      total_secs: total,
      share_percentage: dash.total_secs > 0 ? Math.round((total / dash.total_secs) * 1000) / 10 : 0,
      session_count: appSessions.length,
      longest_session_secs: appSessions.length ? Math.max(...appSessions.map((s) => s.duration_secs)) : 0,
      average_session_secs: appSessions.length ? Math.round(total / appSessions.length) : 0,
      peak_hour_label: total > 0 ? `${String(peakH).padStart(2, "0")}:00 - ${String((peakH + 1) % 24).padStart(2, "0")}:00` : "--",
      hourly_secs: hourlySecs,
      sessions: appSessions,
      seven_day_trend: dash.daily_trend,
    };
  }
  return invoke<AppDetailDto>("get_app_detail", {
    appId,
    date,
    demoMode,
  });
}

export async function fetchSoftwareCatalog(): Promise<CatalogAppDto[]> {
  if (!isTauriRuntime()) {
    return [...previewCatalog];
  }
  return invoke<CatalogAppDto[]>("get_software_catalog");
}

export async function rescanLocalSoftware(): Promise<CatalogAppDto[]> {
  if (!isTauriRuntime()) {
    return [...previewCatalog];
  }
  return invoke<CatalogAppDto[]>("rescan_local_software");
}

export async function updateAppOverride(input: AppOverrideInput): Promise<CatalogAppDto> {
  if (!isTauriRuntime()) {
    const target = previewCatalog.find((a) => a.app_id === input.app_id);
    if (!target) throw new Error("未找到软件");
    if (input.display_name) target.display_name = input.display_name;
    if (input.category) target.category = input.category;
    if (input.description) target.description = input.description;
    if (typeof input.ignored === "boolean") target.ignored = input.ignored;
    return { ...target };
  }
  return invoke<CatalogAppDto>("update_app_override", { input });
}

export async function fetchSettings(): Promise<AppSettings> {
  if (!isTauriRuntime()) {
    return { ...previewSettings };
  }
  return invoke<AppSettings>("get_settings");
}

export async function saveSettings(settings: AppSettings): Promise<AppSettings> {
  if (!isTauriRuntime()) {
    previewSettings = { ...settings };
    return { ...previewSettings };
  }
  return invoke<AppSettings>("update_settings", { settings });
}

export async function toggleTracking(enabled: boolean): Promise<LiveStatusDto> {
  if (!isTauriRuntime()) {
    previewSettings.tracking_enabled = enabled;
    return fetchLiveStatus();
  }
  return invoke<LiveStatusDto>("toggle_tracking", { enabled });
}

export async function exportUsageReport(
  range: string,
  format: "csv" | "json",
  demoMode: boolean,
): Promise<ExportResultDto> {
  if (!isTauriRuntime()) {
    return {
      file_path: `%LOCALAPPDATA%\\PrismTime\\exports\\prismtime-report-${range}.${format}`,
      record_count: 14,
      format,
    };
  }
  return invoke<ExportResultDto>("export_usage_report", { range, format, demoMode });
}

export async function minimizeAppWindow(): Promise<void> {
  if (!isTauriRuntime()) return;
  try {
    await getCurrentWindow().minimize();
  } catch {
    await invoke("window_minimize");
  }
}

export async function toggleMaximizeAppWindow(): Promise<void> {
  if (!isTauriRuntime()) return;
  try {
    await getCurrentWindow().toggleMaximize();
  } catch {
    await invoke("window_toggle_maximize");
  }
}

export async function closeOrHideAppWindow(): Promise<void> {
  if (!isTauriRuntime()) return;
  await invoke("window_close_or_hide");
}
