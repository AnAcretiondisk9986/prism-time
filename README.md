<div align="center">
  <img src="./assets/logo.svg" width="96" height="96" alt="PrismTime Logo" />
  <h1>澄镜 · PrismTime</h1>
  <p>面向 Windows 平台的极简桌面屏幕使用时长、精确时段轨迹、网页浏览洞察与跨设备同步工具</p>
  <p>
    <a href="https://github.com/AnAcretiondisk9986/prism-time/releases/tag/V0.3.0">
      <img src="https://img.shields.io/badge/Version-V0.3.0-3D5AFE?style=flat-square" alt="Version V0.3.0" />
    </a>
    <img src="https://img.shields.io/badge/Platform-Windows%2010%2F11%20x64-0F172A?style=flat-square" alt="Windows 10/11 x64" />
    <img src="https://img.shields.io/badge/Stack-Rust%20%2B%20Tauri%20v2%20%2B%20React%2019-4F6BED?style=flat-square" alt="Rust + Tauri v2 + React 19" />
    <a href="./LICENSE">
      <img src="https://img.shields.io/badge/License-MIT-1A9988?style=flat-square" alt="License MIT" />
    </a>
  </p>
</div>

---

## 目录

- [项目定位与设计理念](#项目定位与设计理念)
- [核心功能特性](#核心功能特性)
- [软件与进程自定义规则](#软件与进程自定义规则)
- [跨设备同步（WebDAV / GitHub 仓库）](#跨设备同步webdav--github-仓库)
- [可视化图表体系](#可视化图表体系)
- [项目结构](#项目结构)
- [下载与安装](#下载与安装)
- [从源码构建](#从源码构建)
- [本地数据目录与隐私承诺](#本地数据目录与隐私承诺)
- [版本记录](#版本记录)
- [开源许可](#开源许可)

---

## 项目定位与设计理念

**澄镜（PrismTime）** 基于 **Rust + Tauri v2 + TypeScript/React** 构建，围绕 **「无感 · 轻盈 · 本地化」** 三大工程准则，专注于在 Windows 10/11 桌面上以原生级超低系统开销持续记录焦点软件、浏览器内前台网页与精确时间段（`HH:MM - HH:MM`）。

- **极简双主题视觉语言**：提供日间通透 **澄白（Alabaster）** 与夜间沉静 **曜石（Obsidian）** 两套界面主题，全仓零 Emoji、零重型第三方图表依赖，全部图表与图标均由原生手绘几何 SVG 实现。
- **纯净可读与完全可控**：内置 90+ 常用桌面软件百科与 60+ 国内外主流网站规则，屏蔽杂乱系统后台守护噪音，并赋予用户最高优先级的自定义进程/软件匹配与归并能力。
- **本地优先与按需同步**：默认 100% 纯离线运行、零账号零遥测；同时支持按需绑定用户自有的 **WebDAV 网盘** 或 **GitHub 私有仓库**，在多台 Windows 设备间无损合并历史会话与自定义规则。

---

## 核心功能特性

- **高精度前台会话追踪**：每秒轻量探测 Windows 前台活跃窗口（`GetForegroundWindow`），自动穿透 `ApplicationFrameHost` 与 `msedgewebview2` 宿主进程，精确还原真实桌面应用。
- **智能空闲与锁屏感知**：基于 `GetLastInputInfo` 检测键鼠空闲离开，播放影音时自动放宽 3 倍阈值（亦可针对任意软件自定义倍率或持续计时）；检测到 `Win+L` 锁屏或 UAC 安全桌面时立即封存当前时段。
- **跨午夜零点切分与断点恢复**：跨自然日会话按本地零点严格切分并守恒总秒数；运行期每 15 秒更新活跃断点快照，断电或强退均可无损恢复。
- **浏览器内网页识别与隐私净化**：兼容 Edge、Chrome、Ego Browser、Firefox、Brave、Arc、Vivaldi、Opera、Zen、夸克、360、QQ 浏览器等主流桌面浏览器；通过浅层 UI Automation 限深探测与窗口标题启发式双重解析当前站点（内存中即时剥离 URL 路径、Query 参数与鉴权令牌，仅保留纯净根域名或本地开发主机名），切站自动分段。
- **跨设备双向同步（`V0.3.0` 新增）**：内置零 C 依赖的原生 Windows WinHTTP 同步引擎，原生遵循系统代理，支持 **WebDAV**（坚果云、Nextcloud、OwnCloud、Alist 等）与 **GitHub 仓库（Contents REST API）** 双向无损合并同步。
- **自定义进程与软件规则（`V0.3.0` 新增）**：支持按进程名、路径关键词或窗口标题关键词创建最高优先级自定义规则，支持多进程别名归并、跨软件时长合并、自定义空闲倍率、强制开启/关闭网页识别、自定义标签与自定义图标上传。
- **单文件 Windows 安装包（`V0.3.0` 新增）**：提供开箱即用的单 `.exe` 安装向导，自动完成部署、桌面/开始菜单快捷方式创建与 Windows 标准卸载项注册。

---

## 软件与进程自定义规则

澄镜采用 **四级识别流水线**，用户自定义规则拥有最高决策权：

0. **第零级：用户自定义进程/软件规则（`custom_rules.json` & `overrides.json`）**
   - 在「软件识别库」点击 **「+ 自定义进程规则」**，可按无拓展名进程名（支持逗号分隔多个）、安装路径关键词或窗口标题关键词新建规则，甚至可覆盖系统默认过滤策略。
   - 点击任意已有软件的编辑按钮，均可自定义：**显示名称**、**软件类别**、**功能简介**、**自定义标签**、**自定义图标（PNG/SVG/JPG）**、**额外绑定进程别名**、**合并统计至其他软件**、**空闲离开判定策略（默认 / 1× / 2× / 3× / 持续计时）**、**浏览器网页识别开关** 与 **统计忽略**，并支持一键恢复默认设置。
1. **第一级：构建期内置软件百科（`catalog.rs`）**
   - 预置 90+ 款常用软件的 9 大类别规则，自动归并多进程子组件并屏蔽数十类系统内核/更新器/崩溃守护噪音。
2. **第二级：本机软件注册表扫描（`scanner.rs`）**
   - 自动扫描 Windows 卸载注册表分支并提取高清原生图标（`SHGetFileInfoW`），支持随时检索“本机装了什么软件”。
3. **第三级：未收录程序的无拓展名进程回退**
   - 自动剥离 `.exe` / `.cmd` / `.scr` 等执行后缀，以纯净无拓展名进程名（如 `Flow.Launcher`）兜底记录。

---

## 跨设备同步（WebDAV / GitHub 仓库）

在 **「监控设置 → 跨设备双向同步」** 中可随时开启或关闭（默认保持「仅本地离线」）：

- **WebDAV 同步**：内置坚果云、Nextcloud、Alist 快捷地址模板，支持基本认证与自动创建远端目录（`MKCOL`），仅同步 `sessions/YYYY-MM-DD.jsonl`、`custom_profile.json` 与 `sync_manifest.json`。
- **GitHub 仓库同步**：基于 GitHub Contents REST API 与 Fine-grained / Classic PAT（仅需目标仓库 `Contents: Read and Write` 权限），强烈建议使用 **Private 私有仓库** 存储个人时间轨迹。
- **无损多端合并算法**：按日分片读取远端与本地记录，按会话唯一 ID 与 `(app_id, start_ts, end_ts, web_site_id)` 元组去重取并集，多台电脑同日使用绝不互相覆盖；同时支持定时后台自动同步（15 分钟 / 30 分钟 / 1 小时 / 3 小时）。

---

## 可视化图表体系

应用内置纯手绘 SVG 多维交互图表，支持顶栏一键切换非破坏性 **「图表演示」** 模式预览全天样本：

- **今日概览（`OverviewView.tsx`）**：4 张核心指标卡、24 小时应用专注泳道与类别时光带、24 小时分类堆叠柱状图、软件类别环形图与高峰窗口、软件排行与热门网站速览。
- **时段趋势（`TimelineView.tsx`）**：日期前后步进、五时段作息节律分布（深夜/清晨/上午/下午/夜晚）、近 7 日 / 30 日堆叠趋势图与精确到秒的时段明细流。
- **网页洞察（`WebInsightsView.tsx`）**：网页总时长与屏幕占比、24 小时网页内容柱状图、8 类网页题材环形图、宿主浏览器载体分流、网站排行榜与切站访问明细。
- **软件识别库与单软件抽屉**：支持多维关键词检索、自定义进程规则管理与点击任意软件滑出当日 24 小时分布与时段列表。

---

## 项目结构

```text
.
├── assets/
│   └── logo.svg                     # SVG 几何品牌标识
├── scripts/
│   └── generate_icons.mjs           # 多尺寸桌面图标与 ICO 资源生成脚本
├── src/                             # React 19 + TypeScript + Tailwind v4 前端界面
│   ├── components/
│   │   ├── charts/                  # 纯手绘 SVG 可视化图表组件
│   │   ├── AppAvatar.tsx
│   │   ├── AppDetailDrawer.tsx
│   │   ├── CatalogView.tsx          # 软件百科、自定义进程规则与扩展定制弹窗
│   │   ├── Icons.tsx
│   │   ├── OverviewView.tsx
│   │   ├── SettingsView.tsx         # 采集设置、WebDAV/GitHub 同步与数据导出
│   │   ├── TimelineView.tsx
│   │   ├── Titlebar.tsx
│   │   └── WebInsightsView.tsx
│   ├── lib/
│   │   ├── format.ts
│   │   └── ipc.ts
│   ├── App.tsx
│   ├── styles.css
│   └── types.ts
├── src-tauri/                       # Rust + Tauri v2 Windows 原生后端
│   ├── installer/                   # 纯 Rust 单文件 Windows 安装程序（PrismTime Setup）
│   ├── src/
│   │   ├── analytics.rs             # 时段切片、泳道聚合与多维统计
│   │   ├── catalog.rs               # 四级识别、自定义进程规则与覆盖流水线
│   │   ├── commands.rs              # Tauri IPC 命令、规则管理、同步与导出
│   │   ├── demo.rs                  # 非破坏性图表样本数据生成器
│   │   ├── lib.rs                   # 后台采样循环、自动同步线程与系统托盘
│   │   ├── main.rs
│   │   ├── models.rs
│   │   ├── scanner.rs               # Windows 注册表已安装软件扫描
│   │   ├── storage.rs               # 按日分片 JSONL 落盘与原子文件读写
│   │   ├── sync.rs                  # WebDAV & GitHub 仓库双向无损合并同步引擎
│   │   ├── web_catalog.rs           # 主流网站百科、隐私域名净化与标题识别
│   │   ├── win_http.rs              # 零 C 依赖 Windows WinHTTP 原生客户端
│   │   └── win_probe.rs             # Win32 前台窗体、浅层 UIA 地址栏与图标提取
│   ├── tools/
│   │   └── dlltool_shim.rs          # 纯 Rust GNU COFF 导入库构建垫片
│   ├── vendor/
│   │   └── tauri-winres/            # 纯 Rust COFF .rsrc 资源编译实现
│   ├── Cargo.toml
│   └── tauri.conf.json
├── build-windows.ps1                # 一键构建主程序与单文件 Windows 安装包
├── package.json
└── LICENSE
```

---

## 下载与安装

### 单文件安装包（推荐）

1. 前往 [Releases (`V0.3.0`)](https://github.com/AnAcretiondisk9986/prism-time/releases/tag/V0.3.0) 下载单文件安装程序 **`PrismTime-V0.3.0-windows-x64-setup.exe`**。
2. 双击运行，按向导点击「确定」即可免管理员权限安装至 `%LOCALAPPDATA%\Programs\PrismTime`：
   - 自动创建桌面快捷方式与开始菜单入口；
   - 自动注册 Windows「设置 → 已安装的应用」标准卸载项；
   - 覆盖升级时自动保留您原有的 `%LOCALAPPDATA%\PrismTime` 历史记录与配置；
   - 支持静默安装参数：`PrismTime-V0.3.0-windows-x64-setup.exe --silent`（可选 `--dir "D:\Apps\PrismTime"`）。
3. 运行环境：Windows 10 / 11（64 位）及系统内置 Microsoft Edge WebView2 运行时。

---

## 从源码构建

### 构建环境要求

- Node.js 20+ 与 npm
- Python 3（用于生成 Zlib 压缩安装包载荷）
- Rust 工具链（`stable-x86_64-pc-windows-gnu`，由根目录 `rust-toolchain.toml` 自动激活）

### 构建步骤

```powershell
# 1. 安装前端依赖
npm install

# 2. 运行前端构建与全套 Rust 单元测试
npm run build
$env:CARGO_TARGET_DIR = "$env:USERPROFILE\.cargo\target-prism"
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:CARGO_TARGET_DIR\tools;$env:PATH"
cargo test --manifest-path .\src-tauri\Cargo.toml

# 3. 一键构建 Thin-LTO 精简主程序与单文件 Windows 安装包（产物输出至 release\ 目录）
.\build-windows.ps1
```

> 说明：项目内置 `src-tauri/tools/dlltool_shim.rs` 与 `src-tauri/vendor/tauri-winres`，可在纯 GNU Rust 工具链环境下零 MSVC 依赖一键产出主程序与单文件安装程序 `release\PrismTime-V0.3.0-windows-x64-setup.exe`。

---

## 本地数据目录与隐私承诺

所有统计、配置与自定义规则默认存放在 `%LOCALAPPDATA%\PrismTime` 目录下：

- `sessions/YYYY-MM-DD.jsonl`：按本地自然日分片保存的会话记录。
- `catalog.json`：已识别软件百科、注册表扫描结果及图标缓存。
- `overrides.json`：用户对已有软件自定义的显示名称、分类、额外进程别名、空闲倍率、网页识别与忽略设置。
- `custom_rules.json`：用户手动创建的最高优先级自定义进程/软件识别规则。
- `settings.json`：采集策略、主题偏好及可选的 WebDAV / GitHub 同步配置。
- `exports/`：主动导出的 UTF-8 BOM CSV / JSON 分析报表。

---

## 版本记录

### V0.3.0 (`0.3.0`)

- **新增 WebDAV 与 GitHub 仓库双向同步引擎**：基于纯 Rust Windows WinHTTP（零 C 依赖、原生遵循系统代理）实现跨设备无损合并同步，支持坚果云 / Nextcloud / Alist 等 WebDAV 服务与 GitHub 仓库 Contents API，支持连接测试、手动即时同步与后台定时自动同步。
- **新增用户自定义进程与软件规则体系**：支持新建最高优先级自定义进程规则（按进程名、路径关键词或窗口标题匹配），并为所有软件扩展支持额外进程别名归并、跨软件时长合并、自定义空闲判定倍率（含持续计时模式）、强制浏览器网页识别开关、自定义标签、自定义图标上传及一键恢复默认。
- **单文件 Windows 安装包与体积瘦身**：新增纯 Rust 单文件安装程序 `PrismTime-V0.3.0-windows-x64-setup.exe`（自动创建快捷方式与 Windows 标准卸载项），并在 Release 配置启用 `strip = true` 与 `lto = "thin"` 显著缩减可执行文件与安装包体积。

### V0.2dev (`0.2.0-dev`)

- 新增主流浏览器内网页识别与统计引擎（浅层 UIA 地址栏探测 + 隐私域名净化 + 60+ 国内外网站百科 + 切站自动分段）。
- 新增「网页洞察」工作区，并在概览、时段趋势、标题栏与 CSV 导出全面集成。

### V0.1beta (`0.1.0-beta`)

- 首个公开发布 Beta 版本，提供 Windows 前台窗口时长与精确时段采集、跨日零点切分、断点保护、本机软件注册表扫描、无拓展名进程回退与澄白/曜石双主题手绘 SVG 图表体系。

---

## 开源许可

本项目基于 [MIT License](./LICENSE) 开源。
