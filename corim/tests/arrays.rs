use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::tag::Required;
use common::{UeidType, UuidType};
use corim::arrays::*;
use corim::choices::*;
use corim::maps::*;

mod utils;
use utils::*;

#[test]
fn attest_key_triple_record_test() {
    let mut encoded_token = vec![];
    let fab = AttestKeyTripleRecordCbor {
        environment: EnvironmentMapCbor {
            class: None,
            instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType(
                TEST_UEID.to_vec(),
            )))),
            group: None,
        },
        key_list: vec![CryptoKeyTypeChoice::Key(Required(
            "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE".to_string(),
        ))],
        conditions: None,
    };

    let _ = into_writer(&fab, &mut encoded_token);
    let dec: AttestKeyTripleRecordCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);
    assert_eq!(
        TEST_UEID.to_vec(),
        match &dec.environment.instance {
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType(v)))) => {
                v.clone()
            }
            _ => panic!(),
        }
    );

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
        id: Some(ClassIdTypeChoiceCbor::uuid(Required(UuidType(
            TEST_UUID.as_bytes().to_vec(),
        )))),
        vendor: None,
        model: None,
        layer: None,
        index: None,
    };
    let environment_map = EnvironmentMapCbor {
        class: Some(c),
        instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType(
            TEST_UEID.to_vec(),
        )))),
        group: None,
    };
    let fab = CoswidTripleRecordCbor {
        environment_map,
        tag_ids: vec![TagIdTypeChoiceCbor::Str("example-tag-id".to_string())],
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
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType(v)))) => {
                v.clone()
            }
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
        domain_id: DomainTypeChoice::Text("example-domain".to_string()),
        trustees: vec![DomainTypeChoice::Text("trusted-domain".to_string())],
    };

    let _ = into_writer(&fab, &mut encoded_token);
    let dec: DomainDependencyTripleRecordCbor =
        from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);

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
            instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType(
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
                raw_value: None,
                raw_value_mask: None,
                mac_addr: None,
                ip_addr: None,
                serial_number: None,
                ueid: None,
                uuid: None,
                name: None,
                cryptokeys: None,
                int_range: None,
                other: None,
            },
            authorized_by: None,
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
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType(v)))) => {
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
fn identity_triple_record_test() {
    let mut encoded_token = vec![];
    let fab = IdentityTripleRecordCbor {
        environment: EnvironmentMapCbor {
            class: None,
            instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType(
                TEST_UEID.to_vec(),
            )))),
            group: None,
        },
        key_list: vec![CryptoKeyTypeChoice::Key(Required(
            "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE".to_string(),
        ))],
        conditions: None,
    };

    let _ = into_writer(&fab, &mut encoded_token);
    let dec: IdentityTripleRecordCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    let mut encoded_token2 = vec![];
    let _ = into_writer(&dec, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);
    assert_eq!(fab, dec);
    assert_eq!(
        TEST_UEID.to_vec(),
        match &dec.environment.instance {
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType(v)))) => {
                v.clone()
            }
            _ => panic!(),
        }
    );

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
            instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType(
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
                raw_value: None,
                raw_value_mask: None,
                mac_addr: None,
                ip_addr: None,
                serial_number: None,
                ueid: None,
                uuid: None,
                name: None,
                cryptokeys: None,
                int_range: None,
                other: None,
            },
            authorized_by: None,
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
            Some(InstanceIdTypeChoice::Ueid(ciborium::tag::Required(UeidType(v)))) => {
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
