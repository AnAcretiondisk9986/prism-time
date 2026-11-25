<div align="center">
  <img src="./assets/logo.svg" width="96" height="96" alt="PrismTime Logo" />
  <h1>澄镜 · PrismTime</h1>
  <p>面向 Windows 平台的极简桌面屏幕使用时长、精确时段轨迹与本机软件洞察工具</p>
  <p>
    <a href="https://github.com/AnAcretiondisk9986/prism-time/releases/tag/V0.1beta">
      <img src="https://img.shields.io/badge/Version-V0.1beta-3D5AFE?style=flat-square" alt="Version V0.1beta" />
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
- [软件与进程识别策略](#软件与进程识别策略)
- [可视化图表体系](#可视化图表体系)
- [项目结构](#项目结构)
- [下载与运行](#下载与运行)
- [从源码构建](#从源码构建)
- [本地数据存储与隐私承诺](#本地数据目录与隐私承诺)
- [版本记录](#版本记录)
- [开源许可](#开源许可)

---

## 项目定位与设计理念

**澄镜（PrismTime）** 基于 **Rust + Tauri v2 + TypeScript/React** 构建，专注于在 Windows 桌面上以极低系统开销持续记录各软件及软件类别的实际活跃时长与精确时间段（`HH:MM - HH:MM`）。

- **极简双主题视觉语言**：提供日间高通透 **澄白（Alabaster）** 与夜间沉静 **曜石（Obsidian）** 两套界面主题，采用无边框自定义标题栏与手绘几何 SVG 图标体系。
- **纯净可读的信息密度**：不把杂乱的系统服务与后台守护进程直接铺陈在用户面前，而是通过内置识别规则、注册表扫描与安全回退机制呈现干净软件画像。
- **完全离线与零遥测**：不依赖外部云端服务，所有会话日志均按日分片保存在本地磁盘，用户完全掌控数据导出、名称覆盖与忽略规则。

---

## 核心功能特性

- **高精度前台会话追踪**：每秒采样 Windows 前台活跃窗口（`GetForegroundWindow`），自动穿透 `ApplicationFrameHost` 与 `msedgewebview2` 宿主进程，精确还原真实桌面应用。
- **智能空闲与锁屏感知**：基于 `GetLastInputInfo` 检测键鼠空闲离开，并在播放影音类软件时自动放宽 3 倍空闲阈值；检测到 `Win+L` 锁屏或 UAC 安全桌面时立即暂停并封存当前时段。
- **跨午夜零点切分**：连续跨日的会话在持久化时严格按照本地自然日边界切分并守恒总时长。
- **后台断点保护与托盘驻留**：运行期每 15 秒更新活跃检查点以防异常退出丢数据；支持关闭窗口最小化至系统托盘及开机自启配置。
- **数据检索与报告导出**：支持按关键字检索会话摘要与软件清单，并支持导出 UTF-8 BOM 的 CSV（含软件名称、无拓展名进程名、类别与窗口摘要）与完整分析 JSON 文件。

---

## 软件与进程识别策略

澄镜采用 **三级识别流水线** 处理采集到的桌面窗口与本机已安装软件，兼顾高准确度与未收录程序的兜底记录：

1. **第一级：构建期内置软件百科（`catalog.rs`）**
   - 预置开发与终端、AI 与智能助手、网页浏览、设计创意、办公学习、社交通讯、影音娱乐、游戏娱乐、效率工具等类别规则。
   - 自动将多组件附属渲染或子进程（如网页内核渲染进程、同套件子应用）汇总至统一软件实体。
   - 严格屏蔽系统核心服务、守护进程、更新器、崩溃处理器与命令行背景进程，避免污染有效屏幕统计。
2. **第二级：本机软件注册表扫描（`scanner.rs`）**
   - 扫描 Windows 卸载注册表分支并解析软件名称、开发商、安装路径与原生图标（`SHGetFileInfoW`），自动剔除运行库补丁、驱动程序与无意义系统组件，支持在界面中搜索“这是什么软件”。
3. **第三级：构建期未识别程序的无拓展名进程回退**
   - 若前台活跃程序未命中构建期内置百科规则，且不属于系统噪音进程，澄镜不会保留 `.exe` / `.EXE` / `.cmd` / `.scr` 等拓展名，而是直接记录其**去除拓展名后的纯净进程名**（例如 `D:\Tools\PixelBench_x64.exe` 将记录为 `PixelBench_x64`）。
   - 所有未内置识别的进程均可随时在「软件识别库」或「详情抽屉」中进行检索、重命名、更改类别或一键忽略，修改结果持久化至本地 `overrides.json`。

---

## 可视化图表体系

应用内置基于手工编写 SVG 构建的多维交互图表，支持在无真实数据阶段开启非破坏性 **「图表演示」** 模式预览全天样本：

- **24 小时使用时间段图谱**：支持按应用拆分连续专注泳道与按类别聚合的时间光带，直观展示全天专注起止区间。
- **24 小时分类堆叠柱状图**：按小时刻度展示各类别占比及单小时最活跃软件前三名。
- **软件类别环形图与高峰窗口**：统计主导软件类别、占比及连续两小时集中度最高的高峰时段。
- **五时段作息节律与多日趋势矩阵**：划分深夜（`00:00 - 06:00`）、清晨（`06:00 - 09:00`）、上午（`09:00 - 12:00`）、下午（`12:00 - 18:00`）、夜晚（`18:00 - 24:00`），并支持近 7 日与近 30 日趋势交互切换。
- **单软件时段洞察抽屉**：点击任意软件条目即可展开单软件当日 24 小时分布、单次最长/平均停留时长与按时间倒序的精确时段明细。

---

## 项目结构

```text
.
├── assets/
│   └── logo.svg                     # SVG 几何品牌标识
├── scripts/
│   └── generate_icons.mjs           # 多尺寸桌面图标与 ICO 资源生成脚本
├── src/                             # React + TypeScript + Tailwind v4 前端界面
│   ├── components/
│   │   ├── charts/                  # 纯手绘 SVG 可视化图表组件
│   │   ├── AppAvatar.tsx
│   │   ├── AppDetailDrawer.tsx
│   │   ├── CatalogView.tsx
│   │   ├── Icons.tsx
│   │   ├── OverviewView.tsx
│   │   ├── SettingsView.tsx
│   │   ├── TimelineView.tsx
│   │   └── Titlebar.tsx
│   ├── lib/
│   │   ├── format.ts
│   │   └── ipc.ts
│   ├── App.tsx
│   ├── styles.css
│   └── types.ts
├── src-tauri/                       # Rust + Tauri v2 Windows 原生后端
│   ├── capabilities/
│   ├── icons/
│   ├── src/
│   │   ├── analytics.rs             # 时段切片、泳道聚合与多维统计
│   │   ├── catalog.rs               # 百科识别、噪音屏蔽与无拓展名进程回退
│   │   ├── commands.rs              # Tauri IPC 命令与 CSV/JSON 导出
│   │   ├── demo.rs                  # 非破坏性图表样本数据生成器
│   │   ├── lib.rs                   # 后台采样循环、托盘与生命周期管理
│   │   ├── main.rs
│   │   ├── models.rs
│   │   ├── scanner.rs               # Windows 注册表已安装软件扫描
│   │   ├── storage.rs               # 按日分片 JSONL 落盘与原子状态读写
│   │   └── win_probe.rs             # Win32 前台窗体、PE 版本信息与图标提取
│   ├── tools/
│   │   └── dlltool_shim.rs          # 纯 Rust GNU COFF 导入库构建垫片
│   ├── vendor/
│   │   └── tauri-winres/            # 纯 Rust COFF .rsrc 资源编译实现
│   ├── Cargo.toml
│   └── tauri.conf.json
├── build-windows.ps1                # Windows 前端编排与独立发布构建脚本
├── package.json
└── LICENSE
```

---

## 下载与运行

### 方式一：直接运行发布包（推荐）

1. 前往 [Releases (`V0.1beta`)](https://github.com/AnAcretiondisk9986/prism-time/releases/tag/V0.1beta) 下载便携压缩包 `PrismTime-V0.1beta-windows-x64.zip`。
2. 解压至本地任意目录（请保持 `prism-time.exe` 与 `WebView2Loader.dll` 处于同一目录下）。
3. 双击 `prism-time.exe` 即可启动使用。
   - 运行依赖 Windows 10 / 11（64 位）及 Microsoft Edge WebView2 运行时（现代 Windows 11 系统已内置）。

---

## 从源码构建

### 构建环境要求

- Node.js 20+ 与 npm
- Rust 工具链（`stable-x86_64-pc-windows-gnu`，可通过根目录 `rust-toolchain.toml` 自动激活）

### 构建步骤

```powershell
# 1. 安装前端依赖
npm install

# 2. 执行单元测试（需将 Cargo 工具链与 target-prism 工具目录加入 PATH）
npm run build
cargo test --manifest-path .\src-tauri\Cargo.toml

# 3. 一键构建内嵌前端资源的独立发布版（产物输出至 release\ 目录）
.\build-windows.ps1
```

> 说明：项目内置 `src-tauri/tools/dlltool_shim.rs` 与 `src-tauri/vendor/tauri-winres`，可在纯 GNU Rust 工具链环境下直接完成 Windows 导入库链接与图标/清单资源嵌入，无需额外安装庞大桌面 C++ 构建套件；如需指定自定义构建缓存路径，可在执行 `.\build-windows.ps1` 前设置环境变量 `CARGO_TARGET_DIR`。

---

## 本地数据目录与隐私承诺

所有统计与识别数据默认存放在 `%LOCALAPPDATA%\PrismTime` 目录下：

- `sessions/YYYY-MM-DD.jsonl`：按本地自然日分片保存的会话记录。
- `catalog.json`：已识别软件百科、注册表扫描结果及未识别进程元数据与图标缓存。
- `overrides.json`：用户在「软件识别库」或抽屉中自定义的显示名称、分类与忽略设置。
- `settings.json`：采样间隔、锁屏暂停、托盘行为、自启及主题偏好配置。
- `exports/`：主动导出的 CSV / JSON 分析报表。

---

## 版本记录

### V0.1beta (`0.1.0-beta`)

- 首个公开发布 Beta 版本，提供 Windows 前台窗口时长与精确时段采集、跨日零点切分与断点保护。
- 实现内置软件百科聚合、系统后台噪音屏蔽、本机软件注册表扫描，以及构建期未识别程序的无拓展名纯净进程名记录能力。
- 完成澄白 / 曜石双主题界面及全套手绘 SVG 可视化图表（24 小时泳道、分时柱状图、类别环形图、五时段节律、7 日 / 30 日趋势与单软件详情抽屉）。

---

## 开源许可

本项目基于 [MIT License](./LICENSE) 开源。
