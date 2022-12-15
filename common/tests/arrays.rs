use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use common::arrays::*;
use hex_literal::hex;

#[test]
fn hash_entry_test() {
    let mut encoded_token = vec![];
    let some_bytes = hex!("a200c11a637cffdc01c11a637d0decffa200c11a637cffdc01c11a637d0decff");
    let fab = HashEntryCbor {
        hash_alg_id: 1,
        hash_value: some_bytes.to_vec(),
    };

    let _ = into_writer(&fab, &mut encoded_token);
    let dec: HashEntryCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);
    assert_eq!(dec.hash_value, some_bytes);
    assert_eq!(dec.hash_alg_id, 1);
    assert!(fab == dec);
    let _s = format!("{fab:?}");

    let hej: HashEntry = dec.try_into().unwrap();
    let hej_c = hej.clone();
    assert_eq!(hej_c, hej);
    assert!(hej_c == hej);
    let _s = format!("{hej:?}");

    let json = serde_json::to_string(&hej).unwrap();
    let hej_d: HashEntry = serde_json::from_str(json.as_str()).unwrap();
    assert_eq!(hej_d, hej);
    let hec: HashEntryCbor = hej.try_into().unwrap();
    let hec_c = hec.clone();
    assert_eq!(hec_c, hec);
    assert!(hec_c == hec);

    let mut encoded_token3 = vec![];
    let _ = into_writer(&hec, &mut encoded_token3);
    assert_eq!(encoded_token, encoded_token3);

    let hej_from_c: HashEntry = hec_c.try_into().unwrap();
    assert_eq!(hej_from_c, hej_c);
}

#[test]
fn hash_entry_test2() {
    let some_bytes = hex!("a200c11a637cffdc01c11a637d0decffa200c11a637cffdc01c11a637d0decff");
    let fab = HashEntry {
        hash_alg_id: 1,
        hash_value: some_bytes.to_vec(),
    };

    let json_encoded = serde_json::to_string(&fab).unwrap();
    let dec: HashEntry = serde_json::from_str(json_encoded.as_str()).unwrap();
    let json_encoded2 = serde_json::to_string(&dec).unwrap();

    assert_eq!(json_encoded, json_encoded2);
    assert_eq!(fab, dec);
    assert_eq!(dec.hash_value, some_bytes);
    assert_eq!(dec.hash_alg_id, 1);
    assert!(fab == dec);
    let _s = format!("{fab:?}");

    let hec: HashEntryCbor = dec.try_into().unwrap();
    let hec_c = hec.clone();
    assert_eq!(hec_c, hec);
    assert!(hec_c == hec);
    let _s = format!("{hec:?}");

    let mut encoded_token = vec![];
    let _ = into_writer(&hec, &mut encoded_token);
    let hec_dec: HashEntryCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    let hej: HashEntry = hec_dec.try_into().unwrap();
    let hej_c = hej.clone();
    assert_eq!(hej_c, hej);
    assert!(hej_c == hej);

    let json_encoded3 = serde_json::to_string(&hej).unwrap();
    assert_eq!(json_encoded, json_encoded3);

    let hec_from_j: HashEntryCbor = hej_c.try_into().unwrap();
    assert_eq!(hec_from_j, hec_c);
}
