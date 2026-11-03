//! Safe decoding of the fixed file-version record returned inside a Win32 version
//! block. The native adapter converts the query pointer to an offset; no memory is
//! ever read through that returned pointer, even if the query result is malformed.

/// VS_FIXEDFILEINFO is thirteen little-endian DWORDs. File version MS/LS are the
/// third and fourth fields; signature is the first. No Rust struct layout is read.
pub(crate) const FIXED_FILE_INFO_BYTES: usize = 13 * 4;

pub(crate) fn decode_fixed_version(
    block: &[u64],
    valid_bytes: usize,
    offset: usize,
    returned_bytes: usize,
) -> Option<String> {
    if valid_bytes > block.len().checked_mul(8)?
        || returned_bytes < FIXED_FILE_INFO_BYTES
        || offset.checked_add(returned_bytes)? > valid_bytes
    {
        return None;
    }
    let dword = |relative: usize| -> Option<u32> {
        let start = offset.checked_add(relative)?;
        let mut bytes = [0; 4];
        for (index, byte) in bytes.iter_mut().enumerate() {
            let position = start.checked_add(index)?;
            *byte = block.get(position / 8)?.to_ne_bytes()[position % 8];
        }
        Some(u32::from_le_bytes(bytes))
    };
    if dword(0)? != 0xfeef04bd {
        return None;
    }
    let ms = dword(8)?;
    let ls = dword(12)?;
    Some(format!(
        "{}.{}.{}.{}",
        ms >> 16,
        ms & 0xffff,
        ls >> 16,
        ls & 0xffff
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(offset: usize) -> (Vec<u64>, usize) {
        let size = offset + FIXED_FILE_INFO_BYTES;
        let mut bytes = vec![0; size.div_ceil(8) * 8];
        bytes[offset..offset + 4].copy_from_slice(&0xfeef04bdu32.to_le_bytes());
        bytes[offset + 8..offset + 12].copy_from_slice(&0x00020003u32.to_le_bytes());
        bytes[offset + 12..offset + 16].copy_from_slice(&0x00040005u32.to_le_bytes());
        let words = bytes
            .as_chunks::<8>()
            .0
            .iter()
            .map(|word| u64::from_ne_bytes(*word))
            .collect();
        (words, size)
    }

    #[test]
    fn decodes_version_at_aligned_and_unaligned_offsets() {
        for offset in [0, 3, 8, 31] {
            let (words, size) = block(offset);
            assert_eq!(
                decode_fixed_version(&words, size, offset, FIXED_FILE_INFO_BYTES).as_deref(),
                Some("2.3.4.5")
            );
        }
    }

    #[test]
    fn rejects_out_of_block_truncated_overflowing_and_bad_signature_results() {
        let (words, size) = block(3);
        for (valid, offset, returned) in [
            (size, 3, FIXED_FILE_INFO_BYTES - 1),
            (size - 1, 3, FIXED_FILE_INFO_BYTES),
            (size, size, FIXED_FILE_INFO_BYTES),
            (size, size + 1, FIXED_FILE_INFO_BYTES),
            (size, usize::MAX, FIXED_FILE_INFO_BYTES),
            (size, 3, usize::MAX),
            (words.len() * 8 + 1, 3, FIXED_FILE_INFO_BYTES),
        ] {
            assert_eq!(decode_fixed_version(&words, valid, offset, returned), None);
        }
        assert_eq!(decode_fixed_version(&[], 0, 0, FIXED_FILE_INFO_BYTES), None);
        assert_eq!(
            decode_fixed_version(&[0; 8], 64, 0, FIXED_FILE_INFO_BYTES),
            None
        );
    }
}
