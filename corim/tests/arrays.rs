use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::tag::Required;
use common::arrays::*;
use common::{TextOrBinary, UeidType, UuidType};
use corim::arrays::*;
use corim::choices::*;
use corim::maps::*;
use hex_literal::hex;

mod utils;
use utils::*;

#[test]
fn attest_key_triple_record_test() {
    let mut encoded_token = vec![];
    let fab = AttestKeyTripleRecordCbor {
        environment_map: EnvironmentMapCbor {
            class: None,
            instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType::Ueid(
                TEST_UEID.to_vec(),
            )))),
            group: None,
        },
        crypto_keys: vec![VerificationKeyMapCbor {
            key: "Some Key".to_string(),
            keychain: None,
        }],
    };

    let _ = into_writer(&fab, &mut encoded_token);
    let dec: AttestKeyTripleRecordCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);
    assert_eq!(
        TEST_UEID.to_vec(),
        match &dec.environment_map.instance {
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType::Ueid(v)))) => {
                v.clone()
            }
            _ => panic!(),
        }
    );
    assert_eq!("Some Key", fab.crypto_keys[0].key);

    let dec_j: AttestKeyTripleRecord = dec.try_into().unwrap();
    let _ = serde_json::to_string(&dec_j).unwrap();
    let dec_from_j: AttestKeyTripleRecordCbor = dec_j.try_into().unwrap();
    let mut encoded_token3 = vec![];
    let _ = into_writer(&dec_from_j, &mut encoded_token3);
    assert_eq!(encoded_token, encoded_token3);
}

#[test]
fn coswid_triple_record_test() {
    let mut encoded_token = vec![];
    let c = ClassMapCbor {
        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType::Uuid(
            TEST_UUID.as_bytes().to_vec(),
        )))),
        vendor: None,
        model: None,
        layer: None,
        index: None,
    };
    let environment_map = EnvironmentMapCbor {
        class: Some(c),
        instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType::Ueid(
            TEST_UEID.to_vec(),
        )))),
        group: None,
    };
    let fab = CoswidTripleRecordCbor {
        environment_map,
        coswid_tags: vec![TextOrBinary::Text("Some CoSWID Tag ID".to_string())],
    };

    let _ = into_writer(&fab, &mut encoded_token);
    println!(
        "Encoded CoswidTripleRecordCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
    let dec: CoswidTripleRecordCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);
    assert_eq!(
        TEST_UEID.to_vec(),
        match &dec.environment_map.instance {
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType::Ueid(v)))) => {
                v.clone()
            }
            _ => panic!(),
        }
    );
    assert_eq!(
        "Some CoSWID Tag ID",
        match &fab.coswid_tags[0] {
            TextOrBinary::Text(s) => s.as_str(),
            _ => panic!(),
        }
    );

    let dec_j: CoswidTripleRecord = dec.try_into().unwrap();
    let _ = serde_json::to_string(&dec_j).unwrap();
    let dec_from_j: CoswidTripleRecordCbor = dec_j.try_into().unwrap();
    let mut encoded_token3 = vec![];
    let _ = into_writer(&dec_from_j, &mut encoded_token3);
    assert_eq!(encoded_token, encoded_token3);
}

#[test]
fn domain_dependency_triple_record_test() {
    let mut encoded_token = vec![];
    let fab = DomainDependencyTripleRecordCbor {
        domain_type_choice: DomainTypeChoice::Text("Some DomainTypeChoice".to_string()),
        domain_type_choices: vec![
            DomainTypeChoice::Text("Some other DomainTypeChoice".to_string()),
            DomainTypeChoice::U64(666u64),
            DomainTypeChoice::Uuid(Required(UuidType::Uuid(TEST_UUID.as_bytes().to_vec()))),
        ],
    };

    let _ = into_writer(&fab, &mut encoded_token);
    let dec: DomainDependencyTripleRecordCbor =
        from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);
    assert_eq!(
        "Some DomainTypeChoice",
        match &fab.domain_type_choice {
            DomainTypeChoice::Text(t) => t.as_str(),
            _ => panic!(),
        }
    );
    assert_eq!(
        "Some other DomainTypeChoice",
        match &fab.domain_type_choices[0] {
            DomainTypeChoice::Text(t) => t.as_str(),
            _ => panic!(),
        }
    );
    assert_eq!(
        666,
        match &fab.domain_type_choices[1] {
            DomainTypeChoice::U64(t) => *t,
            _ => panic!(),
        }
    );
    assert_eq!(
        TEST_UUID.as_bytes().to_vec(),
        match &fab.domain_type_choices[2] {
            DomainTypeChoice::Uuid(ciborium::tag::Required(UuidType::Uuid(v))) => v.clone(),
            _ => panic!(),
        }
    );

    let dec_j: DomainDependencyTripleRecord = dec.try_into().unwrap();
    let _ = serde_json::to_string(&dec_j).unwrap();
    let dec_from_j: DomainDependencyTripleRecordCbor = dec_j.try_into().unwrap();
    let mut encoded_token3 = vec![];
    let _ = into_writer(&dec_from_j, &mut encoded_token3);
    assert_eq!(encoded_token, encoded_token3);
}

#[test]
fn endorsed_triple_record_test() {
    let mut encoded_token = vec![];
    let fab = EndorsedTripleRecordCbor {
        environment_map: EnvironmentMapCbor {
            class: None,
            instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType::Ueid(
                TEST_UEID.to_vec(),
            )))),
            group: None,
        },
        measurement_map: vec![MeasurementMapCbor {
            mkey: None,
            value: MeasurementValuesMapCbor {
                version: Some(VersionMapCbor {
                    version: "1.0.1".to_string(),
                    version_scheme: None,
                }),
                svn: None,
                digests: None,
                flags: None,
                mac_addr: None,
                ip_addr: None,
                serial_number: None,
                ueid: None,
                uuid: None,
                name: None,
                other: None,
            },
        }],
    };

    let _ = into_writer(&fab, &mut encoded_token);
    let dec: EndorsedTripleRecordCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);
    assert_eq!(
        TEST_UEID.to_vec(),
        match &dec.environment_map.instance {
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType::Ueid(v)))) => {
                v.clone()
            }
            _ => panic!(),
        }
    );
    assert_eq!(
        "1.0.1",
        dec.measurement_map[0]
            .value
            .version
            .as_ref()
            .unwrap()
            .version
    );

    let dec_j: EndorsedTripleRecord = dec.try_into().unwrap();
    let _ = serde_json::to_string(&dec_j).unwrap();
    let dec_from_j: EndorsedTripleRecordCbor = dec_j.try_into().unwrap();
    let mut encoded_token3 = vec![];
    let _ = into_writer(&dec_from_j, &mut encoded_token3);
    assert_eq!(encoded_token, encoded_token3);
}

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

    let hej: HashEntry = dec.try_into().unwrap();
    println!("{}", serde_json::to_string(&hej).unwrap());
    let hec: HashEntryCbor = hej.try_into().unwrap();
    let mut encoded_token3 = vec![];
    let _ = into_writer(&hec, &mut encoded_token3);
    assert_eq!(encoded_token, encoded_token3);
}

#[test]
fn identity_triple_record_test() {
    let mut encoded_token = vec![];
    let fab = IdentityTripleRecordCbor {
        environment_map: EnvironmentMapCbor {
            class: None,
            instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType::Ueid(
                TEST_UEID.to_vec(),
            )))),
            group: None,
        },
        crypto_keys: vec![VerificationKeyMapCbor {
            key: "Some Key".to_string(),
            keychain: None,
        }],
    };

    let _ = into_writer(&fab, &mut encoded_token);
    let dec: IdentityTripleRecordCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);
    assert_eq!(
        TEST_UEID.to_vec(),
        match &dec.environment_map.instance {
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType::Ueid(v)))) => {
                v.clone()
            }
            _ => panic!(),
        }
    );
    assert_eq!("Some Key", fab.crypto_keys[0].key);

    let dec_j: IdentityTripleRecord = dec.try_into().unwrap();
    let _ = serde_json::to_string(&dec_j).unwrap();
    let dec_from_j: IdentityTripleRecordCbor = dec_j.try_into().unwrap();
    let mut encoded_token3 = vec![];
    let _ = into_writer(&dec_from_j, &mut encoded_token3);
    assert_eq!(encoded_token, encoded_token3);
}

#[test]
fn reference_triple_record_test() {
    let mut encoded_token = vec![];
    let fab = ReferenceTripleRecordCbor {
        environment_map: EnvironmentMapCbor {
            class: None,
            instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType::Ueid(
                TEST_UEID.to_vec(),
            )))),
            group: None,
        },
        measurement_map: vec![MeasurementMapCbor {
            mkey: None,
            value: MeasurementValuesMapCbor {
                version: Some(VersionMapCbor {
                    version: "1.0.1".to_string(),
                    version_scheme: None,
                }),
                svn: None,
                digests: None,
                flags: None,
                mac_addr: None,
                ip_addr: None,
                serial_number: None,
                ueid: None,
                uuid: None,
                name: None,
                other: None,
            },
        }],
    };

    let _ = into_writer(&fab, &mut encoded_token);
    let dec: ReferenceTripleRecordCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);
    assert_eq!(
        TEST_UEID.to_vec(),
        match &dec.environment_map.instance {
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType::Ueid(v)))) => {
                v.clone()
            }
            _ => panic!(),
        }
    );
    assert_eq!(
        "1.0.1",
        dec.measurement_map[0]
            .value
            .version
            .as_ref()
            .unwrap()
            .version
    );

    let dec_j: ReferenceTripleRecord = dec.try_into().unwrap();
    let _ = serde_json::to_string(&dec_j).unwrap();
    let dec_from_j: ReferenceTripleRecordCbor = dec_j.try_into().unwrap();
    let mut encoded_token3 = vec![];
    let _ = into_writer(&dec_from_j, &mut encoded_token3);
    assert_eq!(encoded_token, encoded_token3);
}
