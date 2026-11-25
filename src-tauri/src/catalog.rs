use crate::models::{AppCategory, AppMetadata, AppOverride};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct KnownAppRule {
    pub app_id: &'static str,
    pub display_name: &'static str,
    pub category: AppCategory,
    pub publisher: &'static str,
    pub description: &'static str,
    pub exe_stems: &'static [&'static str],
    pub path_contains: &'static [&'static str],
    pub reg_keywords: &'static [&'static str],
}

pub const SELF_APP_ID: &str = "prismtime.self";

pub const KNOWN_APPS: &[KnownAppRule] = &[
    // --- 研发与终端 (Dev) ---
    KnownAppRule {
        app_id: "google.antigravity",
        display_name: "Antigravity",
        category: AppCategory::Dev,
        publisher: "Google",
        description: "Google 推出的智能体协作代码编辑器与研发工作台",
        exe_stems: &["antigravity"],
        path_contains: &["antigravity"],
        reg_keywords: &["antigravity"],
    },
    KnownAppRule {
        app_id: "microsoft.vscode",
        display_name: "Visual Studio Code",
        category: AppCategory::Dev,
        publisher: "Microsoft",
        description: "轻量级可扩展源代码编辑器与调试环境",
        exe_stems: &["code", "code - insiders"],
        path_contains: &["microsoft vs code"],
        reg_keywords: &["visual studio code"],
    },
    KnownAppRule {
        app_id: "microsoft.visualstudio",
        display_name: "Visual Studio",
        category: AppCategory::Dev,
        publisher: "Microsoft",
        description: "全功能集成开发环境与 C++/C#/.NET 编译调试套件",
        exe_stems: &["devenv", "blend"],
        path_contains: &["microsoft visual studio"],
        reg_keywords: &["visual studio community", "visual studio enterprise", "visual studio professional"],
    },
    KnownAppRule {
        app_id: "zai.zcode",
        display_name: "ZCode",
        category: AppCategory::Dev,
        publisher: "Z.ai",
        description: "智谱 AI 推出的原生智能体编程开发环境，支持长程目标编程与多端触发",
        exe_stems: &["zcode"],
        path_contains: &["zcode"],
        reg_keywords: &["zcode"],
    },
    KnownAppRule {
        app_id: "qoder.ide",
        display_name: "Qoder",
        category: AppCategory::Dev,
        publisher: "Qoder",
        description: "面向专业工程研发的 Agentic 智能编程 IDE，支持 Editor 与 Quest 协同",
        exe_stems: &["qoder", "qoder cn"],
        path_contains: &["qoder"],
        reg_keywords: &["qoder"],
    },
    KnownAppRule {
        app_id: "bytedance.trae",
        display_name: "Trae",
        category: AppCategory::Dev,
        publisher: "字节跳动",
        description: "AI 原生集成开发环境，提供代码补全与智能工程协同能力",
        exe_stems: &["trae", "trae cn", "trae solo cn"],
        path_contains: &["trae"],
        reg_keywords: &["trae"],
    },
    KnownAppRule {
        app_id: "opencode.ide",
        display_name: "OpenCode",
        category: AppCategory::Dev,
        publisher: "OpenCode",
        description: "开源多模型智能编码桌面工作台",
        exe_stems: &["opencode", "opencode-desktop"],
        path_contains: &["@opencode", "opencode"],
        reg_keywords: &["opencode"],
    },
    KnownAppRule {
        app_id: "t3tools.t3code",
        display_name: "T3 Code",
        category: AppCategory::Dev,
        publisher: "T3 Tools",
        description: "多智能体编程控制平面，统一调度工作树、代码差异审阅与代理任务",
        exe_stems: &["t3 code", "t3code"],
        path_contains: &["t3code"],
        reg_keywords: &["t3 code"],
    },
    KnownAppRule {
        app_id: "pideck.app",
        display_name: "PiDeck",
        category: AppCategory::Dev,
        publisher: "ayuayue",
        description: "集中管理本地编程智能体会话、Git 工作区与历史记录的桌面面板",
        exe_stems: &["pideck"],
        path_contains: &["pideck"],
        reg_keywords: &["pideck"],
    },
    KnownAppRule {
        app_id: "reasonix.studio",
        display_name: "Reasonix Studio",
        category: AppCategory::Dev,
        publisher: "Reasonix",
        description: "基于前缀缓存优化的 DeepSeek 原生智能编程桌面工作室",
        exe_stems: &["reasonix-desktop", "reasonix"],
        path_contains: &["reasonix"],
        reg_keywords: &["reasonix"],
    },
    KnownAppRule {
        app_id: "deepseek.harness",
        display_name: "DeepSeek Harness",
        category: AppCategory::Dev,
        publisher: "DeepSeek",
        description: "DeepSeek 开源插件化智能体运行时与本地工程自动化工具",
        exe_stems: &["deepseek harness", "dsh"],
        path_contains: &["deepseek harness"],
        reg_keywords: &["deepseek harness"],
    },
    KnownAppRule {
        app_id: "ccswitch.app",
        display_name: "CC Switch",
        category: AppCategory::Dev,
        publisher: "CC Switch",
        description: "AI 编程工具一站式配置切换与 MCP 服务管理中枢",
        exe_stems: &["cc switch", "cc-switch"],
        path_contains: &["cc switch"],
        reg_keywords: &["cc switch"],
    },
    KnownAppRule {
        app_id: "waveterm.app",
        display_name: "Wave Terminal",
        category: AppCategory::Dev,
        publisher: "Command Line Inc",
        description: "融合富文本预览、内联编辑器与 AI 辅助的现代化开源终端",
        exe_stems: &["wave", "waveterm"],
        path_contains: &["waveterm", "\\wave\\"],
        reg_keywords: &["wave"],
    },
    KnownAppRule {
        app_id: "wezfurlong.wezterm",
        display_name: "WezTerm",
        category: AppCategory::Dev,
        publisher: "Wez Furlong",
        description: "GPU 加速的跨平台终端模拟器与多路复用器",
        exe_stems: &["wezterm-gui", "wezterm"],
        path_contains: &["wezterm"],
        reg_keywords: &["wezterm"],
    },
    KnownAppRule {
        app_id: "microsoft.windowsterminal",
        display_name: "Windows Terminal",
        category: AppCategory::Dev,
        publisher: "Microsoft",
        description: "Windows 现代多标签终端主机，支持 PowerShell 与 WSL",
        exe_stems: &["windowsterminal"],
        path_contains: &["windowsterminal"],
        reg_keywords: &["windows terminal"],
    },
    KnownAppRule {
        app_id: "microsoft.powershell",
        display_name: "PowerShell",
        category: AppCategory::Dev,
        publisher: "Microsoft",
        description: "跨平台命令行 Shell 与自动化脚本环境",
        exe_stems: &["pwsh", "powershell"],
        path_contains: &["powershell"],
        reg_keywords: &["powershell 7"],
    },
    KnownAppRule {
        app_id: "google.androidstudio",
        display_name: "Android Studio",
        category: AppCategory::Dev,
        publisher: "Google",
        description: "Android 应用官方集成开发环境与模拟器工具链",
        exe_stems: &["studio64"],
        path_contains: &["android studio"],
        reg_keywords: &["android studio"],
    },
    KnownAppRule {
        app_id: "posit.rstudio",
        display_name: "RStudio",
        category: AppCategory::Dev,
        publisher: "Posit Software",
        description: "R 语言统计计算与数据科学集成开发环境",
        exe_stems: &["rstudio"],
        path_contains: &["rstudio"],
        reg_keywords: &["rstudio"],
    },
    KnownAppRule {
        app_id: "mongodb.compass",
        display_name: "MongoDB Compass",
        category: AppCategory::Dev,
        publisher: "MongoDB",
        description: "MongoDB 文档数据库可视化查询与模式分析客户端",
        exe_stems: &["mongodbcompass"],
        path_contains: &["mongodbcompass"],
        reg_keywords: &["mongodb compass"],
    },
    KnownAppRule {
        app_id: "jetbrains.idea",
        display_name: "IntelliJ IDEA",
        category: AppCategory::Dev,
        publisher: "JetBrains",
        description: "Java / Kotlin 与企业级全栈集成开发环境",
        exe_stems: &["idea64"],
        path_contains: &["intellij idea"],
        reg_keywords: &["intellij idea"],
    },
    KnownAppRule {
        app_id: "jetbrains.pycharm",
        display_name: "PyCharm",
        category: AppCategory::Dev,
        publisher: "JetBrains",
        description: "Python 专业开发与数据分析集成环境",
        exe_stems: &["pycharm64"],
        path_contains: &["pycharm"],
        reg_keywords: &["pycharm"],
    },
    KnownAppRule {
        app_id: "jetbrains.rustrover",
        display_name: "RustRover",
        category: AppCategory::Dev,
        publisher: "JetBrains",
        description: "Rust 语言专用智能集成开发环境",
        exe_stems: &["rustrover64"],
        path_contains: &["rustrover"],
        reg_keywords: &["rustrover"],
    },
    KnownAppRule {
        app_id: "cursor.editor",
        display_name: "Cursor",
        category: AppCategory::Dev,
        publisher: "Anysphere",
        description: "AI 原生代码编辑器与智能代码生成工作台",
        exe_stems: &["cursor"],
        path_contains: &["\\cursor\\"],
        reg_keywords: &["cursor"],
    },
    KnownAppRule {
        app_id: "git.gui",
        display_name: "Git Bash",
        category: AppCategory::Dev,
        publisher: "Git Community",
        description: "Windows 下的 Git 命令行与版本控制交互工具",
        exe_stems: &["git-bash", "git-gui", "mintty"],
        path_contains: &["\\git\\"],
        reg_keywords: &["git"],
    },
    KnownAppRule {
        app_id: "mitmproxy.app",
        display_name: "mitmproxy",
        category: AppCategory::Dev,
        publisher: "mitmproxy.org",
        description: "交互式 HTTPS 抓包与网络流量调试代理工具",
        exe_stems: &["mitmweb", "mitmproxy"],
        path_contains: &["mitmproxy"],
        reg_keywords: &["mitmproxy"],
    },

    // --- AI 与智能助手 (Ai) ---
    KnownAppRule {
        app_id: "openai.codex",
        display_name: "OpenAI Codex",
        category: AppCategory::Ai,
        publisher: "OpenAI",
        description: "OpenAI 智能编程代理与自动化桌面协同工作台",
        exe_stems: &["codex"],
        path_contains: &["openai.codex"],
        reg_keywords: &["codex"],
    },
    KnownAppRule {
        app_id: "openai.chatgpt",
        display_name: "ChatGPT",
        category: AppCategory::Ai,
        publisher: "OpenAI",
        description: "OpenAI 对话式 AI 桌面端应用",
        exe_stems: &["chatgpt"],
        path_contains: &["openai.chatgpt"],
        reg_keywords: &["chatgpt"],
    },
    KnownAppRule {
        app_id: "google.gemini",
        display_name: "Google Gemini",
        category: AppCategory::Ai,
        publisher: "Google",
        description: "Google Gemini 多模态人工智能桌面启动客户端",
        exe_stems: &["geminiapplauncher", "gemini"],
        path_contains: &["\\google\\gemini"],
        reg_keywords: &["gemini"],
    },
    KnownAppRule {
        app_id: "tencent.workbuddy",
        display_name: "WorkBuddy",
        category: AppCategory::Ai,
        publisher: "腾讯科技",
        description: "腾讯全场景 AI 桌面智能体工作台，支持多智能体协作与本地文件处理",
        exe_stems: &["workbuddy", "workbuddy ai"],
        path_contains: &["workbuddy"],
        reg_keywords: &["workbuddy"],
    },
    KnownAppRule {
        app_id: "bytedance.doubao",
        display_name: "豆包",
        category: AppCategory::Ai,
        publisher: "字节跳动",
        description: "字节跳动旗下多功能 AI 对话、写作与桌面效率助手",
        exe_stems: &["doubao"],
        path_contains: &["doubao"],
        reg_keywords: &["豆包"],
    },
    KnownAppRule {
        app_id: "moonshot.kimi",
        display_name: "Kimi",
        category: AppCategory::Ai,
        publisher: "月之暗面",
        description: "支持超长上下文解析与深度研究的智能对话桌面端",
        exe_stems: &["kimi", "kimi-desktop"],
        path_contains: &["kimi-desktop", "\\kimi\\"],
        reg_keywords: &["kimi"],
    },
    KnownAppRule {
        app_id: "astrbot.desktop",
        display_name: "AstrBot",
        category: AppCategory::Ai,
        publisher: "AstrBot",
        description: "开源多平台 Agentic AI 助手本地部署与插件控制台",
        exe_stems: &["astrbot-desktop", "astrbot"],
        path_contains: &["astrbot"],
        reg_keywords: &["astrbot"],
    },
    KnownAppRule {
        app_id: "ollama.app",
        display_name: "Ollama",
        category: AppCategory::Ai,
        publisher: "Ollama",
        description: "本地大语言模型运行时管理与推理服务客户端",
        exe_stems: &["ollama app", "ollama"],
        path_contains: &["ollama"],
        reg_keywords: &["ollama"],
    },
    KnownAppRule {
        app_id: "comfyui.app",
        display_name: "ComfyUI",
        category: AppCategory::Ai,
        publisher: "Comfy Org",
        description: "基于节点流的高级生成式 AI 图像与视频工作流引擎",
        exe_stems: &["comfyui"],
        path_contains: &["comfyui"],
        reg_keywords: &["comfyui"],
    },

    // --- 网页浏览 (Browser) ---
    KnownAppRule {
        app_id: "citrolabs.ego",
        display_name: "Ego Browser",
        category: AppCategory::Browser,
        publisher: "Citro Labs",
        description: "面向人类与 AI 代理并行浏览的隔离工作空间浏览器",
        exe_stems: &["ego"],
        path_contains: &["ego lite", "citro labs"],
        reg_keywords: &["ego (lite)", "ego"],
    },
    KnownAppRule {
        app_id: "microsoft.edge",
        display_name: "Microsoft Edge",
        category: AppCategory::Browser,
        publisher: "Microsoft",
        description: "微软 Chromium 内核网页浏览器，集成侧边栏与垂直标签管理",
        exe_stems: &["msedge"],
        path_contains: &["microsoft\\edge"],
        reg_keywords: &["microsoft edge"],
    },
    KnownAppRule {
        app_id: "google.chrome",
        display_name: "Google Chrome",
        category: AppCategory::Browser,
        publisher: "Google",
        description: "高速现代网页浏览器，支持丰富扩展生态",
        exe_stems: &["chrome"],
        path_contains: &["google\\chrome"],
        reg_keywords: &["google chrome"],
    },
    KnownAppRule {
        app_id: "mozilla.firefox",
        display_name: "Mozilla Firefox",
        category: AppCategory::Browser,
        publisher: "Mozilla",
        description: "注重隐私保护与开源标准的独立内核网页浏览器",
        exe_stems: &["firefox"],
        path_contains: &["mozilla firefox"],
        reg_keywords: &["mozilla firefox"],
    },
    KnownAppRule {
        app_id: "quark.browser",
        display_name: "夸克",
        category: AppCategory::Browser,
        publisher: "动悦信息",
        description: "集网页搜索、网盘与文档工具于一体的智能浏览器",
        exe_stems: &["quark"],
        path_contains: &["\\quark\\"],
        reg_keywords: &["夸克"],
    },
    KnownAppRule {
        app_id: "brave.browser",
        display_name: "Brave",
        category: AppCategory::Browser,
        publisher: "Brave Software",
        description: "主打隐私拦截与 Chromium 高性能浏览体验的浏览器",
        exe_stems: &["brave"],
        path_contains: &["bravesoftware\\brave-browser"],
        reg_keywords: &["brave"],
    },
    KnownAppRule {
        app_id: "thebrowsercompany.arc",
        display_name: "Arc",
        category: AppCategory::Browser,
        publisher: "The Browser Company",
        description: "聚焦空间分组与垂直标签体验的现代浏览器",
        exe_stems: &["arc"],
        path_contains: &["thebrowsercompany.arc"],
        reg_keywords: &["arc"],
    },
    KnownAppRule {
        app_id: "vivaldi.browser",
        display_name: "Vivaldi",
        category: AppCategory::Browser,
        publisher: "Vivaldi Technologies",
        description: "高度可定制的效率型多面板网页浏览器",
        exe_stems: &["vivaldi"],
        path_contains: &["vivaldi"],
        reg_keywords: &["vivaldi"],
    },
    KnownAppRule {
        app_id: "opera.browser",
        display_name: "Opera",
        category: AppCategory::Browser,
        publisher: "Opera Software",
        description: "集成侧边工具栏与内置生产力模块的网页浏览器",
        exe_stems: &["opera"],
        path_contains: &["\\opera"],
        reg_keywords: &["opera"],
    },
    KnownAppRule {
        app_id: "zen.browser",
        display_name: "Zen Browser",
        category: AppCategory::Browser,
        publisher: "Zen Team",
        description: "基于 Firefox 内核的极简无干扰专注浏览器",
        exe_stems: &["zen"],
        path_contains: &["zen browser"],
        reg_keywords: &["zen browser"],
    },
    KnownAppRule {
        app_id: "qihoo.360chrome",
        display_name: "360 浏览器",
        category: AppCategory::Browser,
        publisher: "奇虎 360",
        description: "支持双核切换与扩展生态的中文网页浏览器",
        exe_stems: &["360chrome", "360chromex", "360se"],
        path_contains: &["360chrome", "360se"],
        reg_keywords: &["360极速浏览器", "360安全浏览器"],
    },
    KnownAppRule {
        app_id: "tencent.qqbrowser",
        display_name: "QQ 浏览器",
        category: AppCategory::Browser,
        publisher: "腾讯科技",
        description: "集成高速上网、文档阅读与标签管理的浏览器",
        exe_stems: &["qqbrowser"],
        path_contains: &["qqbrowser"],
        reg_keywords: &["qq浏览器"],
    },

    // --- 设计创意 (Design) ---
    KnownAppRule {
        app_id: "adobe.photoshop",
        display_name: "Adobe Photoshop",
        category: AppCategory::Design,
        publisher: "Adobe",
        description: "专业位图图像编辑、合成与视觉设计软件",
        exe_stems: &["photoshop"],
        path_contains: &["adobe photoshop"],
        reg_keywords: &["adobe photoshop"],
    },
    KnownAppRule {
        app_id: "adobe.illustrator",
        display_name: "Adobe Illustrator",
        category: AppCategory::Design,
        publisher: "Adobe",
        description: "行业标准矢量图形插画与品牌平面设计工具",
        exe_stems: &["illustrator"],
        path_contains: &["adobe illustrator"],
        reg_keywords: &["adobe illustrator"],
    },
    KnownAppRule {
        app_id: "adobe.premiere",
        display_name: "Adobe Premiere Pro",
        category: AppCategory::Design,
        publisher: "Adobe",
        description: "专业非线性视频剪辑、调色与多轨音频后期软件",
        exe_stems: &["adobe premiere pro"],
        path_contains: &["adobe premiere"],
        reg_keywords: &["adobe premiere pro"],
    },
    KnownAppRule {
        app_id: "adobe.aftereffects",
        display_name: "Adobe After Effects",
        category: AppCategory::Design,
        publisher: "Adobe",
        description: "动态图形动画、影视视觉特效与合成制作软件",
        exe_stems: &["afterfx"],
        path_contains: &["adobe after effects"],
        reg_keywords: &["adobe after effects"],
    },
    KnownAppRule {
        app_id: "adobe.audition",
        display_name: "Adobe Audition",
        category: AppCategory::Design,
        publisher: "Adobe",
        description: "多轨数字音频工作站与降噪母带处理工具",
        exe_stems: &["adobe audition"],
        path_contains: &["adobe audition"],
        reg_keywords: &["adobe audition"],
    },
    KnownAppRule {
        app_id: "adobe.lightroom",
        display_name: "Adobe Lightroom Classic",
        category: AppCategory::Design,
        publisher: "Adobe",
        description: "RAW 摄影后期调色与大规模照片图库管理系统",
        exe_stems: &["lightroom"],
        path_contains: &["adobe lightroom"],
        reg_keywords: &["adobe lightroom"],
    },
    KnownAppRule {
        app_id: "adobe.bridge",
        display_name: "Adobe Bridge",
        category: AppCategory::Design,
        publisher: "Adobe",
        description: "Adobe 创意资源可视化管理与批量元数据浏览工具",
        exe_stems: &["bridge"],
        path_contains: &["adobe bridge"],
        reg_keywords: &["adobe bridge"],
    },
    KnownAppRule {
        app_id: "maxon.cinema4d",
        display_name: "Cinema 4D",
        category: AppCategory::Design,
        publisher: "Maxon",
        description: "专业三维建模、材质渲染与动态图形制作平台",
        exe_stems: &["cinema 4d"],
        path_contains: &["maxon cinema 4d"],
        reg_keywords: &["cinema 4d"],
    },
    KnownAppRule {
        app_id: "bytedance.jianying",
        display_name: "剪映专业版",
        category: AppCategory::Design,
        publisher: "脸萌科技",
        description: "全能桌面端视频剪辑与智能字幕/特效创作工具",
        exe_stems: &["jianyingpro"],
        path_contains: &["jianyingpro"],
        reg_keywords: &["剪映专业版"],
    },
    KnownAppRule {
        app_id: "obsproject.obs",
        display_name: "OBS Studio",
        category: AppCategory::Design,
        publisher: "OBS Project",
        description: "开源高性能录屏与多源直播推流制作软件",
        exe_stems: &["obs64", "obs32"],
        path_contains: &["obs-studio"],
        reg_keywords: &["obs studio"],
    },
    KnownAppRule {
        app_id: "bilibili.livehime",
        display_name: "哔哩哔哩直播姬",
        category: AppCategory::Design,
        publisher: "哔哩哔哩",
        description: "B站官方桌面推流、虚拟形象与互动直播工具",
        exe_stems: &["livehime"],
        path_contains: &["livehime"],
        reg_keywords: &["哔哩哔哩直播姬"],
    },
    KnownAppRule {
        app_id: "bytedance.webcastmate",
        display_name: "直播伴侣",
        category: AppCategory::Design,
        publisher: "字节跳动",
        description: "抖音官方桌面游戏与应用推流直播伴侣",
        exe_stems: &["webcast_mate", "直播伴侣", "直播伴侣 launcher"],
        path_contains: &["webcast_mate"],
        reg_keywords: &["直播伴侣"],
    },
    KnownAppRule {
        app_id: "upscayl.app",
        display_name: "Upscayl",
        category: AppCategory::Design,
        publisher: "Nayam Amarshe",
        description: "基于神经网络的超分辨率图像无损放大桌面应用",
        exe_stems: &["upscayl"],
        path_contains: &["upscayl"],
        reg_keywords: &["upscayl"],
    },
    KnownAppRule {
        app_id: "processon.app",
        display_name: "ProcessOn",
        category: AppCategory::Design,
        publisher: "大麦地信息",
        description: "流程图、思维导图与架构图在线协作桌面端",
        exe_stems: &["processon"],
        path_contains: &["processon"],
        reg_keywords: &["processon"],
    },
    KnownAppRule {
        app_id: "artcraft.designcraft",
        display_name: "DesignCraft",
        category: AppCategory::Design,
        publisher: "Learning Machines / ArtCraft",
        description: "Rust 原生开源桌面排版与创意出版工具",
        exe_stems: &["designcraft", "artcraft", "photocraft", "vectorcraft", "filmcraft", "cadcraft", "effectcraft", "lightcraft", "soundcraft"],
        path_contains: &["designcraft", "artcraft"],
        reg_keywords: &["designcraft", "artcraft launcher", "photocraft", "vectorcraft", "filmcraft", "cadcraft", "effectcraft", "lightcraft", "soundcraft"],
    },

    // --- 办公与学习 (Office) ---
    KnownAppRule {
        app_id: "kingsoft.wps",
        display_name: "WPS Office",
        category: AppCategory::Office,
        publisher: "金山办公",
        description: "集文字文档、电子表格、演示文稿与 PDF 于一体的办公套件",
        exe_stems: &["wps", "et", "wpp", "wpspdf"],
        path_contains: &["wps office", "kingsoft"],
        reg_keywords: &["wps office"],
    },
    KnownAppRule {
        app_id: "microsoft.word",
        display_name: "Microsoft Word",
        category: AppCategory::Office,
        publisher: "Microsoft",
        description: "专业文档排版、审阅与学术论文撰写软件",
        exe_stems: &["winword"],
        path_contains: &["microsoft office"],
        reg_keywords: &["microsoft word"],
    },
    KnownAppRule {
        app_id: "microsoft.excel",
        display_name: "Microsoft Excel",
        category: AppCategory::Office,
        publisher: "Microsoft",
        description: "电子表格数据分析、函数建模与可视化报表工具",
        exe_stems: &["excel"],
        path_contains: &["microsoft office"],
        reg_keywords: &["microsoft excel"],
    },
    KnownAppRule {
        app_id: "microsoft.powerpoint",
        display_name: "Microsoft PowerPoint",
        category: AppCategory::Office,
        publisher: "Microsoft",
        description: "演示文稿设计与汇报幻灯片制作软件",
        exe_stems: &["powerpnt"],
        path_contains: &["microsoft office"],
        reg_keywords: &["microsoft powerpoint"],
    },
    KnownAppRule {
        app_id: "microsoft.onenote",
        display_name: "Microsoft OneNote",
        category: AppCategory::Office,
        publisher: "Microsoft",
        description: "自由画布数字笔记本与课堂/项目知识整理工具",
        exe_stems: &["onenote", "onenotem"],
        path_contains: &["onenote"],
        reg_keywords: &["onenote"],
    },
    KnownAppRule {
        app_id: "obsidian.notes",
        display_name: "Obsidian",
        category: AppCategory::Office,
        publisher: "Obsidian",
        description: "本地 Markdown 双链知识库与个人思维网络笔记软件",
        exe_stems: &["obsidian"],
        path_contains: &["obsidian"],
        reg_keywords: &["obsidian"],
    },
    KnownAppRule {
        app_id: "marktext.editor",
        display_name: "MarkText",
        category: AppCategory::Office,
        publisher: "Jocs",
        description: "简洁所见即所得的开源实时预览 Markdown 编辑器",
        exe_stems: &["marktext"],
        path_contains: &["marktext"],
        reg_keywords: &["marktext"],
    },
    KnownAppRule {
        app_id: "pdf24.creator",
        display_name: "PDF24 Creator",
        category: AppCategory::Office,
        publisher: "Geek Software",
        description: "轻量高效的 PDF 文档合并、拆分、压缩与转换工具箱",
        exe_stems: &["pdf24-creator", "pdf24-toolbox", "pdf24-reader"],
        path_contains: &["pdf24"],
        reg_keywords: &["pdf24 creator"],
    },
    KnownAppRule {
        app_id: "chaoxing.study",
        display_name: "学习通",
        category: AppCategory::Office,
        publisher: "超星",
        description: "高校在线课程学习、电子教材与学术资源桌面端",
        exe_stems: &["cxstudy"],
        path_contains: &["cxstudy"],
        reg_keywords: &["学习通"],
    },
    KnownAppRule {
        app_id: "enncy.ocs",
        display_name: "OCS Desktop",
        category: AppCategory::Office,
        publisher: "enncy",
        description: "多账号隔离浏览器环境与在线学习脚本桌面管理器",
        exe_stems: &["ocs desktop", "ocs-desktop"],
        path_contains: &["ocs desktop"],
        reg_keywords: &["ocs desktop"],
    },
    KnownAppRule {
        app_id: "acledger.app",
        display_name: "Ac记账",
        category: AppCategory::Office,
        publisher: "AcLedger",
        description: "极简本地化个人收支账目与财务记录桌面应用",
        exe_stems: &["ac-ledger", "ac记账"],
        path_contains: &["ac-ledger"],
        reg_keywords: &["ac记账"],
    },
    KnownAppRule {
        app_id: "nutstore.client",
        display_name: "坚果云",
        category: AppCategory::Office,
        publisher: "亦净网络",
        description: "稳定安全的个人与团队云端文件同步备份客户端",
        exe_stems: &["nutstoreclient"],
        path_contains: &["nutstore"],
        reg_keywords: &["坚果云"],
    },
    KnownAppRule {
        app_id: "baidu.netdisk",
        display_name: "百度网盘",
        category: AppCategory::Office,
        publisher: "度友科技",
        description: "云端文件存储、分享与大文件传输桌面客户端",
        exe_stems: &["baidunetdisk", "baidunetdiskhost"],
        path_contains: &["baidunetdisk"],
        reg_keywords: &["百度网盘"],
    },
    KnownAppRule {
        app_id: "artcraft.office",
        display_name: "WordCraft",
        category: AppCategory::Office,
        publisher: "Learning Machines / ArtCraft",
        description: "Rust 原生开源文档、表格与演示办公软件家族",
        exe_stems: &["wordcraft", "gridcraft", "deckcraft", "pdfcraft"],
        path_contains: &["wordcraft", "gridcraft", "deckcraft", "pdfcraft"],
        reg_keywords: &["wordcraft", "gridcraft", "deckcraft", "pdfcraft"],
    },

    // --- 社交通讯 (Social) ---
    KnownAppRule {
        app_id: "tencent.weixin",
        display_name: "微信",
        category: AppCategory::Social,
        publisher: "腾讯科技",
        description: "桌面端即时通讯、文件传输与小程序生态平台",
        exe_stems: &["weixin", "wechat", "wechatappex", "wechatplayer"],
        path_contains: &["weixin", "wechat", "xwechat"],
        reg_keywords: &["微信"],
    },
    KnownAppRule {
        app_id: "tencent.qq",
        display_name: "QQ",
        category: AppCategory::Social,
        publisher: "腾讯科技",
        description: "全新 NT 架构桌面端即时通讯与社群交流软件",
        exe_stems: &["qq", "qqnt"],
        path_contains: &["qqnt", "\\tencent\\qq"],
        reg_keywords: &["qq"],
    },
    KnownAppRule {
        app_id: "alibaba.dingtalk",
        display_name: "钉钉",
        category: AppCategory::Social,
        publisher: "阿里巴巴",
        description: "企业组织协同办公、即时沟通与审批平台",
        exe_stems: &["dingtalk"],
        path_contains: &["dingding"],
        reg_keywords: &["钉钉"],
    },
    KnownAppRule {
        app_id: "tencent.wemeet",
        display_name: "腾讯会议",
        category: AppCategory::Social,
        publisher: "腾讯科技",
        description: "高清音视频远程会议与屏幕共享协作平台",
        exe_stems: &["wemeetapp"],
        path_contains: &["wemeet"],
        reg_keywords: &["腾讯会议"],
    },
    KnownAppRule {
        app_id: "telegram.desktop",
        display_name: "Telegram",
        category: AppCategory::Social,
        publisher: "Telegram FZ-LLC",
        description: "跨平台高速云同步即时通讯与频道订阅应用",
        exe_stems: &["telegram"],
        path_contains: &["telegram desktop"],
        reg_keywords: &["telegram desktop"],
    },
    KnownAppRule {
        app_id: "discord.app",
        display_name: "Discord",
        category: AppCategory::Social,
        publisher: "Discord",
        description: "面向开发者与游戏社群的语音频道与社区交流平台",
        exe_stems: &["discord"],
        path_contains: &["\\discord\\"],
        reg_keywords: &["discord"],
    },

    // --- 影音娱乐 (Media) ---
    KnownAppRule {
        app_id: "bytedance.douyin",
        display_name: "抖音",
        category: AppCategory::Media,
        publisher: "字节跳动",
        description: "桌面端短视频浏览、直播观看与创作者中心",
        exe_stems: &["douyin"],
        path_contains: &["\\douyin\\"],
        reg_keywords: &["抖音"],
    },
    KnownAppRule {
        app_id: "bilibili.desktop",
        display_name: "哔哩哔哩",
        category: AppCategory::Media,
        publisher: "哔哩哔哩",
        description: "B站桌面客户端，支持高清视频、番剧与弹幕互动",
        exe_stems: &["bilibili"],
        path_contains: &["\\bilibili\\"],
        reg_keywords: &["哔哩哔哩"],
    },
    KnownAppRule {
        app_id: "netease.cloudmusic",
        display_name: "网易云音乐",
        category: AppCategory::Media,
        publisher: "网易公司",
        description: "流媒体音乐播放、歌单管理与评论社区桌面端",
        exe_stems: &["cloudmusic"],
        path_contains: &["wangyiyunyinyue", "cloudmusic"],
        reg_keywords: &["网易云音乐"],
    },
    KnownAppRule {
        app_id: "folia.music",
        display_name: "Folia",
        category: AppCategory::Media,
        publisher: "chthollyphile",
        description: "主打沉浸式全屏动态歌词动效的跨平台音乐播放器",
        exe_stems: &["folia"],
        path_contains: &["\\folia\\"],
        reg_keywords: &["folia"],
    },
    KnownAppRule {
        app_id: "venera.comic",
        display_name: "Venera",
        category: AppCategory::Media,
        publisher: "nyne",
        description: "支持本地图库与可配置多源阅读的跨平台漫画阅读器",
        exe_stems: &["venera"],
        path_contains: &["\\venera\\"],
        reg_keywords: &["venera"],
    },
    KnownAppRule {
        app_id: "kakao.potplayer",
        display_name: "PotPlayer",
        category: AppCategory::Media,
        publisher: "Kakao",
        description: "高可定制硬解本地高清多媒体视频播放器",
        exe_stems: &["potplayermini64", "potplayermini"],
        path_contains: &["potplayer"],
        reg_keywords: &["potplayer"],
    },
    KnownAppRule {
        app_id: "bandisoft.honeyview",
        display_name: "Honeyview",
        category: AppCategory::Media,
        publisher: "Bandisoft",
        description: "极速响应的桌面看图与压缩包内图片直读工具",
        exe_stems: &["honeyview"],
        path_contains: &["honeyview"],
        reg_keywords: &["honeyview"],
    },

    // --- 游戏娱乐 (Game) ---
    KnownAppRule {
        app_id: "valve.steam",
        display_name: "Steam",
        category: AppCategory::Game,
        publisher: "Valve",
        description: "PC 数字游戏发行平台、创意工坊与好友联机中枢",
        exe_stems: &["steam", "steamwebhelper"],
        path_contains: &["\\steam\\"],
        reg_keywords: &["steam"],
    },
    KnownAppRule {
        app_id: "tencent.wegame",
        display_name: "WeGame",
        category: AppCategory::Game,
        publisher: "腾讯游戏",
        description: "腾讯游戏聚合发行、社区攻略与辅助启动平台",
        exe_stems: &["wegame"],
        path_contains: &["\\wegame\\"],
        reg_keywords: &["wegame"],
    },
    KnownAppRule {
        app_id: "microsoft.xbox",
        display_name: "Xbox",
        category: AppCategory::Game,
        publisher: "Microsoft",
        description: "Windows PC Game Pass 游戏库与 Xbox 社交中心",
        exe_stems: &["xboxpcapp", "xboxapp"],
        path_contains: &["microsoft.gamingapp"],
        reg_keywords: &["xbox"],
    },
    KnownAppRule {
        app_id: "hypergryph.launcher",
        display_name: "鹰角启动器",
        category: AppCategory::Game,
        publisher: "鹰角网络",
        description: "鹰角网络 PC 游戏统一更新与启动器",
        exe_stems: &[],
        path_contains: &["hypergryph launcher"],
        reg_keywords: &["鹰角启动器"],
    },
    KnownAppRule {
        app_id: "gryphlink.launcher",
        display_name: "GRYPHLINK",
        category: AppCategory::Game,
        publisher: "鹰角网络",
        description: "鹰角国际版游戏启动与账户管理客户端",
        exe_stems: &[],
        path_contains: &["gryphlink"],
        reg_keywords: &["gryphlink"],
    },
    KnownAppRule {
        app_id: "horizon.launcher",
        display_name: "Horizon Launcher",
        category: AppCategory::Game,
        publisher: "Horizon",
        description: "基于 Rust 构建的现代化开源游戏启动器",
        exe_stems: &["horizon launcher", "horizon-launcher"],
        path_contains: &["horizon launcher"],
        reg_keywords: &["horizon launcher"],
    },
    KnownAppRule {
        app_id: "rockstar.launcher",
        display_name: "Rockstar Games Launcher",
        category: AppCategory::Game,
        publisher: "Rockstar Games",
        description: "Rockstar 旗下 PC 游戏启动与云存档管理平台",
        exe_stems: &[],
        path_contains: &["rockstar games\\launcher"],
        reg_keywords: &["rockstar games launcher"],
    },
    KnownAppRule {
        app_id: "ldplayer.emulator",
        display_name: "雷电模拟器",
        category: AppCategory::Game,
        publisher: "畅指网络",
        description: "高性能 Windows 安卓手游桌面模拟器",
        exe_stems: &["dnplayer"],
        path_contains: &["ldplayer"],
        reg_keywords: &["雷电模拟器"],
    },
    KnownAppRule {
        app_id: "game.gtav",
        display_name: "Grand Theft Auto V",
        category: AppCategory::Game,
        publisher: "Rockstar North",
        description: "开放世界动作冒险游戏（增强版）",
        exe_stems: &["gta5", "gta5_enhanced", "playgtav"],
        path_contains: &["grand theft auto v"],
        reg_keywords: &["grand theft auto v"],
    },
    KnownAppRule {
        app_id: "game.pubg",
        display_name: "PUBG: BATTLEGROUNDS",
        category: AppCategory::Game,
        publisher: "PUBG Corporation",
        description: "多人战术竞技射击游戏",
        exe_stems: &["tslgame"],
        path_contains: &["steamapps\\common\\pubg"],
        reg_keywords: &["pubg: battlegrounds"],
    },
    KnownAppRule {
        app_id: "game.cs2",
        display_name: "Counter-Strike 2",
        category: AppCategory::Game,
        publisher: "Valve",
        description: "第一人称战术竞技射击游戏",
        exe_stems: &["cs2", "csgo"],
        path_contains: &["counter-strike global offensive"],
        reg_keywords: &["counter-strike"],
    },

    // --- 效率与系统 (Utility) ---
    KnownAppRule {
        app_id: "flowlauncher.app",
        display_name: "Flow Launcher",
        category: AppCategory::Utility,
        publisher: "Flow-Launcher Team",
        description: "键盘驱动的 Windows 极速应用启动与全局搜索器",
        exe_stems: &["flow.launcher"],
        path_contains: &["flowlauncher"],
        reg_keywords: &["flow launcher"],
    },
    KnownAppRule {
        app_id: "voidtools.everything",
        display_name: "Everything",
        category: AppCategory::Utility,
        publisher: "voidtools",
        description: "毫秒级全盘 NTFS 文件名实时索引与检索工具",
        exe_stems: &["everything"],
        path_contains: &["\\everything\\"],
        reg_keywords: &["everything"],
    },
    KnownAppRule {
        app_id: "ecopaste.app",
        display_name: "EcoPaste",
        category: AppCategory::Utility,
        publisher: "ayangweb",
        description: "基于 Tauri 构建的本地优先跨平台剪贴板历史管理工具",
        exe_stems: &["ecopaste"],
        path_contains: &["ecopaste"],
        reg_keywords: &["ecopaste"],
    },
    KnownAppRule {
        app_id: "pixpin.app",
        display_name: "PixPin",
        category: AppCategory::Utility,
        publisher: "深度途景",
        description: "支持长截图、贴图、OCR 与标注的多功能截图效率工具",
        exe_stems: &["pixpin"],
        path_contains: &["pixpin"],
        reg_keywords: &["pixpin"],
    },
    KnownAppRule {
        app_id: "snipaste.app",
        display_name: "Snipaste",
        category: AppCategory::Utility,
        publisher: "Snipaste",
        description: "轻量级截图与桌面置顶贴图工具",
        exe_stems: &["snipaste"],
        path_contains: &["snipaste"],
        reg_keywords: &["snipaste"],
    },
    KnownAppRule {
        app_id: "cutscreen.app",
        display_name: "CutScreen",
        category: AppCategory::Utility,
        publisher: "Cut-Screen",
        description: "简洁轻快的桌面区域截图与即时拷贝工具",
        exe_stems: &["cutscreen"],
        path_contains: &["cutscreen"],
        reg_keywords: &["cutscreen"],
    },
    KnownAppRule {
        app_id: "localsend.app",
        display_name: "LocalSend",
        category: AppCategory::Utility,
        publisher: "Tien Do Nam",
        description: "跨平台局域网零配置加密文件高速互传工具",
        exe_stems: &["localsend_app"],
        path_contains: &["localsend"],
        reg_keywords: &["localsend"],
    },
    KnownAppRule {
        app_id: "ghostdownloader.app",
        display_name: "Ghost Downloader",
        category: AppCategory::Utility,
        publisher: "XiaoYouChR",
        description: "基于 AI 分片调度的多线程高速文件下载器",
        exe_stems: &["ghost-downloader-3", "ghost downloader"],
        path_contains: &["ghost downloader"],
        reg_keywords: &["ghost downloader"],
    },
    KnownAppRule {
        app_id: "resdownloader.app",
        display_name: "res-downloader",
        category: AppCategory::Utility,
        publisher: "res-downloader",
        description: "桌面端多媒体与网页网络资源嗅探下载工具",
        exe_stems: &["res-downloader"],
        path_contains: &["res-downloader"],
        reg_keywords: &["res-downloader"],
    },
    KnownAppRule {
        app_id: "bandisoft.bandizip",
        display_name: "Bandizip",
        category: AppCategory::Utility,
        publisher: "Bandisoft",
        description: "高速多核文件解压缩与归档管理软件",
        exe_stems: &["bandizip"],
        path_contains: &["bandizip"],
        reg_keywords: &["bandizip"],
    },
    KnownAppRule {
        app_id: "rustdesk.app",
        display_name: "RustDesk",
        category: AppCategory::Utility,
        publisher: "RustDesk",
        description: "基于 Rust 构建的开源低延迟远程桌面控制软件",
        exe_stems: &["rustdesk"],
        path_contains: &["rustdesk"],
        reg_keywords: &["rustdesk"],
    },
    KnownAppRule {
        app_id: "flclash.app",
        display_name: "FlClash",
        category: AppCategory::Utility,
        publisher: "FlClash",
        description: "基于 Clash Meta 内核的跨平台现代网络代理客户端",
        exe_stems: &["flclash"],
        path_contains: &["flclash"],
        reg_keywords: &["flclash", "ikuuuvpn"],
    },
    KnownAppRule {
        app_id: "loliashizuku.app",
        display_name: "LoliaShizuku",
        category: AppCategory::Utility,
        publisher: "Mxmilu666",
        description: "Lolia FRP 内网穿透客户端与实时隧道状态控制面板",
        exe_stems: &["loliashizuku"],
        path_contains: &["loliashizuku"],
        reg_keywords: &["loliashizuku"],
    },
    KnownAppRule {
        app_id: "jumengcloud.app",
        display_name: "聚盟云",
        category: AppCategory::Utility,
        publisher: "聚盟云",
        description: "多端网络加速与云端互联服务桌面客户端",
        exe_stems: &["jumengcloud"],
        path_contains: &["jumengcloud"],
        reg_keywords: &["聚盟云"],
    },
    KnownAppRule {
        app_id: "famatech.radminvpn",
        display_name: "Radmin VPN",
        category: AppCategory::Utility,
        publisher: "Famatech",
        description: "虚拟局域网组网与远端设备互联工具",
        exe_stems: &["rvpnctrl", "radminvpn"],
        path_contains: &["radmin vpn"],
        reg_keywords: &["radmin vpn"],
    },
    KnownAppRule {
        app_id: "leigod.accelerator",
        display_name: "雷神加速器",
        category: AppCategory::Utility,
        publisher: "雷神科技",
        description: "支持按分钟计费的 PC 联机游戏网络加速工具",
        exe_stems: &["leigod_launcher", "leigod"],
        path_contains: &["leigod_acc"],
        reg_keywords: &["雷神加速器"],
    },
    KnownAppRule {
        app_id: "heybox.accelerator",
        display_name: "小黑盒加速器",
        category: AppCategory::Utility,
        publisher: "清枫科技",
        description: "低延迟海外游戏专线加速与网络诊断客户端",
        exe_stems: &["heyboxaccelerator"],
        path_contains: &["heyboxaccelerator"],
        reg_keywords: &["小黑盒加速器"],
    },
    KnownAppRule {
        app_id: "magpie.scaler",
        display_name: "Magpie",
        category: AppCategory::Utility,
        publisher: "Blinue",
        description: "Windows 高画质窗口缩放与超分滤镜增强工具",
        exe_stems: &["magpie"],
        path_contains: &["\\magpie\\"],
        reg_keywords: &["magpie"],
    },
    KnownAppRule {
        app_id: "fontvault.app",
        display_name: "FontVault",
        category: AppCategory::Utility,
        publisher: "FontVault",
        description: "本地字体资产预览、分类与快速调用管理工具",
        exe_stems: &["fontvault"],
        path_contains: &["fontvault"],
        reg_keywords: &["fontvault"],
    },
    KnownAppRule {
        app_id: "wallpaperengine.app",
        display_name: "Wallpaper Engine",
        category: AppCategory::Utility,
        publisher: "Wallpaper Engine Team",
        description: "动态交互式桌面壁纸渲染与工坊创作管理软件",
        exe_stems: &["wallpaper64", "wallpaper32", "ui32"],
        path_contains: &["wallpaper_engine"],
        reg_keywords: &["wallpaper engine"],
    },
    KnownAppRule {
        app_id: "huorong.security",
        display_name: "火绒安全软件",
        category: AppCategory::Utility,
        publisher: "火绒安全",
        description: "轻量静默型终端安全防护、弹窗拦截与系统诊断软件",
        exe_stems: &["hipsmain"],
        path_contains: &["huorong"],
        reg_keywords: &["火绒安全软件"],
    },
    KnownAppRule {
        app_id: "dawn.tbtoolbox",
        display_name: "图吧工具箱",
        category: AppCategory::Utility,
        publisher: "Dawn Studio",
        description: "开源硬件检测、烤机压测与外设校验工具合集",
        exe_stems: &["图吧工具箱"],
        path_contains: &["图吧工具箱"],
        reg_keywords: &["图吧工具箱"],
    },
    KnownAppRule {
        app_id: "diskgenius.app",
        display_name: "DiskGenius",
        category: AppCategory::Utility,
        publisher: "易数科技",
        description: "磁盘分区管理、数据恢复与扇区检测专业工具",
        exe_stems: &["diskgenius"],
        path_contains: &["diskgenius"],
        reg_keywords: &["diskgenius"],
    },
    KnownAppRule {
        app_id: "lenovo.legionzone",
        display_name: "Legion Zone",
        category: AppCategory::Utility,
        publisher: "联想",
        description: "拯救者设备性能调度、灯效控制与硬件监控中心",
        exe_stems: &["lzmain", "legionzone"],
        path_contains: &["legionzone"],
        reg_keywords: &["legion zone"],
    },
    KnownAppRule {
        app_id: "nvidia.app",
        display_name: "NVIDIA App",
        category: AppCategory::Utility,
        publisher: "NVIDIA",
        description: "GeForce 显卡驱动管理、画质调优与录屏中心",
        exe_stems: &["nvidia app"],
        path_contains: &["nvidia app"],
        reg_keywords: &["nvidia app"],
    },
    KnownAppRule {
        app_id: "razer.synapse",
        display_name: "Razer Synapse",
        category: AppCategory::Utility,
        publisher: "Razer",
        description: "雷蛇外设驱动配置、宏命令与 Chroma 灯效联动中心",
        exe_stems: &["razer synapse 3", "razerappengine"],
        path_contains: &["razer"],
        reg_keywords: &["razer synapse"],
    },
    KnownAppRule {
        app_id: "i4tools.app",
        display_name: "爱思助手",
        category: AppCategory::Utility,
        publisher: "为爱普信息",
        description: "Apple iOS 设备管理、数据备份与固件刷机工具",
        exe_stems: &["i4launcher", "i4tools"],
        path_contains: &["i4tools"],
        reg_keywords: &["爱思助手"],
    },
    KnownAppRule {
        app_id: "microsoft.settings",
        display_name: "Windows 设置",
        category: AppCategory::Utility,
        publisher: "Microsoft",
        description: "Windows 系统参数、个性化与设备管理控制台",
        exe_stems: &["systemsettings"],
        path_contains: &["immersivecontrolpanel"],
        reg_keywords: &[],
    },
    KnownAppRule {
        app_id: "microsoft.explorer",
        display_name: "文件资源管理器",
        category: AppCategory::Utility,
        publisher: "Microsoft",
        description: "Windows 本地磁盘文件浏览与窗口管理界面",
        exe_stems: &["explorer"],
        path_contains: &[],
        reg_keywords: &[],
    },
    KnownAppRule {
        app_id: SELF_APP_ID,
        display_name: "澄镜",
        category: AppCategory::Utility,
        publisher: "PrismTime",
        description: "Windows 屏幕使用时长与使用时段智能洞察软件",
        exe_stems: &["prism-time"],
        path_contains: &["prism-time"],
        reg_keywords: &[],
    },
];

const IGNORED_SYSTEM_STEMS: &[&str] = &[
    "svchost",
    "dwm",
    "csrss",
    "sihost",
    "taskhostw",
    "searchhost",
    "searchapp",
    "shellexperiencehost",
    "startmenuexperiencehost",
    "textinputhost",
    "lockapp",
    "logonui",
    "runtimebroker",
    "dllhost",
    "conhost",
    "openconsole",
    "appactions",
    "crossdeviceresume",
    "dynamicdependencylifetimemanagershadow",
    "pushnotificationslongrunningtask",
    "ctfmon",
    "smartscreen",
    "msiexec",
    "consent",
    "lsass",
    "services",
    "smss",
    "wininit",
    "winlogon",
    "fontdrvhost",
    "spoolsv",
    "audiodg",
    "backgroundtaskhost",
    "systemsettingsbroker",
    "wudfhost",
    "useroobebroker",
    "widgetservice",
    "widgets",
    "phoneexperiencehost",
    "yourphone",
    "gamebar",
    "gamebarftserver",
    "securityhealthsystray",
    "securityhealthservice",
    "msmpeng",
    // Background trays, crashpad handlers, IME engines & guards
    "wetype_server",
    "wetype_update",
    "crashpad_handler",
    "geminicrashpadhandler",
    "nutstore_watchdog",
    "douyin_guard",
    "douyin_tray",
    "hipstray",
    "nvcontainer",
    "nvidia overlay",
    "smartenginehost64",
    "gacontroller",
    "gahighlight",
    "winrtutil64",
    "anticheatexpert",
    "ace-tray",
    "ace-service64",
    "edgegameassist",
    "xboxpcappft",
    "steamservice",
    "steamerrorreporter",
    // Non-GUI child worker / CLI helper processes
    "codex-code-mode-host",
    "codex-computer-use-swift",
    "extension-host",
    "language_server",
    "magpie-bun",
    "node",
    "node_repl",
    "python",
    "pythonw",
    "git",
    "cargo",
    "rustc",
    "rust-analyzer",
    "esbuild",
    "bun",
    "uv",
    "uvx",
    "rtk",
    "translucenttb",
];

const IGNORED_EXPLORER_CLASSES: &[&str] = &[
    "Progman",
    "WorkerW",
    "Shell_TrayWnd",
    "NotifyIconOverflowWindow",
    "TopLevelWindowForOverflowXamlIsland",
];

const ALLOWED_EXPLORER_CLASSES: &[&str] = &["CabinetWClass", "ExploreWClass"];

pub fn extract_process_stem(path_or_name: &str) -> String {
    let trimmed = path_or_name.trim().trim_matches('"');
    if trimmed.is_empty() {
        return String::new();
    }
    let file_name = trimmed
        .rsplit(['\\', '/'])
        .next()
        .unwrap_or(trimmed)
        .trim();
    if let Some(dot_idx) = file_name.rfind('.') {
        let ext = &file_name[dot_idx + 1..];
        if matches!(
            ext.to_ascii_lowercase().as_str(),
            "exe" | "com" | "scr" | "bat" | "cmd" | "msi" | "pif" | "bin" | "tmp" | "appref-ms"
        ) {
            return file_name[..dot_idx].trim().to_string();
        }
    }
    file_name.to_string()
}

pub fn extract_exe_stem(path_or_name: &str) -> String {
    extract_process_stem(path_or_name).to_lowercase()
}

pub fn is_ignored_process(exe_stem_lower: &str, exe_path_lower: &str, window_class: Option<&str>) -> bool {
    if exe_stem_lower.is_empty() {
        return true;
    }

    if exe_stem_lower == "explorer" {
        if let Some(cls) = window_class {
            if IGNORED_EXPLORER_CLASSES.iter().any(|c| c.eq_ignore_ascii_case(cls)) {
                return true;
            }
            if !ALLOWED_EXPLORER_CLASSES.iter().any(|c| c.eq_ignore_ascii_case(cls)) {
                // Unknown explorer shell overlay / desktop popup -> ignore rather than pollute stats
                return true;
            }
        }
    }

    if IGNORED_SYSTEM_STEMS.contains(&exe_stem_lower) {
        return true;
    }

    if exe_stem_lower.starts_with("unins")
        || exe_stem_lower.contains("uninstall")
        || exe_stem_lower.contains("crashpad")
        || exe_stem_lower.ends_with("_update")
        || exe_stem_lower.ends_with("updater")
    {
        return true;
    }

    if exe_path_lower.contains("\\windows\\systemapps\\")
        && exe_stem_lower != "systemsettings"
    {
        return true;
    }

    false
}

pub fn is_lock_screen_process(exe_stem_lower: &str) -> bool {
    matches!(exe_stem_lower, "lockapp" | "logonui")
}

pub fn is_browser_stem(exe_stem_lower: &str) -> bool {
    matches!(
        exe_stem_lower,
        "msedge"
            | "chrome"
            | "ego"
            | "firefox"
            | "quark"
            | "brave"
            | "arc"
            | "vivaldi"
            | "opera"
            | "zen"
            | "360chrome"
            | "360chromex"
            | "360se"
            | "qqbrowser"
            | "sogouexplorer"
            | "chromium"
            | "waterfox"
            | "librewolf"
            | "floorp"
            | "thorium"
    )
}

pub fn match_known_app(
    exe_stem_lower: &str,
    exe_path_lower: &str,
    product_name_lower: &str,
) -> Option<&'static KnownAppRule> {
    // Disambiguate chatgpt.exe between OpenAI Codex and ChatGPT desktop
    if exe_stem_lower == "chatgpt" {
        if exe_path_lower.contains("openai.codex") || product_name_lower.contains("codex") {
            return KNOWN_APPS.iter().find(|r| r.app_id == "openai.codex");
        }
        return KNOWN_APPS.iter().find(|r| r.app_id == "openai.chatgpt");
    }

    // Disambiguate ArtCraft sub-apps individually when needed so each has accurate name
    if matches!(
        exe_stem_lower,
        "designcraft"
            | "photocraft"
            | "vectorcraft"
            | "filmcraft"
            | "cadcraft"
            | "effectcraft"
            | "lightcraft"
            | "soundcraft"
            | "wordcraft"
            | "gridcraft"
            | "deckcraft"
            | "pdfcraft"
    ) {
        if matches!(
            exe_stem_lower,
            "wordcraft" | "gridcraft" | "deckcraft" | "pdfcraft"
        ) {
            return KNOWN_APPS.iter().find(|r| r.app_id == "artcraft.office");
        }
        return KNOWN_APPS.iter().find(|r| r.app_id == "artcraft.designcraft");
    }

    // 1. Exact exe stem match
    if let Some(rule) = KNOWN_APPS
        .iter()
        .find(|r| r.exe_stems.contains(&exe_stem_lower))
    {
        return Some(rule);
    }

    // 2. Path hint match (for generic stems like launcher.exe)
    if !exe_path_lower.is_empty() {
        if let Some(rule) = KNOWN_APPS.iter().find(|r| {
            !r.path_contains.is_empty()
                && r.path_contains
                    .iter()
                    .any(|needle| exe_path_lower.contains(needle))
        }) {
            return Some(rule);
        }
    }

    None
}

pub fn match_known_by_registry_name(display_name_lower: &str, install_loc_lower: &str) -> Option<&'static KnownAppRule> {
    if let Some(rule) = KNOWN_APPS.iter().find(|r| {
        !r.path_contains.is_empty()
            && !install_loc_lower.is_empty()
            && r.path_contains.iter().any(|p| install_loc_lower.contains(p))
    }) {
        return Some(rule);
    }

    KNOWN_APPS.iter().find(|r| {
        !r.reg_keywords.is_empty()
            && r.reg_keywords
                .iter()
                .any(|kw| display_name_lower.contains(kw))
    })
}

pub fn clean_software_display_name(raw: &str) -> String {
    let mut s = raw.trim().trim_matches('"').to_string();

    // Strip common version/architecture suffixes iteratively
    let patterns = [
        " (x64 zh-CN)",
        " (x64)",
        " (x86)",
        " (64-bit)",
        " (32-bit)",
        " (User)",
        " (Current user)",
        " (当前用户, 32 位)",
        " (当前用户, 64 位)",
        " - zh-cn",
        " (Beta)",
        ".exe",
        ".EXE",
    ];
    for p in patterns {
        if let Some(pos) = s.to_lowercase().find(&p.to_lowercase()) {
            let end = pos + p.len();
            if end == s.len() || s[end..].starts_with(' ') {
                s.replace_range(pos..end, "");
            }
        }
    }

    // Strip trailing "版本 X.Y.Z" or "version X.Y.Z" or "V1.2.3" or numeric semver like " 0.4.1"
    let lower = s.to_lowercase();
    for marker in [" 版本 ", " version ", " v"] {
        if let Some(idx) = lower.rfind(marker) {
            let tail = s[idx + marker.len()..].trim();
            if tail.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                s.truncate(idx);
                break;
            }
        }
    }

    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() >= 2 {
        let last = parts.last().unwrap();
        let looks_like_version = last.contains('.')
            && last
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.' || c == '+' || c == '-' || c == 'b');
        if looks_like_version {
            s = parts[..parts.len() - 1].join(" ");
        }
    }

    let trimmed = s.trim().trim_matches('-').trim().to_string();
    if trimmed.is_empty() {
        raw.trim().to_string()
    } else {
        trimmed
    }
}

pub fn infer_category_from_hints(
    name_lower: &str,
    publisher_lower: &str,
    path_lower: &str,
) -> AppCategory {
    let combined = format!("{name_lower} {publisher_lower} {path_lower}");

    if combined.contains("steamapps")
        || combined.contains("game")
        || combined.contains("launcher")
        || combined.contains("rockstar")
        || combined.contains("hypergryph")
        || combined.contains("模拟器")
    {
        return AppCategory::Game;
    }
    if combined.contains("adobe")
        || combined.contains("cinema 4d")
        || combined.contains("blender")
        || combined.contains("figma")
        || combined.contains("剪映")
        || combined.contains("直播")
        || combined.contains("photoshop")
        || combined.contains("illustrator")
        || combined.contains("craft")
    {
        return AppCategory::Design;
    }
    if combined.contains("code")
        || combined.contains("studio")
        || combined.contains("terminal")
        || combined.contains("jetbrains")
        || combined.contains("git")
        || combined.contains("sdk")
        || combined.contains("ide")
        || combined.contains("compiler")
        || combined.contains("python")
        || combined.contains("rust")
    {
        return AppCategory::Dev;
    }
    if combined.contains("ai")
        || combined.contains("gpt")
        || combined.contains("gemini")
        || combined.contains("ollama")
        || combined.contains("comfyui")
        || combined.contains("bot")
        || combined.contains("deepseek")
    {
        return AppCategory::Ai;
    }
    if combined.contains("browser")
        || combined.contains("chrome")
        || combined.contains("edge")
        || combined.contains("firefox")
    {
        return AppCategory::Browser;
    }
    if combined.contains("office")
        || combined.contains("word")
        || combined.contains("excel")
        || combined.contains("powerpoint")
        || combined.contains("note")
        || combined.contains("pdf")
        || combined.contains("markdown")
        || combined.contains("学习")
        || combined.contains("网盘")
        || combined.contains("云")
    {
        return AppCategory::Office;
    }
    if combined.contains("wechat")
        || combined.contains("weixin")
        || combined.contains("qq")
        || combined.contains("meeting")
        || combined.contains("会议")
        || combined.contains("telegram")
        || combined.contains("discord")
        || combined.contains("钉钉")
    {
        return AppCategory::Social;
    }
    if combined.contains("music")
        || combined.contains("音乐")
        || combined.contains("player")
        || combined.contains("video")
        || combined.contains("视频")
        || combined.contains("comic")
        || combined.contains("漫画")
        || combined.contains("bilibili")
        || combined.contains("douyin")
    {
        return AppCategory::Media;
    }

    AppCategory::Utility
}

pub fn humanize_fallback_name(exe_stem: &str) -> String {
    let cleaned = exe_stem
        .replace('_', " ")
        .replace('-', " ")
        .replace('.', " ");
    cleaned
        .split_whitespace()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn slugify_app_id(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut prev_dot = false;
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dot = false;
        } else if !prev_dot && !out.is_empty() {
            out.push('.');
            prev_dot = true;
        }
    }
    let trimmed = out.trim_matches('.').to_string();
    if trimmed.is_empty() {
        // Stable hex hash for non-ASCII app names
        let mut hash: u64 = 14695981039346656037;
        for b in raw.as_bytes() {
            hash ^= u64::from(*b);
            hash = hash.wrapping_mul(1099511628211);
        }
        format!("app.{hash:012x}")
    } else {
        format!("scanned.{trimmed}")
    }
}

pub fn apply_override(meta: &mut AppMetadata, ov: &AppOverride) {
    if let Some(name) = &ov.display_name {
        let trimmed = name.trim();
        if !trimmed.is_empty() {
            meta.display_name = trimmed.to_string();
        }
    }
    if let Some(cat) = ov.category {
        meta.category = cat;
    }
    if let Some(desc) = &ov.description {
        let trimmed = desc.trim();
        if !trimmed.is_empty() {
            meta.description = trimmed.to_string();
        }
    }
    if let Some(ignored) = ov.ignored {
        meta.ignored = ignored;
    }
}

pub fn resolve_observed_app(
    exe_path: &str,
    pe_product: Option<&str>,
    pe_description: Option<&str>,
    pe_company: Option<&str>,
    _window_title: Option<&str>,
    catalog: &HashMap<String, AppMetadata>,
) -> AppMetadata {
    let process_stem = extract_process_stem(exe_path);
    let exe_stem_lower = process_stem.to_lowercase();
    let exe_path_lower = exe_path.to_lowercase();
    let product_lower = pe_product.unwrap_or_default().to_lowercase();

    // 1. Build-time recognized encyclopedia rule match
    if let Some(rule) = match_known_app(&exe_stem_lower, &exe_path_lower, &product_lower) {
        if let Some(existing) = catalog.get(rule.app_id) {
            return existing.clone();
        }
        return AppMetadata {
            app_id: rule.app_id.to_string(),
            display_name: rule.display_name.to_string(),
            category: rule.category,
            description: rule.description.to_string(),
            publisher: rule.publisher.to_string(),
            icon_data_url: None,
            exe_paths: if exe_path.is_empty() { vec![] } else { vec![exe_path.to_string()] },
            process_names: if process_stem.is_empty() {
                vec![]
            } else {
                vec![process_stem]
            },
            source: "known".to_string(),
            installed: true,
            ignored: false,
            last_seen_ts: None,
        };
    }

    // 2. Program unrecognized at build time -> record its extensionless process name
    let fallback_stem = if process_stem.is_empty() {
        "unknown_process".to_string()
    } else {
        process_stem
    };
    let proc_app_id = if exe_stem_lower.is_empty() {
        "proc.unknown".to_string()
    } else {
        format!("proc.{exe_stem_lower}")
    };

    // 2a. Reuse existing observed entry (preserves cached icons & user overrides)
    if let Some(existing) = catalog.get(&proc_app_id).filter(|m| m.source == "observed") {
        return existing.clone();
    }
    if !exe_stem_lower.is_empty() {
        if let Some(existing) = catalog.values().find(|item| {
            item.source == "observed"
                && item
                    .process_names
                    .iter()
                    .any(|p| p.eq_ignore_ascii_case(&exe_stem_lower))
        }) {
            return existing.clone();
        }
    }

    // 2b. Check runtime registry scanned entries (by path, process stem, or install directory)
    let dir_hint_match = Path::new(exe_path)
        .parent()
        .and_then(|p| p.to_str())
        .map(|d| d.to_lowercase())
        .filter(|d| d.len() > 6);

    let scanned_match = catalog.values().find(|item| {
        item.source == "scanned"
            && ((!exe_path_lower.is_empty()
                && item
                    .exe_paths
                    .iter()
                    .any(|p| p.eq_ignore_ascii_case(exe_path)))
                || (!exe_stem_lower.is_empty()
                    && item
                        .process_names
                        .iter()
                        .any(|p| p.eq_ignore_ascii_case(&exe_stem_lower)))
                || dir_hint_match.as_ref().is_some_and(|dir_l| {
                    item.exe_paths.iter().any(|p| {
                        let pl = p.to_lowercase();
                        dir_l.starts_with(&pl) || pl.starts_with(dir_l)
                    })
                }))
    });

    if let Some(scanned) = scanned_match {
        let description = if !scanned.display_name.eq_ignore_ascii_case(&fallback_stem) {
            format!("未内置识别进程 · 关联安装项：{}", scanned.display_name)
        } else {
            format!("未内置识别进程 · {}", scanned.description)
        };
        let mut exe_paths = scanned.exe_paths.clone();
        if !exe_path.is_empty() && !exe_paths.iter().any(|p| p.eq_ignore_ascii_case(exe_path)) {
            exe_paths.push(exe_path.to_string());
        }
        return AppMetadata {
            app_id: scanned.app_id.clone(),
            display_name: fallback_stem.clone(),
            category: scanned.category,
            description,
            publisher: scanned.publisher.clone(),
            icon_data_url: scanned.icon_data_url.clone(),
            exe_paths,
            process_names: vec![fallback_stem],
            source: "observed".to_string(),
            installed: true,
            ignored: scanned.ignored,
            last_seen_ts: scanned.last_seen_ts,
        };
    }

    // 2c. Standalone/portable program unrecognized at build time
    let publisher = pe_company
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "本地未识别程序".to_string());

    let category = infer_category_from_hints(
        &exe_stem_lower,
        &publisher.to_lowercase(),
        &exe_path_lower,
    );

    let pe_hint = pe_description
        .or(pe_product)
        .map(clean_software_display_name)
        .filter(|s| {
            !s.is_empty()
                && !s.eq_ignore_ascii_case(&fallback_stem)
                && !s.to_lowercase().contains("operating system")
        });

    let description = match pe_hint {
        Some(hint) => format!("未内置识别进程 · {hint}"),
        None => format!("未内置识别进程 · {}", category.label()),
    };

    AppMetadata {
        app_id: proc_app_id,
        display_name: fallback_stem.clone(),
        category,
        description,
        publisher,
        icon_data_url: None,
        exe_paths: if exe_path.is_empty() { vec![] } else { vec![exe_path.to_string()] },
        process_names: vec![fallback_stem],
        source: "observed".to_string(),
        installed: true,
        ignored: false,
        last_seen_ts: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_and_daemon_processes_are_blocked() {
        for proc_name in [
            "svchost.exe",
            "SearchHost.exe",
            "wetype_server.exe",
            "crashpad_handler.exe",
            "codex-code-mode-host.exe",
            "HipsTray.exe",
            "LockApp.exe",
            "unins000.exe",
        ] {
            let stem = extract_exe_stem(proc_name);
            assert!(
                is_ignored_process(&stem, "", None),
                "expected {proc_name} to be ignored"
            );
        }
    }

    #[test]
    fn explorer_desktop_clicks_ignored_but_file_manager_allowed() {
        assert!(is_ignored_process("explorer", "", Some("Progman")));
        assert!(is_ignored_process("explorer", "", Some("WorkerW")));
        assert!(is_ignored_process("explorer", "", Some("Shell_TrayWnd")));
        assert!(!is_ignored_process("explorer", "", Some("CabinetWClass")));
    }

    #[test]
    fn child_processes_aggregate_into_parent_software() {
        let catalog = HashMap::new();

        let wx = resolve_observed_app(
            r"C:\Users\Test\AppData\Roaming\Tencent\xwechat\WeChatAppEx.exe",
            None,
            None,
            None,
            None,
            &catalog,
        );
        assert_eq!(wx.app_id, "tencent.weixin");
        assert_eq!(wx.display_name, "微信");
        assert_eq!(wx.category, AppCategory::Social);

        let steam = resolve_observed_app(
            r"C:\Program Files (x86)\Steam\bin\cef\cef.win64\steamwebhelper.exe",
            None,
            None,
            None,
            None,
            &catalog,
        );
        assert_eq!(steam.app_id, "valve.steam");
        assert_eq!(steam.display_name, "Steam");
        assert_eq!(steam.category, AppCategory::Game);

        let codex = resolve_observed_app(
            r"C:\Program Files\WindowsApps\OpenAI.Codex_26.1120.6357.0_x64__2p2nqsd0c76g0\app\ChatGPT.exe",
            Some("Codex"),
            Some("Codex"),
            Some("OpenAI"),
            None,
            &catalog,
        );
        assert_eq!(codex.app_id, "openai.codex");
        assert_eq!(codex.display_name, "OpenAI Codex");
        assert_eq!(codex.category, AppCategory::Ai);

        let ego = resolve_observed_app(
            r"C:\Program Files\Citro Labs\ego lite\Application\ego.exe",
            Some("ego (lite)"),
            None,
            Some("Citro Labs"),
            None,
            &catalog,
        );
        assert_eq!(ego.app_id, "citrolabs.ego");
        assert_eq!(ego.display_name, "Ego Browser");
        assert_eq!(ego.category, AppCategory::Browser);
    }

    #[test]
    fn extract_process_stem_strips_extensions_and_preserves_case() {
        assert_eq!(extract_process_stem(r"D:\Tools\PixelBench_x64.exe"), "PixelBench_x64");
        assert_eq!(extract_process_stem(r"C:\WINDOWS\system32\NOTEPAD.EXE"), "NOTEPAD");
        assert_eq!(extract_process_stem(r"C:\App\Flow.Launcher.exe"), "Flow.Launcher");
        assert_eq!(extract_process_stem(r"D:\Scripts\Mixed.Exe"), "Mixed");
        assert_eq!(extract_process_stem(r"C:\Tools\helper.cmd"), "helper");
        assert_eq!(extract_process_stem(r"C:\Tools\saver.scr"), "saver");
        assert_eq!(extract_process_stem(r"C:\Tools\script.bat"), "script");
    }

    #[test]
    fn unrecognized_programs_record_process_name_without_extension() {
        let mut catalog = HashMap::new();

        // Unknown standalone portable binary -> extensionless process name even if PE ProductName exists
        let unknown = resolve_observed_app(
            r"D:\Portable\PixelBench_x64.exe",
            Some("Electron"),
            Some("Bench Runner"),
            Some("Pixel Studio"),
            Some("Benchmark Window"),
            &catalog,
        );
        assert_eq!(unknown.app_id, "proc.pixelbench_x64");
        assert_eq!(unknown.display_name, "PixelBench_x64");
        assert_eq!(unknown.process_names, vec!["PixelBench_x64"]);
        assert_eq!(unknown.source, "observed");
        assert!(!unknown.display_name.to_lowercase().ends_with(".exe"));

        // Re-observing after manual override preserves user rename
        let mut edited = unknown.clone();
        let edited_id = edited.app_id.clone();
        apply_override(
            &mut edited,
            &AppOverride {
                app_id: edited_id,
                display_name: Some("像素基准测试".to_string()),
                category: Some(AppCategory::Utility),
                description: None,
                ignored: None,
            },
        );
        catalog.insert(edited.app_id.clone(), edited);
        let after_override = resolve_observed_app(
            r"D:\Portable\PixelBench_x64.exe",
            Some("Electron"),
            None,
            None,
            None,
            &catalog,
        );
        assert_eq!(after_override.display_name, "像素基准测试");

        // Pre-existing "scanned" registry app unrecognized at build time resolves with extensionless stem
        catalog.insert(
            "scanned.customtool".to_string(),
            AppMetadata {
                app_id: "scanned.customtool".to_string(),
                display_name: "Custom Tool Suite".to_string(),
                category: AppCategory::Utility,
                description: "效率与系统 · Acme".to_string(),
                publisher: "Acme".to_string(),
                icon_data_url: None,
                exe_paths: vec![r"C:\Program Files\CustomTool\ct_runner.exe".to_string()],
                process_names: vec!["ct_runner".to_string()],
                source: "scanned".to_string(),
                installed: true,
                ignored: false,
                last_seen_ts: None,
            },
        );
        let observed_scanned = resolve_observed_app(
            r"C:\Program Files\CustomTool\ct_runner.exe",
            None,
            None,
            None,
            None,
            &catalog,
        );
        assert_eq!(observed_scanned.app_id, "scanned.customtool");
        assert_eq!(observed_scanned.display_name, "ct_runner");
        assert_eq!(observed_scanned.process_names, vec!["ct_runner"]);
        assert_eq!(observed_scanned.source, "observed");
    }
}
