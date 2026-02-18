use crate::common::{HwpError, HwpResult, check_signature, read_u8, read_u16_le, read_u32_le};

/// OLE2 (Object Linking and Embedding) header structure
/// Standard Compound Document Format v3
#[derive(Debug, Clone)]
pub struct Ole2Header {
    pub signature: [u8; 8],
    pub minor_version: u16,
    pub major_version: u16,
    pub byte_order: u16, // 0xFFFE = little-endian
    pub sector_size_power: u16,
    pub mini_sector_size_power: u16,
    pub num_sectors: u32,
    pub num_fat_sectors: u32,
    pub first_dir_sector: u32,
    pub first_minifat_sector: u32,
    pub num_minifat_sectors: u32,
    pub fat: Vec<u32>, // FAT array (first 109 entries)
}

impl Ole2Header {
    pub fn parse(data: &[u8]) -> HwpResult<Self> {
        if data.len() < 512 {
            return Err(HwpError::InvalidFormat(
                "OLE2 header must be at least 512 bytes".to_string(),
            ));
        }

        // Check signature: D0CF11E0 A1B11AE1
        if !check_signature(data, 0, &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]) {
            return Err(HwpError::InvalidSignature);
        }

        let mut signature = [0u8; 8];
        signature.copy_from_slice(&data[0..8]);

        let minor_version = read_u16_le(data, 0x18).ok_or(HwpError::ParseError(
            "Cannot read minor version".to_string(),
        ))?;
        let major_version = read_u16_le(data, 0x1A).ok_or(HwpError::ParseError(
            "Cannot read major version".to_string(),
        ))?;
        let byte_order = read_u16_le(data, 0x1C)
            .ok_or(HwpError::ParseError("Cannot read byte order".to_string()))?;
        let sector_size_power = read_u16_le(data, 0x1E).ok_or(HwpError::ParseError(
            "Cannot read sector size power".to_string(),
        ))?;
        let mini_sector_size_power = read_u16_le(data, 0x20).ok_or(HwpError::ParseError(
            "Cannot read mini sector size power".to_string(),
        ))?;
        let num_sectors = read_u32_le(data, 0x30)
            .ok_or(HwpError::ParseError("Cannot read num sectors".to_string()))?;
        let num_fat_sectors = read_u32_le(data, 0x2C).ok_or(HwpError::ParseError(
            "Cannot read num FAT sectors".to_string(),
        ))?;
        let first_dir_sector = read_u32_le(data, 0x34).ok_or(HwpError::ParseError(
            "Cannot read first dir sector".to_string(),
        ))?;
        let first_minifat_sector = read_u32_le(data, 0x3C).ok_or(HwpError::ParseError(
            "Cannot read first minifat sector".to_string(),
        ))?;
        let num_minifat_sectors = read_u32_le(data, 0x40).ok_or(HwpError::ParseError(
            "Cannot read num minifat sectors".to_string(),
        ))?;

        // Read FAT array (first 109 entries at offset 0x4C)
        let mut fat = Vec::with_capacity(109);
        for i in 0..109 {
            if let Some(val) = read_u32_le(data, 0x4C + i * 4) {
                fat.push(val);
            } else {
                break;
            }
        }

        Ok(Ole2Header {
            signature,
            minor_version,
            major_version,
            byte_order,
            sector_size_power,
            mini_sector_size_power,
            num_sectors,
            num_fat_sectors,
            first_dir_sector,
            first_minifat_sector,
            num_minifat_sectors,
            fat,
        })
    }

    pub fn sector_size(&self) -> usize {
        1 << self.sector_size_power
    }

    pub fn mini_sector_size(&self) -> usize {
        1 << self.mini_sector_size_power
    }
}

/// OLE2 directory entry (64 bytes)
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub name_len: u16,
    pub entry_type: u8, // 1=storage, 2=stream, 3=root
    pub color: u8,      // 0=red, 1=black
    pub left_sibling: u32,
    pub right_sibling: u32,
    pub child: u32,
    pub start_sector: u32,
    pub stream_size: u32,
}

impl DirEntry {
    pub fn parse(data: &[u8]) -> HwpResult<Self> {
        if data.len() < 64 {
            return Err(HwpError::ParseError(
                "DirEntry must be 64 bytes".to_string(),
            ));
        }

        // Read UTF-16LE name (first 64 bytes, minus 4 for type/color/etc)
        let name_bytes = &data[0..64];
        let name_len = read_u16_le(data, 64).unwrap_or(0);

        let name = if name_len > 0 && name_len <= 32 {
            let utf16_str: Result<Vec<u16>, ()> = (0..name_len as usize - 1)
                .step_by(2)
                .map(|i| {
                    let b0 = name_bytes.get(i).copied().ok_or(())?;
                    let b1 = name_bytes.get(i + 1).copied().ok_or(())?;
                    Ok::<u16, ()>(u16::from_le_bytes([b0, b1]))
                })
                .collect();
            String::from_utf16_lossy(&utf16_str.unwrap_or_default()).to_string()
        } else {
            String::new()
        };

        let entry_type =
            read_u8(data, 66).ok_or(HwpError::ParseError("Cannot read entry type".to_string()))?;
        let color =
            read_u8(data, 67).ok_or(HwpError::ParseError("Cannot read color".to_string()))?;
        let left_sibling = read_u32_le(data, 68)
            .ok_or(HwpError::ParseError("Cannot read left sibling".to_string()))?;
        let right_sibling = read_u32_le(data, 72).ok_or(HwpError::ParseError(
            "Cannot read right sibling".to_string(),
        ))?;
        let child =
            read_u32_le(data, 76).ok_or(HwpError::ParseError("Cannot read child".to_string()))?;
        let start_sector = read_u32_le(data, 116)
            .ok_or(HwpError::ParseError("Cannot read start sector".to_string()))?;
        let stream_size = read_u32_le(data, 120)
            .ok_or(HwpError::ParseError("Cannot read stream size".to_string()))?;

        Ok(DirEntry {
            name,
            name_len,
            entry_type,
            color,
            left_sibling,
            right_sibling,
            child,
            start_sector,
            stream_size,
        })
    }
}

/// OLE2 container for reading streams
pub struct Ole2 {
    pub header: Ole2Header,
    pub data: Vec<u8>,
}

impl Ole2 {
    pub fn parse(data: Vec<u8>) -> HwpResult<Self> {
        let header = Ole2Header::parse(&data)?;
        Ok(Ole2 { header, data })
    }

    /// Read FAT chain starting from sector_id
    pub fn read_fat_chain(&self, mut sector_id: u32, max_sectors: usize) -> HwpResult<Vec<u8>> {
        let sector_size = self.header.sector_size();
        let mut result = Vec::new();
        let mut count = 0;

        while sector_id != 0xFFFFFFFE && count < max_sectors {
            let offset = 512 + (sector_id as usize) * sector_size;
            if offset + sector_size > self.data.len() {
                break;
            }
            result.extend_from_slice(&self.data[offset..offset + sector_size]);

            // Read next sector from FAT
            if sector_id < self.header.fat.len() as u32 {
                sector_id = self.header.fat[sector_id as usize];
            } else {
                break;
            }
            count += 1;
        }

        Ok(result)
    }

    /// Read a directory entry
    pub fn read_dir_entry(&self, entry_id: u32) -> HwpResult<DirEntry> {
        let sector_size = self.header.sector_size();
        let entries_per_sector = sector_size / 128;
        let sector_id =
            self.header.first_dir_sector + (entry_id as usize / entries_per_sector) as u32;
        let entry_offset = (entry_id as usize % entries_per_sector) * 128;

        let sector_data = self.read_fat_chain(sector_id, 1)?;
        if entry_offset + 128 > sector_data.len() {
            return Err(HwpError::ParseError(
                "Invalid directory entry offset".to_string(),
            ));
        }

        DirEntry::parse(&sector_data[entry_offset..entry_offset + 128])
    }

    /// List all streams (recursively from root)
    pub fn list_streams(&self) -> HwpResult<Vec<(String, DirEntry)>> {
        let mut result = Vec::new();
        let mut queue = vec![(String::new(), 0u32)]; // (path, entry_id)

        while let Some((path, entry_id)) = queue.pop() {
            let entry = self.read_dir_entry(entry_id)?;

            let current_path = if path.is_empty() {
                entry.name.clone()
            } else {
                format!("{}/{}", path, entry.name)
            };

            if entry.entry_type == 2 {
                // Stream
                result.push((current_path, entry.clone()));
            } else if entry.entry_type == 1 || entry.entry_type == 5 {
                // Storage (directory)
                if entry.child != 0xFFFFFFFF {
                    queue.push((current_path, entry.child));
                }
            }

            if entry.right_sibling != 0xFFFFFFFF {
                queue.push((path.clone(), entry.right_sibling));
            }
            if entry.left_sibling != 0xFFFFFFFF {
                queue.push((path, entry.left_sibling));
            }
        }

        Ok(result)
    }

    /// Get stream by name
    pub fn get_stream(&self, name: &str) -> HwpResult<Vec<u8>> {
        let streams = self.list_streams()?;
        for (stream_name, entry) in streams {
            if stream_name == name {
                return self.read_fat_chain(entry.start_sector, usize::MAX);
            }
        }
        Err(HwpError::NotFound(format!("Stream '{}' not found", name)))
    }
}
