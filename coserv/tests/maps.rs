use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use coserv::maps::*;
use hex_literal::hex;

mod utils;
use crate::utils::*;

// Test vectors from https://github.com/veraison/coserv-rs/tree/main/testdata

// {
//   0: "tag:example.com,2025:cc-platform#1.0.0",
//   1: {
//     0: 2,
//     1: {
//       0: [ [
//         {
//           0: 560(h'00112233'),
//           1: "Example Vendor",
//           2: "Example Model"
//         }
//       ] ]
//     },
//     2: 0
//   }
// }
#[test]
fn example_class_selector_decode() {
    let expected = read_cbor(&Some(
        "./tests/examples/example-class-selector.cbor".to_string(),
    ));
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();
    println!("Decoded CoservMapCbor: {:?}", decoded);

    // round-trip encode
    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected, encoded);
}

#[test]
fn example_class_selector_noindent_decode() {
    let expected = read_cbor(&Some(
        "./tests/examples/example-class-selector-noindent.cbor".to_string(),
    ));
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();

    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected, encoded);
}

#[test]
fn example_class_selector_json_roundtrip() {
    let cbor_data = read_cbor(&Some(
        "./tests/examples/example-class-selector.cbor".to_string(),
    ));
    let decoded_cbor: CoservMapCbor = from_reader(cbor_data.as_slice()).unwrap();

    // convert to JSON-friendly struct
    let json_form: CoservMap = decoded_cbor.clone().try_into().unwrap();
    let json = serde_json::to_string(&json_form).unwrap();
    println!("JSON: {}", json);
    let dec_json: CoservMap = serde_json::from_str(json.as_str()).unwrap();

    // convert back to CBOR-friendly struct
    let roundtrip: CoservMapCbor = dec_json.try_into().unwrap();
    assert_eq!(decoded_cbor, roundtrip);

    // re-encode and compare
    let mut actual = vec![];
    let _ = into_writer(&roundtrip, &mut actual);
    assert_eq!(cbor_data, actual);
}

// {
//   0: "tag:example.com,2025:cc-platform#1.0.0",
//   1: {
//     0: 2,
//     1: {
//       1: [
//         [ 550(h'02DEADBEEFDEAD') ],
//         [ 560(h'8999786556') ]
//       ]
//     },
//     2: 2
//   }
// }
#[test]
fn example_instance_selector_decode() {
    let expected = read_cbor(&Some(
        "./tests/examples/example-instance-selector.cbor".to_string(),
    ));
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();
    println!("Decoded instance selector: {:?}", decoded);

    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected, encoded);
}

#[test]
fn example_instance_selector_json_roundtrip() {
    let cbor_data = read_cbor(&Some(
        "./tests/examples/example-instance-selector.cbor".to_string(),
    ));
    let decoded_cbor: CoservMapCbor = from_reader(cbor_data.as_slice()).unwrap();

    let json_form: CoservMap = decoded_cbor.clone().try_into().unwrap();
    let json = serde_json::to_string(&json_form).unwrap();
    println!("JSON: {}", json);
    let dec_json: CoservMap = serde_json::from_str(json.as_str()).unwrap();

    let roundtrip: CoservMapCbor = dec_json.try_into().unwrap();
    assert_eq!(decoded_cbor, roundtrip);
}

// {
//   0: "tag:example.com,2025:cc-platform#1.0.0",
//   1: {
//     0: 2,
//     1: {
//       2: [
//         [ 560(h'8999786556') ],
//         [ 37(h'31FB5ABF023E4992AA4E95F9C1503BFA') ]
//       ]
//     },
//     2: 1
//   }
// }
#[test]
fn example_group_selector_decode() {
    let expected = read_cbor(&Some(
        "./tests/examples/example-group-selector.cbor".to_string(),
    ));
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();
    println!("Decoded group selector: {:?}", decoded);

    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected, encoded);
}

#[test]
fn example_group_selector_json_roundtrip() {
    let cbor_data = read_cbor(&Some(
        "./tests/examples/example-group-selector.cbor".to_string(),
    ));
    let decoded_cbor: CoservMapCbor = from_reader(cbor_data.as_slice()).unwrap();

    let json_form: CoservMap = decoded_cbor.clone().try_into().unwrap();
    let json = serde_json::to_string(&json_form).unwrap();
    println!("JSON: {}", json);
    let dec_json: CoservMap = serde_json::from_str(json.as_str()).unwrap();

    let roundtrip: CoservMapCbor = dec_json.try_into().unwrap();
    assert_eq!(decoded_cbor, roundtrip);
}

// rv-class-simple: query only, no results, class selector without measurements
#[test]
fn rv_class_simple_decode() {
    let expected = read_cbor(&Some("./tests/examples/rv-class-simple.cbor".to_string()));
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();
    println!("Decoded rv-class-simple: {:?}", decoded);

    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected, encoded);
}

// rv-class-stateful: query only, class selector with measurements
#[test]
fn rv_class_stateful_decode() {
    let expected = read_cbor(&Some("./tests/examples/rv-class-stateful.cbor".to_string()));
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();
    println!("Decoded rv-class-stateful: {:?}", decoded);

    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected, encoded);
}

#[test]
fn rv_class_stateful_json_roundtrip() {
    let cbor_data = read_cbor(&Some("./tests/examples/rv-class-stateful.cbor".to_string()));
    let decoded_cbor: CoservMapCbor = from_reader(cbor_data.as_slice()).unwrap();

    let json_form: CoservMap = decoded_cbor.clone().try_into().unwrap();
    let json = serde_json::to_string(&json_form).unwrap();
    println!("JSON: {}", json);
    let dec_json: CoservMap = serde_json::from_str(json.as_str()).unwrap();

    let roundtrip: CoservMapCbor = dec_json.try_into().unwrap();
    assert_eq!(decoded_cbor, roundtrip);
}

// rv-class-simple-results: query + results with refval-quad-map entries
#[test]
fn rv_class_simple_results_decode() {
    let expected = read_cbor(&Some(
        "./tests/examples/rv-class-simple-results.cbor".to_string(),
    ));
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();
    println!("Decoded rv-class-simple-results: {:?}", decoded);

    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected, encoded);
}

#[test]
fn rv_class_simple_results_json_roundtrip() {
    let cbor_data = read_cbor(&Some(
        "./tests/examples/rv-class-simple-results.cbor".to_string(),
    ));
    let decoded_cbor: CoservMapCbor = from_reader(cbor_data.as_slice()).unwrap();

    let json_form: CoservMap = decoded_cbor.clone().try_into().unwrap();
    let json = serde_json::to_string(&json_form).unwrap();
    println!("JSON: {}", json);
    let dec_json: CoservMap = serde_json::from_str(json.as_str()).unwrap();

    let roundtrip: CoservMapCbor = dec_json.try_into().unwrap();
    assert_eq!(decoded_cbor, roundtrip);
}

// rv-class-simple-results-source-artifacts: query + results with source-artifacts (cmw.cbor-record)
#[test]
fn rv_class_simple_results_source_artifacts_decode() {
    let expected = read_cbor(&Some(
        "./tests/examples/rv-class-simple-results-source-artifacts.cbor".to_string(),
    ));
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();
    println!(
        "Decoded rv-class-simple-results-source-artifacts: {:?}",
        decoded
    );

    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected, encoded);
}

#[test]
fn rv_class_simple_results_source_artifacts_json_roundtrip() {
    let cbor_data = read_cbor(&Some(
        "./tests/examples/rv-class-simple-results-source-artifacts.cbor".to_string(),
    ));
    let decoded_cbor: CoservMapCbor = from_reader(cbor_data.as_slice()).unwrap();

    let json_form: CoservMap = decoded_cbor.clone().try_into().unwrap();
    let json = serde_json::to_string(&json_form).unwrap();
    println!("JSON: {}", json);
    let dec_json: CoservMap = serde_json::from_str(json.as_str()).unwrap();

    let roundtrip: CoservMapCbor = dec_json.try_into().unwrap();
    assert_eq!(decoded_cbor, roundtrip);
}

// rv-results: query + results with more complex refval-quad entries
#[test]
fn rv_results_decode() {
    let expected = read_cbor(&Some("./tests/examples/rv-results.cbor".to_string()));
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();
    println!("Decoded rv-results: {:?}", decoded);

    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected, encoded);
}

#[test]
fn rv_results_json_roundtrip() {
    let cbor_data = read_cbor(&Some("./tests/examples/rv-results.cbor".to_string()));
    let decoded_cbor: CoservMapCbor = from_reader(cbor_data.as_slice()).unwrap();

    let json_form: CoservMap = decoded_cbor.clone().try_into().unwrap();
    let json = serde_json::to_string(&json_form).unwrap();
    println!("JSON: {}", json);
    let dec_json: CoservMap = serde_json::from_str(json.as_str()).unwrap();

    let roundtrip: CoservMapCbor = dec_json.try_into().unwrap();
    assert_eq!(decoded_cbor, roundtrip);
}

// Inline hex test for the simplest selector (verifies exact hex matching)
#[test]
fn example_class_selector_hex() {
    let expected = hex!(
        "a20078267461673a6578616d706c652e636f6d2c323032353a63632d706c6174666f726d23312e302e30"
        "01a3000201a1008281a300d902304400112233016e4578616d706c652056656e646f72026d4578616d70"
        "6c65204d6f64656c81a100d8255031fb5abf023e4992aa4e95f9c1503bfa0200"
    );
    let decoded: CoservMapCbor = from_reader(expected.as_slice()).unwrap();
    let mut encoded = vec![];
    let _ = into_writer(&decoded, &mut encoded);
    assert_eq!(expected.to_vec(), encoded);
}
