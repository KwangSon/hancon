use crate::common::HwpResult;

/// Calculate CRC32 checksum
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg() & 0xEDB8_8320;
            crc = (crc >> 1) ^ mask;
        }
    }
    !crc
}

/// Push u16 little-endian to buffer
pub fn push_u16_le(buf: &mut Vec<u8>, value: u16) {
    buf.extend_from_slice(&value.to_le_bytes());
}

/// Push u32 little-endian to buffer
pub fn push_u32_le(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_le_bytes());
}

/// Write ZIP file with stored (uncompressed) entries
pub fn write_zip_stored(entries: &[(&str, Vec<u8>)]) -> HwpResult<Vec<u8>> {
    let mut out = Vec::new();
    let mut central = Vec::new();

    for &(name, ref data) in entries {
        let name_bytes = name.as_bytes();
        let offset = out.len() as u32;
        let size = data.len() as u32;
        let crc = crc32(data);

        // Local file header
        push_u32_le(&mut out, 0x0403_4B50);
        push_u16_le(&mut out, 20); // version needed
        push_u16_le(&mut out, 0); // flags
        push_u16_le(&mut out, 0); // method: stored (no compression)
        push_u16_le(&mut out, 0); // mtime
        push_u16_le(&mut out, 0); // mdate
        push_u32_le(&mut out, crc);
        push_u32_le(&mut out, size);
        push_u32_le(&mut out, size);
        push_u16_le(&mut out, name_bytes.len() as u16);
        push_u16_le(&mut out, 0); // extra length
        out.extend_from_slice(name_bytes);
        out.extend_from_slice(data);

        // Central directory header
        push_u32_le(&mut central, 0x0201_4B50);
        push_u16_le(&mut central, 20); // version made by
        push_u16_le(&mut central, 20); // version needed
        push_u16_le(&mut central, 0); // flags
        push_u16_le(&mut central, 0); // method
        push_u16_le(&mut central, 0); // mtime
        push_u16_le(&mut central, 0); // mdate
        push_u32_le(&mut central, crc);
        push_u32_le(&mut central, size);
        push_u32_le(&mut central, size);
        push_u16_le(&mut central, name_bytes.len() as u16);
        push_u16_le(&mut central, 0); // extra length
        push_u16_le(&mut central, 0); // comment length
        push_u16_le(&mut central, 0); // disk number
        push_u16_le(&mut central, 0); // internal attrs
        push_u32_le(&mut central, 0); // external attrs
        push_u32_le(&mut central, offset);
        central.extend_from_slice(name_bytes);
    }

    let central_offset = out.len() as u32;
    out.extend_from_slice(&central);
    let central_size = central.len() as u32;

    // EOCD
    push_u32_le(&mut out, 0x0605_4B50);
    push_u16_le(&mut out, 0); // disk number
    push_u16_le(&mut out, 0); // central start disk
    push_u16_le(&mut out, entries.len() as u16);
    push_u16_le(&mut out, entries.len() as u16);
    push_u32_le(&mut out, central_size);
    push_u32_le(&mut out, central_offset);
    push_u16_le(&mut out, 0); // comment length

    Ok(out)
}
