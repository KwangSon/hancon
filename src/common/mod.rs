pub mod error;
pub mod types;

pub use error::{HwpError, HwpResult};
pub use types::{Alignment, Color, HwpUnit, LineStyle, Margin, Position, Rect, Size, VAlignment};

/// Read u16 little-endian from bytes
pub fn read_u16_le(data: &[u8], offset: usize) -> Option<u16> {
    if offset + 2 <= data.len() {
        Some(u16::from_le_bytes([data[offset], data[offset + 1]]))
    } else {
        None
    }
}

/// Read u32 little-endian from bytes
pub fn read_u32_le(data: &[u8], offset: usize) -> Option<u32> {
    if offset + 4 <= data.len() {
        Some(u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]))
    } else {
        None
    }
}

/// Read i32 little-endian from bytes
pub fn read_i32_le(data: &[u8], offset: usize) -> Option<i32> {
    read_u32_le(data, offset).map(|v| v as i32)
}

/// Read u8 from bytes
pub fn read_u8(data: &[u8], offset: usize) -> Option<u8> {
    if offset < data.len() {
        Some(data[offset])
    } else {
        None
    }
}

/// Check if bytes match a signature at offset
pub fn check_signature(data: &[u8], offset: usize, sig: &[u8]) -> bool {
    if offset + sig.len() > data.len() {
        return false;
    }
    &data[offset..offset + sig.len()] == sig
}
