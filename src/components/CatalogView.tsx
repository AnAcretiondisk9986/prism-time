import React, { useMemo, useState } from "react";
import type { AppCategory, CatalogAppDto } from "../types";
import { CATEGORY_LIST, CATEGORY_MAP } from "../types";
import { formatDuration, formatTimestampHm } from "../lib/format";
import { rescanLocalSoftware, updateAppOverride } from "../lib/ipc";
import { AppAvatar } from "./AppAvatar";
import { EditIcon, RefreshIcon, SearchIcon } from "./Icons";

interface CatalogViewProps {
  catalog: CatalogAppDto[];
  onRefreshCatalog: (updated: CatalogAppDto[]) => void;
  onSelectApp: (appId: string) => void;
}

export const CatalogView: React.FC<CatalogViewProps> = ({
  catalog,
  onRefreshCatalog,
  onSelectApp,
}) => {
  const [search, setSearch] = useState("");
  const [selectedCategory, setSelectedCategory] = useState<AppCategory | "all" | "ignored">("all");
  const [scanning, setScanning] = useState(false);
  const [editingApp, setEditingApp] = useState<CatalogAppDto | null>(null);
  const [editName, setEditName] = useState("");
  const [editCategory, setEditCategory] = useState<AppCategory>("utility");
  const [editDesc, setEditDesc] = useState("");
  const [editIgnored, setEditIgnored] = useState(false);
  const [saving, setSaving] = useState(false);

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

  return (
    <div className="flex flex-col gap-5 pb-6">
      {/* Header */}
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h1 className="text-[20px] font-semibold tracking-tight text-[var(--text-primary)]">
            本机软件识别库
          </h1>
          <p className="text-[12.5px] text-[var(--text-secondary)] mt-0.5 tabular-nums">
            已收录 {visibleCount} 款桌面软件与进程（系统守护噪音已屏蔽；构建期未内置识别程序自动按无拓展名进程名记录）
          </p>
        </div>

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
            placeholder="搜索软件名称、无拓展名进程名、功能说明或开发商..."
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
              app.source === "known"
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
                      <div className="flex items-center gap-1.5 mt-0.5">
                        <span
                          className="px-2 py-0.5 rounded-full text-[10.5px] font-medium shrink-0"
                          style={{
                            backgroundColor: `${cat.color}16`,
                            color: cat.color,
                          }}
                        >
                          {cat.label}
                        </span>
                        <span className="text-[10.5px] text-[var(--text-muted)] truncate">
                          {app.publisher}
                        </span>
                      </div>
                    </div>
                  </div>

                  <button
                    type="button"
                    onClick={(e) => openEditor(app, e)}
                    title="修改软件名称、分类或忽略"
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

      {/* Edit Modal */}
      {editingApp && (
        <div
          className="fixed inset-0 z-50 bg-black/35 backdrop-blur-[2px] flex items-center justify-center p-4"
          onClick={() => setEditingApp(null)}
        >
          <form
            onClick={(e) => e.stopPropagation()}
            onSubmit={handleSaveEdit}
            className="card-surface w-full max-w-md p-5 flex flex-col gap-4"
            style={{ boxShadow: "var(--shadow-popover)" }}
          >
            <div className="flex items-center gap-3">
              <AppAvatar
                name={editingApp.display_name}
                category={editCategory}
                iconDataUrl={editingApp.icon_data_url}
                size={36}
              />
              <div>
                <h3 className="text-[15px] font-semibold text-[var(--text-primary)]">
                  编辑软件信息与归类
                </h3>
                <p className="text-[11.5px] text-[var(--text-muted)]">{editingApp.publisher}</p>
              </div>
            </div>

            <label className="flex flex-col gap-1.5">
              <span className="text-[12px] font-medium text-[var(--text-secondary)]">显示名称</span>
              <input
                type="text"
                value={editName}
                onChange={(e) => setEditName(e.target.value)}
                className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[13px] text-[var(--text-primary)] focus:outline-none"
              />
            </label>

            <label className="flex flex-col gap-1.5">
              <span className="text-[12px] font-medium text-[var(--text-secondary)]">软件类别</span>
              <select
                value={editCategory}
                onChange={(e) => setEditCategory(e.target.value as AppCategory)}
                className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[13px] text-[var(--text-primary)] focus:outline-none"
              >
                {CATEGORY_LIST.map((cat) => (
                  <option key={cat.id} value={cat.id}>
                    {cat.label}
                  </option>
                ))}
              </select>
            </label>

            <label className="flex flex-col gap-1.5">
              <span className="text-[12px] font-medium text-[var(--text-secondary)]">功能简介</span>
              <textarea
                rows={2}
                value={editDesc}
                onChange={(e) => setEditDesc(e.target.value)}
                className="px-3 py-2 rounded-xl bg-[var(--bg-subtle)] border border-[var(--border-hairline)] focus:border-[var(--border-strong)] text-[13px] text-[var(--text-primary)] focus:outline-none resize-none"
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

            <div className="flex items-center justify-end gap-2 pt-1">
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
                {saving ? "保存中..." : "保存修改"}
              </button>
            </div>
          </form>
        </div>
      )}
    </div>
  );
};
