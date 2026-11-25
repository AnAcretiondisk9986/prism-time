mod coff_rsrc;
mod helpers;

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use coff_rsrc::{emit_coff_rsrc, IconInput};
use helpers::{escape_string, parse_cargo_toml};

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum VersionInfo {
    FILEVERSION,
    PRODUCTVERSION,
    FILEOS,
    FILETYPE,
    FILESUBTYPE,
    FILEFLAGSMASK,
    FILEFLAGS,
}

#[derive(Debug)]
struct Icon {
    path: String,
    name_id: String,
}

#[derive(Debug)]
pub struct WindowsResource {
    properties: BTreeMap<String, String>,
    version_info: BTreeMap<VersionInfo, u64>,
    rc_file: Option<String>,
    icons: Vec<Icon>,
    language: u16,
    manifest: Option<String>,
    manifest_file: Option<String>,
    append_rc_content: String,
}

#[allow(clippy::new_without_default)]
impl WindowsResource {
    pub fn new() -> Self {
        let mut props: BTreeMap<String, String> = BTreeMap::new();
        let mut ver: BTreeMap<VersionInfo, u64> = BTreeMap::new();

        props.insert(
            "FileVersion".to_string(),
            env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string()),
        );
        props.insert(
            "ProductVersion".to_string(),
            env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string()),
        );
        props.insert(
            "ProductName".to_string(),
            env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "app".to_string()),
        );
        let description = if let Ok(description) = env::var("CARGO_PKG_DESCRIPTION") {
            if !description.is_empty() {
                description
            } else {
                env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "app".to_string())
            }
        } else {
            env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "app".to_string())
        };
        props.insert("FileDescription".to_string(), description);

        let _ = parse_cargo_toml(&mut props);

        let mut version = 0_u64;
        version |= env::var("CARGO_PKG_VERSION_MAJOR")
            .unwrap_or_default()
            .parse()
            .unwrap_or(0)
            << 48;
        version |= env::var("CARGO_PKG_VERSION_MINOR")
            .unwrap_or_default()
            .parse()
            .unwrap_or(0)
            << 32;
        version |= env::var("CARGO_PKG_VERSION_PATCH")
            .unwrap_or_default()
            .parse()
            .unwrap_or(0)
            << 16;

        ver.insert(VersionInfo::FILEVERSION, version);
        ver.insert(VersionInfo::PRODUCTVERSION, version);
        ver.insert(VersionInfo::FILEOS, 0x00040004);
        ver.insert(VersionInfo::FILETYPE, 1);
        ver.insert(VersionInfo::FILESUBTYPE, 0);
        ver.insert(VersionInfo::FILEFLAGSMASK, 0x3F);
        ver.insert(VersionInfo::FILEFLAGS, 0);

        WindowsResource {
            properties: props,
            version_info: ver,
            rc_file: None,
            icons: Vec::new(),
            language: 0,
            manifest: None,
            manifest_file: None,
            append_rc_content: String::new(),
        }
    }

    pub fn set<'a>(&mut self, name: &'a str, value: &'a str) -> &mut Self {
        self.properties.insert(name.to_string(), value.to_string());
        self
    }

    pub fn set_language(&mut self, language: u16) -> &mut Self {
        self.language = language;
        self
    }

    pub fn set_icon(&mut self, path: &str) -> &mut Self {
        self.set_icon_with_id(path, "1")
    }

    pub fn set_icon_with_id<'a>(&mut self, path: &'a str, name_id: &'a str) -> &mut Self {
        self.icons.push(Icon {
            path: dunce::canonicalize(path)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| path.to_string()),
            name_id: name_id.into(),
        });
        self
    }

    pub fn set_version_info(&mut self, field: VersionInfo, value: u64) -> &mut Self {
        self.version_info.insert(field, value);
        self
    }

    pub fn set_manifest(&mut self, manifest: &str) -> &mut Self {
        self.manifest_file = None;
        self.manifest = Some(manifest.to_string());
        self
    }

    pub fn set_manifest_file(&mut self, file: &str) -> &mut Self {
        self.manifest_file = Some(file.to_string());
        self.manifest = None;
        self
    }

    pub fn write_resource_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut f = fs::File::create(path)?;
        writeln!(f, "#pragma code_page(65001)")?;
        writeln!(f, "1 VERSIONINFO")?;
        for (k, v) in self.version_info.iter() {
            match *k {
                VersionInfo::FILEVERSION | VersionInfo::PRODUCTVERSION => writeln!(
                    f,
                    "{:?} {}, {}, {}, {}",
                    k,
                    (*v >> 48) as u16,
                    (*v >> 32) as u16,
                    (*v >> 16) as u16,
                    *v as u16
                )?,
                _ => writeln!(f, "{:?} {:#x}", k, v)?,
            };
        }
        writeln!(f, "{{\nBLOCK \"StringFileInfo\"")?;
        writeln!(f, "{{\nBLOCK \"{:04x}04b0\"\n{{", self.language)?;
        for (k, v) in self.properties.iter() {
            if !v.is_empty() {
                writeln!(
                    f,
                    "VALUE \"{}\", \"{}\"",
                    escape_string(k),
                    escape_string(v)
                )?;
            }
        }
        writeln!(f, "}}\n}}")?;

        writeln!(f, "BLOCK \"VarFileInfo\" {{")?;
        writeln!(f, "VALUE \"Translation\", {:#x}, 0x04b0", self.language)?;
        writeln!(f, "}}\n}}")?;
        for icon in &self.icons {
            writeln!(
                f,
                "{} ICON \"{}\"",
                escape_string(&icon.name_id),
                escape_string(&icon.path)
            )?;
        }
        if let Some(e) = self.version_info.get(&VersionInfo::FILETYPE) {
            if let Some(manf) = self.manifest.as_ref() {
                writeln!(f, "{} 24", e)?;
                writeln!(f, "{{")?;
                for line in manf.lines() {
                    writeln!(f, "\" {} \"", escape_string(line.trim()))?;
                }
                writeln!(f, "}}")?;
            } else if let Some(manf) = self.manifest_file.as_ref() {
                writeln!(f, "{} 24 \"{}\"", e, escape_string(manf))?;
            }
        }
        writeln!(f, "{}", self.append_rc_content)?;
        Ok(())
    }

    pub fn set_resource_file(&mut self, path: &str) -> &mut Self {
        self.rc_file = Some(path.to_string());
        self
    }

    pub fn append_rc_content(&mut self, content: &str) -> &mut Self {
        if !(self.append_rc_content.ends_with('\n') || self.append_rc_content.is_empty()) {
            self.append_rc_content.push('\n');
        }
        self.append_rc_content.push_str(content);
        self
    }

    fn emit_fallback_coff(&self, output: &Path) -> io::Result<PathBuf> {
        let coff_path = output.join("tauri_rsrc.o");
        let icons: Vec<IconInput> = self
            .icons
            .iter()
            .map(|i| IconInput {
                path: i.path.clone(),
                name_id: i.name_id.clone(),
            })
            .collect();
        let manifest_id = self
            .version_info
            .get(&VersionInfo::FILETYPE)
            .copied()
            .unwrap_or(1) as u32;
        emit_coff_rsrc(
            &icons,
            self.manifest.as_deref(),
            self.manifest_file.as_deref(),
            self.language,
            manifest_id,
            &coff_path,
        )?;
        Ok(coff_path)
    }

    fn prepare_rc(&self, output: &Path) -> io::Result<PathBuf> {
        let rc = output.join("resource.rc");
        if let Some(s) = self.rc_file.as_ref() {
            fs::write(&rc, s)?;
        } else {
            self.write_resource_file(&rc)?;
        }
        Ok(rc)
    }

    pub fn compile(&self) -> io::Result<()> {
        let output = PathBuf::from(env::var("OUT_DIR").unwrap_or_else(|_| ".".to_string()));
        let rc = self.prepare_rc(&output)?;
        match embed_resource::compile(&rc, embed_resource::NONE) {
            embed_resource::CompilationResult::Ok
            | embed_resource::CompilationResult::NotWindows => Ok(()),
            embed_resource::CompilationResult::NotAttempted(_) => {
                let coff_path = self.emit_fallback_coff(&output)?;
                println!("cargo:rustc-link-arg={}", coff_path.display());
                Ok(())
            }
            embed_resource::CompilationResult::Failed(err) => {
                Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
            }
        }
    }

    pub fn compile_for(&self, binaries: &[&str]) -> io::Result<()> {
        let output = PathBuf::from(env::var("OUT_DIR").unwrap_or_else(|_| ".".to_string()));
        let rc = self.prepare_rc(&output)?;
        match embed_resource::compile_for(&rc, binaries, embed_resource::NONE) {
            embed_resource::CompilationResult::Ok
            | embed_resource::CompilationResult::NotWindows => Ok(()),
            embed_resource::CompilationResult::NotAttempted(_) => {
                let coff_path = self.emit_fallback_coff(&output)?;
                for bin in binaries {
                    println!("cargo:rustc-link-arg-bin={}={}", bin, coff_path.display());
                }
                Ok(())
            }
            embed_resource::CompilationResult::Failed(err) => {
                Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
            }
        }
    }

    pub fn compile_for_everything(&self) -> io::Result<()> {
        let output = PathBuf::from(env::var("OUT_DIR").unwrap_or_else(|_| ".".to_string()));
        let rc = self.prepare_rc(&output)?;
        match embed_resource::compile_for_everything(&rc, embed_resource::NONE) {
            embed_resource::CompilationResult::Ok
            | embed_resource::CompilationResult::NotWindows => Ok(()),
            embed_resource::CompilationResult::NotAttempted(_) => {
                let coff_path = self.emit_fallback_coff(&output)?;
                println!("cargo:rustc-link-arg={}", coff_path.display());
                Ok(())
            }
            embed_resource::CompilationResult::Failed(err) => {
                Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
            }
        }
    }
}

impl WindowsResource {
    #[deprecated(since = "0.1.1", note = "no-op")]
    pub fn set_toolkit_path(&mut self, _path: &str) -> &mut Self {
        self
    }
    #[deprecated(since = "0.1.1", note = "no-op")]
    pub fn set_windres_path(&mut self, _path: &str) -> &mut Self {
        self
    }
    #[deprecated(since = "0.1.1", note = "no-op")]
    pub fn set_ar_path(&mut self, _path: &str) -> &mut Self {
        self
    }
    #[deprecated(since = "0.1.1", note = "no-op")]
    pub fn add_toolkit_include(&mut self, _add: bool) -> &mut Self {
        self
    }
    #[deprecated(since = "0.1.1", note = "no-op")]
    pub fn set_output_directory(&mut self, _path: &str) -> &mut Self {
        self
    }
}
