//! Pure-Rust zero-C-dependency HTTPS/HTTP client built directly on Windows WinHTTP.
//! Supports automatic Windows system proxy, TLS 1.2/1.3 via Schannel, and WebDAV/REST verbs.

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedUrl {
    pub is_https: bool,
    pub host: String,
    pub port: u16,
    pub path_and_query: String,
}

pub fn parse_http_url(raw: &str) -> Result<ParsedUrl, String> {
    let trimmed = raw.trim();
    let (is_https, rest) = if let Some(r) = trimmed.strip_prefix("https://") {
        (true, r)
    } else if let Some(r) = trimmed.strip_prefix("http://") {
        (false, r)
    } else {
        return Err("URL 必须以 https:// 或 http:// 开头".to_string());
    };

    let (authority, path_and_query) = match rest.find('/') {
        Some(idx) => (&rest[..idx], rest[idx..].to_string()),
        None => (rest, "/".to_string()),
    };

    if authority.is_empty() {
        return Err("URL 缺少主机名".to_string());
    }

    let default_port = if is_https { 443 } else { 80 };
    let (host, port) = if let Some(colon) = authority.rfind(':') {
        let maybe_port = &authority[colon + 1..];
        if let Ok(p) = maybe_port.parse::<u16>() {
            (authority[..colon].to_string(), p)
        } else {
            (authority.to_string(), default_port)
        }
    } else {
        (authority.to_string(), default_port)
    };

    if host.is_empty() {
        return Err("无效的主机地址".to_string());
    }

    Ok(ParsedUrl {
        is_https,
        host,
        port,
        path_and_query,
    })
}

#[cfg(windows)]
pub fn http_request(
    method: &str,
    url: &str,
    headers: &[(&str, String)],
    body: Option<&[u8]>,
) -> Result<HttpResponse, String> {
    use std::ffi::c_void;
    use windows::core::{PCWSTR, PWSTR};
    use windows::Win32::Networking::WinHttp::{
        WinHttpAddRequestHeaders, WinHttpCloseHandle, WinHttpConnect, WinHttpOpen,
        WinHttpOpenRequest, WinHttpQueryDataAvailable, WinHttpQueryHeaders, WinHttpReadData,
        WinHttpReceiveResponse, WinHttpSendRequest, WinHttpSetTimeouts,
        WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY, WINHTTP_ADDREQ_FLAG_ADD, WINHTTP_ADDREQ_FLAG_REPLACE,
        WINHTTP_FLAG_SECURE, WINHTTP_OPEN_REQUEST_FLAGS, WINHTTP_QUERY_FLAG_NUMBER,
        WINHTTP_QUERY_STATUS_CODE,
    };

    struct HandleGuard(*mut c_void);
    impl Drop for HandleGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    let _ = WinHttpCloseHandle(self.0);
                }
            }
        }
    }

    fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    let parsed = parse_http_url(url)?;
    let ua_w = to_wide("PrismTime/0.3.0");
    let host_w = to_wide(&parsed.host);
    let method_w = to_wide(method);
    let path_w = to_wide(&parsed.path_and_query);

    unsafe {
        let h_session = WinHttpOpen(
            PCWSTR(ua_w.as_ptr()),
            WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY,
            PCWSTR::null(),
            PCWSTR::null(),
            0,
        );
        if h_session.is_null() {
            return Err("初始化 Windows WinHTTP 会话失败".to_string());
        }
        let _session_guard = HandleGuard(h_session);

        let _ = WinHttpSetTimeouts(h_session, 10_000, 12_000, 20_000, 30_000);

        let h_connect = WinHttpConnect(h_session, PCWSTR(host_w.as_ptr()), parsed.port, 0);
        if h_connect.is_null() {
            return Err(format!("无法连接到主机 {}:{}", parsed.host, parsed.port));
        }
        let _connect_guard = HandleGuard(h_connect);

        let flags = if parsed.is_https {
            WINHTTP_FLAG_SECURE
        } else {
            WINHTTP_OPEN_REQUEST_FLAGS(0)
        };

        let h_req = WinHttpOpenRequest(
            h_connect,
            PCWSTR(method_w.as_ptr()),
            PCWSTR(path_w.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            std::ptr::null(),
            flags,
        );
        if h_req.is_null() {
            return Err(format!("创建 {method} 请求失败"));
        }
        let _req_guard = HandleGuard(h_req);

        for (k, v) in headers {
            let line = format!("{k}: {v}\r\n");
            let wide: Vec<u16> = line.encode_utf16().collect();
            let _ = WinHttpAddRequestHeaders(
                h_req,
                &wide,
                WINHTTP_ADDREQ_FLAG_ADD | WINHTTP_ADDREQ_FLAG_REPLACE,
            );
        }

        let (body_ptr, body_len) = match body {
            Some(bytes) if !bytes.is_empty() => (Some(bytes.as_ptr() as *const c_void), bytes.len() as u32),
            _ => (None, 0u32),
        };

        WinHttpSendRequest(h_req, None, body_ptr, body_len, body_len, 0)
            .map_err(|e| format!("网络请求发送失败（请检查网络/代理或服务器地址）: {e}"))?;

        WinHttpReceiveResponse(h_req, std::ptr::null_mut())
            .map_err(|e| format!("等待服务器响应失败: {e}"))?;

        let mut status_code: u32 = 0;
        let mut status_size: u32 = std::mem::size_of::<u32>() as u32;
        WinHttpQueryHeaders(
            h_req,
            WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
            PCWSTR::null(),
            Some(&mut status_code as *mut u32 as *mut c_void),
            &mut status_size,
            std::ptr::null_mut(),
        )
        .map_err(|e| format!("读取 HTTP 状态码失败: {e}"))?;

        let mut response_body = Vec::new();
        loop {
            let mut avail: u32 = 0;
            WinHttpQueryDataAvailable(h_req, &mut avail)
                .map_err(|e| format!("读取响应长度失败: {e}"))?;
            if avail == 0 {
                break;
            }
            let mut chunk = vec![0u8; avail as usize];
            let mut downloaded: u32 = 0;
            WinHttpReadData(
                h_req,
                chunk.as_mut_ptr() as *mut c_void,
                avail,
                &mut downloaded,
            )
            .map_err(|e| format!("读取响应数据失败: {e}"))?;
            if downloaded == 0 {
                break;
            }
            response_body.extend_from_slice(&chunk[..downloaded as usize]);
        }

        let _ = PWSTR::null(); // keep import clean if unused
        Ok(HttpResponse {
            status: status_code as u16,
            body: response_body,
        })
    }
}

#[cfg(not(windows))]
pub fn http_request(
    _method: &str,
    _url: &str,
    _headers: &[(&str, String)],
    _body: Option<&[u8]>,
) -> Result<HttpResponse, String> {
    Err("当前仅在 Windows 平台支持 WinHTTP 同步".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_standard_and_custom_port_urls() {
        let a = parse_http_url("https://dav.jianguoyun.com/dav/PrismTime/").unwrap();
        assert!(a.is_https);
        assert_eq!(a.host, "dav.jianguoyun.com");
        assert_eq!(a.port, 443);
        assert_eq!(a.path_and_query, "/dav/PrismTime/");

        let b = parse_http_url("http://192.168.1.10:5244/dav").unwrap();
        assert!(!b.is_https);
        assert_eq!(b.host, "192.168.1.10");
        assert_eq!(b.port, 5244);
        assert_eq!(b.path_and_query, "/dav");
    }
}
