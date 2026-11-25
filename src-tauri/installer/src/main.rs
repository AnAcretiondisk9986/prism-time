#![windows_subsystem = "windows"]

use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, IDOK, IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONQUESTION, MB_OK,
    MB_OKCANCEL, MB_YESNO, MESSAGEBOX_STYLE,
};
use windows_registry::CURRENT_USER;

const APP_VERSION: &str = "0.3.0";
const APP_TITLE: &str = "澄镜 · PrismTime V0.3.0 安装程序";
const PAYLOAD_MAGIC: &[u8; 4] = b"PTI1";
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

static COMPRESSED_PAYLOAD: &[u8] = include_bytes!(env!("PRISM_PAYLOAD_PATH"));

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn msg_box(title: &str, text: &str, flags: MESSAGEBOX_STYLE) -> i32 {
    let t = to_wide(title);
    let m = to_wide(text);
    unsafe { MessageBoxW(None, PCWSTR(m.as_ptr()), PCWSTR(t.as_ptr()), flags).0 }
}

fn default_install_dir() -> PathBuf {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("Programs")
        .join("PrismTime")
}

fn stop_running_app() {
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "prism-time.exe"])
        .creation_flags(CREATE_NO_WINDOW)
        .status();
    std::thread::sleep(std::time::Duration::from_millis(350));
}

fn read_u32(slice: &[u8], cursor: &mut usize) -> Result<u32, String> {
    if *cursor + 4 > slice.len() {
        return Err("安装包载荷格式损坏（长度越界）".to_string());
    }
    let val = u32::from_le_bytes([
        slice[*cursor],
        slice[*cursor + 1],
        slice[*cursor + 2],
        slice[*cursor + 3],
    ]);
    *cursor += 4;
    Ok(val)
}

fn extract_payload(target_dir: &Path) -> Result<u64, String> {
    let mut decoder = ZlibDecoder::new(COMPRESSED_PAYLOAD);
    let mut raw = Vec::new();
    decoder
        .read_to_end(&mut raw)
        .map_err(|e| format!("解压安装包内容失败: {e}"))?;

    if raw.len() < 8 || &raw[0..4] != PAYLOAD_MAGIC {
        return Err("安装包签名校验失败".to_string());
    }

    fs::create_dir_all(target_dir)
        .map_err(|e| format!("无法创建安装目录 {}: {e}", target_dir.display()))?;

    let mut cursor = 4usize;
    let file_count = read_u32(&raw, &mut cursor)?;
    let mut total_bytes = 0u64;

    for _ in 0..file_count {
        let name_len = read_u32(&raw, &mut cursor)? as usize;
        if cursor + name_len > raw.len() {
            return Err("安装包文件名长度异常".to_string());
        }
        let name = std::str::from_utf8(&raw[cursor..cursor + name_len])
            .map_err(|e| format!("无效文件名编码: {e}"))?;
        cursor += name_len;

        let data_len = read_u32(&raw, &mut cursor)? as usize;
        if cursor + data_len > raw.len() {
            return Err(format!("文件 {name} 数据长度异常"));
        }
        let data = &raw[cursor..cursor + data_len];
        cursor += data_len;

        // Prevent path traversal
        if name.contains("..") || Path::new(name).is_absolute() {
            return Err(format!("非法安装路径: {name}"));
        }

        let dest = target_dir.join(name);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建子目录失败: {e}"))?;
        }
        fs::write(&dest, data)
            .map_err(|e| format!("写入文件 {} 失败（请先退出正在运行的澄镜）: {e}", dest.display()))?;
        total_bytes += data_len as u64;
    }

    Ok(total_bytes)
}

fn create_shortcut(lnk_path: &Path, target_exe: &Path, work_dir: &Path) {
    let lnk_s = lnk_path.to_string_lossy().replace('\'', "''");
    let exe_s = target_exe.to_string_lossy().replace('\'', "''");
    let wd_s = work_dir.to_string_lossy().replace('\'', "''");
    let script = format!(
        "$ws = New-Object -ComObject WScript.Shell; \
         $s = $ws.CreateShortcut('{lnk_s}'); \
         $s.TargetPath = '{exe_s}'; \
         $s.WorkingDirectory = '{wd_s}'; \
         $s.IconLocation = '{exe_s},0'; \
         $s.Description = '澄镜 · PrismTime 桌面屏幕时长与网页洞察'; \
         $s.Save()"
    );
    let _ = Command::new("powershell")
        .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
        .creation_flags(CREATE_NO_WINDOW)
        .status();
}

fn create_shortcuts(install_dir: &Path) {
    let exe = install_dir.join("prism-time.exe");
    if let Some(profile) = std::env::var_os("USERPROFILE") {
        let desktop = PathBuf::from(profile).join("Desktop").join("澄镜 PrismTime.lnk");
        create_shortcut(&desktop, &exe, install_dir);
    }
    if let Some(appdata) = std::env::var_os("APPDATA") {
        let programs = PathBuf::from(appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs");
        let _ = fs::create_dir_all(&programs);
        let start_lnk = programs.join("澄镜 PrismTime.lnk");
        create_shortcut(&start_lnk, &exe, install_dir);
    }
}

fn remove_shortcuts() {
    if let Some(profile) = std::env::var_os("USERPROFILE") {
        let _ = fs::remove_file(PathBuf::from(profile).join("Desktop").join("澄镜 PrismTime.lnk"));
    }
    if let Some(appdata) = std::env::var_os("APPDATA") {
        let _ = fs::remove_file(
            PathBuf::from(appdata)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("澄镜 PrismTime.lnk"),
        );
    }
}

fn register_uninstall_entry(install_dir: &Path, total_bytes: u64) {
    if let Ok(key) = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\PrismTime") {
        let exe = install_dir.join("prism-time.exe");
        let uninstaller = install_dir.join("uninstall.exe");
        let _ = key.set_string("DisplayName", "澄镜 · PrismTime");
        let _ = key.set_string("DisplayVersion", APP_VERSION);
        let _ = key.set_string("Publisher", "PrismTime");
        let _ = key.set_string("URLInfoAbout", "https://github.com/AnAcretiondisk9986/prism-time");
        let _ = key.set_string("InstallLocation", install_dir.to_string_lossy().as_ref());
        let _ = key.set_string("DisplayIcon", format!("{},0", exe.to_string_lossy()));
        let _ = key.set_string(
            "UninstallString",
            format!("\"{}\" --uninstall", uninstaller.to_string_lossy()),
        );
        let _ = key.set_u32("EstimatedSize", (total_bytes / 1024).max(1024) as u32);
        let _ = key.set_u32("NoModify", 1);
        let _ = key.set_u32("NoRepair", 1);
    }
}

fn run_uninstall(silent: bool) {
    if !silent {
        let confirm = msg_box(
            "卸载 澄镜 · PrismTime",
            "确定要卸载「澄镜 · PrismTime」程序文件与快捷方式吗？\n\n您的历史屏幕时长数据（%LOCALAPPDATA%\\PrismTime）将会保留，不会丢失。",
            MB_YESNO | MB_ICONQUESTION,
        );
        if confirm != IDYES.0 {
            return;
        }
    }

    stop_running_app();
    remove_shortcuts();

    if let Ok(run_key) = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Run") {
        let _ = run_key.remove_value("PrismTime");
    }
    let _ = CURRENT_USER.remove_tree(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\PrismTime");

    let install_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(default_install_dir);

    for fname in ["prism-time.exe", "WebView2Loader.dll", "LICENSE", "README.md"] {
        let _ = fs::remove_file(install_dir.join(fname));
    }

    // Schedule clean removal of uninstall.exe and empty folder after exit
    let dir_s = install_dir.to_string_lossy().to_string();
    let _ = Command::new("cmd")
        .args([
            "/C",
            &format!("ping 127.0.0.1 -n 3 > nul & rmdir /S /Q \"{dir_s}\""),
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();

    if !silent {
        msg_box(
            "卸载完成",
            "已成功卸载「澄镜 · PrismTime」。\n历史记录保留在 %LOCALAPPDATA%\\PrismTime，如需彻底清除可手动删除该目录。",
            MB_OK | MB_ICONINFORMATION,
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let silent = args.iter().any(|a| a.eq_ignore_ascii_case("/S") || a == "--silent");
    let uninstall = args.iter().any(|a| a == "--uninstall");
    let no_shortcuts = args.iter().any(|a| a == "--no-shortcuts");

    if uninstall {
        run_uninstall(silent);
        return;
    }

    let custom_dir = args
        .windows(2)
        .find(|w| w[0] == "--dir")
        .map(|w| PathBuf::from(&w[1]));
    let install_dir = custom_dir.unwrap_or_else(default_install_dir);

    if !silent {
        let prompt = format!(
            "欢迎安装 澄镜 · PrismTime V{APP_VERSION}\n\n\
             安装目录：\n{}\n\n\
             安装将自动完成以下操作：\n\
             • 部署独立客户端程序与 WebView2 加载组件\n\
             • 创建桌面快捷方式与开始菜单入口\n\
             • 注册 Windows「已安装的应用」标准卸载项\n\
             • 完整保留您原有的历史屏幕时长与同步配置\n\n\
             点击「确定」立即安装。",
            install_dir.display()
        );
        if msg_box(APP_TITLE, &prompt, MB_OKCANCEL | MB_ICONINFORMATION) != IDOK.0 {
            return;
        }
    }

    stop_running_app();

    let total_bytes = match extract_payload(&install_dir) {
        Ok(b) => b,
        Err(err) => {
            if !silent {
                msg_box(APP_TITLE, &format!("安装失败：\n{err}"), MB_OK | MB_ICONERROR);
            }
            std::process::exit(1);
        }
    };

    if let Ok(self_exe) = std::env::current_exe() {
        let _ = fs::copy(self_exe, install_dir.join("uninstall.exe"));
    }

    if !no_shortcuts {
        create_shortcuts(&install_dir);
        register_uninstall_entry(&install_dir, total_bytes);
    }

    if !silent {
        let launch = msg_box(
            APP_TITLE,
            &format!(
                "澄镜 · PrismTime V{APP_VERSION} 安装完成！\n\n\
                 安装路径：{}\n\n\
                 是否立即启动「澄镜 · PrismTime」？",
                install_dir.display()
            ),
            MB_YESNO | MB_ICONINFORMATION,
        );
        if launch == IDYES.0 {
            let _ = Command::new(install_dir.join("prism-time.exe"))
                .current_dir(&install_dir)
                .spawn();
        }
    }
}
