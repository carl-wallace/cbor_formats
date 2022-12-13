use hex_literal::hex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use uuid::{uuid, Uuid};

lazy_static! {
    pub static ref TEST_UUID: Uuid = uuid!("31fb5abf-023e-4992-aa4e-95f9c1503bfa");
    pub static ref TEST_UEID: Vec<u8> = hex!("02deadbeefdead").to_vec();
    pub static ref TEST_IMPL_ID: Vec<u8> =
        hex!("61636d652d696d706c656d656e746174696f6e2d69642d303030303030303031").to_vec();
}
//TestUUID       = UUID(uuid.Must(uuid.Parse(TestUUIDString)))

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
