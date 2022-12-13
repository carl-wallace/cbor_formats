use ciborium::de::from_reader;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use webauthn_asf::*;

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

#[test]
fn attestation_object_test() {
    let v = get_file_as_byte_vec(Path::new(
        "tests/examples/9c_device_cert.scep.attestation.cbor",
    ));
    let ao: AttestationObject = from_reader(v.as_slice()).unwrap();
    println!("{:?}", ao);
}
