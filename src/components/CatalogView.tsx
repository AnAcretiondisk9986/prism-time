import React, { useEffect, useMemo, useState } from "react";
import type { AppCategory, CatalogAppDto, CustomAppRule } from "../types";
import { CATEGORY_LIST, CATEGORY_MAP } from "../types";
import { formatDuration, formatTimestampHm } from "../lib/format";
import {
  deleteCustomAppRule,
  fetchCustomAppRules,
  fetchSoftwareCatalog,
  rescanLocalSoftware,
  resetAppOverride,
  saveCustomAppRule,
  updateAppOverride,
} from "../lib/ipc";
import { AppAvatar } from "./AppAvatar";
import { EditIcon, PlusIcon, RefreshIcon, SearchIcon, TrashIcon } from "./Icons";

interface CatalogViewProps {
  catalog: CatalogAppDto[];
  onRefreshCatalog: (updated: CatalogAppDto[]) => void;
  onSelectApp: (appId: string) => void;
}

const IDLE_MULT_OPTIONS: Array<{ value: string; label: string }> = [
  { value: "default", label: "跟随全局设置" },
  { value: "1", label: "标准（1 倍）" },
  { value: "2", label: "宽松（2 倍）" },
  { value: "3", label: "影音/演示（3 倍）" },
  { value: "0", label: "持续计时（永不判定空闲）" },
];

function splitCommaList(raw: string): string[] {
  return raw
    .split(/[,，;\n]/)
    .map((s) => s.trim())
    .filter(Boolean);
}

function readIconFileAsDataUrl(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    if (file.size > 384 * 1024) {
      reject(new Error("图标文件请小于 384 KB"));
      return;
    }
    const reader = new FileReader();
    reader.onload = () => resolve(String(reader.result || ""));
    reader.onerror = () => reject(new Error("读取图标文件失败"));
    reader.readAsDataURL(file);
  });
}

export const CatalogView: React.FC<CatalogViewProps> = ({
  catalog,
  onRefreshCatalog,
  onSelectApp,
}) => {
  const [search, setSearch] = useState("");
  const [selectedCategory, setSelectedCategory] = useState<AppCategory | "all" | "ignored">("all");
  const [scanning, setScanning] = useState(false);

  // Custom process rules state
  const [customRules, setCustomRules] = useState<CustomAppRule[]>([]);
  const [ruleModalOpen, setRuleModalOpen] = useState(false);
  const [editingRuleId, setEditingRuleId] = useState<string | null>(null);
  const [ruleName, setRuleName] = useState("");
  const [ruleCategory, setRuleCategory] = useState<AppCategory>("utility");
  const [ruleDesc, setRuleDesc] = useState("");
  const [ruleTag, setRuleTag] = useState("");
  const [ruleProcs, setRuleProcs] = useState("");
  const [rulePaths, setRulePaths] = useState("");
  const [ruleTitles, setRuleTitles] = useState("");
  const [ruleIdleMult, setRuleIdleMult] = useState<string>("1");
  const [ruleAsBrowser, setRuleAsBrowser] = useState(false);
  const [ruleIgnored, setRuleIgnored] = useState(false);
  const [ruleMergeInto, setRuleMergeInto] = useState("");
  const [ruleIcon, setRuleIcon] = useState<string | null>(null);
  const [ruleError, setRuleError] = useState<string | null>(null);
  const [savingRule, setSavingRule] = useState(false);

  // Existing app override modal state
  const [editingApp, setEditingApp] = useState<CatalogAppDto | null>(null);
  const [editName, setEditName] = useState("");
  const [editCategory, setEditCategory] = useState<AppCategory>("utility");
  const [editDesc, setEditDesc] = useState("");
  const [editIgnored, setEditIgnored] = useState(false);
  const [editExtraProcs, setEditExtraProcs] = useState("");
  const [editIdleMult, setEditIdleMult] = useState<string>("default");
  const [editBrowserMode, setEditBrowserMode] = useState<"auto" | "force_on" | "force_off">("auto");
  const [editMergeInto, setEditMergeInto] = useState("");
  const [editTag, setEditTag] = useState("");
  const [editCustomIcon, setEditCustomIcon] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    void fetchCustomAppRules().then(setCustomRules);
  }, []);

  const visibleCount = useMemo(
    () => catalog.filter((a) => !a.ignored).length,
    [catalog],
  );

  const filtered = useMemo(() => {
    const q = search.trim().toLowerCase();
    return catalog.filter((app) => {
      if (selectedCategory === "ignored") {
        if (!app.ignored) return false;
      } else {
        if (app.ignored) return false;
        if (selectedCategory !== "all" && app.category !== selectedCategory) return false;
      }
      if (!q) return true;
      return (
        app.display_name.toLowerCase().includes(q) ||
        app.process_names.some((p) => p.toLowerCase().includes(q)) ||
        app.description.toLowerCase().includes(q) ||
        app.publisher.toLowerCase().includes(q) ||
        (app.custom_tag || "").toLowerCase().includes(q) ||
        CATEGORY_MAP[app.category].label.toLowerCase().includes(q)
      );
    });
  }, [catalog, search, selectedCategory]);

  const handleRescan = async () => {
    setScanning(true);
    try {
      const updated = await rescanLocalSoftware();
      onRefreshCatalog(updated);
    } finally {
      setScanning(false);
    }
  };

  const openEditor = (app: CatalogAppDto, e: React.MouseEvent) => {
    e.stopPropagation();
    setEditingApp(app);
    setEditName(app.display_name);
    setEditCategory(app.category);
    setEditDesc(app.description);
    setEditIgnored(app.ignored);
    setEditExtraProcs((app.extra_process_names || []).join(", "));
    setEditIdleMult(
      app.idle_multiplier === null || app.idle_multiplier === undefined
        ? "default"
        : String(app.idle_multiplier),
    );
    setEditBrowserMode(
      app.treat_as_browser === true
        ? "force_on"
        : app.treat_as_browser === false
          ? "force_off"
          : "auto",
    );
    setEditMergeInto(app.merge_into_app_id || "");
    setEditTag(app.custom_tag || "");
    setEditCustomIcon(null);
  };

  const handleSaveEdit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editingApp) return;
    setSaving(true);
    try {
      const updatedItem = await updateAppOverride({
        app_id: editingApp.app_id,
        display_name: editName.trim() || editingApp.display_name,
        category: editCategory,
        description: editDesc.trim() || editingApp.description,
        ignored: editIgnored,
        extra_process_names: splitCommaList(editExtraProcs),
        idle_multiplier: editIdleMult === "default" ? null : Number(editIdleMult),
        treat_as_browser:
          editBrowserMode === "force_on"
            ? true
            : editBrowserMode === "force_off"
              ? false
              : null,
        merge_into_app_id: editMergeInto.trim() || null,
        custom_tag: editTag.trim() || null,
        custom_icon_data_url: editCustomIcon,
      });
      onRefreshCatalog(
        catalog.map((item) =>
          item.app_id === updatedItem.app_id ? { ...item, ...updatedItem } : item,
        ),
      );
      setEditingApp(null);
    } finally {
      setSaving(false);
    }
  };

  const handleResetOverride = async () => {
    if (!editingApp) return;
    setSaving(true);
    try {
      const refreshed = await resetAppOverride(editingApp.app_id);
      onRefreshCatalog(refreshed);
      setEditingApp(null);
    } finally {
      setSaving(false);
    }
  };

  const openNewRuleModal = () => {
    setEditingRuleId(null);
    setRuleName("");
    setRuleCategory("dev");
    setRuleDesc("");
    setRuleTag("");
    setRuleProcs("");
    setRulePaths("");
    setRuleTitles("");
    setRuleIdleMult("1");
    setRuleAsBrowser(false);
    setRuleIgnored(false);
    setRuleMergeInto("");
    setRuleIcon(null);
    setRuleError(null);
    setRuleModalOpen(true);
  };

  const openEditRuleModal = (rule: CustomAppRule) => {
    setEditingRuleId(rule.rule_id);
    setRuleName(rule.display_name);
    setRuleCategory(rule.category);
    setRuleDesc(rule.description);
    setRuleTag(rule.custom_tag || "");
    setRuleProcs(rule.process_names.join(", "));
    setRulePaths(rule.path_contains.join(", "));
    setRuleTitles(rule.title_contains.join(", "));
    setRuleIdleMult(String(rule.idle_multiplier));
    setRuleAsBrowser(rule.treat_as_browser);
    setRuleIgnored(rule.ignored);
    setRuleMergeInto(rule.merge_into_app_id || "");
    setRuleIcon(rule.custom_icon_data_url || null);
    setRuleError(null);
    setRuleModalOpen(true);
  };

  const handleSaveRule = async (e: React.FormEvent) => {
    e.preventDefault();
    setRuleError(null);
    const procs = splitCommaList(ruleProcs);
    const paths = splitCommaList(rulePaths);
    const titles = splitCommaList(ruleTitles);
    if (!ruleName.trim()) {
      setRuleError("请填写软件名称");
      return;
    }
    if (procs.length === 0 && paths.length === 0 && titles.length === 0) {
      setRuleError("请至少填写一个进程名、路径关键词或窗口标题关键词");
      return;
    }
    setSavingRule(true);
    try {
      const updatedRules = await saveCustomAppRule({
        rule_id: editingRuleId,
        display_name: ruleName.trim(),
        category: ruleCategory,
        description: ruleDesc.trim() || null,
        process_names: procs,
        path_contains: paths,
        title_contains: titles,
        ignored: ruleIgnored,
        treat_as_browser: ruleAsBrowser,
        idle_multiplier: Number(ruleIdleMult),
        merge_into_app_id: ruleMergeInto.trim() || null,
        custom_tag: ruleTag.trim() || null,
        custom_icon_data_url: ruleIcon,
      });
      setCustomRules(updatedRules);
      const latestCatalog = await fetchSoftwareCatalog();
      onRefreshCatalog(latestCatalog);
      setRuleModalOpen(false);
    } catch (err) {
      setRuleError(String(err));
    } finally {
      setSavingRule(false);
    }
  };

  const handleDeleteRule = async (ruleId: string) => {
    const updatedRules = await deleteCustomAppRule(ruleId);
    setCustomRules(updatedRules);
    const latestCatalog = await fetchSoftwareCatalog();
    onRefreshCatalog(latestCatalog);
  };

  const mergeTargets = useMemo(
    () =>
      catalog
        .filter((a) => !a.ignored && a.app_id !== editingApp?.app_id)
        .sort((a, b) => a.display_name.localeCompare(b.display_name)),
    [catalog, editingApp],
  );

  return (
    <div className="flex flex-col gap-5 pb-6">
      {/* Header */}
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h1 className="text-[20px] font-semibold tracking-tight text-[var(--text-primary)]">
            本机软件识别库
          </h1>
          <p className="text-[12.5px] text-[var(--text-secondary)] mt-0.5 tabular-nums">
            已收录 {visibleCount} 款桌面软件与进程 · 支持自定义进程规则、别名归并、空闲倍率与浏览器网页识别标记
          </p>
        </div>

        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={openNewRuleModal}
            className="inline-flex items-center gap-1.5 px-3.5 py-2 rounded-xl bg-[var(--accent)] text-white text-[12.5px] font-medium hover:opacity-90 transition-opacity cursor-pointer"
          >
            <PlusIcon size={14} />
            <span>自定义进程规则</span>
          </button>

          <button
            type="button"
            onClick={() => void handleRescan()}
            disabled={scanning}
            className="inline-flex items-center gap-2 px-3.5 py-2 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-hairline)] hover:border-[var(--border-strong)] text-[12.5px] font-medium text-[var(--text-primary)] transition-colors cursor-pointer disabled:opacity-60"
          >
            <RefreshIcon size={14} className={scanning ? "animate-spin" : ""} />
            <span>{scanning ? "正在扫描本机软件..." : "重新扫描本机软件"}</span>
          </button>
        </div>
      </div>

      {/* Custom Process Rules Section (when any exist) */}
      {customRules.length > 0 && (
        <div className="card-surface p-4 flex flex-col gap-3">
          <div className="flex items-center justify-between">
            <h3 className="text-[13.5px] font-semibold text-[var(--text-primary)]">
              我的自定义进程规则 ({customRules.length})
            </h3>
            <span className="text-[11px] text-[var(--text-muted)]">
              最高优先级匹配 · 可覆盖默认识别与噪音过滤
            </span>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-2.5">
            {customRules.map((rule) => {
              const cat = CATEGORY_MAP[rule.category];
              return (
                <div
                  key={rule.rule_id}
                  className="p-3 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] flex items-start justify-between gap-2.5"
                >
                  <div className="min-w-0 flex-1">
                    <div className="flex items-center gap-2 flex-wrap">
                      <span className="text-[13px] font-semibold text-[var(--text-primary)] truncate">
                        {rule.display_name}
                      </span>
                      <span
                        className="px-2 py-0.5 rounded-full text-[10px] font-medium"
                        style={{ backgroundColor: `${cat.color}18`, color: cat.color }}
                      >
                        {cat.shortLabel}
                      </span>
                      {rule.treat_as_browser && (
                        <span className="px-1.5 py-0.5 rounded-md bg-[var(--accent-soft)] text-[var(--accent)] text-[10px] font-medium">
                          识别网页
                        </span>
                      )}
                      {rule.idle_multiplier === 0 && (
                        <span className="px-1.5 py-0.5 rounded-md bg-[#109e70]/12 text-[var(--status-ok)] text-[10px] font-medium">
                          持续计时
                        </span>
                      )}
                      {rule.ignored && (
                        <span className="px-1.5 py-0.5 rounded-md bg-[#dc2626]/10 text-[#dc2626] text-[10px] font-medium">
                          已忽略
                        </span>
                      )}
                    </div>
                    <div className="text-[11px] text-[var(--text-secondary)] mt-1 truncate">
                      {rule.process_names.length > 0 && `进程：${rule.process_names.join(", ")}`}
                      {rule.path_contains.length > 0 && ` · 路径：${rule.path_contains.join(", ")}`}
                      {rule.title_contains.length > 0 && ` · 标题：${rule.title_contains.join(", ")}`}
                    </div>
                  </div>
                  <div className="flex items-center gap-1 shrink-0">
                    <button
                      type="button"
                      onClick={() => openEditRuleModal(rule)}
                      title="编辑规则"
                      className="w-7 h-7 rounded-lg flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-surface)] cursor-pointer"
                    >
                      <EditIcon size={13} />
                    </button>
                    <button
                      type="button"
                      onClick={() => void handleDeleteRule(rule.rule_id)}
                      title="删除规则"
                      className="w-7 h-7 rounded-lg flex items-center justify-center text-[var(--text-muted)] hover:text-[#dc2626] hover:bg-[#dc2626]/10 cursor-pointer"
                    >
                      <TrashIcon size={13} />
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* Search + Category Pills */}
      <div className="flex flex-col gap-3">
        <div className="relative max-w-md">
          <SearchIcon
            size={15}
            className="absolute left-3.5 top-1/2 -translate-y-1/2 text-[var(--text-muted)]"
          />
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="搜索软件名称、无拓展名进程名、自定义标签或开发商..."
            className="w-full pl-9 pr-3.5 py-2 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[13px] text-[var(--text-primary)] placeholder:text-[var(--text-muted)] focus:outline-none"
          />
        </div>

        <div className="flex flex-wrap items-center gap-1.5">
          <button
            type="button"
            onClick={() => setSelectedCategory("all")}
            className={`px-3 py-1.5 rounded-xl text-[12px] font-medium border transition-colors cursor-pointer ${
              selectedCategory === "all"
                ? "bg-[var(--text-primary)] text-[var(--bg-surface)] border-transparent"
                : "bg-[var(--bg-surface)] text-[var(--text-secondary)] border-[var(--border-hairline)] hover:text-[var(--text-primary)]"
            }`}
          >
            全部 ({visibleCount})
          </button>

          {CATEGORY_LIST.map((cat) => {
            const cnt = catalog.filter((a) => !a.ignored && a.category === cat.id).length;
            if (cnt === 0) return null;
            const active = selectedCategory === cat.id;
            return (
              <button
                key={cat.id}
                type="button"
                onClick={() => setSelectedCategory(cat.id)}
                className={`inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-[12px] font-medium border transition-colors cursor-pointer ${
                  active
                    ? "bg-[var(--text-primary)] text-[var(--bg-surface)] border-transparent"
                    : "bg-[var(--bg-surface)] text-[var(--text-secondary)] border-[var(--border-hairline)] hover:text-[var(--text-primary)]"
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

          {catalog.some((a) => a.ignored) && (
            <button
              type="button"
              onClick={() => setSelectedCategory("ignored")}
              className={`px-3 py-1.5 rounded-xl text-[12px] font-medium border transition-colors cursor-pointer ${
                selectedCategory === "ignored"
                  ? "bg-[var(--text-primary)] text-[var(--bg-surface)] border-transparent"
                  : "bg-[var(--bg-surface)] text-[var(--text-muted)] border-[var(--border-hairline)] hover:text-[var(--text-primary)]"
              }`}
            >
              已忽略 ({catalog.filter((a) => a.ignored).length})
            </button>
          )}
        </div>
      </div>

      {/* Software Cards Grid */}
      {filtered.length === 0 ? (
        <div className="card-surface py-14 text-center text-[13px] text-[var(--text-muted)]">
          当前筛选条件下未匹配到已识别软件
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-3.5">
          {filtered.map((app) => {
            const cat = CATEGORY_MAP[app.category];
            const sourceLabel =
              app.source === "custom"
                ? "自定义规则"
                : app.source === "known"
                  ? "百科精确识别"
                  : app.source === "scanned"
                    ? "注册表解析"
                    : "未内置识别 · 进程记录";
            return (
              <div
                key={app.app_id}
                onClick={() => onSelectApp(app.app_id)}
                className="card-surface p-4 flex flex-col justify-between gap-3 hover:border-[var(--border-strong)] transition-colors cursor-pointer group"
              >
                <div className="flex items-start justify-between gap-2.5">
                  <div className="flex items-center gap-3 min-w-0">
                    <AppAvatar
                      name={app.display_name}
                      category={app.category}
                      iconDataUrl={app.icon_data_url}
                      size={40}
                    />
                    <div className="min-w-0">
                      <h4 className="text-[14px] font-semibold text-[var(--text-primary)] group-hover:text-[var(--accent)] transition-colors truncate">
                        {app.display_name}
                      </h4>
                      <div className="flex items-center gap-1.5 mt-0.5 flex-wrap">
                        <span
                          className="px-2 py-0.5 rounded-full text-[10.5px] font-medium shrink-0"
                          style={{
                            backgroundColor: `${cat.color}16`,
                            color: cat.color,
                          }}
                        >
                          {cat.label}
                        </span>
                        {app.custom_tag && (
                          <span className="px-1.5 py-0.5 rounded-full text-[10px] font-medium bg-[var(--accent-soft)] text-[var(--accent)]">
                            {app.custom_tag}
                          </span>
                        )}
                        {app.has_override && (
                          <span className="px-1.5 py-0.5 rounded-full text-[10px] font-medium bg-[var(--bg-subtle)] text-[var(--text-secondary)] border border-[var(--border-hairline)]">
                            已定制
                          </span>
                        )}
                        <span className="text-[10.5px] text-[var(--text-muted)] truncate">
                          {app.publisher}
                        </span>
                      </div>
                    </div>
                  </div>

                  <button
                    type="button"
                    onClick={(e) => openEditor(app, e)}
                    title="自定义软件名称、归类、进程别名、空闲倍率或忽略"
                    className="w-7 h-7 rounded-lg flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-subtle)] transition-colors cursor-pointer shrink-0"
                  >
                    <EditIcon size={13} />
                  </button>
                </div>

                <p className="text-[12px] leading-relaxed text-[var(--text-secondary)] line-clamp-2 min-h-[38px]">
                  {app.description}
                </p>

                <div className="pt-2.5 border-t border-[var(--border-hairline)] flex items-center justify-between gap-2 text-[11px] text-[var(--text-muted)] tabular-nums">
                  <div className="flex items-center gap-1.5 min-w-0">
                    <span className="shrink-0">{sourceLabel}</span>
                    {app.process_names.length > 0 && (
                      <span className="px-1.5 py-0.5 rounded-md bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[10px] text-[var(--text-secondary)] truncate">
                        进程 · {app.process_names[0]}
                      </span>
                    )}
                  </div>
                  <span className="shrink-0">
                    {app.today_secs > 0
                      ? `今日 ${formatDuration(app.today_secs, true)}`
                      : `最近活跃 ${formatTimestampHm(app.last_seen_ts)}`}
                  </span>
                </div>
              </div>
            );
          })}
        </div>
      )}

      {/* Enhanced Existing App Customization Modal */}
      {editingApp && (
        <div
          className="fixed inset-0 z-50 bg-black/40 backdrop-blur-[2px] flex items-center justify-center p-4"
          onClick={() => setEditingApp(null)}
        >
          <form
            onClick={(e) => e.stopPropagation()}
            onSubmit={handleSaveEdit}
            className="card-surface w-full max-w-lg max-h-[88vh] scroll-area p-5 flex flex-col gap-3.5"
            style={{ boxShadow: "var(--shadow-popover)" }}
          >
            <div className="flex items-center justify-between gap-3">
              <div className="flex items-center gap-3">
                <AppAvatar
                  name={editName || editingApp.display_name}
                  category={editCategory}
                  iconDataUrl={editCustomIcon || editingApp.icon_data_url}
                  size={38}
                />
                <div>
                  <h3 className="text-[15px] font-semibold text-[var(--text-primary)]">
                    自定义软件与进程选项
                  </h3>
                  <p className="text-[11.5px] text-[var(--text-muted)]">{editingApp.app_id}</p>
                </div>
              </div>
              <label className="px-2.5 py-1.5 rounded-lg bg-[var(--bg-subtle)] border border-[var(--border-hairline)] hover:border-[var(--border-strong)] text-[11.5px] font-medium text-[var(--text-secondary)] cursor-pointer">
                更换图标
                <input
                  type="file"
                  accept="image/png,image/svg+xml,image/jpeg,image/webp"
                  className="hidden"
                  onChange={(e) => {
                    const f = e.target.files?.[0];
                    if (f) {
                      void readIconFileAsDataUrl(f).then(setEditCustomIcon).catch(() => {});
                    }
                  }}
                />
              </label>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">显示名称</span>
                <input
                  type="text"
                  value={editName}
                  onChange={(e) => setEditName(e.target.value)}
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[13px] text-[var(--text-primary)] focus:outline-none"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">软件类别</span>
                <select
                  value={editCategory}
                  onChange={(e) => setEditCategory(e.target.value as AppCategory)}
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[13px] text-[var(--text-primary)] focus:outline-none"
                >
                  {CATEGORY_LIST.map((cat) => (
                    <option key={cat.id} value={cat.id}>
                      {cat.label}
                    </option>
                  ))}
                </select>
              </label>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  自定义标签（可选）
                </span>
                <input
                  type="text"
                  value={editTag}
                  onChange={(e) => setEditTag(e.target.value)}
                  placeholder="例如：主力工作 / 副业 / 深度学习"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  空闲离开判定策略
                </span>
                <select
                  value={editIdleMult}
                  onChange={(e) => setEditIdleMult(e.target.value)}
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
                >
                  {IDLE_MULT_OPTIONS.map((o) => (
                    <option key={o.value} value={o.value}>
                      {o.label}
                    </option>
                  ))}
                </select>
              </label>
            </div>

            <label className="flex flex-col gap-1">
              <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                绑定额外进程名（逗号分隔，自动去除 .exe，归并子进程至此软件）
              </span>
              <input
                type="text"
                value={editExtraProcs}
                onChange={(e) => setEditExtraProcs(e.target.value)}
                placeholder="例如：mytool_helper, sub_renderer"
                className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
              />
            </label>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  浏览器网页识别行为
                </span>
                <select
                  value={editBrowserMode}
                  onChange={(e) =>
                    setEditBrowserMode(e.target.value as "auto" | "force_on" | "force_off")
                  }
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
                >
                  <option value="auto">自动判定（默认）</option>
                  <option value="force_on">强制作为浏览器识别网页</option>
                  <option value="force_off">禁用此软件的网页识别</option>
                </select>
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  将统计时长合并至其他软件
                </span>
                <select
                  value={editMergeInto}
                  onChange={(e) => setEditMergeInto(e.target.value)}
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
                >
                  <option value="">独立统计（不合并）</option>
                  {mergeTargets.map((t) => (
                    <option key={t.app_id} value={t.app_id}>
                      合并至：{t.display_name}
                    </option>
                  ))}
                </select>
              </label>
            </div>

            <label className="flex flex-col gap-1">
              <span className="text-[12px] font-medium text-[var(--text-secondary)]">功能简介</span>
              <textarea
                rows={2}
                value={editDesc}
                onChange={(e) => setEditDesc(e.target.value)}
                className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none resize-none"
              />
            </label>

            <label className="flex items-center justify-between px-3 py-2.5 rounded-xl bg-[var(--bg-subtle)] cursor-pointer">
              <div>
                <div className="text-[12.5px] font-medium text-[var(--text-primary)]">
                  在时长统计中忽略该软件
                </div>
                <div className="text-[11px] text-[var(--text-muted)]">
                  开启后将不再计入屏幕总时长与图表排行
                </div>
              </div>
              <input
                type="checkbox"
                checked={editIgnored}
                onChange={(e) => setEditIgnored(e.target.checked)}
                className="w-4 h-4 accent-[var(--accent)]"
              />
            </label>

            <div className="flex items-center justify-between gap-2 pt-1">
              {editingApp.has_override ? (
                <button
                  type="button"
                  disabled={saving}
                  onClick={() => void handleResetOverride()}
                  className="px-3 py-1.5 rounded-xl text-[12px] font-medium text-[#dc2626] hover:bg-[#dc2626]/10 cursor-pointer"
                >
                  恢复默认设置
                </button>
              ) : (
                <span />
              )}

              <div className="flex items-center gap-2">
                <button
                  type="button"
                  onClick={() => setEditingApp(null)}
                  className="px-3.5 py-1.5 rounded-xl text-[12.5px] font-medium text-[var(--text-secondary)] hover:bg-[var(--bg-subtle)] cursor-pointer"
                >
                  取消
                </button>
                <button
                  type="submit"
                  disabled={saving}
                  className="px-4 py-1.5 rounded-xl text-[12.5px] font-medium bg-[var(--accent)] text-white hover:opacity-90 cursor-pointer disabled:opacity-60"
                >
                  {saving ? "保存中..." : "保存自定义设置"}
                </button>
              </div>
            </div>
          </form>
        </div>
      )}

      {/* Create / Edit Custom Process Rule Modal */}
      {ruleModalOpen && (
        <div
          className="fixed inset-0 z-50 bg-black/40 backdrop-blur-[2px] flex items-center justify-center p-4"
          onClick={() => setRuleModalOpen(false)}
        >
          <form
            onClick={(e) => e.stopPropagation()}
            onSubmit={handleSaveRule}
            className="card-surface w-full max-w-lg max-h-[88vh] scroll-area p-5 flex flex-col gap-3.5"
            style={{ boxShadow: "var(--shadow-popover)" }}
          >
            <div className="flex items-center justify-between gap-2">
              <div>
                <h3 className="text-[15px] font-semibold text-[var(--text-primary)]">
                  {editingRuleId ? "编辑自定义进程规则" : "新建自定义进程/软件规则"}
                </h3>
                <p className="text-[11.5px] text-[var(--text-muted)] mt-0.5">
                  按进程名、路径或窗口标题最高优先级匹配，可为便携软件、自研工具或冷门浏览器定制识别行为
                </p>
              </div>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  软件显示名称 *
                </span>
                <input
                  type="text"
                  value={ruleName}
                  onChange={(e) => setRuleName(e.target.value)}
                  placeholder="例如：本地渲染器 / 我的便携浏览器"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[13px] text-[var(--text-primary)] focus:outline-none"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">归属类别</span>
                <select
                  value={ruleCategory}
                  onChange={(e) => setRuleCategory(e.target.value as AppCategory)}
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[13px] text-[var(--text-primary)] focus:outline-none"
                >
                  {CATEGORY_LIST.map((cat) => (
                    <option key={cat.id} value={cat.id}>
                      {cat.label}
                    </option>
                  ))}
                </select>
              </label>
            </div>

            <label className="flex flex-col gap-1">
              <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                匹配进程名（逗号分隔，自动忽略 .exe 与大小写）
              </span>
              <input
                type="text"
                value={ruleProcs}
                onChange={(e) => setRuleProcs(e.target.value)}
                placeholder="例如：MyTool.exe, MyToolHelper"
                className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
              />
            </label>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  匹配路径关键词（可选）
                </span>
                <input
                  type="text"
                  value={rulePaths}
                  onChange={(e) => setRulePaths(e.target.value)}
                  placeholder="例如：D:\Portable\MyApp"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  匹配窗口标题关键词（可选）
                </span>
                <input
                  type="text"
                  value={ruleTitles}
                  onChange={(e) => setRuleTitles(e.target.value)}
                  placeholder="例如：Dashboard - Local"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
                />
              </label>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  空闲离开倍率
                </span>
                <select
                  value={ruleIdleMult}
                  onChange={(e) => setRuleIdleMult(e.target.value)}
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
                >
                  {IDLE_MULT_OPTIONS.filter((o) => o.value !== "default").map((o) => (
                    <option key={o.value} value={o.value}>
                      {o.label}
                    </option>
                  ))}
                </select>
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  合并至已有软件（可选）
                </span>
                <select
                  value={ruleMergeInto}
                  onChange={(e) => setRuleMergeInto(e.target.value)}
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
                >
                  <option value="">作为独立软件统计</option>
                  {catalog
                    .filter((a) => !a.ignored)
                    .map((a) => (
                      <option key={a.app_id} value={a.app_id}>
                        合并至：{a.display_name}
                      </option>
                    ))}
                </select>
              </label>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  自定义标签（可选）
                </span>
                <input
                  type="text"
                  value={ruleTag}
                  onChange={(e) => setRuleTag(e.target.value)}
                  placeholder="例如：自研工具"
                  className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
                />
              </label>

              <label className="flex flex-col gap-1">
                <span className="text-[12px] font-medium text-[var(--text-secondary)]">
                  自定义图标（可选）
                </span>
                <input
                  type="file"
                  accept="image/png,image/svg+xml,image/jpeg,image/webp"
                  onChange={(e) => {
                    const f = e.target.files?.[0];
                    if (f) {
                      void readIconFileAsDataUrl(f)
                        .then(setRuleIcon)
                        .catch((err) => setRuleError(String(err)));
                    }
                  }}
                  className="px-2.5 py-1.5 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[11.5px] text-[var(--text-secondary)]"
                />
              </label>
            </div>

            <label className="flex flex-col gap-1">
              <span className="text-[12px] font-medium text-[var(--text-secondary)]">说明备注</span>
              <input
                type="text"
                value={ruleDesc}
                onChange={(e) => setRuleDesc(e.target.value)}
                placeholder="简短描述该进程用途"
                className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] text-[12.5px] text-[var(--text-primary)] focus:outline-none"
              />
            </label>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
              <label className="flex items-center justify-between px-3 py-2 rounded-xl bg-[var(--bg-subtle)] cursor-pointer">
                <span className="text-[12px] font-medium text-[var(--text-primary)]">
                  作为浏览器识别网页
                </span>
                <input
                  type="checkbox"
                  checked={ruleAsBrowser}
                  onChange={(e) => setRuleAsBrowser(e.target.checked)}
                  className="w-4 h-4 accent-[var(--accent)]"
                />
              </label>

              <label className="flex items-center justify-between px-3 py-2 rounded-xl bg-[var(--bg-subtle)] cursor-pointer">
                <span className="text-[12px] font-medium text-[var(--text-primary)]">
                  屏蔽忽略（不计入统计）
                </span>
                <input
                  type="checkbox"
                  checked={ruleIgnored}
                  onChange={(e) => setRuleIgnored(e.target.checked)}
                  className="w-4 h-4 accent-[var(--accent)]"
                />
              </label>
            </div>

            {ruleError && (
              <div className="px-3 py-2 rounded-xl bg-[#dc2626]/10 border border-[#dc2626]/30 text-[12px] text-[#dc2626]">
                {ruleError}
              </div>
            )}

            <div className="flex items-center justify-end gap-2 pt-1">
              <button
                type="button"
                onClick={() => setRuleModalOpen(false)}
                className="px-3.5 py-1.5 rounded-xl text-[12.5px] font-medium text-[var(--text-secondary)] hover:bg-[var(--bg-subtle)] cursor-pointer"
              >
                取消
              </button>
              <button
                type="submit"
                disabled={savingRule}
                className="px-4 py-1.5 rounded-xl text-[12.5px] font-medium bg-[var(--accent)] text-white hover:opacity-90 cursor-pointer disabled:opacity-60"
              >
                {savingRule ? "保存中..." : "保存规则"}
              </button>
            </div>
          </form>
        </div>
      )}
    </div>
  );
};
