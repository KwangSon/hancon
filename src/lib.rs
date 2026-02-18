use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)] // Added PartialEq
pub struct ConversionSuccess {
    pub data: Vec<u8>,
    pub message: String,
    pub warnings: Vec<String>,
}

// 테스트 가능한 핵심 로직
pub fn validate_and_convert(file_data: &[u8]) -> Result<ConversionSuccess, String> {
    // Validate file is not empty
    if file_data.is_empty() {
        return Err("File data is empty".to_string());
    }

    // Validate HWP file signature
    if file_data.len() < 3 || &file_data[0..3] != b"HWP" {
        return Err("Invalid HWP file signature".to_string());
    }

    let mut warnings = vec![];

    // Example: Check file size and add warning
    if file_data.len() > 10_000_000 {
        warnings.push("File size exceeds 10MB, conversion may be slow".to_string());
    }

    // TODO: Implement actual HWP to ODT conversion logic here
    let converted_data = file_data.to_vec();

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
    fn test_valid_hwp() {
        let hwp_data = b"HWP\x00\x01\x02";
        let result = validate_and_convert(hwp_data);
        assert!(result.is_ok());
        let success = result.unwrap();
        assert_eq!(success.message, "Conversion completed successfully");
    }

    #[test]
    fn test_empty_file() {
        let result = validate_and_convert(&[]);
        assert_eq!(result, Err("File data is empty".to_string()));
    }

    #[test]
    fn test_invalid_header() {
        let result = validate_and_convert(&[1, 2, 3]);
        assert_eq!(result, Err("Invalid HWP file signature".to_string()));
    }
}
