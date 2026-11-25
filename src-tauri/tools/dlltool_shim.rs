use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
const IMAGE_REL_AMD64_ADDR32NB: u16 = 0x0003;
const IMAGE_REL_AMD64_REL32: u16 = 0x0004;
const IMAGE_SYM_CLASS_EXTERNAL: u8 = 2;
const IMAGE_SYM_CLASS_STATIC: u8 = 3;

#[derive(Debug, Clone)]
struct ExportEntry {
    ext_name: String,
    imp_name: String,
    ordinal: Option<u16>,
    noname: bool,
    is_data: bool,
}

struct CoffSection {
    name: &'static str,
    data: Vec<u8>,
    relocs: Vec<(u32, u32, u16)>, // (va_offset, sym_idx, type)
    characteristics: u32,
}

struct CoffSymbol {
    name: String,
    value: u32,
    section_number: i16, // 1-based, 0 = external undefined
    sym_type: u16,
    storage_class: u8,
}

fn build_coff(sections: &[CoffSection], symbols: &[CoffSymbol]) -> Vec<u8> {
    let num_sec = sections.len() as u16;
    let num_sym = symbols.len() as u32;

    let mut data_offset = 20u32 + u32::from(num_sec) * 40;
    let mut sec_raw_offsets = Vec::with_capacity(sections.len());
    let mut sec_rel_offsets = Vec::with_capacity(sections.len());

    for sec in sections {
        if sec.data.is_empty() {
            sec_raw_offsets.push(0u32);
        } else {
            sec_raw_offsets.push(data_offset);
            data_offset += sec.data.len() as u32;
        }
        if sec.relocs.is_empty() {
            sec_rel_offsets.push(0u32);
        } else {
            sec_rel_offsets.push(data_offset);
            data_offset += (sec.relocs.len() as u32) * 10;
        }
    }

    let ptr_symtab = data_offset;
    let mut buf = Vec::with_capacity(ptr_symtab as usize + symbols.len() * 18 + 256);

    // IMAGE_FILE_HEADER (20 bytes)
    buf.extend_from_slice(&IMAGE_FILE_MACHINE_AMD64.to_le_bytes());
    buf.extend_from_slice(&num_sec.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes()); // TimeDateStamp
    buf.extend_from_slice(&ptr_symtab.to_le_bytes());
    buf.extend_from_slice(&num_sym.to_le_bytes());
    buf.extend_from_slice(&0u16.to_le_bytes()); // SizeOfOptionalHeader
    buf.extend_from_slice(&0u16.to_le_bytes()); // Characteristics

    // Section headers (40 bytes each)
    for (idx, sec) in sections.iter().enumerate() {
        let mut name8 = [0u8; 8];
        let nb = sec.name.as_bytes();
        let copy_len = nb.len().min(8);
        name8[..copy_len].copy_from_slice(&nb[..copy_len]);
        buf.extend_from_slice(&name8);
        buf.extend_from_slice(&0u32.to_le_bytes()); // VirtualSize
        buf.extend_from_slice(&0u32.to_le_bytes()); // VirtualAddress
        buf.extend_from_slice(&(sec.data.len() as u32).to_le_bytes());
        buf.extend_from_slice(&sec_raw_offsets[idx].to_le_bytes());
        buf.extend_from_slice(&sec_rel_offsets[idx].to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes()); // PointerToLinenumbers
        buf.extend_from_slice(&(sec.relocs.len() as u16).to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes()); // NumberOfLinenumbers
        buf.extend_from_slice(&sec.characteristics.to_le_bytes());
    }

    // Section raw data and relocations
    for sec in sections {
        buf.extend_from_slice(&sec.data);
        for &(va, sym_idx, rel_type) in &sec.relocs {
            buf.extend_from_slice(&va.to_le_bytes());
            buf.extend_from_slice(&sym_idx.to_le_bytes());
            buf.extend_from_slice(&rel_type.to_le_bytes());
        }
    }

    // Symbol table + string table
    let mut strtab = Vec::new();
    strtab.extend_from_slice(&0u32.to_le_bytes()); // placeholder for strtab size

    for sym in symbols {
        let sb = sym.name.as_bytes();
        if sb.len() <= 8 {
            let mut name8 = [0u8; 8];
            name8[..sb.len()].copy_from_slice(sb);
            buf.extend_from_slice(&name8);
        } else {
            let str_off = strtab.len() as u32;
            strtab.extend_from_slice(sb);
            strtab.push(0);
            buf.extend_from_slice(&0u32.to_le_bytes());
            buf.extend_from_slice(&str_off.to_le_bytes());
        }
        buf.extend_from_slice(&sym.value.to_le_bytes());
        buf.extend_from_slice(&sym.section_number.to_le_bytes());
        buf.extend_from_slice(&sym.sym_type.to_le_bytes());
        buf.push(sym.storage_class);
        buf.push(0u8); // NumberOfAuxSymbols
    }

    let strtab_len = strtab.len() as u32;
    strtab[0..4].copy_from_slice(&strtab_len.to_le_bytes());
    buf.extend_from_slice(&strtab);

    buf
}

fn fnv1a_64(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn parse_def(def_text: &str, cli_dll_name: Option<&str>) -> (String, Vec<ExportEntry>) {
    let mut dll_name = cli_dll_name
        .map(|s| s.trim().trim_matches('"').to_string())
        .unwrap_or_default();
    let mut exports = Vec::new();

    for raw_line in def_text.lines() {
        let no_comment = raw_line.split(';').next().unwrap_or("").trim();
        if no_comment.is_empty() {
            continue;
        }
        let upper = no_comment.to_ascii_uppercase();
        if upper == "EXPORTS" || upper.starts_with("EXPORTS ") {
            continue;
        }
        if let Some(rest) = upper
            .strip_prefix("LIBRARY")
            .or_else(|| upper.strip_prefix("NAME"))
        {
            if rest.is_empty() || rest.starts_with(char::is_whitespace) {
                if dll_name.is_empty() {
                    let orig_rest = no_comment[no_comment.len() - rest.len()..].trim();
                    if let Some(first) = orig_rest.split_whitespace().next() {
                        dll_name = first.trim_matches('"').to_string();
                    }
                }
                continue;
            }
        }
        if upper.starts_with("SECTIONS")
            || upper.starts_with("HEAPSIZE")
            || upper.starts_with("STACKSIZE")
            || upper.starts_with("VERSION")
        {
            continue;
        }

        // Tokenize trailing attributes (DATA, NONAME, PRIVATE, CONSTANT, @ordinal)
        let mut tokens: Vec<&str> = no_comment.split_whitespace().collect();
        let mut is_data = false;
        let mut noname = false;
        let mut ordinal: Option<u16> = None;

        while let Some(&last) = tokens.last() {
            let lu = last.to_ascii_uppercase();
            if lu == "DATA" {
                is_data = true;
                tokens.pop();
            } else if lu == "NONAME" {
                noname = true;
                tokens.pop();
            } else if lu == "PRIVATE" || lu == "CONSTANT" {
                tokens.pop();
            } else if let Some(ord_str) = last.strip_prefix('@') {
                if let Ok(ord) = ord_str.parse::<u16>() {
                    ordinal = Some(ord);
                    tokens.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if tokens.is_empty() {
            continue;
        }

        let joined = tokens.join(" ");
        let (ext_raw, imp_raw) = if let Some((l, r)) = joined.split_once("==") {
            (l, r)
        } else if let Some((l, r)) = joined.split_once('=') {
            (l, r)
        } else {
            (joined.as_str(), joined.as_str())
        };

        let ext_name = ext_raw.trim().trim_matches('"').to_string();
        let imp_name = imp_raw.trim().trim_matches('"').to_string();
        if ext_name.is_empty() || imp_name.is_empty() {
            continue;
        }

        exports.push(ExportEntry {
            ext_name,
            imp_name,
            ordinal,
            noname,
            is_data,
        });
    }

    if dll_name.is_empty() {
        dll_name = "unknown.dll".to_string();
    }

    (dll_name, exports)
}

fn build_head_member(head_sym: &str, iname_sym: &str) -> (Vec<u8>, Vec<String>) {
    let sections = vec![
        CoffSection {
            name: ".idata$2",
            data: vec![0u8; 20],
            relocs: vec![
                (0, 2, IMAGE_REL_AMD64_ADDR32NB),  // .idata$4
                (12, 4, IMAGE_REL_AMD64_ADDR32NB), // iname_sym
                (16, 1, IMAGE_REL_AMD64_ADDR32NB), // .idata$5
            ],
            characteristics: 0xC030_0040,
        },
        CoffSection {
            name: ".idata$5",
            data: Vec::new(),
            relocs: Vec::new(),
            characteristics: 0xC030_0040,
        },
        CoffSection {
            name: ".idata$4",
            data: Vec::new(),
            relocs: Vec::new(),
            characteristics: 0xC030_0040,
        },
    ];
    let symbols = vec![
        CoffSymbol {
            name: ".idata$2".to_string(),
            value: 0,
            section_number: 1,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: ".idata$5".to_string(),
            value: 0,
            section_number: 2,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: ".idata$4".to_string(),
            value: 0,
            section_number: 3,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: head_sym.to_string(),
            value: 0,
            section_number: 1,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_EXTERNAL,
        },
        CoffSymbol {
            name: iname_sym.to_string(),
            value: 0,
            section_number: 0,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_EXTERNAL,
        },
    ];
    (build_coff(&sections, &symbols), vec![head_sym.to_string()])
}

fn build_stub_member(exp: &ExportEntry, head_sym: &str) -> (Vec<u8>, Vec<String>) {
    let imp_sym = format!("__imp_{}", exp.ext_name);
    let by_ord = exp.noname && exp.ordinal.is_some();

    let text_data = if exp.is_data {
        Vec::new()
    } else {
        vec![0xFF, 0x25, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90]
    };
    let text_relocs = if exp.is_data {
        Vec::new()
    } else {
        vec![(2, 5, IMAGE_REL_AMD64_REL32)] // sym[5] = __imp_<ext>
    };

    let (thunk_data, thunk_relocs, idata6_data) = if by_ord {
        let ord_val = 0x8000_0000_0000_0000u64 | u64::from(exp.ordinal.unwrap());
        (ord_val.to_le_bytes().to_vec(), Vec::new(), Vec::new())
    } else {
        let mut hint_name = Vec::with_capacity(2 + exp.imp_name.len() + 2);
        hint_name.extend_from_slice(&0u16.to_le_bytes());
        hint_name.extend_from_slice(exp.imp_name.as_bytes());
        hint_name.push(0);
        if hint_name.len() % 2 != 0 {
            hint_name.push(0);
        }
        (
            vec![0u8; 8],
            vec![(0, 4, IMAGE_REL_AMD64_ADDR32NB)], // sym[4] = .idata$6
            hint_name,
        )
    };

    let sections = vec![
        CoffSection {
            name: ".text",
            data: text_data,
            relocs: text_relocs,
            characteristics: 0x6030_0020,
        },
        CoffSection {
            name: ".idata$7",
            data: vec![0u8; 4],
            relocs: vec![(0, 6, IMAGE_REL_AMD64_ADDR32NB)], // sym[6] = head_sym
            characteristics: 0xC030_0000,
        },
        CoffSection {
            name: ".idata$5",
            data: thunk_data.clone(),
            relocs: thunk_relocs.clone(),
            characteristics: 0xC030_0000,
        },
        CoffSection {
            name: ".idata$4",
            data: thunk_data,
            relocs: thunk_relocs,
            characteristics: 0xC030_0000,
        },
        CoffSection {
            name: ".idata$6",
            data: idata6_data,
            relocs: Vec::new(),
            characteristics: 0xC020_0000,
        },
    ];

    let mut symbols = vec![
        CoffSymbol {
            name: ".text".to_string(),
            value: 0,
            section_number: 1,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: ".idata$7".to_string(),
            value: 0,
            section_number: 2,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: ".idata$5".to_string(),
            value: 0,
            section_number: 3,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: ".idata$4".to_string(),
            value: 0,
            section_number: 4,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: ".idata$6".to_string(),
            value: 0,
            section_number: 5,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: imp_sym.clone(),
            value: 0,
            section_number: 3,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_EXTERNAL,
        },
        CoffSymbol {
            name: head_sym.to_string(),
            value: 0,
            section_number: 0,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_EXTERNAL,
        },
    ];

    let mut exported = vec![imp_sym];
    if !exp.is_data {
        symbols.push(CoffSymbol {
            name: exp.ext_name.clone(),
            value: 0,
            section_number: 1,
            sym_type: 0x20,
            storage_class: IMAGE_SYM_CLASS_EXTERNAL,
        });
        exported.push(exp.ext_name.clone());
    }

    (build_coff(&sections, &symbols), exported)
}

fn build_tail_member(dll_name: &str, iname_sym: &str) -> (Vec<u8>, Vec<String>) {
    let mut dll_bytes = dll_name.as_bytes().to_vec();
    dll_bytes.push(0);
    if dll_bytes.len() % 2 != 0 {
        dll_bytes.push(0);
    }

    let sections = vec![
        CoffSection {
            name: ".idata$4",
            data: vec![0u8; 8],
            relocs: Vec::new(),
            characteristics: 0xC030_0040,
        },
        CoffSection {
            name: ".idata$5",
            data: vec![0u8; 8],
            relocs: Vec::new(),
            characteristics: 0xC030_0040,
        },
        CoffSection {
            name: ".idata$7",
            data: dll_bytes,
            relocs: Vec::new(),
            characteristics: 0xC030_0040,
        },
    ];
    let symbols = vec![
        CoffSymbol {
            name: ".idata$4".to_string(),
            value: 0,
            section_number: 1,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: ".idata$5".to_string(),
            value: 0,
            section_number: 2,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: ".idata$7".to_string(),
            value: 0,
            section_number: 3,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_STATIC,
        },
        CoffSymbol {
            name: iname_sym.to_string(),
            value: 0,
            section_number: 3,
            sym_type: 0,
            storage_class: IMAGE_SYM_CLASS_EXTERNAL,
        },
    ];

    (build_coff(&sections, &symbols), vec![iname_sym.to_string()])
}

fn ar_header(name: &str, size: usize) -> [u8; 60] {
    let s = format!("{:<16}{:<12}{:<6}{:<6}{:<8}{:<10}`\n", name, "0", "0", "0", "100644", size);
    assert_eq!(s.len(), 60);
    let mut out = [0u8; 60];
    out.copy_from_slice(s.as_bytes());
    out
}

fn build_archive(members: Vec<(String, Vec<u8>, Vec<String>)>) -> Vec<u8> {
    let total_syms: usize = members.iter().map(|(_, _, s)| s.len()).sum();
    let str_bytes: usize = members
        .iter()
        .flat_map(|(_, _, syms)| syms.iter())
        .map(|s| s.len() + 1)
        .sum();
    let symtab_body_len = 4 + total_syms * 4 + str_bytes;
    let symtab_padded = symtab_body_len + (symtab_body_len % 2);

    let first_member_offset = 8 + 60 + symtab_padded;
    let mut member_offsets = Vec::with_capacity(members.len());
    let mut cursor = first_member_offset;
    for (_, body, _) in &members {
        member_offsets.push(cursor as u32);
        cursor += 60 + body.len() + (body.len() % 2);
    }

    let mut archive = Vec::with_capacity(cursor);
    archive.extend_from_slice(b"!<arch>\n");

    // "/" symbol directory member
    archive.extend_from_slice(&ar_header("/", symtab_body_len));
    archive.extend_from_slice(&(total_syms as u32).to_be_bytes());
    for (idx, (_, _, syms)) in members.iter().enumerate() {
        let off_be = member_offsets[idx].to_be_bytes();
        for _ in syms {
            archive.extend_from_slice(&off_be);
        }
    }
    for (_, _, syms) in &members {
        for sym in syms {
            archive.extend_from_slice(sym.as_bytes());
            archive.push(0);
        }
    }
    if symtab_body_len % 2 != 0 {
        archive.push(b'\n');
    }

    // Object members in strict lexical order: d000h.o/, d000sXXXXX.o/, d000t.o/
    for (name, body, _) in members {
        archive.extend_from_slice(&ar_header(&name, body.len()));
        archive.extend_from_slice(&body);
        if body.len() % 2 != 0 {
            archive.push(b'\n');
        }
    }

    archive
}

fn expand_args() -> Vec<String> {
    let mut out = Vec::new();
    for arg in env::args().skip(1) {
        if let Some(rsp_path) = arg.strip_prefix('@') {
            if let Ok(content) = fs::read_to_string(rsp_path) {
                for line in content.lines() {
                    let trimmed = line.trim().trim_matches('"');
                    if !trimmed.is_empty() {
                        out.push(trimmed.to_string());
                    }
                }
                continue;
            }
        }
        out.push(arg);
    }
    out
}

fn main() -> ExitCode {
    let args = expand_args();

    let mut def_path: Option<String> = None;
    let mut dll_name: Option<String> = None;
    let mut out_lib: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if a == "-d" || a == "--input-def" {
            if i + 1 < args.len() {
                def_path = Some(args[i + 1].clone());
                i += 2;
                continue;
            }
        } else if let Some(v) = a.strip_prefix("--input-def=") {
            def_path = Some(v.to_string());
        } else if a == "-D" || a == "--dllname" {
            if i + 1 < args.len() {
                dll_name = Some(args[i + 1].clone());
                i += 2;
                continue;
            }
        } else if let Some(v) = a.strip_prefix("--dllname=") {
            dll_name = Some(v.to_string());
        } else if a == "-l" || a == "--output-lib" {
            if i + 1 < args.len() {
                out_lib = Some(args[i + 1].clone());
                i += 2;
                continue;
            }
        } else if let Some(v) = a.strip_prefix("--output-lib=") {
            out_lib = Some(v.to_string());
        }
        i += 1;
    }

    let (Some(def_path), Some(out_lib)) = (def_path, out_lib) else {
        eprintln!("dlltool_shim: missing -d/--input-def or -l/--output-lib");
        return ExitCode::from(1);
    };

    let def_text = match fs::read_to_string(&def_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dlltool_shim: failed to read {def_path}: {e}");
            return ExitCode::from(1);
        }
    };

    let (resolved_dll, exports) = parse_def(&def_text, dll_name.as_deref());
    let h64 = fnv1a_64(&format!("{out_lib}|{resolved_dll}"));
    let uid = format!("imp_{:016x}", h64);
    let prefix = format!("i{:07x}", ((h64 ^ (h64 >> 32)) & 0x0fff_ffff) as u32);
    let head_sym = format!("_head_{uid}");
    let iname_sym = format!("_{uid}_iname");

    let mut members = Vec::with_capacity(exports.len() + 2);
    let (head_obj, head_syms) = build_head_member(&head_sym, &iname_sym);
    members.push((format!("{prefix}h.o/"), head_obj, head_syms));

    for (k, exp) in exports.iter().enumerate() {
        let (stub_obj, stub_syms) = build_stub_member(exp, &head_sym);
        members.push((format!("{prefix}s{k:04x}.o/"), stub_obj, stub_syms));
    }

    let (tail_obj, tail_syms) = build_tail_member(&resolved_dll, &iname_sym);
    members.push((format!("{prefix}t.o/"), tail_obj, tail_syms));

    let archive = build_archive(members);
    if let Some(parent) = Path::new(&out_lib).parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Err(e) = fs::write(&out_lib, &archive) {
        eprintln!("dlltool_shim: failed to write {out_lib}: {e}");
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}
