use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::value::Value;
use common::TextOrBinary;
use std::path::Path;

use eat::arrays::*;

mod utils;
use utils::*;

#[test]
fn detached_submodule_digest_test() {
    // todo!("detached_submodule_digest_test")
}

#[test]
fn dloa_type_test() {
    // todo!("dloa_type_test")
}

#[test]
fn hardware_version_type_test() {
    // todo!("hardware_version_type_test")
}

#[test]
fn individual_result_test() {
    // todo!("individual_result_test")
}

#[test]
fn manifest_spdx_test() {
    // CoAP Content-Formats for SPDX and CycloneDX have not been assigned yet. The values
    // 2112 and 2113 are assigned.

    // example1.spdx is from https://github.com/spdx/spdx-examples
    let spdx_bytes = get_file_as_byte_vec(Path::new(&"tests/examples/example1.spdx".to_string()));
    let spdx_str = core::str::from_utf8(spdx_bytes.as_slice())
        .unwrap()
        .to_string();
    let scratch = ManifestFormatCbor {
        content_type: 2112,
        content_format: TextOrBinary::Text(spdx_str.clone()),
    };

    let scratch_clone = scratch.clone();
    let _s = format!("{scratch:?}");

    assert_eq!(scratch_clone, scratch);
    assert!(scratch_clone == scratch);
    assert!(!(scratch_clone != scratch));

    let mut encoded_cbor = vec![];
    let _ = into_writer(&scratch, &mut encoded_cbor);

    let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
    let cbor_from_value: ManifestFormatCbor = ManifestFormatCbor::try_from(&value).unwrap();
    assert!(scratch == cbor_from_value);

    let mut busted = encoded_cbor.clone();
    busted[0] = 0xA2;
    let value: Result<ManifestFormatCbor, _> = from_reader(busted.clone().as_slice());
    assert!(value.is_err());

    let decoded: ManifestFormatCbor = from_reader(encoded_cbor.clone().as_slice()).unwrap();
    assert_eq!(decoded, scratch);
    assert!(scratch == decoded);
    match &decoded.content_format {
        TextOrBinary::Text(s) => assert_eq!(s, &spdx_str),
        _ => panic!(),
    }
    let mut roundtrip = vec![];
    let _ = into_writer(&decoded, &mut roundtrip);
    assert_eq!(roundtrip, encoded_cbor);

    let mut decoded2: ManifestFormatCbor = decoded.clone();
    decoded2.content_type = 1;
    assert_ne!(scratch, decoded2);

    {
        let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let cbor_from_value: ManifestFormatCbor = ManifestFormatCbor::try_from(&value).unwrap();
        let cbor_from_value2: ManifestFormatCbor = ManifestFormatCbor::try_from(value).unwrap();
        assert!(scratch == cbor_from_value);
        assert!(scratch == cbor_from_value2);
    }

    let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
    let value2 = &value.clone();
    let cbor_from_value2: ManifestFormatCbor = value.try_into().unwrap();
    let cbor_from_value: ManifestFormatCbor = value2.try_into().unwrap();
    assert!(scratch == cbor_from_value);
    assert!(scratch == cbor_from_value2);

    {
        let tt = &cbor_from_value2;
        let vec_value: Vec<Value> = tt.try_into().unwrap();
        let tt2: ManifestFormatCbor = vec_value.try_into().unwrap();
        assert_eq!(tt, &tt2);
    }
    {
        pub type TempType = Vec<Value>;
        let vec_value = TempType::try_from(&cbor_from_value2).unwrap();
        let tt2: ManifestFormatCbor = ManifestFormatCbor::try_from(vec_value).unwrap();
        assert_eq!(&cbor_from_value2, &tt2);
    }

    let json_from_cbor3: ManifestFormat = ManifestFormat::try_from(&decoded).unwrap();
    let tmp_cbor = &decoded.clone();
    let json_from_cbor2: ManifestFormat = ManifestFormat::try_from(tmp_cbor).unwrap();

    let json_from_cbor: ManifestFormat = decoded.try_into().unwrap();
    println!("STRUCT: {json_from_cbor:?}");
    assert_eq!(json_from_cbor, json_from_cbor2);
    assert_eq!(json_from_cbor, json_from_cbor3);
    let json_from_cbor_clone = json_from_cbor.clone();
    assert_eq!(json_from_cbor_clone, json_from_cbor);
    assert!(json_from_cbor_clone == json_from_cbor);
    let _s = format!("{json_from_cbor:?}");

    let encoded_json = serde_json::to_string(&json_from_cbor).unwrap();
    println!("JSON: {encoded_json:?}");
    let decoded_json: ManifestFormat = serde_json::from_str(encoded_json.as_str()).unwrap();
    println!("STRUCT 2: {decoded_json:?}");
    assert_eq!(decoded_json, json_from_cbor);

    let cbor_from_json: ManifestFormatCbor = json_from_cbor.try_into().unwrap();
    let tmp_json = decoded_json.clone();
    let _s = format!("{tmp_json:?}");

    assert_eq!(tmp_json, decoded_json);
    assert!(tmp_json == decoded_json);
    assert!(!(tmp_json != decoded_json));

    let cbor_from_json2: ManifestFormatCbor = ManifestFormatCbor::try_from(&tmp_json).unwrap();
    assert_eq!(cbor_from_json2, cbor_from_json);

    let cbor_clone = cbor_from_json.clone();
    assert_eq!(cbor_clone, cbor_from_json);
    assert!(cbor_clone == cbor_from_json);

    let mut encoded_cbor_from_json = vec![];
    let _ = into_writer(&cbor_from_json, &mut encoded_cbor_from_json);
    assert_eq!(encoded_cbor, encoded_cbor_from_json);
}

#[test]
fn manifest_spdx_as_binary_test() {
    // CoAP Content-Formats for SPDX and CycloneDX have not been assigned yet. The values
    // 2112 and 2113 are assigned.

    // example1.spdx is from https://github.com/spdx/spdx-examples
    let spdx_bytes = get_file_as_byte_vec(Path::new(&"tests/examples/example1.spdx".to_string()));
    let scratch = ManifestFormatCbor {
        content_type: 2112,
        content_format: TextOrBinary::Binary(spdx_bytes.clone()),
    };

    let scratch_clone = scratch.clone();
    let _s = format!("{scratch:?}");

    assert_eq!(scratch_clone, scratch);
    assert!(scratch_clone == scratch);
    assert!(!(scratch_clone != scratch));

    let mut encoded_cbor = vec![];
    let _ = into_writer(&scratch, &mut encoded_cbor);

    let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
    let cbor_from_value: ManifestFormatCbor = ManifestFormatCbor::try_from(&value).unwrap();
    assert!(scratch == cbor_from_value);

    let mut busted = encoded_cbor.clone();
    busted[0] = 0xA2;
    let value: Result<ManifestFormatCbor, _> = from_reader(busted.clone().as_slice());
    assert!(value.is_err());

    let decoded: ManifestFormatCbor = from_reader(encoded_cbor.clone().as_slice()).unwrap();
    assert_eq!(decoded, scratch);
    assert!(scratch == decoded);
    match &decoded.content_format {
        TextOrBinary::Binary(s) => assert_eq!(s, &spdx_bytes),
        _ => panic!(),
    }
    let mut roundtrip = vec![];
    let _ = into_writer(&decoded, &mut roundtrip);
    assert_eq!(roundtrip, encoded_cbor);

    let mut decoded2: ManifestFormatCbor = decoded.clone();
    decoded2.content_type = 1;
    assert_ne!(scratch, decoded2);

    {
        let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let cbor_from_value: ManifestFormatCbor = ManifestFormatCbor::try_from(&value).unwrap();
        let cbor_from_value2: ManifestFormatCbor = ManifestFormatCbor::try_from(value).unwrap();
        assert!(scratch == cbor_from_value);
        assert!(scratch == cbor_from_value2);
    }

    let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
    let value2 = &value.clone();
    let cbor_from_value2: ManifestFormatCbor = value.try_into().unwrap();
    let cbor_from_value: ManifestFormatCbor = value2.try_into().unwrap();
    assert!(scratch == cbor_from_value);
    assert!(scratch == cbor_from_value2);

    {
        let tt = &cbor_from_value2;
        let vec_value: Vec<Value> = tt.try_into().unwrap();
        let tt2: ManifestFormatCbor = vec_value.try_into().unwrap();
        assert_eq!(tt, &tt2);
    }
    {
        pub type TempType = Vec<Value>;
        let vec_value = TempType::try_from(&cbor_from_value2).unwrap();
        let tt2: ManifestFormatCbor = ManifestFormatCbor::try_from(vec_value).unwrap();
        assert_eq!(&cbor_from_value2, &tt2);
    }

    let json_from_cbor3: ManifestFormat = ManifestFormat::try_from(&decoded).unwrap();
    let tmp_cbor = &decoded.clone();
    let json_from_cbor2: ManifestFormat = ManifestFormat::try_from(tmp_cbor).unwrap();

    let json_from_cbor: ManifestFormat = decoded.try_into().unwrap();
    println!("STRUCT: {json_from_cbor:?}");
    assert_eq!(json_from_cbor, json_from_cbor2);
    assert_eq!(json_from_cbor, json_from_cbor3);
    let json_from_cbor_clone = json_from_cbor.clone();
    assert_eq!(json_from_cbor_clone, json_from_cbor);
    assert!(json_from_cbor_clone == json_from_cbor);
    let _s = format!("{json_from_cbor:?}");

    let encoded_json = serde_json::to_string(&json_from_cbor).unwrap();
    println!("JSON: {encoded_json:?}");
    let decoded_json: ManifestFormat = serde_json::from_str(encoded_json.as_str()).unwrap();
    println!("STRUCT 2: {decoded_json:?}");
    assert_eq!(decoded_json, json_from_cbor);

    let cbor_from_json: ManifestFormatCbor = json_from_cbor.try_into().unwrap();
    let tmp_json = decoded_json.clone();
    let _s = format!("{tmp_json:?}");

    assert_eq!(tmp_json, decoded_json);
    assert!(tmp_json == decoded_json);
    assert!(!(tmp_json != decoded_json));

    let cbor_from_json2: ManifestFormatCbor = ManifestFormatCbor::try_from(&tmp_json).unwrap();
    assert_eq!(cbor_from_json2, cbor_from_json);

    let cbor_clone = cbor_from_json.clone();
    assert_eq!(cbor_clone, cbor_from_json);
    assert!(cbor_clone == cbor_from_json);

    let mut encoded_cbor_from_json = vec![];
    let _ = into_writer(&cbor_from_json, &mut encoded_cbor_from_json);
    assert_eq!(encoded_cbor, encoded_cbor_from_json);
}

#[test]
fn mearurements_test() {
    let coswid_bytes = get_file_as_byte_vec(Path::new(&"tests/examples/coswid_1.cbor".to_string()));
    let scratch = MeasurementsFormatCbor {
        content_type: 258,
        content_format: TextOrBinary::Binary(coswid_bytes.clone()),
    };

    let scratch_clone = scratch.clone();
    let _s = format!("{scratch:?}");

    assert_eq!(scratch_clone, scratch);
    assert!(scratch_clone == scratch);
    assert!(!(scratch_clone != scratch));

    let mut encoded_cbor = vec![];
    let _ = into_writer(&scratch, &mut encoded_cbor);
    println!(
        "Encoded ClaimsSetClaims: {:?}",
        buffer_to_hex(encoded_cbor.as_slice())
    );

    let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
    let cbor_from_value: MeasurementsFormatCbor = MeasurementsFormatCbor::try_from(&value).unwrap();
    assert!(scratch == cbor_from_value);

    let mut busted = encoded_cbor.clone();
    busted[0] = 0xA2;
    let value: Result<MeasurementsFormatCbor, _> = from_reader(busted.clone().as_slice());
    assert!(value.is_err());

    let decoded: MeasurementsFormatCbor = from_reader(encoded_cbor.clone().as_slice()).unwrap();
    assert_eq!(decoded, scratch);
    assert!(scratch == decoded);
    match &decoded.content_format {
        TextOrBinary::Binary(s) => assert_eq!(s, &coswid_bytes),
        _ => panic!(),
    }
    let mut roundtrip = vec![];
    let _ = into_writer(&decoded, &mut roundtrip);
    assert_eq!(roundtrip, encoded_cbor);

    let mut decoded2: MeasurementsFormatCbor = decoded.clone();
    decoded2.content_type = 1;
    assert_ne!(scratch, decoded2);

    {
        let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let cbor_from_value: MeasurementsFormatCbor =
            MeasurementsFormatCbor::try_from(&value).unwrap();
        let cbor_from_value2: MeasurementsFormatCbor =
            MeasurementsFormatCbor::try_from(value).unwrap();
        assert!(scratch == cbor_from_value);
        assert!(scratch == cbor_from_value2);
    }

    let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
    let value2 = &value.clone();
    let cbor_from_value2: MeasurementsFormatCbor = value.try_into().unwrap();
    let cbor_from_value: MeasurementsFormatCbor = value2.try_into().unwrap();
    assert!(scratch == cbor_from_value);
    assert!(scratch == cbor_from_value2);

    {
        let tt = &cbor_from_value2;
        let vec_value: Vec<Value> = tt.try_into().unwrap();
        let tt2: MeasurementsFormatCbor = vec_value.try_into().unwrap();
        assert_eq!(tt, &tt2);
    }
    {
        pub type TempType = Vec<Value>;
        let vec_value = TempType::try_from(&cbor_from_value2).unwrap();
        let tt2: MeasurementsFormatCbor = MeasurementsFormatCbor::try_from(vec_value).unwrap();
        assert_eq!(&cbor_from_value2, &tt2);
    }

    let json_from_cbor3: MeasurementsFormat = MeasurementsFormat::try_from(&decoded).unwrap();
    let tmp_cbor = &decoded.clone();
    let json_from_cbor2: MeasurementsFormat = MeasurementsFormat::try_from(tmp_cbor).unwrap();

    let json_from_cbor: MeasurementsFormat = decoded.try_into().unwrap();
    println!("STRUCT: {json_from_cbor:?}");
    assert_eq!(json_from_cbor, json_from_cbor2);
    assert_eq!(json_from_cbor, json_from_cbor3);
    let json_from_cbor_clone = json_from_cbor.clone();
    assert_eq!(json_from_cbor_clone, json_from_cbor);
    assert!(json_from_cbor_clone == json_from_cbor);
    let _s = format!("{json_from_cbor:?}");

    let encoded_json = serde_json::to_string(&json_from_cbor).unwrap();
    println!("JSON: {encoded_json:?}");
    let decoded_json: MeasurementsFormat = serde_json::from_str(encoded_json.as_str()).unwrap();
    println!("STRUCT 2: {decoded_json:?}");
    assert_eq!(decoded_json, json_from_cbor);

    let cbor_from_json: MeasurementsFormatCbor = json_from_cbor.try_into().unwrap();
    let tmp_json = decoded_json.clone();
    let _s = format!("{tmp_json:?}");

    assert_eq!(tmp_json, decoded_json);
    assert!(tmp_json == decoded_json);
    assert!(!(tmp_json != decoded_json));

    let cbor_from_json2: MeasurementsFormatCbor =
        MeasurementsFormatCbor::try_from(&tmp_json).unwrap();
    assert_eq!(cbor_from_json2, cbor_from_json);

    let cbor_clone = cbor_from_json.clone();
    assert_eq!(cbor_clone, cbor_from_json);
    assert!(cbor_clone == cbor_from_json);

    let mut encoded_cbor_from_json = vec![];
    let _ = into_writer(&cbor_from_json, &mut encoded_cbor_from_json);
    assert_eq!(encoded_cbor, encoded_cbor_from_json);
}

#[test]
fn measurement_results_group_test() {
    // todo!("measurement_results_group_test")
}

#[test]
fn sw_version_type_test() {
    // todo!("sw_version_type_test")
}
