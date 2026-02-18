pub mod common;
pub mod converter;
pub mod format;
pub mod model;
pub mod parser;
pub mod writer;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConversionSuccess {
    pub data: Vec<u8>,
    pub message: String,
    pub warnings: Vec<String>,
}

/// Main conversion function: HWP/HWPX → ODT
pub fn validate_and_convert(file_data: &[u8]) -> Result<ConversionSuccess, String> {
    // Validate file is not empty
    if file_data.is_empty() {
        return Err("File data is empty".to_string());
    }

    // Detect file format
    let format = format::detect_format(file_data).map_err(|e| e.to_string())?;

    let mut warnings = vec![];

    // Check file size
    if file_data.len() > 10_000_000 {
        warnings.push("File size exceeds 10MB, conversion may be slow".to_string());
    }

    // Parse document
    let doc = match format {
        format::FileFormat::HWP => format::parse_hwp(file_data).map_err(|e| e.to_string())?,
        format::FileFormat::HWPX => format::parse_hwpx(file_data).map_err(|e| e.to_string())?,
    };

    if doc.sections.is_empty() {
        warnings.push("Document has no sections".to_string());
    }

    // Convert to ODT
    let converted_data = writer::generate_odt(&doc).map_err(|e| e.to_string())?;

    Ok(ConversionSuccess {
        data: converted_data,
        message: "Conversion completed successfully".to_string(),
        warnings,
    })
}

// WASM 바인딩 래퍼
#[wasm_bindgen]
pub fn convert_hwp_to_odt(file_data: &[u8]) -> Result<JsValue, String> {
    let result = validate_and_convert(file_data)?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_file() {
        let result = validate_and_convert(&[]);
        assert_eq!(result, Err("File data is empty".to_string()));
    }

    #[test]
    fn test_invalid_format() {
        let result = validate_and_convert(&[1, 2, 3, 4, 5]);
        assert!(result.is_err());
    }
}
