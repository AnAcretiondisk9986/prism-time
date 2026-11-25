#[derive(Debug, Clone)]
pub struct ForegroundSample {
    #[allow(dead_code)]
    pub pid: u32,
    pub hwnd_key: isize,
    pub exe_path: String,
    pub exe_stem_lower: String,
    pub window_title: String,
    pub window_class: String,
    pub pe_product: Option<String>,
    pub pe_description: Option<String>,
    pub pe_company: Option<String>,
}

#[cfg(windows)]
mod win_impl {
    use super::ForegroundSample;
    use crate::catalog::extract_exe_stem;
    use crate::web_catalog::extract_sanitized_host;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use base64::Engine;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::ffi::OsStr;
    use std::mem::{size_of, zeroed};
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;
    use windows::core::{BOOL, PCWSTR, PWSTR};
    use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM};
    use windows::Win32::Graphics::Gdi::{
        CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC,
        BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HGDIOBJ,
    };
    use windows::Win32::Storage::FileSystem::{
        GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, FILE_FLAGS_AND_ATTRIBUTES,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
    };
    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    };
    use windows::Win32::System::StationsAndDesktops::{
        CloseDesktop, OpenInputDesktop, DESKTOP_ACCESS_FLAGS, DESKTOP_CONTROL_FLAGS,
    };
    use windows::Win32::System::SystemInformation::GetTickCount64;
    use windows::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT,
        PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows::Win32::UI::Accessibility::{
        CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationTreeWalker,
        IUIAutomationValuePattern, UIA_DocumentControlTypeId, UIA_EditControlTypeId,
        UIA_ValuePatternId,
    };
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};
    use windows::Win32::UI::WindowsAndMessaging::{
        DestroyIcon, EnumChildWindows, GetClassNameW, GetForegroundWindow, GetIconInfo,
        GetWindowTextW, GetWindowThreadProcessId, ICONINFO,
    };

    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }

    fn from_wide_slice(buf: &[u16]) -> String {
        let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        String::from_utf16_lossy(&buf[..len])
    }

    pub fn get_idle_seconds() -> u64 {
        unsafe {
            let mut lii = LASTINPUTINFO {
                cbSize: size_of::<LASTINPUTINFO>() as u32,
                dwTime: 0,
            };
            if GetLastInputInfo(&mut lii).as_bool() {
                let tick32 = GetTickCount64() as u32;
                let elapsed_ms = tick32.wrapping_sub(lii.dwTime);
                u64::from(elapsed_ms / 1000)
            } else {
                0
            }
        }
    }

    pub fn is_workstation_locked() -> bool {
        unsafe {
            // DESKTOP_SWITCHDESKTOP = 0x0100; fails when UAC secure desktop or lock screen is active
            match OpenInputDesktop(DESKTOP_CONTROL_FLAGS(0), false, DESKTOP_ACCESS_FLAGS(0x0100)) {
                Ok(hdesk) => {
                    let _ = CloseDesktop(hdesk);
                    false
                }
                Err(_) => true,
            }
        }
    }

    fn get_window_text(hwnd: HWND) -> String {
        let mut buf = [0u16; 512];
        let len = unsafe { GetWindowTextW(hwnd, &mut buf) };
        if len > 0 {
            String::from_utf16_lossy(&buf[..len as usize])
                .trim()
                .to_string()
        } else {
            String::new()
        }
    }

    fn get_window_class(hwnd: HWND) -> String {
        let mut buf = [0u16; 256];
        let len = unsafe { GetClassNameW(hwnd, &mut buf) };
        if len > 0 {
            String::from_utf16_lossy(&buf[..len as usize])
                .trim()
                .to_string()
        } else {
            String::new()
        }
    }

    fn query_process_image(pid: u32) -> Option<String> {
        if pid == 0 {
            return None;
        }
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
            let mut buf = vec![0u16; 1024];
            let mut size = buf.len() as u32;
            let res = QueryFullProcessImageNameW(
                handle,
                PROCESS_NAME_FORMAT(0),
                PWSTR(buf.as_mut_ptr()),
                &mut size,
            );
            let _ = CloseHandle(handle);
            res.ok()?;
            Some(String::from_utf16_lossy(&buf[..size as usize]))
        }
    }

    struct UwpChildSearch {
        host_pid: u32,
        found_pid: Option<u32>,
    }

    unsafe extern "system" fn enum_uwp_child(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let state = &mut *(lparam.0 as *mut UwpChildSearch);
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid != 0 && pid != state.host_pid {
            state.found_pid = Some(pid);
            return BOOL(0);
        }
        BOOL(1)
    }

    fn find_parent_pid(target_pid: u32) -> Option<u32> {
        unsafe {
            let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
            let mut entry: PROCESSENTRY32W = zeroed();
            entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;
            let mut parent = None;
            if Process32FirstW(snap, &mut entry).is_ok() {
                loop {
                    if entry.th32ProcessID == target_pid {
                        if entry.th32ParentProcessID != 0 {
                            parent = Some(entry.th32ParentProcessID);
                        }
                        break;
                    }
                    if Process32NextW(snap, &mut entry).is_err() {
                        break;
                    }
                }
            }
            let _ = CloseHandle(snap);
            parent
        }
    }

    pub fn probe_foreground_window() -> Option<ForegroundSample> {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.is_invalid() || hwnd.0.is_null() {
                return None;
            }
            let mut pid = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid == 0 {
                return None;
            }

            let title = get_window_text(hwnd);
            let class_name = get_window_class(hwnd);
            let mut exe_path = query_process_image(pid)?;
            let mut stem = extract_exe_stem(&exe_path);

            // Unwrap UWP ApplicationFrameHost.exe child window process
            if stem == "applicationframehost" {
                let mut search = UwpChildSearch {
                    host_pid: pid,
                    found_pid: None,
                };
                let _ = EnumChildWindows(
                    Some(hwnd),
                    Some(enum_uwp_child),
                    LPARAM(&mut search as *mut UwpChildSearch as isize),
                );
                if let Some(child_pid) = search.found_pid {
                    if let Some(child_path) = query_process_image(child_pid) {
                        pid = child_pid;
                        exe_path = child_path;
                        stem = extract_exe_stem(&exe_path);
                    }
                }
            }

            // Unwrap msedgewebview2.exe child renderers back to host desktop app if needed
            if stem == "msedgewebview2" {
                let mut cur = pid;
                for _ in 0..4 {
                    let Some(ppid) = find_parent_pid(cur) else {
                        break;
                    };
                    if let Some(ppath) = query_process_image(ppid) {
                        let pstem = extract_exe_stem(&ppath);
                        if pstem != "msedgewebview2" && pstem != "explorer" && !pstem.is_empty() {
                            pid = ppid;
                            exe_path = ppath;
                            stem = pstem;
                            break;
                        }
                    }
                    cur = ppid;
                }
            }

            let (pe_product, pe_description, pe_company) = read_pe_version_strings(&exe_path);

            Some(ForegroundSample {
                pid,
                hwnd_key: hwnd.0 as isize,
                exe_path,
                exe_stem_lower: stem,
                window_title: title,
                window_class: class_name,
                pe_product,
                pe_description,
                pe_company,
            })
        }
    }

    pub fn read_pe_version_strings(exe_path: &str) -> (Option<String>, Option<String>, Option<String>) {
        if exe_path.is_empty() || !Path::new(exe_path).exists() {
            return (None, None, None);
        }
        unsafe {
            let wide_path = to_wide(exe_path);
            let mut dummy = 0u32;
            let size = GetFileVersionInfoSizeW(PCWSTR(wide_path.as_ptr()), Some(&mut dummy));
            if size == 0 {
                return (None, None, None);
            }
            let mut data = vec![0u8; size as usize];
            if GetFileVersionInfoW(
                PCWSTR(wide_path.as_ptr()),
                None,
                size,
                data.as_mut_ptr() as *mut _,
            )
            .is_err()
            {
                return (None, None, None);
            }

            let mut codepages: Vec<String> = Vec::new();
            let trans_query = to_wide("\\VarFileInfo\\Translation");
            let mut trans_ptr = std::ptr::null_mut();
            let mut trans_len = 0u32;
            if VerQueryValueW(
                data.as_ptr() as *const _,
                PCWSTR(trans_query.as_ptr()),
                &mut trans_ptr,
                &mut trans_len,
            )
            .as_bool()
                && !trans_ptr.is_null()
                && trans_len >= 4
            {
                let pairs = std::slice::from_raw_parts(
                    trans_ptr as *const [u16; 2],
                    (trans_len as usize) / 4,
                );
                for &[lang, cp] in pairs {
                    codepages.push(format!("{lang:04x}{cp:04x}"));
                }
            }
            for fallback in ["080404b0", "040904b0", "040904e4", "000004b0"] {
                if !codepages.iter().any(|c| c.eq_ignore_ascii_case(fallback)) {
                    codepages.push(fallback.to_string());
                }
            }

            let query_key = |key: &str| -> Option<String> {
                for cp in &codepages {
                    let sub = to_wide(&format!("\\StringFileInfo\\{cp}\\{key}"));
                    let mut val_ptr = std::ptr::null_mut();
                    let mut val_len = 0u32;
                    if VerQueryValueW(
                        data.as_ptr() as *const _,
                        PCWSTR(sub.as_ptr()),
                        &mut val_ptr,
                        &mut val_len,
                    )
                    .as_bool()
                        && !val_ptr.is_null()
                        && val_len > 1
                    {
                        let slice =
                            std::slice::from_raw_parts(val_ptr as *const u16, val_len as usize);
                        let s = from_wide_slice(slice).trim().to_string();
                        if !s.is_empty() {
                            return Some(s);
                        }
                    }
                }
                None
            };

            (
                query_key("ProductName"),
                query_key("FileDescription"),
                query_key("CompanyName"),
            )
        }
    }

    pub fn extract_icon_png_data_url(file_path: &str) -> Option<String> {
        if file_path.is_empty() || !Path::new(file_path).exists() {
            return None;
        }
        unsafe {
            let wide = to_wide(file_path);
            let mut shfi: SHFILEINFOW = zeroed();
            let ok = SHGetFileInfoW(
                PCWSTR(wide.as_ptr()),
                FILE_FLAGS_AND_ATTRIBUTES(0),
                Some(&mut shfi),
                size_of::<SHFILEINFOW>() as u32,
                SHGFI_ICON | SHGFI_LARGEICON,
            );
            if ok == 0 || shfi.hIcon.is_invalid() {
                return None;
            }
            let hicon = shfi.hIcon;
            let result = hicon_to_png_data_url(hicon);
            let _ = DestroyIcon(hicon);
            result
        }
    }

    unsafe fn hicon_to_png_data_url(hicon: windows::Win32::UI::WindowsAndMessaging::HICON) -> Option<String> {
        let mut info: ICONINFO = zeroed();
        if GetIconInfo(hicon, &mut info).is_err() {
            return None;
        }

        let target_bitmap = if !info.hbmColor.is_invalid() {
            info.hbmColor
        } else {
            info.hbmMask
        };

        let mut bmp: BITMAP = zeroed();
        let got_obj = GetObjectW(
            HGDIOBJ(target_bitmap.0),
            size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut BITMAP as *mut _),
        );

        let result = if got_obj > 0 && bmp.bmWidth > 0 && bmp.bmHeight > 0 {
            let w = bmp.bmWidth;
            let h = bmp.bmHeight;
            let screen_dc = GetDC(None);
            let mem_dc = CreateCompatibleDC(Some(screen_dc));

            let mut bi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w,
                    biHeight: -h, // top-down scanlines
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [zeroed()],
            };

            let pixel_count = (w as usize) * (h as usize);
            let mut bgra = vec![0u8; pixel_count * 4];
            let scan_lines = if !info.hbmColor.is_invalid() {
                GetDIBits(
                    mem_dc,
                    info.hbmColor,
                    0,
                    h as u32,
                    Some(bgra.as_mut_ptr() as *mut _),
                    &mut bi,
                    DIB_RGB_COLORS,
                )
            } else {
                0
            };

            let out = if scan_lines > 0 {
                let has_alpha = bgra.chunks_exact(4).any(|px| px[3] != 0);
                if !has_alpha && !info.hbmMask.is_invalid() {
                    let mut mask_bgra = vec![0u8; pixel_count * 4];
                    let mask_lines = GetDIBits(
                        mem_dc,
                        info.hbmMask,
                        0,
                        h as u32,
                        Some(mask_bgra.as_mut_ptr() as *mut _),
                        &mut bi,
                        DIB_RGB_COLORS,
                    );
                    if mask_lines > 0 {
                        for (px, mpx) in bgra.chunks_exact_mut(4).zip(mask_bgra.chunks_exact(4)) {
                            // Mask black (0) = opaque, white (255) = transparent
                            px[3] = if mpx[0] > 128 { 0 } else { 255 };
                        }
                    } else {
                        for px in bgra.chunks_exact_mut(4) {
                            px[3] = 255;
                        }
                    }
                }

                // Convert BGRA -> RGBA
                for px in bgra.chunks_exact_mut(4) {
                    px.swap(0, 2);
                }

                encode_rgba_to_data_url(w as u32, h as u32, &bgra)
            } else {
                None
            };

            let _ = DeleteDC(mem_dc);
            let _ = ReleaseDC(None, screen_dc);
            out
        } else {
            None
        };

        if !info.hbmColor.is_invalid() {
            let _ = DeleteObject(HGDIOBJ(info.hbmColor.0));
        }
        if !info.hbmMask.is_invalid() {
            let _ = DeleteObject(HGDIOBJ(info.hbmMask.0));
        }
        result
    }

    fn encode_rgba_to_data_url(width: u32, height: u32, rgba: &[u8]) -> Option<String> {
        let mut png_bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut png_bytes, width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().ok()?;
            writer.write_image_data(rgba).ok()?;
        }
        Some(format!("data:image/png;base64,{}", BASE64.encode(&png_bytes)))
    }

    struct UiaThreadCtx {
        uia: IUIAutomation,
        walker: IUIAutomationTreeWalker,
    }

    thread_local! {
        static UIA_CTX: RefCell<Option<UiaThreadCtx>> = const { RefCell::new(None) };
    }

    fn with_uia<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&IUIAutomation, &IUIAutomationTreeWalker) -> Option<R>,
    {
        UIA_CTX.with(|slot| {
            let mut borrow = slot.borrow_mut();
            if borrow.is_none() {
                unsafe {
                    let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
                    let uia: IUIAutomation =
                        CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).ok()?;
                    let walker = uia.ControlViewWalker().ok()?;
                    *borrow = Some(UiaThreadCtx { uia, walker });
                }
            }
            let ctx = borrow.as_ref()?;
            f(&ctx.uia, &ctx.walker)
        })
    }

    fn is_likely_omnibox_control(cls_lower: &str, name_lower: &str) -> bool {
        cls_lower.contains("omnibox")
            || cls_lower.contains("moz-input")
            || cls_lower.contains("urlbar")
            || name_lower.contains("address")
            || name_lower.contains("search or enter")
            || name_lower.contains("search bar")
            || name_lower.contains("url")
            || name_lower.contains("地址")
            || name_lower.contains("搜索和输入网址")
            || name_lower.contains("搜索或输入网址")
    }

    pub fn probe_browser_omnibox_url(hwnd_key: isize) -> Option<String> {
        if hwnd_key == 0 {
            return None;
        }
        with_uia(|uia, walker| unsafe {
            let hwnd = HWND(hwnd_key as *mut core::ffi::c_void);
            let root = uia.ElementFromHandle(hwnd).ok()?;

            const MAX_NODES: usize = 96;
            const MAX_DEPTH: usize = 7;
            const MAX_SIBLINGS: usize = 24;

            let mut queue: VecDeque<(IUIAutomationElement, usize)> = VecDeque::new();
            queue.push_back((root, 0));
            let mut visited = 0usize;
            let mut fallback_url_candidate: Option<String> = None;

            while let Some((elem, depth)) = queue.pop_front() {
                visited += 1;
                if visited > MAX_NODES {
                    break;
                }

                let ctrl_type = elem.CurrentControlType().unwrap_or_default();
                let cls = elem
                    .CurrentClassName()
                    .map(|b| b.to_string())
                    .unwrap_or_default();
                let cls_lower = cls.to_ascii_lowercase();

                // Never traverse into live browser page DOM containers
                if ctrl_type == UIA_DocumentControlTypeId
                    || cls_lower.contains("renderwidgethost")
                    || cls_lower.contains("intermediate d3d")
                {
                    continue;
                }

                if ctrl_type == UIA_EditControlTypeId {
                    if let Ok(pattern) =
                        elem.GetCurrentPatternAs::<IUIAutomationValuePattern>(UIA_ValuePatternId)
                    {
                        if let Ok(val_bstr) = pattern.CurrentValue() {
                            let val = val_bstr.to_string().trim().to_string();
                            if extract_sanitized_host(&val).is_some() {
                                let name_lower = elem
                                    .CurrentName()
                                    .map(|b| b.to_string().to_lowercase())
                                    .unwrap_or_default();
                                if is_likely_omnibox_control(&cls_lower, &name_lower) {
                                    return Some(val);
                                }
                                if fallback_url_candidate.is_none() {
                                    fallback_url_candidate = Some(val);
                                }
                            }
                        }
                    }
                }

                if depth < MAX_DEPTH {
                    if let Ok(mut child) = walker.GetFirstChildElement(&elem) {
                        for _ in 0..MAX_SIBLINGS {
                            let next = walker.GetNextSiblingElement(&child);
                            queue.push_back((child, depth + 1));
                            match next {
                                Ok(sib) => child = sib,
                                Err(_) => break,
                            }
                        }
                    }
                }
            }

            fallback_url_candidate
        })
    }
}

#[cfg(not(windows))]
mod win_impl {
    use super::ForegroundSample;
    pub fn get_idle_seconds() -> u64 {
        0
    }
    pub fn is_workstation_locked() -> bool {
        false
    }
    pub fn probe_foreground_window() -> Option<ForegroundSample> {
        None
    }
    pub fn probe_browser_omnibox_url(_: isize) -> Option<String> {
        None
    }
    pub fn read_pe_version_strings(_: &str) -> (Option<String>, Option<String>, Option<String>) {
        (None, None, None)
    }
    pub fn extract_icon_png_data_url(_: &str) -> Option<String> {
        None
    }
}

pub use win_impl::{
    extract_icon_png_data_url, get_idle_seconds, is_workstation_locked,
    probe_browser_omnibox_url, probe_foreground_window,
};
