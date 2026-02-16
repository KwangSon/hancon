use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_hwp_to_odt(file_data: &[u8]) -> Vec<u8> {
    // 1. 여기서 나중에 진짜 변환 로직이 들어갈 예정입니다.
    // 2. 지금은 테스트를 위해 받은 데이터를 그대로 반환(clone)합니다.

    let result = file_data.to_vec();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = convert_hwp_to_odt(&[1, 2, 3]);
        assert_eq!(result, vec![1, 2, 3]);
    }
}
