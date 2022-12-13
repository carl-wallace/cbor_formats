use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use cots::arrays::{ConciseTaStoresCbor, TrustAnchorCbor};
use hex_literal::hex;

use cots::maps::*;
mod utils;
use crate::utils::*;

#[test]
fn abbreviated_swid_tag_test() {
    let expected = hex!("a102a2181f715a657374792048616e64732c20496e632e182102");
    let egl_d: AbbreviatedSwidTagCbor = from_reader(expected.clone().as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn cas_and_tas_test() {
    //A100818202585B3059301306072A8648CE3D020106082A8648CE3D03010703420004AD8A0C01DA9EDA0253DC2BC27227D9C7213DF8DF13E89CB9CDB7A8E4B62D9CE8A99A2D705C0F7F80DB65C006D1091422B47FC611CBD46869733D9C483884D5FE
    let expected = hex!("A100818202585B3059301306072A8648CE3D020106082A8648CE3D03010703420004AD8A0C01DA9EDA0253DC2BC27227D9C7213DF8DF13E89CB9CDB7A8E4B62D9CE8A99A2D705C0F7F80DB65C006D1091422B47FC611CBD46869733D9C483884D5FE");
    let egl_d: CasAndTasMapCbor = from_reader(expected.clone().as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn concise_ta_store_test() {
    let files = vec![
        "./tests/examples/tas1.cbor",
        "./tests/examples/tas2.cbor",
        "./tests/examples/tas3.cbor",
    ];
    for f in files {
        let expected = read_cbor(&Some(f.to_string()));
        let csc_d: ConciseTaStoreMapCbor = from_reader(expected.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);

        // convert to JSON-friendly struct, encode as JSON, and decode from JSON
        let meta_j: ConciseTaStoreMap = csc_d.clone().try_into().unwrap();
        let json = serde_json::to_string(&meta_j).unwrap();
        let dec_meta_j: ConciseTaStoreMap = serde_json::from_str(json.as_str()).unwrap();

        // encode as CBOR (with textual map keys)
        let mut enc_with_text_keys = vec![];
        let _ = into_writer(&dec_meta_j, &mut enc_with_text_keys);

        // convert to CBOR-friendly struct, encode as CBOR (with integer map keys), then compare with expected
        let roundtrip: ConciseTaStoreMapCbor = dec_meta_j.try_into().unwrap();
        assert_eq!(csc_d, roundtrip);
        let mut actual = vec![];
        let _ = into_writer(&roundtrip, &mut actual);
        assert_eq!(encoded_token, actual);
    }

    let invalid = vec![
        "./tests/examples/tas1_invalid.cbor", // outer tag is an array instead of map
        "./tests/examples/tas2_invalid.cbor", // second tag is an array instead of map
        "./tests/examples/tas3_invalid.cbor", // incorrect length for text field
    ];
    for f in invalid {
        let expected = read_cbor(&Some(f.to_string()));
        let csc_d: Result<ConciseTaStoreMapCbor, _> = from_reader(expected.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn cots_test() {
    let expected = read_cbor(&Some("./tests/examples/cots_trunc.cbor".to_string()));

    let csc_d: ConciseTaStoresCbor = from_reader(expected.clone().as_slice()).unwrap();
    //println!("Decoded ConciseMidTag: {:?}", comid_d);
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded ConciseTaStoresCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    //assert_eq!(expected.to_vec(), encoded_token);

    // let csc_json: ConciseTaStoreMap = csc_d.try_into().unwrap();
    // println!("{}", serde_json::to_string(&csc_json).unwrap());
    //
    // let mut encoded_token2 = vec![];
    // let _ = into_writer(&csc_json, &mut encoded_token2);
    // println!(
    //     "Encoded ClaimsSetClaims with string keys: {:?}",
    //     buffer_to_hex(encoded_token2.as_slice())
    // );
    //
    // let csc_cbor: ConciseTaStoresCbor = csc_json.try_into().unwrap();
    // let mut encoded_token3 = vec![];
    // let _ = into_writer(&csc_cbor, &mut encoded_token3);
    // //assert_eq!(expected.to_vec(), encoded_token3);
    // println!(
    //     "Re-encoded ClaimsSetClaims with integer keys: {:?}",
    //     buffer_to_hex(encoded_token3.as_slice())
    // );
}

#[test]
fn env_group_list_map_test() {
    //a1036d536f6d652054412053746f7265
    let expected = hex!("a1036d536f6d652054412053746f7265");
    let egl_d: EnvironmentGroupListMapCbor = from_reader(expected.clone().as_slice()).unwrap();
    println!("Decoded EnvironmentGroupListMapCbor: {:?}", egl_d);
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    println!(
        "Encoded EnvironmentGroupListMapCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
}
#[test]
fn env_group_list_map_test2() {
    let expected = hex!("a101a100a100d86f442a030405");
    let egl_d: EnvironmentGroupListMapCbor = from_reader(expected.clone().as_slice()).unwrap();
    println!("Decoded EnvironmentGroupListMapCbor: {:?}", egl_d);
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    println!(
        "Encoded EnvironmentGroupListMapCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
}

#[test]
fn trust_anchor_test() {
    //8202585B3059301306072A8648CE3D020106082A8648CE3D03010703420004AD8A0C01DA9EDA0253DC2BC27227D9C7213DF8DF13E89CB9CDB7A8E4B62D9CE8A99A2D705C0F7F80DB65C006D1091422B47FC611CBD46869733D9C483884D5FE
    let expected = hex!("8202585B3059301306072A8648CE3D020106082A8648CE3D03010703420004AD8A0C01DA9EDA0253DC2BC27227D9C7213DF8DF13E89CB9CDB7A8E4B62D9CE8A99A2D705C0F7F80DB65C006D1091422B47FC611CBD46869733D9C483884D5FE");
    let egl_d: TrustAnchorCbor = from_reader(expected.clone().as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    assert_eq!(expected.to_vec(), encoded_token);
}
