use crate::models::WebCategory;

#[derive(Debug, Clone)]
pub struct KnownWebSiteRule {
    pub site_id: &'static str,
    pub display_name: &'static str,
    pub canonical_domain: &'static str,
    pub category: WebCategory,
    pub domain_suffixes: &'static [&'static str],
    pub title_markers: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct ResolvedWebPage {
    pub site_id: String,
    pub display_name: String,
    pub domain: String,
    pub category: WebCategory,
    pub clean_page_title: Option<String>,
}

const INTERNAL_SCHEMES: &[&str] = &[
    "edge://",
    "chrome://",
    "brave://",
    "vivaldi://",
    "opera://",
    "arc://",
    "quark://",
    "chrome-extension://",
    "extension://",
];

pub const KNOWN_WEB_SITES: &[KnownWebSiteRule] = &[
    // --- 代码与技术 (DevTech) ---
    KnownWebSiteRule {
        site_id: "web.github",
        display_name: "GitHub",
        canonical_domain: "github.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["github.com", "github.io", "gist.github.com", "github.dev"],
        title_markers: &["· github", "- github", "github:"],
    },
    KnownWebSiteRule {
        site_id: "web.gitlab",
        display_name: "GitLab",
        canonical_domain: "gitlab.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["gitlab.com"],
        title_markers: &["· gitlab", "- gitlab"],
    },
    KnownWebSiteRule {
        site_id: "web.gitee",
        display_name: "Gitee 码云",
        canonical_domain: "gitee.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["gitee.com"],
        title_markers: &["- gitee.com", "- gitee", "码云"],
    },
    KnownWebSiteRule {
        site_id: "web.stackoverflow",
        display_name: "Stack Overflow",
        canonical_domain: "stackoverflow.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["stackoverflow.com", "stackexchange.com"],
        title_markers: &["- stack overflow", "- server fault", "- super user"],
    },
    KnownWebSiteRule {
        site_id: "web.juejin",
        display_name: "稀土掘金",
        canonical_domain: "juejin.cn",
        category: WebCategory::DevTech,
        domain_suffixes: &["juejin.cn"],
        title_markers: &["- 掘金", "- 稀土掘金"],
    },
    KnownWebSiteRule {
        site_id: "web.csdn",
        display_name: "CSDN",
        canonical_domain: "csdn.net",
        category: WebCategory::DevTech,
        domain_suffixes: &["csdn.net"],
        title_markers: &["- csdn博客", "_csdn博客", "- csdn"],
    },
    KnownWebSiteRule {
        site_id: "web.linuxdo",
        display_name: "LINUX DO",
        canonical_domain: "linux.do",
        category: WebCategory::DevTech,
        domain_suffixes: &["linux.do"],
        title_markers: &["- linux do", "linux do"],
    },
    KnownWebSiteRule {
        site_id: "web.v2ex",
        display_name: "V2EX",
        canonical_domain: "v2ex.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["v2ex.com"],
        title_markers: &["- v2ex", "v2ex ›"],
    },
    KnownWebSiteRule {
        site_id: "web.cnblogs",
        display_name: "博客园",
        canonical_domain: "cnblogs.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["cnblogs.com"],
        title_markers: &["- 博客园"],
    },
    KnownWebSiteRule {
        site_id: "web.segmentfault",
        display_name: "SegmentFault 思否",
        canonical_domain: "segmentfault.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["segmentfault.com"],
        title_markers: &["- segmentfault", "- 思否"],
    },
    KnownWebSiteRule {
        site_id: "web.rust",
        display_name: "Rust 文档与包库",
        canonical_domain: "docs.rs",
        category: WebCategory::DevTech,
        domain_suffixes: &["docs.rs", "crates.io", "rust-lang.org"],
        title_markers: &["- docs.rs", "- crates.io", "doc.rust-lang.org"],
    },
    KnownWebSiteRule {
        site_id: "web.mdn",
        display_name: "MDN Web Docs",
        canonical_domain: "developer.mozilla.org",
        category: WebCategory::DevTech,
        domain_suffixes: &["developer.mozilla.org"],
        title_markers: &["| mdn", "- mdn web docs"],
    },
    KnownWebSiteRule {
        site_id: "web.npmjs",
        display_name: "npm",
        canonical_domain: "npmjs.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["npmjs.com"],
        title_markers: &["- npm"],
    },
    KnownWebSiteRule {
        site_id: "web.pypi",
        display_name: "PyPI",
        canonical_domain: "pypi.org",
        category: WebCategory::DevTech,
        domain_suffixes: &["pypi.org"],
        title_markers: &["· pypi"],
    },
    KnownWebSiteRule {
        site_id: "web.huggingface",
        display_name: "Hugging Face",
        canonical_domain: "huggingface.co",
        category: WebCategory::DevTech,
        domain_suffixes: &["huggingface.co"],
        title_markers: &["- a hugging face space", "· hugging face", "- hugging face"],
    },
    KnownWebSiteRule {
        site_id: "web.kaggle",
        display_name: "Kaggle",
        canonical_domain: "kaggle.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["kaggle.com"],
        title_markers: &["| kaggle"],
    },
    KnownWebSiteRule {
        site_id: "web.leetcode",
        display_name: "LeetCode 力扣",
        canonical_domain: "leetcode.cn",
        category: WebCategory::DevTech,
        domain_suffixes: &["leetcode.cn", "leetcode.com"],
        title_markers: &["- 力扣（leetcode）", "- leetcode"],
    },
    KnownWebSiteRule {
        site_id: "web.vercel",
        display_name: "Vercel",
        canonical_domain: "vercel.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["vercel.com", "vercel.app"],
        title_markers: &["– vercel", "- vercel"],
    },
    KnownWebSiteRule {
        site_id: "web.cloudflare",
        display_name: "Cloudflare",
        canonical_domain: "cloudflare.com",
        category: WebCategory::DevTech,
        domain_suffixes: &["cloudflare.com"],
        title_markers: &["| cloudflare"],
    },
    KnownWebSiteRule {
        site_id: "web.localhost",
        display_name: "本地开发预览",
        canonical_domain: "localhost",
        category: WebCategory::DevTech,
        domain_suffixes: &["localhost", "127.0.0.1"],
        title_markers: &["localhost:"],
    },

    // --- AI 智能平台 (AiWeb) ---
    KnownWebSiteRule {
        site_id: "web.chatgpt",
        display_name: "ChatGPT",
        canonical_domain: "chatgpt.com",
        category: WebCategory::AiWeb,
        domain_suffixes: &["chatgpt.com", "openai.com"],
        title_markers: &["chatgpt"],
    },
    KnownWebSiteRule {
        site_id: "web.claude",
        display_name: "Claude",
        canonical_domain: "claude.ai",
        category: WebCategory::AiWeb,
        domain_suffixes: &["claude.ai", "anthropic.com"],
        title_markers: &["- claude", "claude.ai"],
    },
    KnownWebSiteRule {
        site_id: "web.gemini",
        display_name: "Google Gemini",
        canonical_domain: "gemini.google.com",
        category: WebCategory::AiWeb,
        domain_suffixes: &["gemini.google.com", "aistudio.google.com"],
        title_markers: &["google gemini", "google ai studio"],
    },
    KnownWebSiteRule {
        site_id: "web.deepseek",
        display_name: "DeepSeek",
        canonical_domain: "deepseek.com",
        category: WebCategory::AiWeb,
        domain_suffixes: &["deepseek.com"],
        title_markers: &["deepseek"],
    },
    KnownWebSiteRule {
        site_id: "web.kimi",
        display_name: "Kimi",
        canonical_domain: "kimi.moonshot.cn",
        category: WebCategory::AiWeb,
        domain_suffixes: &["kimi.moonshot.cn", "kimi.com", "moonshot.cn"],
        title_markers: &["kimi.ai", "- kimi", "kimi 智能助手"],
    },
    KnownWebSiteRule {
        site_id: "web.doubao",
        display_name: "豆包网页版",
        canonical_domain: "doubao.com",
        category: WebCategory::AiWeb,
        domain_suffixes: &["doubao.com"],
        title_markers: &["- 豆包", "豆包 -"],
    },
    KnownWebSiteRule {
        site_id: "web.qwen",
        display_name: "通义千问",
        canonical_domain: "qwen.ai",
        category: WebCategory::AiWeb,
        domain_suffixes: &["qwen.ai", "tongyi.aliyun.com", "qianwen.com"],
        title_markers: &["通义千问", "qwen chat"],
    },
    KnownWebSiteRule {
        site_id: "web.zhipu",
        display_name: "智谱清言",
        canonical_domain: "chatglm.cn",
        category: WebCategory::AiWeb,
        domain_suffixes: &["chatglm.cn", "bigmodel.cn", "z.ai"],
        title_markers: &["智谱清言", "chatglm"],
    },
    KnownWebSiteRule {
        site_id: "web.perplexity",
        display_name: "Perplexity",
        canonical_domain: "perplexity.ai",
        category: WebCategory::AiWeb,
        domain_suffixes: &["perplexity.ai"],
        title_markers: &["- perplexity"],
    },
    KnownWebSiteRule {
        site_id: "web.grok",
        display_name: "Grok",
        canonical_domain: "grok.com",
        category: WebCategory::AiWeb,
        domain_suffixes: &["grok.com"],
        title_markers: &["/ grok", "- grok"],
    },
    KnownWebSiteRule {
        site_id: "web.poe",
        display_name: "Poe",
        canonical_domain: "poe.com",
        category: WebCategory::AiWeb,
        domain_suffixes: &["poe.com"],
        title_markers: &["- poe"],
    },

    // --- 视频与音乐 (VideoStream) ---
    KnownWebSiteRule {
        site_id: "web.bilibili",
        display_name: "哔哩哔哩",
        canonical_domain: "bilibili.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["bilibili.com", "b23.tv", "biligame.com"],
        title_markers: &["_哔哩哔哩", "- 哔哩哔哩", "bilibili"],
    },
    KnownWebSiteRule {
        site_id: "web.youtube",
        display_name: "YouTube",
        canonical_domain: "youtube.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["youtube.com", "youtu.be"],
        title_markers: &["- youtube"],
    },
    KnownWebSiteRule {
        site_id: "web.douyin",
        display_name: "抖音网页版",
        canonical_domain: "douyin.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["douyin.com"],
        title_markers: &["- 抖音", "抖音精选"],
    },
    KnownWebSiteRule {
        site_id: "web.vqq",
        display_name: "腾讯视频",
        canonical_domain: "v.qq.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["v.qq.com"],
        title_markers: &["- 腾讯视频", "_腾讯视频"],
    },
    KnownWebSiteRule {
        site_id: "web.iqiyi",
        display_name: "爱奇艺",
        canonical_domain: "iqiyi.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["iqiyi.com"],
        title_markers: &["- 爱奇艺", "_爱奇艺"],
    },
    KnownWebSiteRule {
        site_id: "web.youku",
        display_name: "优酷",
        canonical_domain: "youku.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["youku.com"],
        title_markers: &["- 优酷", "—优酷"],
    },
    KnownWebSiteRule {
        site_id: "web.mgtv",
        display_name: "芒果 TV",
        canonical_domain: "mgtv.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["mgtv.com"],
        title_markers: &["- 芒果tv"],
    },
    KnownWebSiteRule {
        site_id: "web.douyu",
        display_name: "斗鱼直播",
        canonical_domain: "douyu.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["douyu.com"],
        title_markers: &["- 斗鱼", "_斗鱼直播"],
    },
    KnownWebSiteRule {
        site_id: "web.huya",
        display_name: "虎牙直播",
        canonical_domain: "huya.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["huya.com"],
        title_markers: &["- 虎牙直播"],
    },
    KnownWebSiteRule {
        site_id: "web.twitch",
        display_name: "Twitch",
        canonical_domain: "twitch.tv",
        category: WebCategory::VideoStream,
        domain_suffixes: &["twitch.tv"],
        title_markers: &["- twitch"],
    },
    KnownWebSiteRule {
        site_id: "web.netflix",
        display_name: "Netflix",
        canonical_domain: "netflix.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["netflix.com"],
        title_markers: &["- netflix"],
    },
    KnownWebSiteRule {
        site_id: "web.music163",
        display_name: "网易云音乐网页版",
        canonical_domain: "music.163.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["music.163.com"],
        title_markers: &["- 网易云音乐"],
    },
    KnownWebSiteRule {
        site_id: "web.yqq",
        display_name: "QQ 音乐网页版",
        canonical_domain: "y.qq.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["y.qq.com"],
        title_markers: &["- qq音乐"],
    },
    KnownWebSiteRule {
        site_id: "web.spotify",
        display_name: "Spotify",
        canonical_domain: "open.spotify.com",
        category: WebCategory::VideoStream,
        domain_suffixes: &["open.spotify.com", "spotify.com"],
        title_markers: &["| spotify", "- spotify"],
    },

    // --- 社交与社区 (SocialForum) ---
    KnownWebSiteRule {
        site_id: "web.zhihu",
        display_name: "知乎",
        canonical_domain: "zhihu.com",
        category: WebCategory::SocialForum,
        domain_suffixes: &["zhihu.com"],
        title_markers: &["- 知乎", "知乎 -"],
    },
    KnownWebSiteRule {
        site_id: "web.xiaohongshu",
        display_name: "小红书",
        canonical_domain: "xiaohongshu.com",
        category: WebCategory::SocialForum,
        domain_suffixes: &["xiaohongshu.com"],
        title_markers: &["- 小红书", "小红书 -"],
    },
    KnownWebSiteRule {
        site_id: "web.weibo",
        display_name: "微博",
        canonical_domain: "weibo.com",
        category: WebCategory::SocialForum,
        domain_suffixes: &["weibo.com", "weibo.cn"],
        title_markers: &["- 微博", "微博 –"],
    },
    KnownWebSiteRule {
        site_id: "web.douban",
        display_name: "豆瓣",
        canonical_domain: "douban.com",
        category: WebCategory::SocialForum,
        domain_suffixes: &["douban.com"],
        title_markers: &["(豆瓣)", "- 豆瓣"],
    },
    KnownWebSiteRule {
        site_id: "web.tieba",
        display_name: "百度贴吧",
        canonical_domain: "tieba.baidu.com",
        category: WebCategory::SocialForum,
        domain_suffixes: &["tieba.baidu.com"],
        title_markers: &["【图片】", "_百度贴吧"],
    },
    KnownWebSiteRule {
        site_id: "web.x",
        display_name: "X (Twitter)",
        canonical_domain: "x.com",
        category: WebCategory::SocialForum,
        domain_suffixes: &["x.com", "twitter.com"],
        title_markers: &["/ x", "on x:", "- twitter"],
    },
    KnownWebSiteRule {
        site_id: "web.reddit",
        display_name: "Reddit",
        canonical_domain: "reddit.com",
        category: WebCategory::SocialForum,
        domain_suffixes: &["reddit.com"],
        title_markers: &["- reddit", ": reddit"],
    },
    KnownWebSiteRule {
        site_id: "web.discord",
        display_name: "Discord 网页版",
        canonical_domain: "discord.com",
        category: WebCategory::SocialForum,
        domain_suffixes: &["discord.com"],
        title_markers: &["discord |", "- discord"],
    },
    KnownWebSiteRule {
        site_id: "web.telegram",
        display_name: "Telegram Web",
        canonical_domain: "web.telegram.org",
        category: WebCategory::SocialForum,
        domain_suffixes: &["web.telegram.org", "t.me"],
        title_markers: &["telegram web"],
    },
    KnownWebSiteRule {
        site_id: "web.weixinweb",
        display_name: "微信网页版",
        canonical_domain: "wx.qq.com",
        category: WebCategory::SocialForum,
        domain_suffixes: &["wx.qq.com", "weixin.qq.com"],
        title_markers: &["微信网页版"],
    },

    // --- 文档与学术 (DocsWork) ---
    KnownWebSiteRule {
        site_id: "web.feishu",
        display_name: "飞书文档与工作台",
        canonical_domain: "feishu.cn",
        category: WebCategory::DocsWork,
        domain_suffixes: &["feishu.cn", "larksuite.com"],
        title_markers: &["- 飞书", "- 飞书云文档", "- lark"],
    },
    KnownWebSiteRule {
        site_id: "web.docsqq",
        display_name: "腾讯文档",
        canonical_domain: "docs.qq.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["docs.qq.com"],
        title_markers: &["- 腾讯文档"],
    },
    KnownWebSiteRule {
        site_id: "web.kdocs",
        display_name: "金山文档",
        canonical_domain: "kdocs.cn",
        category: WebCategory::DocsWork,
        domain_suffixes: &["kdocs.cn"],
        title_markers: &["- 金山文档"],
    },
    KnownWebSiteRule {
        site_id: "web.yuque",
        display_name: "语雀",
        canonical_domain: "yuque.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["yuque.com"],
        title_markers: &["· 语雀", "- 语雀"],
    },
    KnownWebSiteRule {
        site_id: "web.notion",
        display_name: "Notion",
        canonical_domain: "notion.so",
        category: WebCategory::DocsWork,
        domain_suffixes: &["notion.so", "notion.site"],
        title_markers: &["- notion", "| notion"],
    },
    KnownWebSiteRule {
        site_id: "web.figma",
        display_name: "Figma",
        canonical_domain: "figma.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["figma.com"],
        title_markers: &["– figma", "- figma"],
    },
    KnownWebSiteRule {
        site_id: "web.canva",
        display_name: "Canva 可画",
        canonical_domain: "canva.cn",
        category: WebCategory::DocsWork,
        domain_suffixes: &["canva.cn", "canva.com"],
        title_markers: &["- canva"],
    },
    KnownWebSiteRule {
        site_id: "web.processon",
        display_name: "ProcessOn",
        canonical_domain: "processon.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["processon.com"],
        title_markers: &["- processon"],
    },
    KnownWebSiteRule {
        site_id: "web.googledocs",
        display_name: "Google Docs / Drive",
        canonical_domain: "docs.google.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["docs.google.com", "drive.google.com"],
        title_markers: &["- google docs", "- google sheets", "- google slides", "- google drive"],
    },
    KnownWebSiteRule {
        site_id: "web.gmail",
        display_name: "Gmail",
        canonical_domain: "mail.google.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["mail.google.com"],
        title_markers: &["- gmail"],
    },
    KnownWebSiteRule {
        site_id: "web.outlook",
        display_name: "Outlook 邮箱",
        canonical_domain: "outlook.live.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["outlook.live.com", "outlook.office.com", "outlook.office365.com"],
        title_markers: &["- outlook"],
    },
    KnownWebSiteRule {
        site_id: "web.qqmail",
        display_name: "QQ 邮箱",
        canonical_domain: "mail.qq.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["mail.qq.com"],
        title_markers: &["qq邮箱"],
    },
    KnownWebSiteRule {
        site_id: "web.neteasemail",
        display_name: "网易邮箱",
        canonical_domain: "mail.163.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["mail.163.com", "mail.126.com"],
        title_markers: &["163网易免费邮", "126网易免费邮"],
    },
    KnownWebSiteRule {
        site_id: "web.chaoxing",
        display_name: "超星学习通",
        canonical_domain: "chaoxing.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["chaoxing.com"],
        title_markers: &["超星学习通", "- 学习通"],
    },
    KnownWebSiteRule {
        site_id: "web.icourse163",
        display_name: "中国大学 MOOC",
        canonical_domain: "icourse163.org",
        category: WebCategory::DocsWork,
        domain_suffixes: &["icourse163.org"],
        title_markers: &["中国大学mooc"],
    },
    KnownWebSiteRule {
        site_id: "web.yuketang",
        display_name: "雨课堂",
        canonical_domain: "yuketang.cn",
        category: WebCategory::DocsWork,
        domain_suffixes: &["yuketang.cn"],
        title_markers: &["雨课堂"],
    },
    KnownWebSiteRule {
        site_id: "web.arxiv",
        display_name: "arXiv",
        canonical_domain: "arxiv.org",
        category: WebCategory::DocsWork,
        domain_suffixes: &["arxiv.org"],
        title_markers: &["arxiv.org"],
    },
    KnownWebSiteRule {
        site_id: "web.scholar",
        display_name: "Google Scholar",
        canonical_domain: "scholar.google.com",
        category: WebCategory::DocsWork,
        domain_suffixes: &["scholar.google.com", "scholar.google.com.hk"],
        title_markers: &["google 学术", "google scholar"],
    },
    KnownWebSiteRule {
        site_id: "web.pubmed",
        display_name: "PubMed",
        canonical_domain: "pubmed.ncbi.nlm.nih.gov",
        category: WebCategory::DocsWork,
        domain_suffixes: &["ncbi.nlm.nih.gov"],
        title_markers: &["- pubmed"],
    },
    KnownWebSiteRule {
        site_id: "web.cnki",
        display_name: "中国知网 CNKI",
        canonical_domain: "cnki.net",
        category: WebCategory::DocsWork,
        domain_suffixes: &["cnki.net"],
        title_markers: &["- 中国知网"],
    },
    KnownWebSiteRule {
        site_id: "web.wikipedia",
        display_name: "维基百科 Wikipedia",
        canonical_domain: "wikipedia.org",
        category: WebCategory::DocsWork,
        domain_suffixes: &["wikipedia.org"],
        title_markers: &["- 维基百科", "- wikipedia"],
    },

    // --- 搜索与门户 (SearchPortal) ---
    KnownWebSiteRule {
        site_id: "web.google",
        display_name: "Google 搜索",
        canonical_domain: "google.com",
        category: WebCategory::SearchPortal,
        domain_suffixes: &["google.com", "google.com.hk"],
        title_markers: &["- google 搜索", "- google search"],
    },
    KnownWebSiteRule {
        site_id: "web.bing",
        display_name: "必应 Bing",
        canonical_domain: "bing.com",
        category: WebCategory::SearchPortal,
        domain_suffixes: &["bing.com", "cn.bing.com"],
        title_markers: &["- 国内版 bing", "- 必应", "- bing", "- search"],
    },
    KnownWebSiteRule {
        site_id: "web.baidu",
        display_name: "百度",
        canonical_domain: "baidu.com",
        category: WebCategory::SearchPortal,
        domain_suffixes: &["baidu.com"],
        title_markers: &["百度一下", "_百度搜索", "- 百度百科", "- 百度知道"],
    },
    KnownWebSiteRule {
        site_id: "web.duckduckgo",
        display_name: "DuckDuckGo",
        canonical_domain: "duckduckgo.com",
        category: WebCategory::SearchPortal,
        domain_suffixes: &["duckduckgo.com"],
        title_markers: &["at duckduckgo"],
    },

    // --- 电商与消费 (ShoppingLife) ---
    KnownWebSiteRule {
        site_id: "web.taobao",
        display_name: "淘宝网",
        canonical_domain: "taobao.com",
        category: WebCategory::ShoppingLife,
        domain_suffixes: &["taobao.com"],
        title_markers: &["- 淘宝网", "- 淘宝"],
    },
    KnownWebSiteRule {
        site_id: "web.tmall",
        display_name: "天猫",
        canonical_domain: "tmall.com",
        category: WebCategory::ShoppingLife,
        domain_suffixes: &["tmall.com"],
        title_markers: &["- tmall天猫", "- 天猫"],
    },
    KnownWebSiteRule {
        site_id: "web.jd",
        display_name: "京东",
        canonical_domain: "jd.com",
        category: WebCategory::ShoppingLife,
        domain_suffixes: &["jd.com"],
        title_markers: &["- 京东", "京东(jd.com)"],
    },
    KnownWebSiteRule {
        site_id: "web.pinduoduo",
        display_name: "拼多多",
        canonical_domain: "pinduoduo.com",
        category: WebCategory::ShoppingLife,
        domain_suffixes: &["yangkeduo.com", "pinduoduo.com"],
        title_markers: &["拼多多"],
    },
    KnownWebSiteRule {
        site_id: "web.goofish",
        display_name: "闲鱼",
        canonical_domain: "goofish.com",
        category: WebCategory::ShoppingLife,
        domain_suffixes: &["goofish.com"],
        title_markers: &["- 闲鱼"],
    },
    KnownWebSiteRule {
        site_id: "web.steamweb",
        display_name: "Steam 网页端",
        canonical_domain: "store.steampowered.com",
        category: WebCategory::ShoppingLife,
        domain_suffixes: &["steampowered.com", "steamcommunity.com"],
        title_markers: &["steam 社区 ::", "on steam"],
    },
    KnownWebSiteRule {
        site_id: "web.amazon",
        display_name: "Amazon",
        canonical_domain: "amazon.com",
        category: WebCategory::ShoppingLife,
        domain_suffixes: &["amazon.com", "amazon.cn", "amazon.co.jp"],
        title_markers: &["amazon.com:", ": amazon.com"],
    },
];

/// Sanitizes an omnibox value or URL into strictly a privacy-safe host/domain (no URL paths, queries, or tokens).
pub fn extract_sanitized_host(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_matches('"').trim_matches('\'');
    if trimmed.is_empty() || trimmed.chars().any(char::is_whitespace) {
        return None;
    }

    let lower = trimmed.to_ascii_lowercase();

    for scheme in INTERNAL_SCHEMES {
        if let Some(rest) = lower.strip_prefix(scheme) {
            let seg = rest
                .split(['/', '?', '#'])
                .next()
                .unwrap_or("")
                .trim_end_matches('.');
            if seg.is_empty() {
                return Some(scheme.trim_end_matches("://").to_string());
            }
            return Some(format!("{scheme}{seg}"));
        }
    }

    if let Some(rest) = lower.strip_prefix("about:") {
        let seg = rest.split(['/', '?', '#']).next().unwrap_or("");
        if seg.is_empty() {
            return Some("about:blank".to_string());
        }
        return Some(format!("about:{seg}"));
    }

    if lower.starts_with("file://") {
        return Some("local-file".to_string());
    }

    let after_scheme = lower
        .strip_prefix("https://")
        .or_else(|| lower.strip_prefix("http://"))
        .unwrap_or(&lower);

    // Reject unknown explicit schemes like `javascript:` or `mailto:` or `data:`
    if !lower.starts_with("http://") && !lower.starts_with("https://") {
        let head = after_scheme.split(['/', '?', '#']).next().unwrap_or("");
        if let Some((before_colon, after_colon)) = head.split_once(':') {
            let is_localhost_or_ip_port = (before_colon == "localhost" || before_colon == "127.0.0.1")
                && !after_colon.is_empty()
                && after_colon.chars().all(|c| c.is_ascii_digit());
            if !is_localhost_or_ip_port
                && !(head.starts_with('[') && head.contains(']'))
                && !after_colon.chars().all(|c| c.is_ascii_digit())
            {
                return None;
            }
        }
    }

    let authority = after_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or("")
        .trim_end_matches('.');
    let host_port = authority.rsplit('@').next().unwrap_or(authority);
    if host_port.is_empty() {
        return None;
    }

    if host_port == "localhost" || host_port == "127.0.0.1" || host_port == "[::1]" {
        return Some(host_port.to_string());
    }
    if let Some((base, port)) = host_port.rsplit_once(':') {
        if (base == "localhost" || base == "127.0.0.1" || base == "[::1]")
            && !port.is_empty()
            && port.chars().all(|c| c.is_ascii_digit())
        {
            return Some(format!("{base}:{port}"));
        }
    }

    let host_only = match host_port.rsplit_once(':') {
        Some((h, port)) if !port.is_empty() && port.chars().all(|c| c.is_ascii_digit()) => h,
        _ => host_port,
    };

    if host_only.contains(':') {
        return None;
    }

    if host_only.parse::<std::net::Ipv4Addr>().is_ok() {
        return Some(host_only.to_string());
    }

    let normalized = if let Some(rest) = host_only.strip_prefix("www.") {
        if rest.contains('.') {
            rest
        } else {
            host_only
        }
    } else if let Some(rest) = host_only.strip_prefix("m.") {
        if rest.contains('.') && !matches!(rest, "cn" | "com" | "org" | "net" | "io") {
            rest
        } else {
            host_only
        }
    } else {
        host_only
    };

    if !normalized.contains('.') || normalized.starts_with('.') || normalized.ends_with('.') {
        return None;
    }

    let labels: Vec<&str> = normalized.split('.').collect();
    if labels.len() < 2 {
        return None;
    }
    for label in &labels {
        if label.is_empty()
            || label.starts_with('-')
            || label.ends_with('-')
            || !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return None;
        }
    }
    let tld = labels.last().copied().unwrap_or("");
    if tld.len() < 2 || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }

    Some(normalized.to_string())
}

fn strip_tab_count_suffix(s: &str) -> String {
    let trimmed = s.trim_end();
    if let Some(pos) = trimmed.rfind(" 和另外 ") {
        let rest = &trimmed[pos + " 和另外 ".len()..];
        if rest.ends_with("个页面") || rest.ends_with("个标签页") {
            return trimmed[..pos].trim_end().to_string();
        }
    }
    for marker in [" and "] {
        if let Some(pos) = trimmed.to_ascii_lowercase().rfind(marker) {
            let rest = &trimmed[pos + marker.len()..].to_ascii_lowercase();
            if (rest.ends_with(" more pages") || rest.ends_with(" more page"))
                && rest.chars().next().is_some_and(|c| c.is_ascii_digit())
            {
                return trimmed[..pos].trim_end().to_string();
            }
        }
    }
    trimmed.to_string()
}

pub fn strip_browser_window_suffix(raw_title: &str) -> String {
    let mut cleaned: String = raw_title
        .chars()
        .filter(|&c| !matches!(c, '\u{200b}' | '\u{200e}' | '\u{200f}' | '\u{feff}'))
        .collect();
    cleaned = cleaned.trim().to_string();

    const SUFFIXES: &[&str] = &[
        " - Microsoft\u{200b} Edge",
        " - Microsoft Edge",
        " - Google Chrome",
        " - Chromium",
        " - Mozilla Firefox",
        " — Mozilla Firefox",
        " - Firefox Developer Edition",
        " - ego (lite)",
        " - Ego Browser",
        " - Ego",
        " - Brave",
        " - Vivaldi",
        " - Opera",
        " - Zen Browser",
        " - Arc",
        " - Quark",
        " - 夸克",
        " - QQ浏览器",
        " - 360极速浏览器",
        " - 360安全浏览器",
        " - 搜狗高速浏览器",
        " - InPrivate",
        " (InPrivate)",
        " (Incognito)",
        " (Private Browsing)",
        " (无痕模式)",
        " - 个人",
        " - Personal",
        " - 工作",
        " - Work",
    ];

    for _ in 0..4 {
        let before = cleaned.clone();
        cleaned = strip_tab_count_suffix(&cleaned);
        let lower = cleaned.to_lowercase();
        for suf in SUFFIXES {
            let suf_lower = suf.to_lowercase();
            if lower.ends_with(&suf_lower) {
                let keep_len = cleaned.len().saturating_sub(suf.len());
                if cleaned.is_char_boundary(keep_len) {
                    cleaned = cleaned[..keep_len].trim_end().to_string();
                    break;
                }
            }
        }
        if cleaned == before {
            break;
        }
    }

    cleaned.trim().to_string()
}

fn strip_leading_unread_count(title: &str) -> &str {
    let trimmed = title.trim();
    if let Some(after_open) = trimmed.strip_prefix('(') {
        if let Some(close_idx) = after_open.find(')') {
            let inside = &after_open[..close_idx];
            if !inside.is_empty()
                && inside.len() <= 5
                && inside.chars().all(|c| c.is_ascii_digit() || c == '+')
            {
                return after_open[close_idx + 1..].trim_start();
            }
        }
    }
    trimmed
}

fn domain_matches_rule(host: &str, rule: &KnownWebSiteRule) -> bool {
    let base_host = match host.rsplit_once(':') {
        Some((h, port)) if !port.is_empty() && port.chars().all(|c| c.is_ascii_digit()) => h,
        _ => host,
    };
    rule.domain_suffixes
        .iter()
        .any(|suf| base_host == *suf || base_host.ends_with(&format!(".{suf}")))
}

pub fn infer_web_category(host: &str, title_lower: &str) -> WebCategory {
    let combined = format!("{host} {title_lower}");
    if combined.contains("github")
        || combined.contains("gitlab")
        || combined.contains("gitee")
        || combined.contains("git")
        || combined.contains("docs")
        || combined.contains("api")
        || combined.contains("dev")
        || combined.contains("localhost")
        || combined.contains("127.0.0.1")
        || combined.contains("rust")
        || combined.contains("cargo")
        || combined.contains("npm")
    {
        return WebCategory::DevTech;
    }
    if combined.contains("ai.")
        || combined.contains("gpt")
        || combined.contains("llm")
        || combined.contains("claude")
        || combined.contains("gemini")
        || combined.contains("deepseek")
    {
        return WebCategory::AiWeb;
    }
    if combined.contains("video")
        || combined.contains("tv")
        || combined.contains("live")
        || combined.contains("music")
        || combined.contains("stream")
        || combined.contains("anime")
    {
        return WebCategory::VideoStream;
    }
    if combined.contains("edu.")
        || combined.contains(".edu")
        || combined.contains("wiki")
        || combined.contains("scholar")
        || combined.contains("journal")
        || combined.contains("mail.")
        || combined.contains("notion")
        || combined.contains("study")
        || combined.contains("课程")
    {
        return WebCategory::DocsWork;
    }
    if combined.contains("forum")
        || combined.contains("bbs")
        || combined.contains("community")
        || combined.contains("social")
    {
        return WebCategory::SocialForum;
    }
    if combined.contains("shop")
        || combined.contains("store")
        || combined.contains("mall")
        || combined.contains("buy")
    {
        return WebCategory::ShoppingLife;
    }
    WebCategory::SearchPortal
}

fn is_internal_browser_host(host: &str) -> bool {
    INTERNAL_SCHEMES.iter().any(|s| host.starts_with(s))
        || host.starts_with("about:")
        || host == "local-file"
        || host == "newtab"
}

fn is_new_tab_title(title_lower: &str) -> bool {
    matches!(
        title_lower,
        "" | "新标签页"
            | "新建标签页"
            | "new tab"
            | "起始页"
            | "start page"
            | "blank page"
            | "空白页"
            | "about:blank"
            | "inprivate"
            | "无痕浏览"
    )
}

pub fn resolve_browser_page(raw_url: Option<&str>, window_title: &str) -> ResolvedWebPage {
    let clean_title = strip_browser_window_suffix(window_title);
    let normalized_title = strip_leading_unread_count(&clean_title).to_lowercase();
    let clean_opt = if clean_title.is_empty() {
        None
    } else {
        Some(clean_title.clone())
    };

    if let Some(host) = raw_url.and_then(extract_sanitized_host) {
        if is_internal_browser_host(&host) {
            return ResolvedWebPage {
                site_id: "web.internal".to_string(),
                display_name: "浏览器内置页与新标签".to_string(),
                domain: host,
                category: WebCategory::BrowserInternal,
                clean_page_title: clean_opt,
            };
        }

        if let Some(rule) = KNOWN_WEB_SITES.iter().find(|r| domain_matches_rule(&host, r)) {
            let domain = if host.starts_with("localhost") || host.starts_with("127.0.0.1") {
                host
            } else {
                rule.canonical_domain.to_string()
            };
            return ResolvedWebPage {
                site_id: rule.site_id.to_string(),
                display_name: rule.display_name.to_string(),
                domain,
                category: rule.category,
                clean_page_title: clean_opt,
            };
        }

        let category = infer_web_category(&host, &normalized_title);
        return ResolvedWebPage {
            site_id: format!("domain.{host}"),
            display_name: host.clone(),
            domain: host,
            category,
            clean_page_title: clean_opt,
        };
    }

    // Fallback: URL unavailable (e.g. non-UIA tab or title-only heuristic)
    if is_new_tab_title(&normalized_title) {
        return ResolvedWebPage {
            site_id: "web.internal".to_string(),
            display_name: "浏览器内置页与新标签".to_string(),
            domain: "newtab".to_string(),
            category: WebCategory::BrowserInternal,
            clean_page_title: clean_opt,
        };
    }

    if let Some(rule) = KNOWN_WEB_SITES.iter().find(|r| {
        r.title_markers
            .iter()
            .any(|marker| normalized_title.contains(marker))
    }) {
        return ResolvedWebPage {
            site_id: rule.site_id.to_string(),
            display_name: rule.display_name.to_string(),
            domain: rule.canonical_domain.to_string(),
            category: rule.category,
            clean_page_title: clean_opt,
        };
    }

    // Check if the cleaned page title itself exposes a valid domain or localhost:port
    for token in normalized_title.split(|c: char| c.is_whitespace() || matches!(c, '|' | '—' | ',' | ';')) {
        if let Some(host) = extract_sanitized_host(token) {
            if let Some(rule) = KNOWN_WEB_SITES.iter().find(|r| domain_matches_rule(&host, r)) {
                return ResolvedWebPage {
                    site_id: rule.site_id.to_string(),
                    display_name: rule.display_name.to_string(),
                    domain: host,
                    category: rule.category,
                    clean_page_title: clean_opt,
                };
            }
            let category = infer_web_category(&host, &normalized_title);
            return ResolvedWebPage {
                site_id: format!("domain.{host}"),
                display_name: host.clone(),
                domain: host,
                category,
                clean_page_title: clean_opt,
            };
        }
    }

    ResolvedWebPage {
        site_id: "web.other".to_string(),
        display_name: "其他浏览网页".to_string(),
        domain: "web".to_string(),
        category: WebCategory::SearchPortal,
        clean_page_title: clean_opt,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_url_strips_paths_queries_and_preserves_clean_domain() {
        assert_eq!(
            extract_sanitized_host("https://github.com/AnAcretiondisk9986/prism-time?tab=repositories#readme").as_deref(),
            Some("github.com")
        );
        assert_eq!(
            extract_sanitized_host("www.bilibili.com/video/BV1xx411c7mD/?p=2").as_deref(),
            Some("bilibili.com")
        );
        assert_eq!(
            extract_sanitized_host("http://localhost:1420/dashboard?demo=1").as_deref(),
            Some("localhost:1420")
        );
        assert_eq!(
            extract_sanitized_host("edge://settings/profiles").as_deref(),
            Some("edge://settings")
        );
        assert_eq!(extract_sanitized_host("rust windows ui automation search"), None);
    }

    #[test]
    fn resolves_major_sites_via_omnibox_url_and_title_fallback() {
        let gh = resolve_browser_page(
            Some("github.com/tauri-apps/tauri/issues/1234"),
            "Issue #1234 · tauri-apps/tauri 和另外 4 个页面 - 个人 - Microsoft\u{200b} Edge",
        );
        assert_eq!(gh.site_id, "web.github");
        assert_eq!(gh.display_name, "GitHub");
        assert_eq!(gh.domain, "github.com");
        assert_eq!(gh.category, WebCategory::DevTech);
        assert_eq!(
            gh.clean_page_title.as_deref(),
            Some("Issue #1234 · tauri-apps/tauri")
        );

        let bili = resolve_browser_page(
            None,
            "(3) 【Rust教程】Tauri v2 实战_哔哩哔哩_bilibili - Google Chrome",
        );
        assert_eq!(bili.site_id, "web.bilibili");
        assert_eq!(bili.display_name, "哔哩哔哩");
        assert_eq!(bili.category, WebCategory::VideoStream);

        let zhihu = resolve_browser_page(None, "如何评价 Rust 在 Windows 桌面的生态？ - 知乎 - Mozilla Firefox");
        assert_eq!(zhihu.site_id, "web.zhihu");
        assert_eq!(zhihu.category, WebCategory::SocialForum);
    }

    #[test]
    fn unlisted_real_domain_records_by_sanitized_host() {
        let custom = resolve_browser_page(
            Some("https://dean.wmu.edu.cn/system/login?token=secret123"),
            "教务系统登录 - Microsoft Edge",
        );
        assert_eq!(custom.site_id, "domain.dean.wmu.edu.cn");
        assert_eq!(custom.display_name, "dean.wmu.edu.cn");
        assert_eq!(custom.domain, "dean.wmu.edu.cn");
        assert_eq!(custom.category, WebCategory::DocsWork);
        assert!(!custom.domain.contains("token"));
        assert!(!custom.domain.contains("login"));
    }
}
