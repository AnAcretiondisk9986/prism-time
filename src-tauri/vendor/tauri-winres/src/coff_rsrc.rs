use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 0x0000_0040;
const IMAGE_SCN_MEM_READ: u32 = 0x4000_0000;
const IMAGE_REL_AMD64_ADDR32NB: u16 = 0x0003;
const IMAGE_SYM_CLASS_STATIC: u8 = 3;

const RT_ICON: u32 = 3;
const RT_GROUP_ICON: u32 = 14;
const RT_MANIFEST: u32 = 24;

#[derive(Debug, Clone)]
pub(crate) struct IconInput {
    pub path: String,
    pub name_id: String,
}

type ResourceTree = BTreeMap<u32, BTreeMap<u32, BTreeMap<u32, Vec<u8>>>>;

pub(crate) fn emit_coff_rsrc(
    icons: &[IconInput],
    manifest: Option<&str>,
    manifest_file: Option<&str>,
    language: u16,
    manifest_id: u32,
    out_path: &Path,
) -> io::Result<()> {
    let mut tree: ResourceTree = BTreeMap::new();
    let lang = u32::from(language);

    let mut next_icon_id: u32 = 1;
    for icon in icons {
        let ico_bytes = fs::read(&icon.path)?;
        let (group_payload, frames) = parse_ico_for_resources(&ico_bytes, &mut next_icon_id)?;
        for (icon_id, frame_bytes) in frames {
            tree.entry(RT_ICON)
                .or_default()
                .entry(icon_id)
                .or_default()
                .insert(lang, frame_bytes);
        }
        let group_id = icon.name_id.parse::<u32>().unwrap_or(1);
        tree.entry(RT_GROUP_ICON)
            .or_default()
            .entry(group_id)
            .or_default()
            .insert(lang, group_payload);
    }

    if let Some(man_str) = manifest {
        tree.entry(RT_MANIFEST)
            .or_default()
            .entry(manifest_id)
            .or_default()
            .insert(lang, man_str.as_bytes().to_vec());
    } else if let Some(man_path) = manifest_file {
        let bytes = fs::read(man_path)?;
        tree.entry(RT_MANIFEST)
            .or_default()
            .entry(manifest_id)
            .or_default()
            .insert(lang, bytes);
    }

    let obj = build_coff_object(&tree);
    fs::write(out_path, obj)
}

fn parse_ico_for_resources(
    ico: &[u8],
    next_icon_id: &mut u32,
) -> io::Result<(Vec<u8>, Vec<(u32, Vec<u8>)>)> {
    if ico.len() < 6 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ICO header too short",
        ));
    }
    let reserved = u16::from_le_bytes([ico[0], ico[1]]);
    let ico_type = u16::from_le_bytes([ico[2], ico[3]]);
    let count = u16::from_le_bytes([ico[4], ico[5]]) as usize;
    if reserved != 0 || ico_type != 1 || count == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid ICO file header",
        ));
    }
    let dir_end = 6 + count * 16;
    if ico.len() < dir_end {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ICO directory entries truncated",
        ));
    }

    let mut group = Vec::with_capacity(6 + count * 14);
    group.extend_from_slice(&0u16.to_le_bytes());
    group.extend_from_slice(&1u16.to_le_bytes());
    group.extend_from_slice(&(count as u16).to_le_bytes());

    let mut frames = Vec::with_capacity(count);
    for i in 0..count {
        let e = &ico[6 + i * 16..6 + (i + 1) * 16];
        let bytes_in_res = u32::from_le_bytes([e[8], e[9], e[10], e[11]]) as usize;
        let image_offset = u32::from_le_bytes([e[12], e[13], e[14], e[15]]) as usize;
        if image_offset + bytes_in_res > ico.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ICO frame offset out of bounds",
            ));
        }

        let icon_id = *next_icon_id;
        *next_icon_id += 1;

        group.extend_from_slice(&e[0..12]);
        group.extend_from_slice(&(icon_id as u16).to_le_bytes());
        frames.push((icon_id, ico[image_offset..image_offset + bytes_in_res].to_vec()));
    }

    Ok((group, frames))
}

fn align4(val: usize) -> usize {
    (val + 3) & !3
}

fn push_dir_header(buf: &mut Vec<u8>, id_entries: u16) {
    buf.extend_from_slice(&0u32.to_le_bytes()); // Characteristics
    buf.extend_from_slice(&0u32.to_le_bytes()); // TimeDateStamp
    buf.extend_from_slice(&0u16.to_le_bytes()); // MajorVersion
    buf.extend_from_slice(&0u16.to_le_bytes()); // MinorVersion
    buf.extend_from_slice(&0u16.to_le_bytes()); // NumberOfNamedEntries
    buf.extend_from_slice(&id_entries.to_le_bytes()); // NumberOfIdEntries
}

pub(crate) fn build_coff_object(tree: &ResourceTree) -> Vec<u8> {
    let num_types = tree.len();
    let num_names: usize = tree.values().map(|m| m.len()).sum();
    let num_leaves: usize = tree
        .values()
        .flat_map(|m| m.values())
        .map(|l| l.len())
        .sum();

    // Compute exact section offsets for the 3-level resource directory tree
    let root_dir_off = 0usize;
    let root_dir_size = 16 + num_types * 8;

    let mut level2_offsets = Vec::with_capacity(num_types);
    let mut cursor = root_dir_off + root_dir_size;
    for names in tree.values() {
        level2_offsets.push(cursor);
        cursor += 16 + names.len() * 8;
    }

    let mut level3_offsets = Vec::with_capacity(num_names);
    for names in tree.values() {
        for langs in names.values() {
            level3_offsets.push(cursor);
            cursor += 16 + langs.len() * 8;
        }
    }

    let data_entries_start = cursor;
    let payloads_start = align4(data_entries_start + num_leaves * 16);

    let mut payload_offsets = Vec::with_capacity(num_leaves);
    let mut p_cursor = payloads_start;
    for names in tree.values() {
        for langs in names.values() {
            for payload in langs.values() {
                payload_offsets.push(p_cursor);
                p_cursor = align4(p_cursor + payload.len());
            }
        }
    }
    let rsrc_size = align4(p_cursor);

    let mut rsrc = Vec::with_capacity(rsrc_size);

    // Level 1 directory
    push_dir_header(&mut rsrc, num_types as u16);
    for (idx, type_id) in tree.keys().enumerate() {
        rsrc.extend_from_slice(&type_id.to_le_bytes());
        rsrc.extend_from_slice(&(0x8000_0000u32 | (level2_offsets[idx] as u32)).to_le_bytes());
    }

    // Level 2 directories
    let mut l3_idx = 0usize;
    for names in tree.values() {
        push_dir_header(&mut rsrc, names.len() as u16);
        for name_id in names.keys() {
            rsrc.extend_from_slice(&name_id.to_le_bytes());
            rsrc.extend_from_slice(
                &(0x8000_0000u32 | (level3_offsets[l3_idx] as u32)).to_le_bytes(),
            );
            l3_idx += 1;
        }
    }

    // Level 3 directories
    let mut leaf_idx = 0usize;
    for names in tree.values() {
        for langs in names.values() {
            push_dir_header(&mut rsrc, langs.len() as u16);
            for lang_id in langs.keys() {
                let data_entry_off = data_entries_start + leaf_idx * 16;
                rsrc.extend_from_slice(&lang_id.to_le_bytes());
                rsrc.extend_from_slice(&(data_entry_off as u32).to_le_bytes());
                leaf_idx += 1;
            }
        }
    }

    // IMAGE_RESOURCE_DATA_ENTRY table + COFF relocations
    let mut relocs: Vec<(u32, u32, u16)> = Vec::with_capacity(num_leaves);
    let mut leaf_idx2 = 0usize;
    for names in tree.values() {
        for langs in names.values() {
            for payload in langs.values() {
                let data_entry_off = (data_entries_start + leaf_idx2 * 16) as u32;
                let payload_off = payload_offsets[leaf_idx2] as u32;
                relocs.push((data_entry_off, 0, IMAGE_REL_AMD64_ADDR32NB));

                rsrc.extend_from_slice(&payload_off.to_le_bytes()); // OffsetToData (relocated via ADDR32NB)
                rsrc.extend_from_slice(&(payload.len() as u32).to_le_bytes()); // Size
                rsrc.extend_from_slice(&65001u32.to_le_bytes()); // CodePage UTF-8
                rsrc.extend_from_slice(&0u32.to_le_bytes()); // Reserved
                leaf_idx2 += 1;
            }
        }
    }

    while rsrc.len() < payloads_start {
        rsrc.push(0);
    }

    for names in tree.values() {
        for langs in names.values() {
            for payload in langs.values() {
                rsrc.extend_from_slice(payload);
                while rsrc.len() % 4 != 0 {
                    rsrc.push(0);
                }
            }
        }
    }
    while rsrc.len() < rsrc_size {
        rsrc.push(0);
    }

    // Assemble x86_64 COFF object
    let ptr_to_raw = 20u32 + 40u32;
    let ptr_to_relocs = ptr_to_raw + (rsrc.len() as u32);
    let ptr_to_symtab = ptr_to_relocs + ((relocs.len() * 10) as u32);

    let mut coff = Vec::with_capacity(ptr_to_symtab as usize + 18 + 4);

    // IMAGE_FILE_HEADER (20 bytes)
    coff.extend_from_slice(&IMAGE_FILE_MACHINE_AMD64.to_le_bytes());
    coff.extend_from_slice(&1u16.to_le_bytes()); // NumberOfSections
    coff.extend_from_slice(&0u32.to_le_bytes()); // TimeDateStamp
    coff.extend_from_slice(&ptr_to_symtab.to_le_bytes());
    coff.extend_from_slice(&1u32.to_le_bytes()); // NumberOfSymbols
    coff.extend_from_slice(&0u16.to_le_bytes()); // SizeOfOptionalHeader
    coff.extend_from_slice(&0u16.to_le_bytes()); // Characteristics

    // IMAGE_SECTION_HEADER (40 bytes)
    coff.extend_from_slice(b".rsrc\0\0\0");
    coff.extend_from_slice(&0u32.to_le_bytes()); // VirtualSize
    coff.extend_from_slice(&0u32.to_le_bytes()); // VirtualAddress
    coff.extend_from_slice(&(rsrc.len() as u32).to_le_bytes()); // SizeOfRawData
    coff.extend_from_slice(&ptr_to_raw.to_le_bytes()); // PointerToRawData
    coff.extend_from_slice(&ptr_to_relocs.to_le_bytes()); // PointerToRelocations
    coff.extend_from_slice(&0u32.to_le_bytes()); // PointerToLinenumbers
    coff.extend_from_slice(&(relocs.len() as u16).to_le_bytes()); // NumberOfRelocations
    coff.extend_from_slice(&0u16.to_le_bytes()); // NumberOfLinenumbers
    coff.extend_from_slice(&(IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ).to_le_bytes());

    // Section raw bytes
    coff.extend_from_slice(&rsrc);

    // Relocation entries (10 bytes each)
    for (va, sym_idx, rel_type) in relocs {
        coff.extend_from_slice(&va.to_le_bytes());
        coff.extend_from_slice(&sym_idx.to_le_bytes());
        coff.extend_from_slice(&rel_type.to_le_bytes());
    }

    // Symbol table entry 0: .rsrc static section symbol (18 bytes)
    coff.extend_from_slice(b".rsrc\0\0\0");
    coff.extend_from_slice(&0u32.to_le_bytes()); // Value
    coff.extend_from_slice(&1i16.to_le_bytes()); // SectionNumber (1-based)
    coff.extend_from_slice(&0u16.to_le_bytes()); // Type
    coff.push(IMAGE_SYM_CLASS_STATIC);
    coff.push(0u8); // NumberOfAuxSymbols

    // Empty COFF string table (4-byte size)
    coff.extend_from_slice(&4u32.to_le_bytes());

    coff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coff_object_header_and_relocs_valid() {
        let mut tree: ResourceTree = BTreeMap::new();
        tree.entry(RT_MANIFEST)
            .or_default()
            .entry(1)
            .or_default()
            .insert(0, b"<assembly/>".to_vec());
        let obj = build_coff_object(&tree);
        assert_eq!(u16::from_le_bytes([obj[0], obj[1]]), 0x8664);
        assert_eq!(u16::from_le_bytes([obj[2], obj[3]]), 1);
        assert_eq!(&obj[20..25], b".rsrc");
        let n_relocs = u16::from_le_bytes([obj[52], obj[53]]);
        assert_eq!(n_relocs, 1);
        let ptr_sym = u32::from_le_bytes([obj[8], obj[9], obj[10], obj[11]]) as usize;
        assert_eq!(&obj[ptr_sym..ptr_sym + 5], b".rsrc");
        assert_eq!(u32::from_le_bytes([obj[obj.len() - 4], obj[obj.len() - 3], obj[obj.len() - 2], obj[obj.len() - 1]]), 4);
    }
}
