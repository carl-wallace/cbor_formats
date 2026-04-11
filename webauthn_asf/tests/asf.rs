use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::value::Value;
use common::BytesType;
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

    // Verify fields are populated
    assert!(!ao.authData.is_empty());
    assert_eq!(ao.fmt, "apple-appattest");
    match &ao.attStmt {
        Supported::AppleAppAttest(apple) => {
            assert!(!apple.x5c.is_empty());
            assert!(!apple.receipt.is_empty());
        }
        Supported::Any(_) => panic!("Expected AppleAppAttest variant"),
    }

    // Roundtrip test
    let mut buf = vec![];
    into_writer(&ao, &mut buf).unwrap();
    let decoded: AttestationObject = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ao, decoded);
}

#[test]
fn apple_attestation_object_roundtrip() {
    let apple = AppleAttestationObject {
        x5c: vec![
            BytesType(vec![0x30, 0x82, 0x01]),
            BytesType(vec![0x30, 0x82, 0x02]),
        ],
        receipt: vec![0xAA, 0xBB, 0xCC],
    };
    let mut buf = vec![];
    into_writer(&apple, &mut buf).unwrap();
    let decoded: AppleAttestationObject = from_reader(buf.as_slice()).unwrap();
    assert_eq!(apple, decoded);
    assert_eq!(decoded.x5c.len(), 2);
    assert_eq!(decoded.receipt, vec![0xAA, 0xBB, 0xCC]);
}

#[test]
fn supported_any_variant() {
    // Test the Any variant with a generic attestation statement
    let any_stmt = Supported::Any(Value::Map(vec![(
        Value::Text("sig".to_string()),
        Value::Bytes(vec![0x01, 0x02]),
    )]));
    let ao = AttestationObject {
        authData: vec![0x00; 37],
        fmt: "packed".to_string(),
        attStmt: any_stmt,
    };
    let mut buf = vec![];
    into_writer(&ao, &mut buf).unwrap();
    let decoded: AttestationObject = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ao.fmt, decoded.fmt);
    assert_eq!(ao.authData, decoded.authData);
}

#[test]
fn attestation_object_3871_test() {
    let v = get_file_as_byte_vec(Path::new(
        "tests/examples/3871A9C2-B897-42BE-9619-3AC7035D1CE5.aa.bin",
    ));
    let ao: AttestationObject = from_reader(v.as_slice()).unwrap();

    assert!(!ao.authData.is_empty());
    assert_eq!(ao.fmt, "apple-appattest");
    match &ao.attStmt {
        Supported::AppleAppAttest(apple) => {
            assert!(!apple.x5c.is_empty());
            assert!(!apple.receipt.is_empty());
        }
        Supported::Any(_) => panic!("Expected AppleAppAttest variant"),
    }

    // Roundtrip test
    let mut buf = vec![];
    into_writer(&ao, &mut buf).unwrap();
    let decoded: AttestationObject = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ao, decoded);
}

#[test]
fn attestation_object_7212_test() {
    let v = get_file_as_byte_vec(Path::new(
        "tests/examples/72122602-24CE-414F-B103-257C9D1B2355.aa.bin",
    ));
    let ao: AttestationObject = from_reader(v.as_slice()).unwrap();

    assert!(!ao.authData.is_empty());
    assert_eq!(ao.fmt, "apple-appattest");
    match &ao.attStmt {
        Supported::AppleAppAttest(apple) => {
            assert!(!apple.x5c.is_empty());
            assert!(!apple.receipt.is_empty());
        }
        Supported::Any(_) => panic!("Expected AppleAppAttest variant"),
    }

    // Roundtrip test
    let mut buf = vec![];
    into_writer(&ao, &mut buf).unwrap();
    let decoded: AttestationObject = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ao, decoded);
}

#[test]
fn assertion_object_3871_test() {
    let v = get_file_as_byte_vec(Path::new(
        "tests/examples/3871A9C2-B897-42BE-9619-3AC7035D1CE5.aac.bin",
    ));
    let ao: AppleAssertionObject = from_reader(v.as_slice()).unwrap();

    assert!(!ao.signature.is_empty());
    assert!(!ao.authenticatorData.is_empty());

    // Roundtrip test
    let mut buf = vec![];
    into_writer(&ao, &mut buf).unwrap();
    let decoded: AppleAssertionObject = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ao, decoded);
}

#[test]
fn apple_assertion_object_roundtrip() {
    let assertion = AppleAssertionObject {
        signature: vec![0x30, 0x44, 0x02, 0x20],
        authenticatorData: vec![0x00; 37],
    };
    let mut buf = vec![];
    into_writer(&assertion, &mut buf).unwrap();
    let decoded: AppleAssertionObject = from_reader(buf.as_slice()).unwrap();
    assert_eq!(assertion, decoded);
}
