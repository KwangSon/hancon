use crate::common::{HwpError, HwpResult, check_signature};
use crate::model::Document;
use crate::parser::{Ole2, RecordStream};

/// Detect file format (HWP or HWPX)
pub fn detect_format(data: &[u8]) -> HwpResult<FileFormat> {
    if data.len() < 4 {
        return Err(HwpError::InvalidFormat("File too small".to_string()));
    }

    // HWP: OLE2 signature (D0CF11E0 A1B11AE1)
    if check_signature(data, 0, &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]) {
        return Ok(FileFormat::HWP);
    }

    // HWPX: ZIP signature (504B0304 = PK..)
    if check_signature(data, 0, &[0x50, 0x4B, 0x03, 0x04]) {
        return Ok(FileFormat::HWPX);
    }

    Err(HwpError::InvalidFormat("Unknown file format".to_string()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    HWP,
    HWPX,
}

/// Parse HWP file into document model
pub fn parse_hwp(data: &[u8]) -> HwpResult<Document> {
    let ole2 = Ole2::parse(data.to_vec())?;

    // Get FileHeader to validate signature
    let _file_header = ole2.get_stream("FileHeader")?;

    let mut doc = Document::new();

    // Parse DocInfo (contains styles, fonts, etc.)
    if let Ok(docinfo_data) = ole2.get_stream("DocInfo") {
        parse_docinfo(&docinfo_data, &mut doc)?;
    }

    // Parse BodyText sections
    let mut section_idx = 0;
    loop {
        let stream_name = format!("BodyText/Section{}", section_idx);
        match ole2.get_stream(&stream_name) {
            Ok(bodytext_data) => {
                let section = parse_bodytext(&bodytext_data, &mut doc)?;
                doc.sections.push(section);
                section_idx += 1;
            }
            Err(_) => break,
        }
    }

    Ok(doc)
}

/// Parse HWPX file into document model
pub fn parse_hwpx(_data: &[u8]) -> HwpResult<Document> {
    // HWPX is a ZIP format with content.xml inside
    // For now, return a placeholder
    // Full implementation requires ZIP parsing
    Err(HwpError::ParseError(
        "HWPX parsing not yet implemented".to_string(),
    ))
}

/// Parse DocInfo stream
fn parse_docinfo(data: &[u8], doc: &mut Document) -> HwpResult<()> {
    let mut stream = RecordStream::new(data.to_vec());

    while let Some(record) = stream.next_record()? {
        match record.tagid {
            19 => {
                // HWPTAG_FACE_NAME
                let name = parse_face_name(&record.payload)?;
                doc.fonts.push(name);
            }
            21 => {
                // HWPTAG_CHAR_SHAPE
                let char_shape = parse_char_shape(&record.payload, doc.char_shapes.len() as u32)?;
                doc.char_shapes.push(char_shape);
            }
            25 => {
                // HWPTAG_PARA_SHAPE
                let para_shape = parse_para_shape(&record.payload, doc.para_shapes.len() as u32)?;
                doc.para_shapes.push(para_shape);
            }
            26 => {
                // HWPTAG_STYLE
                let style = parse_style(&record.payload, doc.styles.len() as u32)?;
                doc.styles.push(style);
            }
            20 => {
                // HWPTAG_BORDER_FILL
                let border_fill =
                    parse_border_fill(&record.payload, doc.border_fills.len() as u32)?;
                doc.border_fills.push(border_fill);
            }
            _ => {}
        }
    }

    Ok(())
}

/// Parse BodyText stream
fn parse_bodytext(data: &[u8], _doc: &mut Document) -> HwpResult<crate::model::Section> {
    let section = crate::model::Section::new();
    let mut stream = RecordStream::new(data.to_vec());

    while let Some(_record) = stream.next_record()? {
        // TODO: Parse paragraph, table, etc.
    }

    Ok(section)
}

/// Parse face name (글꼴 이름)
fn parse_face_name(payload: &[u8]) -> HwpResult<String> {
    if payload.is_empty() {
        return Ok(String::new());
    }

    // HWP face name is UTF-16LE encoded
    let utf16: Result<Vec<u16>, ()> = (0..payload.len())
        .step_by(2)
        .map(|i| {
            let b0 = payload.get(i).copied().ok_or(())?;
            let b1 = payload.get(i + 1).copied().ok_or(())?;
            Ok::<u16, ()>(u16::from_le_bytes([b0, b1]))
        })
        .collect();

    match utf16 {
        Ok(utf16_vec) => {
            let s = String::from_utf16_lossy(&utf16_vec);
            Ok(s.trim_end_matches('\0').to_string())
        }
        Err(_) => Ok(String::new()),
    }
}

/// Parse char shape (문자 모양)
fn parse_char_shape(_payload: &[u8], id: u32) -> HwpResult<crate::model::CharShape> {
    let char_shape = crate::model::CharShape::new(id);

    if _payload.len() >= 28 {
        // Parse basic char shape fields
        // font_id, font_size, attributes, color, etc.
        // This is simplified; full parsing requires understanding HWP binary format
    }

    Ok(char_shape)
}

/// Parse para shape (문단 모양)
fn parse_para_shape(_payload: &[u8], id: u32) -> HwpResult<crate::model::ParaShape> {
    let para_shape = crate::model::ParaShape::new(id);

    if !_payload.is_empty() {
        // Parse para shape fields (indent, spacing, alignment, etc.)
        // This is simplified; full parsing requires understanding HWP binary format
    }

    Ok(para_shape)
}

/// Parse style (스타일)
fn parse_style(_payload: &[u8], id: u32) -> HwpResult<crate::model::Style> {
    let style = crate::model::Style {
        id,
        name: String::new(),
        style_type: crate::model::StyleType::Paragraph,
        parent_id: 0,
        char_shape_id: 0,
        para_shape_id: 0,
    };

    Ok(style)
}

/// Parse border fill (테두리/배경)
fn parse_border_fill(_payload: &[u8], id: u32) -> HwpResult<crate::model::BorderFill> {
    let border_fill = crate::model::BorderFill {
        id,
        left: crate::model::Border::none(),
        right: crate::model::Border::none(),
        top: crate::model::Border::none(),
        bottom: crate::model::Border::none(),
        diagonal: crate::model::Border::none(),
        fill_type: crate::model::FillType::None,
        fill_color: crate::common::Color(0),
        background_color: crate::common::Color(0xFFFFFF),
    };

    Ok(border_fill)
}
