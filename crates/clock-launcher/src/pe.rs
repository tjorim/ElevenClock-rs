//! Checks that a file is an x64 Windows program that names itself ElevenClock-rs.
//!
//! The bytes come from a file share an administrator controls, but are still treated
//! as untrusted: every read is bounds checked and a malformed file is simply
//! rejected. Nothing is loaded or executed; the PE headers and the `RT_VERSION`
//! resource are read straight from the bytes, so the check is portable and the same
//! on every host.

/// `ProductName` of the app and of the launcher.
pub const PRODUCT_NAME: &str = "ElevenClock-rs";

const MACHINE_AMD64: u16 = 0x8664;
const MAGIC_PE32_PLUS: u16 = 0x20b;
const RESOURCE_DIRECTORY_INDEX: usize = 2;
const RT_VERSION: u32 = 16;
/// A resource directory is a tree three levels deep; anything beyond is malformed.
const MAX_BLOCKS: usize = 256;

/// Why a file is not accepted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rejection {
    NotX64Program,
    NoVersionInfo,
    WrongProduct,
}
impl Rejection {
    /// A fixed phrase for logs: no file names, no paths.
    pub fn describe(self) -> &'static str {
        match self {
            Self::NotX64Program => "not an x64 Windows program",
            Self::NoVersionInfo => "no version information",
            Self::WrongProduct => "version information names another product",
        }
    }
}

/// Accepts `bytes` when they are an x64 PE file whose version information has
/// `ProductName` ElevenClock-rs and `OriginalFilename` `original_filename`. The file
/// name tells the app from the launcher, which share the product name.
pub fn validate(bytes: &[u8], original_filename: &str) -> Result<(), Rejection> {
    let resource = version_resource(bytes)?;
    let strings = version_strings(resource).ok_or(Rejection::NoVersionInfo)?;
    let find = |name: &str| {
        strings
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    };
    let product_ok = find("ProductName") == Some(PRODUCT_NAME);
    let name_ok =
        find("OriginalFilename").is_some_and(|value| value.eq_ignore_ascii_case(original_filename));
    if product_ok && name_ok {
        Ok(())
    } else {
        Err(Rejection::WrongProduct)
    }
}

fn u16_at(bytes: &[u8], at: usize) -> Option<u16> {
    let end = at.checked_add(2)?;
    Some(u16::from_le_bytes(bytes.get(at..end)?.try_into().ok()?))
}
fn u32_at(bytes: &[u8], at: usize) -> Option<u32> {
    let end = at.checked_add(4)?;
    Some(u32::from_le_bytes(bytes.get(at..end)?.try_into().ok()?))
}

struct Section {
    virtual_address: u32,
    virtual_size: u32,
    raw_offset: u32,
    raw_size: u32,
}

/// The raw bytes of the file's version resource.
fn version_resource(bytes: &[u8]) -> Result<&[u8], Rejection> {
    let (sections, directory_rva) = headers(bytes).ok_or(Rejection::NotX64Program)?;
    let to_offset = |rva: u32| -> Option<usize> {
        sections.iter().find_map(|section| {
            let span = section.virtual_size.max(section.raw_size);
            let inside = rva
                .checked_sub(section.virtual_address)
                .filter(|d| *d < span)?;
            // Bytes past the raw data would be zero-filled by the loader; not readable here.
            (inside < section.raw_size).then(|| (section.raw_offset as usize) + (inside as usize))
        })
    };
    let missing = Rejection::NoVersionInfo;
    let root = directory_rva
        .and_then(to_offset)
        .filter(|root| *root != 0)
        .ok_or(missing)?;
    let enter = |entry: u32, want_directory: bool| -> Option<usize> {
        let is_directory = entry & 0x8000_0000 != 0;
        (is_directory == want_directory).then(|| root + (entry & 0x7fff_ffff) as usize)
    };
    // Level 1 selects the type, level 2 the name, level 3 the language: take the
    // first entry of the last two, since a program carries one version resource.
    let first_entry = |directory: usize, wanted_id: Option<u32>| -> Option<u32> {
        let named = usize::from(u16_at(bytes, directory + 12)?);
        let ids = usize::from(u16_at(bytes, directory + 14)?);
        (0..named + ids).find_map(|index| {
            let entry = directory + 16 + index * 8;
            let id = u32_at(bytes, entry)?;
            let target = u32_at(bytes, entry + 4)?;
            wanted_id
                .is_none_or(|wanted| id == wanted)
                .then_some(target)
        })
    };
    let version_directory = first_entry(root, Some(RT_VERSION))
        .and_then(|entry| enter(entry, true))
        .ok_or(missing)?;
    let name_directory = first_entry(version_directory, None)
        .and_then(|entry| enter(entry, true))
        .ok_or(missing)?;
    let data_entry = first_entry(name_directory, None)
        .and_then(|entry| enter(entry, false))
        .ok_or(missing)?;
    let rva = u32_at(bytes, data_entry).ok_or(missing)?;
    let size = u32_at(bytes, data_entry + 4).ok_or(missing)? as usize;
    let start = to_offset(rva).ok_or(missing)?;
    bytes
        .get(start..start.checked_add(size).ok_or(missing)?)
        .ok_or(missing)
}

/// The section table and the resource directory's RVA of an x64 PE file.
fn headers(bytes: &[u8]) -> Option<(Vec<Section>, Option<u32>)> {
    if bytes.get(..2)? != b"MZ" {
        return None;
    }
    let pe = u32_at(bytes, 0x3c)? as usize;
    if bytes.get(pe..pe.checked_add(4)?)? != b"PE\0\0" {
        return None;
    }
    if u16_at(bytes, pe + 4)? != MACHINE_AMD64 {
        return None;
    }
    let section_count = usize::from(u16_at(bytes, pe + 6)?);
    let optional_size = usize::from(u16_at(bytes, pe + 20)?);
    let optional = pe + 24;
    if u16_at(bytes, optional)? != MAGIC_PE32_PLUS {
        return None;
    }
    let directory_count = u32_at(bytes, optional + 108)? as usize;
    let directory_rva = (directory_count > RESOURCE_DIRECTORY_INDEX)
        .then(|| u32_at(bytes, optional + 112 + RESOURCE_DIRECTORY_INDEX * 8))
        .flatten()
        .filter(|rva| *rva != 0);
    let table = optional.checked_add(optional_size)?;
    let mut sections = Vec::with_capacity(section_count.min(96));
    for index in 0..section_count.min(96) {
        let at = table + index * 40;
        sections.push(Section {
            virtual_size: u32_at(bytes, at + 8)?,
            virtual_address: u32_at(bytes, at + 12)?,
            raw_size: u32_at(bytes, at + 16)?,
            raw_offset: u32_at(bytes, at + 20)?,
        });
    }
    Some((sections, directory_rva))
}

/// One `VS_VERSIONINFO` style block: a key, a value and child blocks.
struct Block<'a> {
    key: String,
    value: &'a [u8],
    is_text: bool,
    children: &'a [u8],
}

fn align4(value: usize) -> usize {
    (value + 3) & !3
}

/// Reads the block at `at` and returns it with the offset of the next sibling.
fn block(data: &[u8], at: usize) -> Option<(Block<'_>, usize)> {
    let length = usize::from(u16_at(data, at)?);
    let value_length = usize::from(u16_at(data, at + 2)?);
    let is_text = u16_at(data, at + 4)? == 1;
    let end = at.checked_add(length)?;
    if length < 6 || end > data.len() {
        return None;
    }
    let mut units = Vec::new();
    let mut cursor = at + 6;
    loop {
        let unit = u16_at(data, cursor)?;
        cursor += 2;
        if cursor > end {
            return None;
        }
        if unit == 0 {
            break;
        }
        units.push(unit);
    }
    let value_start = align4(cursor - at) + at;
    let value_size = if is_text {
        value_length * 2
    } else {
        value_length
    };
    let value_end = value_start.checked_add(value_size)?;
    if value_end > end {
        return None;
    }
    let children_start = (align4(value_end - at) + at).min(end);
    Some((
        Block {
            key: String::from_utf16_lossy(&units),
            value: &data[value_start..value_end],
            is_text,
            children: &data[children_start..end],
        },
        align4(length) + at,
    ))
}

fn children(data: &[u8]) -> Vec<Block<'_>> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < data.len() && out.len() < MAX_BLOCKS {
        let Some((found, next)) = block(data, at) else {
            break;
        };
        out.push(found);
        at = next;
    }
    out
}

/// Every `key = text` pair of the file's string tables.
fn version_strings(resource: &[u8]) -> Option<Vec<(String, String)>> {
    let (root, _) = block(resource, 0)?;
    if root.key != "VS_VERSION_INFO" {
        return None;
    }
    let mut pairs = Vec::new();
    for info in children(root.children) {
        if info.key != "StringFileInfo" {
            continue;
        }
        for table in children(info.children) {
            for string in children(table.children) {
                if string.is_text {
                    let units: Vec<u16> = string
                        .value
                        .as_chunks::<2>()
                        .0
                        .iter()
                        .map(|pair| u16::from_le_bytes(*pair))
                        .take_while(|unit| *unit != 0)
                        .collect();
                    pairs.push((string.key, String::from_utf16_lossy(&units)));
                }
            }
        }
    }
    Some(pairs)
}

/// Builds minimal PE files for tests in this crate.
#[cfg(test)]
pub(crate) mod fixture {
    fn utf16(text: &str) -> Vec<u8> {
        text.encode_utf16()
            .chain(Some(0))
            .flat_map(u16::to_le_bytes)
            .collect()
    }
    fn pad4(bytes: &mut Vec<u8>) {
        while !bytes.len().is_multiple_of(4) {
            bytes.push(0);
        }
    }
    fn block(key: &str, value: &[u8], text: bool, children: &[u8]) -> Vec<u8> {
        let mut out = vec![0u8; 6];
        out.extend(utf16(key));
        pad4(&mut out);
        out.extend_from_slice(value);
        pad4(&mut out);
        out.extend_from_slice(children);
        let length = out.len() as u16;
        let value_length = if text { value.len() / 2 } else { value.len() } as u16;
        out[0..2].copy_from_slice(&length.to_le_bytes());
        out[2..4].copy_from_slice(&value_length.to_le_bytes());
        out[4..6].copy_from_slice(&u16::from(text).to_le_bytes());
        out
    }

    pub fn version_block(strings: &[(&str, &str)]) -> Vec<u8> {
        let mut entries = Vec::new();
        for (key, value) in strings {
            entries.extend(block(key, &utf16(value), true, &[]));
            pad4(&mut entries);
        }
        let mut table = block("040904B0", &[], true, &entries);
        pad4(&mut table);
        let mut info = block("StringFileInfo", &[], true, &table);
        pad4(&mut info);
        block("VS_VERSION_INFO", &[0u8; 52], false, &info)
    }

    /// A tiny PE file. `extra` makes otherwise identical programs differ (a new build).
    pub fn program(
        machine: u16,
        magic: u16,
        strings: Option<&[(&str, &str)]>,
        extra: &[u8],
    ) -> Vec<u8> {
        const RESOURCE_RVA: u32 = 0x1000;
        const RESOURCE_OFFSET: usize = 0x200;
        let version = strings.map(version_block).unwrap_or_default();
        let mut resource = Vec::new();
        // Directory levels: type 16 -> name 1 -> language 0x409 -> data entry.
        for (id, target) in [(16u32, 0x8000_0018u32), (1, 0x8000_0030), (0x409, 0x48)] {
            resource.extend([0u8; 12]);
            resource.extend(0u16.to_le_bytes());
            resource.extend(1u16.to_le_bytes());
            resource.extend(id.to_le_bytes());
            resource.extend(target.to_le_bytes());
        }
        resource.extend((RESOURCE_RVA + 0x58).to_le_bytes());
        resource.extend((version.len() as u32).to_le_bytes());
        resource.extend([0u8; 8]);
        assert_eq!(resource.len(), 0x58);
        resource.extend(&version);
        resource.extend(extra);

        let mut file = vec![0u8; RESOURCE_OFFSET];
        file[0..2].copy_from_slice(b"MZ");
        file[0x3c..0x40].copy_from_slice(&0x40u32.to_le_bytes());
        file[0x40..0x44].copy_from_slice(b"PE\0\0");
        file[0x44..0x46].copy_from_slice(&machine.to_le_bytes());
        file[0x46..0x48].copy_from_slice(&1u16.to_le_bytes());
        file[0x54..0x56].copy_from_slice(&240u16.to_le_bytes());
        let optional = 0x58;
        file[optional..optional + 2].copy_from_slice(&magic.to_le_bytes());
        file[optional + 108..optional + 112].copy_from_slice(&16u32.to_le_bytes());
        let directory = optional + 112 + 2 * 8;
        file[directory..directory + 4].copy_from_slice(&RESOURCE_RVA.to_le_bytes());
        file[directory + 4..directory + 8].copy_from_slice(&(resource.len() as u32).to_le_bytes());
        let section = optional + 240;
        file[section..section + 5].copy_from_slice(b".rsrc");
        file[section + 8..section + 12].copy_from_slice(&(resource.len() as u32).to_le_bytes());
        file[section + 12..section + 16].copy_from_slice(&RESOURCE_RVA.to_le_bytes());
        file[section + 16..section + 20].copy_from_slice(&(resource.len() as u32).to_le_bytes());
        file[section + 20..section + 24].copy_from_slice(&(RESOURCE_OFFSET as u32).to_le_bytes());
        file.extend(resource);
        file
    }

    pub const AMD64: u16 = 0x8664;
    pub const PE32_PLUS: u16 = 0x20b;

    pub fn app(build: &str) -> Vec<u8> {
        program(
            AMD64,
            PE32_PLUS,
            Some(&[
                ("ProductName", "ElevenClock-rs"),
                ("OriginalFilename", "elevenclock-rs.exe"),
            ]),
            build.as_bytes(),
        )
    }
    pub fn launcher(build: &str) -> Vec<u8> {
        program(
            AMD64,
            PE32_PLUS,
            Some(&[
                ("ProductName", "ElevenClock-rs"),
                ("OriginalFilename", "elevenclock-rs-launcher.exe"),
            ]),
            build.as_bytes(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::fixture::*;
    use super::*;

    #[test]
    fn the_app_and_the_launcher_each_pass_only_under_their_own_name() {
        assert_eq!(validate(&app("1"), "elevenclock-rs.exe"), Ok(()));
        assert_eq!(
            validate(&launcher("1"), "elevenclock-rs-launcher.exe"),
            Ok(())
        );
        assert_eq!(
            validate(&launcher("1"), "elevenclock-rs.exe"),
            Err(Rejection::WrongProduct)
        );
        assert_eq!(
            validate(&app("1"), "elevenclock-rs-launcher.exe"),
            Err(Rejection::WrongProduct)
        );
        // Windows file names ignore case.
        assert_eq!(validate(&app("1"), "ElevenClock-RS.exe"), Ok(()));
    }

    #[test]
    fn other_products_and_missing_strings_are_refused() {
        let other = program(
            AMD64,
            PE32_PLUS,
            Some(&[
                ("ProductName", "Something Else"),
                ("OriginalFilename", "elevenclock-rs.exe"),
            ]),
            b"",
        );
        assert_eq!(
            validate(&other, "elevenclock-rs.exe"),
            Err(Rejection::WrongProduct)
        );
        let nameless = program(
            AMD64,
            PE32_PLUS,
            Some(&[("ProductName", "ElevenClock-rs")]),
            b"",
        );
        assert_eq!(
            validate(&nameless, "elevenclock-rs.exe"),
            Err(Rejection::WrongProduct)
        );
        let none = program(AMD64, PE32_PLUS, None, b"");
        assert_eq!(
            validate(&none, "elevenclock-rs.exe"),
            Err(Rejection::NoVersionInfo)
        );
    }

    #[test]
    fn only_x64_pe_files_are_programs() {
        let strings: &[(&str, &str)] = &[
            ("ProductName", "ElevenClock-rs"),
            ("OriginalFilename", "elevenclock-rs.exe"),
        ];
        // x86 and ARM64 machines, and a 32-bit optional header.
        for (machine, magic) in [(0x14c, PE32_PLUS), (0xaa64, PE32_PLUS), (AMD64, 0x10b)] {
            assert_eq!(
                validate(
                    &program(machine, magic, Some(strings), b""),
                    "elevenclock-rs.exe"
                ),
                Err(Rejection::NotX64Program),
                "{machine:x} {magic:x}"
            );
        }
        for text in [&b""[..], b"MZ", b"not a program at all", &[0u8; 4096]] {
            assert_eq!(
                validate(text, "elevenclock-rs.exe"),
                Err(Rejection::NotX64Program)
            );
        }
    }

    #[test]
    fn every_truncation_is_rejected_without_a_panic() {
        let whole = app("build");
        for length in 0..whole.len() {
            let _ = validate(&whole[..length], "elevenclock-rs.exe");
        }
        // Damaged length fields must not read out of bounds either.
        for position in 0..whole.len().min(0x300) {
            let mut damaged = whole.clone();
            damaged[position] = 0xff;
            let _ = validate(&damaged, "elevenclock-rs.exe");
        }
    }
}
