#[allow(dead_code)]
pub fn buffer_to_hex(buffer: &[u8]) -> String {
    let hex = subtle_encoding::hex::encode_upper(buffer);
    let r = std::str::from_utf8(hex.as_slice());
    if let Ok(s) = r {
        s.to_string()
    } else {
        "".to_string()
    }
}
