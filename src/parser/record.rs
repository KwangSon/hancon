use crate::common::{HwpError, HwpResult, read_u32_le};

/// HWP Record header (4 bytes)
/// Bits [0-9]:   tagid (0-1023)
/// Bits [10-19]: level (0-1023)
/// Bits [20-31]: size (0-4095)
/// If size == 4095: Extended size (4 bytes UINT32)
#[derive(Debug, Clone)]
pub struct RecordHeader {
    pub tagid: u16,
    pub level: u16,
    pub size: u32,
}

impl RecordHeader {
    pub fn parse(data: &[u8]) -> HwpResult<(Self, usize)> {
        if data.len() < 4 {
            return Err(HwpError::ParseError(
                "Record header must be at least 4 bytes".to_string(),
            ));
        }

        let header_u32 = read_u32_le(data, 0).ok_or(HwpError::ParseError(
            "Cannot read record header".to_string(),
        ))?;

        let tagid = (header_u32 & 0x3FF) as u16;
        let level = ((header_u32 >> 10) & 0x3FF) as u16;
        let size_bits = ((header_u32 >> 20) & 0xFFF) as u16;

        let (size, bytes_read) = if size_bits == 4095 {
            // Extended size
            if data.len() < 8 {
                return Err(HwpError::ParseError(
                    "Extended record size requires 8 bytes".to_string(),
                ));
            }
            let ext_size = read_u32_le(data, 4).ok_or(HwpError::ParseError(
                "Cannot read extended size".to_string(),
            ))?;
            (ext_size, 8)
        } else {
            (size_bits as u32, 4)
        };

        Ok((RecordHeader { tagid, level, size }, bytes_read))
    }
}

/// HWP Record
#[derive(Debug, Clone)]
pub struct Record {
    pub tagid: u16,
    pub tagname: String,
    pub level: u16,
    pub size: u32,
    pub payload: Vec<u8>,
}

/// HWP Record stream parser
pub struct RecordStream {
    data: Vec<u8>,
    pos: usize,
}

impl RecordStream {
    pub fn new(data: Vec<u8>) -> Self {
        RecordStream { data, pos: 0 }
    }

    pub fn next_record(&mut self) -> HwpResult<Option<Record>> {
        if self.pos >= self.data.len() {
            return Ok(None);
        }

        let (header, header_size) = RecordHeader::parse(&self.data[self.pos..])?;

        if self.pos + header_size + header.size as usize > self.data.len() {
            return Err(HwpError::ParseError(format!(
                "Record payload out of bounds: pos={}, header_size={}, size={}",
                self.pos, header_size, header.size
            )));
        }

        let payload_start = self.pos + header_size;
        let payload_end = payload_start + header.size as usize;
        let payload = self.data[payload_start..payload_end].to_vec();

        let tagname = format_tagname(header.tagid);

        self.pos = payload_end;

        Ok(Some(Record {
            tagid: header.tagid,
            tagname,
            level: header.level,
            size: header.size,
            payload,
        }))
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn position(&self) -> usize {
        self.pos
    }
}

impl Iterator for RecordStream {
    type Item = HwpResult<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_record() {
            Ok(Some(record)) => Some(Ok(record)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// Format HWP tag ID to tag name
fn format_tagname(tagid: u16) -> String {
    match tagid {
        // DocInfo tags
        0 => "HWP_ID".to_string(),
        3 => "HWPTAG_PARA_TEXT".to_string(),
        4 => "HWPTAG_PARA_CHAR_SHAPE".to_string(),
        5 => "HWPTAG_PARA_LINE_SEG".to_string(),
        6 => "HWPTAG_PARA_RANGE_TAG".to_string(),
        7 => "HWPTAG_CTRL_HEADER".to_string(),
        9 => "HWPTAG_LIST_HEADER".to_string(),
        10 => "HWPTAG_TABLE_CELL".to_string(),
        11 => "HWPTAG_SECTION_DEF".to_string(),
        16 => "HWPTAG_DOC_INFO".to_string(),
        17 => "HWPTAG_ID_MAPPINGS".to_string(),
        18 => "HWPTAG_BIN_DATA".to_string(),
        19 => "HWPTAG_FACE_NAME".to_string(),
        20 => "HWPTAG_BORDER_FILL".to_string(),
        21 => "HWPTAG_CHAR_SHAPE".to_string(),
        22 => "HWPTAG_TAB_DEF".to_string(),
        23 => "HWPTAG_NUMBERING".to_string(),
        24 => "HWPTAG_BULLET".to_string(),
        25 => "HWPTAG_PARA_SHAPE".to_string(),
        26 => "HWPTAG_STYLE".to_string(),
        27 => "HWPTAG_DOC_DATA".to_string(),
        28 => "HWPTAG_DISTRIBUTE_DOC_DATA".to_string(),
        29 => "HWPTAG_RESERVED".to_string(),
        30 => "HWPTAG_COMPATIBLE_DOC_DATA".to_string(),
        31 => "HWPTAG_LAYOUT_COMPATIBILITY".to_string(),

        // BodyText tags
        50 => "HWPTAG_PARAGRAPH".to_string(),
        51 => "HWPTAG_PARA_TEXT".to_string(),
        52 => "HWPTAG_PARA_CHAR_SHAPE".to_string(),
        53 => "HWPTAG_PARA_LINE_SEG".to_string(),
        54 => "HWPTAG_PARA_RANGE_TAG".to_string(),

        // Shape tags
        61 => "HWPTAG_TABLE".to_string(),
        62 => "HWPTAG_SHAPE_COMPONENT_RECT".to_string(),
        63 => "HWPTAG_SHAPE_COMPONENT_ELLIPSE".to_string(),
        64 => "HWPTAG_SHAPE_COMPONENT_ARC".to_string(),
        65 => "HWPTAG_SHAPE_COMPONENT_POLYGON".to_string(),
        66 => "HWPTAG_SHAPE_COMPONENT_CURVE".to_string(),
        67 => "HWPTAG_SHAPE_COMPONENT_OLE".to_string(),
        68 => "HWPTAG_SHAPE_COMPONENT_PICTURE".to_string(),
        69 => "HWPTAG_SHAPE_COMPONENT_CONTAINER".to_string(),
        70 => "HWPTAG_CTRL_DATA".to_string(),
        71 => "HWPTAG_EQEDIT".to_string(),
        72 => "HWPTAG_SHAPE_COMPONENT_TEXTBOX".to_string(),

        _ => format!("HWPTAG_{}", tagid),
    }
}
