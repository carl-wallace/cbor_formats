use std::fs::File;
use std::io::Read;
use std::path::Path;

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

#[allow(dead_code)]
pub fn get_file_as_byte_vec(filename: &Path) -> Vec<u8> {
    match File::open(filename) {
        Ok(mut f) => match std::fs::metadata(filename) {
            Ok(metadata) => {
                let mut buffer = vec![0; metadata.len() as usize];
                match f.read_exact(&mut buffer) {
                    Ok(_) => buffer,
                    Err(_e) => panic!(),
                }
            }
            Err(_e) => panic!(),
        },
        Err(_e) => panic!(),
    }
}

#[allow(dead_code)]
pub fn read_cbor(filename: &Option<String>) -> Vec<u8> {
    if let Some(filename) = filename {
        let p = Path::new(filename.as_str());
        if Path::exists(p) {
            return get_file_as_byte_vec(p);
        }
    }
    vec![]
}
