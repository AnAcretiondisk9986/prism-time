use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::{self, Read},
    path::Path,
};

pub(crate) fn parse_cargo_toml(props: &mut BTreeMap<String, String>) -> io::Result<()> {
    let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") else {
        return Ok(());
    };
    let cargo = Path::new(&manifest_dir).join("Cargo.toml");
    let mut f = File::open(cargo)?;
    let mut cargo_toml = String::new();
    f.read_to_string(&mut cargo_toml)?;
    if let Ok(ml) = toml::from_str::<toml::Value>(&cargo_toml) {
        if let Some(pkg) = ml
            .get("package")
            .and_then(|p| p.get("metadata"))
            .and_then(|p| p.get("tauri-winres"))
            .and_then(|p| p.as_table())
        {
            for (k, v) in pkg {
                if let Some(v) = v.as_str() {
                    props.insert(k.clone(), v.to_string());
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn escape_string(string: &str) -> String {
    let mut escaped = String::new();
    for chr in string.chars() {
        match chr {
            '"' => escaped.push_str("\"\""),
            '\'' => escaped.push_str("\\'"),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            _ => escaped.push(chr),
        };
    }
    escaped
}
