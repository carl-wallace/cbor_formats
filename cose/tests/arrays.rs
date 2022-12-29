use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use cose::arrays::*;
use cose::maps::*;
use hex_literal::hex;
use std::path::Path;

mod utils;
use utils::*;

#[test]
fn untagged_signed_test() {
    // sample from TestSignedCorim_FromCOSE_ok test in corim project from https://github.com/veraison/corim
    let expected =
        get_file_as_byte_vec(Path::new(&"tests/examples/untagged_sign1.cbor".to_string()));
    println!(
        "Encoded signed CoRIM from veraison: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: CoseSign1Cbor = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded CoseSign1: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn tagged_signed_test() {
    // sample from TestSignedCorim_FromCOSE_ok test in corim project from https://github.com/veraison/corim
    let expected = get_file_as_byte_vec(Path::new(&"tests/examples/tagged_sign1.cbor".to_string()));
    println!(
        "Encoded signed CoRIM from veraison: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: TaggedCoseSign1 = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded CoseSign1: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn cosesign_test() {
    // sample from ecdsa-01 test in corim project from https://github.com/cose-wg/Examples
    let expected = get_file_as_byte_vec(Path::new(&"tests/examples/ecdsa-01.cbor".to_string()));
    println!(
        "Encoded TaggedCoseSign from cose-wg/Examples: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: TaggedCoseSign = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded TaggedCoseSign: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn cosesign1_test() {
    // sample from ecdsa-01 test in corim project from https://github.com/cose-wg/Examples
    let expected = get_file_as_byte_vec(Path::new(&"tests/examples/ecdsa-sig-01.cbor".to_string()));
    println!(
        "Encoded TaggedCoseSign1 from cose-wg/Examples: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: TaggedCoseSign1 = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded TaggedCoseSign1: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn header_map_test() {
    let expected = hex!("A104423131");
    println!(
        "Encoded HeaderMap from cose-wg/Examples: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: HeaderMapCbor = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded HeaderMap: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn cose_signature_test() {
    // sample from ecdsa-01 test in corim project from https://github.com/cose-wg/Examples
    let expected = hex!("8343A10126A1044231315840D71C05DB52C9CE7F1BF5AAC01334BBEACAC1D86A2303E6EEAA89266F45C01ED602CA649EAF790D8BC99D2458457CA6A872061940E7AFBE48E289DFAC146AE258");
    println!(
        "Encoded CoseSignatureCbor from cose-wg/Examples: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: CoseSignatureCbor = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded CoseSignatureCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn encrypt_test() {
    // sample from ecdsa-01 test in corim project from https://github.com/cose-wg/Examples
    let expected = get_file_as_byte_vec(Path::new(&"tests/examples/aes-ccm-01.cbor".to_string()));
    println!(
        "Encoded TaggedCoseEncrypt from cose-wg/Examples: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: TaggedCoseEncrypt = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded TaggedCoseEncrypt: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn encrypt0_test() {
    // sample from ecdsa-01 test in corim project from https://github.com/cose-wg/Examples
    let expected =
        get_file_as_byte_vec(Path::new(&"tests/examples/aes-ccm-enc-01.cbor".to_string()));
    println!(
        "Encoded TaggedCoseEncrypt from cose-wg/Examples: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: TaggedCoseEncrypt0 = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded TaggedCoseEncrypt: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn cosemac_test() {
    // sample from ecdsa-01 test in corim project from https://github.com/cose-wg/Examples
    let expected = get_file_as_byte_vec(Path::new(&"tests/examples/cbc-mac-01.cbor".to_string()));
    println!(
        "Encoded TaggedCoseMac from cose-wg/Examples: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: TaggedCoseMac = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded TaggedCoseMac: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn cosemac0_test() {
    // sample from ecdsa-01 test in corim project from https://github.com/cose-wg/Examples
    let expected =
        get_file_as_byte_vec(Path::new(&"tests/examples/cbc-mac-enc-01.cbor".to_string()));
    println!(
        "Encoded TaggedCoseMac from cose-wg/Examples: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: TaggedCoseMac0 = from_reader(expected.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded TaggedCoseMac: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);
}
