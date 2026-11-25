fn main() {
    println!("cargo:rerun-if-env-changed=PRISM_PAYLOAD_PATH");
    println!("cargo:rerun-if-changed=../icons/icon.ico");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = tauri_winres::WindowsResource::new();
        res.set_icon("../icons/icon.ico");
        res.set("ProductName", "澄镜 · PrismTime 安装程序");
        res.set("FileDescription", "澄镜 · PrismTime V0.3.0 Windows 单文件安装包");
        res.set("CompanyName", "PrismTime");
        res.set("LegalCopyright", "MIT License");
        res.set("FileVersion", "0.3.0.0");
        res.set("ProductVersion", "0.3.0.0");
        if let Err(e) = res.compile() {
            println!("cargo:warning=tauri-winres installer resource compile skipped: {e}");
        }
    }
}
