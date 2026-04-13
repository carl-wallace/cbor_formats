use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::tag::Required;
use ciborium::value::Value;
use common::{BytesType, TaggedUriTypeCbor, TimeCbor, UeidType, UuidType};
use corim::choices::*;
use corim::maps::*;
use hex_literal::hex;

mod utils;
use crate::utils::*;

#[test]
fn class_map_test2() {
    let expected = hex!("a100d86f442a030405");
    let egl_d: ClassMapCbor = from_reader(expected.clone().as_slice()).unwrap();
    println!("Decoded ClassMapCbor: {:?}", egl_d);
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    println!(
        "Encoded ClassMapCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
}

#[test]
fn class_map_uuid_full_test() {
    // {0: 37(h'31FB5ABF023E4992AA4E95F9C1503BFA'), 1: "ACME Ltd", 2: "Roadrunner", 3: 1, 4: 2}
    let expected = hex!(
        "a500d8255031fb5abf023e4992aa4e95f9c1503bfa016841434d45204c7464026a526f616472756e6e657203010402"
    );
    let e = ClassMapCbor {
        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType(
            TEST_UUID.as_bytes().to_vec(),
        )))),
        vendor: Some("ACME Ltd".to_string()),
        model: Some("Roadrunner".to_string()),
        layer: Some(1),
        index: Some(2),
    };
    let mut actual = vec![];
    let _ = into_writer(&e, &mut actual);
    assert_eq!(expected.to_vec(), actual);
}

#[test]
fn class_map_impl_full_test() {
    // {0: 560(h'61636D652D696D706C656D656E746174696F6E2D69642D303030303030303031'), 1: "EMCA Ltd", 2: "Rennurdaor", 3: 2, 4: 1}
    let e = ClassMapCbor {
        id: Some(ClassIdTypeChoiceCbor::Bytes(Required(BytesType(
            TEST_IMPL_ID.to_vec(),
        )))),
        vendor: Some("EMCA Ltd".to_string()),
        model: Some("Rennurdaor".to_string()),
        layer: Some(2),
        index: Some(1),
    };
    let mut actual = vec![];
    let _ = into_writer(&e, &mut actual);
    // round-trip test
    let dec: ClassMapCbor = from_reader(actual.as_slice()).unwrap();
    assert_eq!(e, dec);
}

#[test]
fn class_map_class_id_only_test() {
    // {0: 37(h'31FB5ABF023E4992AA4E95F9C1503BFA')}
    let expected = hex!("a100d8255031fb5abf023e4992aa4e95f9c1503bfa");
    let e = ClassMapCbor {
        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType(
            TEST_UUID.as_bytes().to_vec(),
        )))),
        vendor: None,
        model: None,
        layer: None,
        index: None,
    };
    let mut actual = vec![];
    let _ = into_writer(&e, &mut actual);
    assert_eq!(expected.to_vec(), actual);
}

#[test]
fn concise_mid_tag_roundtrip() {
    use corim::arrays::ReferenceTripleRecordCbor;

    let env = EnvironmentMapCbor {
        class: Some(ClassMapCbor {
            id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType(
                TEST_UUID.as_bytes().to_vec(),
            )))),
            vendor: Some("ACME Ltd".to_string()),
            model: Some("Roadrunner".to_string()),
            layer: None,
            index: None,
        }),
        instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType(
            TEST_UEID.to_vec(),
        )))),
        group: None,
    };
    let mvm = MeasurementValuesMapCbor {
        version: Some(VersionMapCbor {
            version: "1.0.0".to_string(),
            version_scheme: None,
        }),
        svn: None,
        digests: Some(vec![common::arrays::HashEntryCbor {
            hash_alg_id: 1,
            hash_value: vec![0xAA; 32],
        }]),
        flags: None,
        raw_value: None,
        raw_value_mask: None,
        mac_addr: None,
        ip_addr: None,
        serial_number: Some("SN-001".to_string()),
        ueid: None,
        uuid: None,
        name: None,
        cryptokeys: None,
        int_range: None,
        other: None,
    };
    let comid = ConciseMidTagCbor {
        language: None,
        other: None,
        tag_identity: Some(TagIdentityMapCbor {
            tag_id: TagIdTypeChoiceCbor::Uuid(UuidType(TEST_UUID.as_bytes().to_vec())),
            tag_version: Some(TagVersionType::U64(1)),
        }),
        entities: None,
        linked_tags: None,
        triples: TriplesMapCbor {
            reference_triples: Some(vec![ReferenceTripleRecordCbor {
                environment_map: env,
                measurement_map: vec![MeasurementMapCbor {
                    mkey: None,
                    value: mvm,
                    authorized_by: None,
                }],
            }]),
            endorsed_triples: None,
            identity_triples: None,
            attest_key_triples: None,
            dependency_triples: None,
            membership_triples: None,
            coswid_triples: None,
            conditional_endorsement_series_triples: None,
            conditional_endorsement_triples: None,
            other: None,
        },
    };

    // CBOR round-trip
    let mut buf = vec![];
    let _ = into_writer(&comid, &mut buf);
    let decoded: ConciseMidTagCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(comid, decoded);

    // CBOR -> JSON -> CBOR round-trip
    let json: ConciseMidTag = decoded.try_into().unwrap();
    let json_str = serde_json::to_string(&json).unwrap();
    let json_back: ConciseMidTag = serde_json::from_str(&json_str).unwrap();
    let cbor_back: ConciseMidTagCbor = json_back.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_back, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn corim_locator_map_test() {
    //todo fix to feature hashentry
    // let mut encoded_token = vec![];
    // let b = hex!("a200c11a637cffdc01c11a637d0decffa200c11a637cffdc01c11a637d0decff");
    // let clm = CorimLocatorMap {
    //     href: "https://example.com/corim999".to_string(),
    //     thumbprint: Some(b.to_vec()),
    // };
    // let _ = into_writer(&clm, &mut encoded_token);
    // let clm_d: CorimLocatorMap = from_reader(encoded_token.clone().as_slice()).unwrap();
    // assert_eq!(clm_d, clm);
    // assert_eq!(clm_d.href, "https://example.com/corim999".to_string());
    // assert_eq!(clm_d.thumbprint.unwrap(), b.to_vec());
    //
    // let mut encoded_token2 = vec![];
    // let clm2 = CorimLocatorMap {
    //     href: "https://example.com/corim999".to_string(),
    //     thumbprint: None,
    // };
    // let _ = into_writer(&clm2, &mut encoded_token2);
    // let clm_d2: CorimLocatorMap = from_reader(encoded_token2.clone().as_slice()).unwrap();
    // assert_eq!(clm_d2, clm2);
    // assert_eq!(clm_d2.href, "https://example.com/corim999".to_string());
    // assert!(clm_d2.thumbprint.is_none());
}

#[test]
fn corim_map_roundtrip() {
    use corim::arrays::ReferenceTripleRecordCbor;

    // Build a minimal CoMID
    let comid = ConciseMidTagCbor {
        language: None,
        other: None,
        tag_identity: Some(TagIdentityMapCbor {
            tag_id: TagIdTypeChoiceCbor::Uuid(UuidType(TEST_UUID.as_bytes().to_vec())),
            tag_version: Some(TagVersionType::U64(0)),
        }),
        entities: None,
        linked_tags: None,
        triples: TriplesMapCbor {
            reference_triples: Some(vec![ReferenceTripleRecordCbor {
                environment_map: EnvironmentMapCbor {
                    class: Some(ClassMapCbor {
                        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType(
                            TEST_UUID.as_bytes().to_vec(),
                        )))),
                        vendor: Some("Test Vendor".to_string()),
                        model: None,
                        layer: None,
                        index: None,
                    }),
                    instance: None,
                    group: None,
                },
                measurement_map: vec![MeasurementMapCbor {
                    mkey: None,
                    value: MeasurementValuesMapCbor {
                        version: None,
                        svn: None,
                        digests: Some(vec![common::arrays::HashEntryCbor {
                            hash_alg_id: 1,
                            hash_value: vec![0xBB; 32],
                        }]),
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
            }]),
            endorsed_triples: None,
            identity_triples: None,
            attest_key_triples: None,
            dependency_triples: None,
            membership_triples: None,
            coswid_triples: None,
            conditional_endorsement_series_triples: None,
            conditional_endorsement_triples: None,
            other: None,
        },
    };

    // Serialize the CoMID as a tagged (506) CBOR byte string for embedding in CoRIM
    let tagged_comid = TaggedComidCbor(Required(comid));
    let mut comid_bytes = vec![];
    let _ = into_writer(&tagged_comid, &mut comid_bytes);

    // Build a CoRIM containing the CoMID
    let corim = CorimMapCbor {
        id: CorimIdTypeChoice::Str("test-corim-id".to_string()),
        tags: vec![BytesType(comid_bytes.clone())],
        dependent_rims: None,
        profile: None,
        rim_validity: None,
        entities: None,
    };

    // CBOR round-trip
    let mut buf = vec![];
    let _ = into_writer(&corim, &mut buf);
    let decoded: CorimMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(corim, decoded);
    assert_eq!(decoded.tags.len(), 1);

    // Verify the embedded tag can be parsed back as a tagged CoMID
    let BytesType(tag_bytes) = &decoded.tags[0];
    let tag_value: Value = from_reader(tag_bytes.as_slice()).unwrap();
    match &tag_value {
        Value::Tag(506, inner) => {
            let comid_dec: ConciseMidTagCbor = inner.as_ref().clone().try_into().unwrap();
            assert_eq!(
                comid_dec.tag_identity.unwrap().tag_version,
                Some(TagVersionType::U64(0))
            );
        }
        other => panic!("expected tag 506, got: {:?}", other),
    }

    // CBOR -> JSON -> CBOR round-trip
    let json: CorimMap = decoded.try_into().unwrap();
    let json_str = serde_json::to_string(&json).unwrap();
    let json_back: CorimMap = serde_json::from_str(&json_str).unwrap();
    let cbor_back: CorimMapCbor = json_back.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_back, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn corim_meta_map_full_test() {
    let mut encoded_token = vec![];
    // {0: {0: "ACME Ltd.", 1: 32("https://acme.example")}, 1: {0: 1(1601424000), 1: 1(1632960000)}}
    let enc_meta = hex!(
        "a200a2006941434d45204c74642e01d8207468747470733a2f2f61636d652e6578616d706c6501a200c11a5f73ca8001c11a6154fe00"
    );
    let dec: CorimMetaMapCbor = from_reader(enc_meta.to_vec().as_slice()).unwrap();
    let _ = into_writer(&dec, &mut encoded_token);
    assert_eq!(encoded_token, enc_meta);
    match &dec.signer.entity_name {
        EntityNameTypeChoice::Text(v) => assert_eq!(*v, "ACME Ltd.".to_string()),
        _ => panic!("Expected Text variant"),
    };
    match &dec.signer.reg_id {
        Some(TaggedUriTypeCbor::U(v)) => assert_eq!(v.0, "https://acme.example".to_string()),
        None => panic!(),
    };
    match &dec.validity {
        Some(v) => {
            match v.not_before {
                Some(TimeCbor::T(t)) => assert_eq!(t.0, 1601424000),
                None => panic!(),
            }
            assert_eq!(v.not_after, Some(TimeCbor::T(Required(1632960000))))
        }
        None => panic!(),
    };

    // convert to JSON-friendly struct, encode as JSON, and decode from JSON
    let meta_j: CorimMetaMap = dec.try_into().unwrap();
    let json = serde_json::to_string(&meta_j).unwrap();
    println!("JSON: {}", json);
    let dec_meta_j: CorimMetaMap = serde_json::from_str(json.as_str()).unwrap();

    // encode as CBOR (with textual map keys)
    let mut enc_with_text_keys = vec![];
    let _ = into_writer(&dec_meta_j, &mut enc_with_text_keys);
    println!(
        "CBOR with text map keys: {:?}",
        buffer_to_hex(enc_with_text_keys.as_slice())
    );

    // convert to CBOR-friendly struct, encode as CBOR (with integer map keys), then compare with expected
    let roundtrip: CorimMetaMapCbor = dec_meta_j.try_into().unwrap();
    let mut actual = vec![];
    let _ = into_writer(&roundtrip, &mut actual);
    println!(
        "CBOR with integer map keys: {:?}",
        buffer_to_hex(actual.as_slice())
    );
    assert_eq!(enc_meta.to_vec(), actual);

    // scratch build an instance
    let scratch = CorimMetaMapCbor {
        signer: CorimSignerMapCbor {
            entity_name: EntityNameTypeChoice::Text("ACME Ltd.".to_string()),
            reg_id: Some(TaggedUriTypeCbor::U(Required(
                "https://acme.example".to_string(),
            ))),
        },
        validity: Some(ValidityMapCbor {
            not_before: Some(TimeCbor::T(Required(1601424000))),
            not_after: Some(TimeCbor::T(Required(1632960000))),
        }),
    };
    let mut scratch_actual = vec![];
    let _ = into_writer(&scratch, &mut scratch_actual);
    assert_eq!(enc_meta.to_vec(), scratch_actual);
}

#[test]
fn corim_meta_map_mandatory_only_test() {
    let mut encoded_token = vec![];
    // {0: {0: "ACME Ltd."}, 1: {1: 1(1605181526)}}
    let enc_meta = hex!("a200a1006941434d45204c74642e01a101c11a6154fe00");
    let dec: CorimMetaMapCbor = from_reader(enc_meta.to_vec().as_slice()).unwrap();
    let _ = into_writer(&dec, &mut encoded_token);
    assert_eq!(encoded_token, enc_meta);
}

#[test]
fn corim_signer_map_test() {
    let mut encoded_token = vec![];
    // encoded value produced using code from the veraison corim repo
    let enc_signer =
        hex!("a2006941434d45204c74642e01d8207468747470733a2f2f61636d652e6578616d706c65");
    let dec: CorimSignerMapCbor = from_reader(enc_signer.to_vec().as_slice()).unwrap();
    let _ = into_writer(&dec, &mut encoded_token);
    assert_eq!(encoded_token, enc_signer);
    match &dec.entity_name {
        EntityNameTypeChoice::Text(t) => assert_eq!("ACME Ltd.", t),
        _ => panic!("Expected Text variant"),
    };
    if let Some(tut) = &dec.reg_id {
        match tut {
            TaggedUriTypeCbor::U(uri) => assert_eq!("https://acme.example", uri.0),
        };
    }

    let fab = CorimSignerMapCbor {
        entity_name: EntityNameTypeChoice::Text("ACME Ltd.".to_string()),
        reg_id: Some(TaggedUriTypeCbor::U(Required(
            "https://acme.example".to_string(),
        ))),
    };
    let mut encoded_token2 = vec![];
    let _ = into_writer(&fab, &mut encoded_token2);
    assert_eq!(encoded_token, encoded_token2);

    let fab2 = CorimSignerMapCbor {
        entity_name: EntityNameTypeChoice::Text("ACME Ltd.".to_string()),
        reg_id: None,
    };
    let mut encoded_token3 = vec![];
    let _ = into_writer(&fab2, &mut encoded_token3);
    let enc_meta_no_reg_id = hex!("a1006941434d45204c74642e");
    assert_eq!(enc_meta_no_reg_id.to_vec(), encoded_token3);
}

#[test]
fn entity_map_test() {
    let em = EntityMapCbor {
        name: EntityNameTypeChoice::Text("ACME Ltd.".to_string()),
        regid: Some(TaggedUriTypeCbor::U(Required(
            "https://acme.example".to_string(),
        ))),
        roles: vec![
            CorimRoleTypeChoiceCbor::Value(CorimRoleTypeChoiceCbor::MANIFEST_CREATOR),
            CorimRoleTypeChoiceCbor::Value(CorimRoleTypeChoiceCbor::MANIFEST_SIGNER),
        ],
    };
    let mut encoded_token = vec![];
    let _ = into_writer(&em, &mut encoded_token);
    println!(
        "Encoded EntityMapCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
}

#[test]
fn environment_map_test2() {
    let expected = hex!("a100a100d86f442a030405");
    let egl_d: EnvironmentMapCbor = from_reader(expected.clone().as_slice()).unwrap();
    println!("Decoded EnvironmentMapCbor: {:?}", egl_d);
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    println!(
        "Encoded EnvironmentMapCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
}

#[test]
fn environment_map_to_cbor_class_only_test() {
    // {0: {0: 37(h'31FB5ABF023E4992AA4E95F9C1503BFA')}}
    let expected = hex!("a100a100d8255031fb5abf023e4992aa4e95f9c1503bfa");
    let c = ClassMapCbor {
        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType(
            TEST_UUID.as_bytes().to_vec(),
        )))),
        vendor: None,
        model: None,
        layer: None,
        index: None,
    };
    let e = EnvironmentMapCbor {
        class: Some(c),
        instance: None,
        group: None,
    };
    let mut actual = vec![];
    let _ = into_writer(&e, &mut actual);
    assert_eq!(expected.to_vec(), actual);
}

#[test]
fn environment_map_to_cbor_class_and_instance_test() {
    // {0: {0: 37(h'31FB5ABF023E4992AA4E95F9C1503BFA')}, 1: 550(h'02DEADBEEFDEAD')}
    let expected = hex!("a200a100d8255031fb5abf023e4992aa4e95f9c1503bfa01d902264702deadbeefdead");
    let c = ClassMapCbor {
        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType(
            TEST_UUID.as_bytes().to_vec(),
        )))),
        vendor: None,
        model: None,
        layer: None,
        index: None,
    };
    let e = EnvironmentMapCbor {
        class: Some(c),
        instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType(
            TEST_UEID.to_vec(),
        )))),
        group: None,
    };
    let mut actual = vec![];
    let _ = into_writer(&e, &mut actual);
    assert_eq!(expected.to_vec(), actual);
}

#[test]
fn environment_map_to_cbor_instance_only_test() {
    // {1: 550(h'02DEADBEEFDEAD')}
    let expected = hex!("a101d902264702deadbeefdead");
    let e = EnvironmentMapCbor {
        class: None,
        instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType(
            TEST_UEID.to_vec(),
        )))),
        group: None,
    };
    let mut actual = vec![];
    let _ = into_writer(&e, &mut actual);
    assert_eq!(expected.to_vec(), actual);
}

#[test]
fn environment_map_to_cbor_group_only_test() {
    // {2: 37(h'31FB5ABF023E4992AA4E95F9C1503BFA')}
    let expected = hex!("a102d8255031fb5abf023e4992aa4e95f9c1503bfa");
    let e = EnvironmentMapCbor {
        class: None,
        instance: None,
        group: Some(GroupIdTypeChoice::Uuid(Required(UuidType(
            TEST_UUID.as_bytes().to_vec(),
        )))),
    };
    let mut actual = vec![];
    let _ = into_writer(&e, &mut actual);
    assert_eq!(expected.to_vec(), actual);
}

#[test]
fn flags_map_test() {
    // drop for the moment
    // let mut encoded_token = vec![];
    // let fm = FlagsMapCbor {
    //     configured: Some(true),
    //     secure: None,
    //     recovery: Some(false),
    //     debug: None,
    //     replay_protected: Some(true),
    //     integrity_protected: None,
    //     other: None,
    // };
    // let _ = into_writer(&fm, &mut encoded_token);
    // let fm_d: FlagsMapCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    // assert_eq!(fm, fm_d);
    // assert!(fm.configured.unwrap());
    // assert!(!fm.recovery.unwrap());
    // assert!(fm.replay_protected.unwrap());
    // assert!(fm.secure.is_none());
    // assert!(fm.debug.is_none());
    // assert!(fm.integrity_protected.is_none());
}

#[test]
fn linked_tag_map_test() {
    let ltm = LinkedTagMapCbor {
        linked_tag_id: TagIdTypeChoiceCbor::Str("test-tag-id".to_string()),
        tag_rel: TagRelTypeChoice::Value(TagRelTypeChoice::SUPPLEMENTS),
    };
    let mut buf = vec![];
    let _ = into_writer(&ltm, &mut buf);
    let decoded: LinkedTagMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ltm, decoded);

    let json: LinkedTagMap = decoded.try_into().unwrap();
    let cbor_roundtrip: LinkedTagMapCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn measurement_map_test() {
    let mm = MeasurementMapCbor {
        mkey: None,
        value: MeasurementValuesMapCbor {
            version: None,
            svn: None,
            digests: None,
            flags: None,
            raw_value: None,
            raw_value_mask: None,
            mac_addr: None,
            ip_addr: None,
            serial_number: Some("SN-12345".to_string()),
            ueid: None,
            uuid: None,
            name: Some("test-measurement".to_string()),
            cryptokeys: None,
            int_range: None,
            other: None,
        },
        authorized_by: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&mm, &mut buf);
    let decoded: MeasurementMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(mm, decoded);

    let json: MeasurementMap = decoded.try_into().unwrap();
    let cbor_roundtrip: MeasurementMapCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn measurement_values_map_test() {
    let mvm = MeasurementValuesMapCbor {
        version: Some(VersionMapCbor {
            version: "1.0.0".to_string(),
            version_scheme: None,
        }),
        svn: None,
        digests: Some(vec![common::arrays::HashEntryCbor {
            hash_alg_id: 1,
            hash_value: vec![0xBB; 32],
        }]),
        flags: None,
        raw_value: None,
        raw_value_mask: None,
        mac_addr: Some(vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55]),
        ip_addr: None,
        serial_number: None,
        ueid: None,
        uuid: None,
        name: None,
        cryptokeys: None,
        int_range: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&mvm, &mut buf);
    let decoded: MeasurementValuesMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(mvm, decoded);

    let json: MeasurementValuesMap = decoded.try_into().unwrap();
    let cbor_roundtrip: MeasurementValuesMapCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn protected_corim_header_map_test() {
    let pchm = ProtectedCorimHeaderMapCbor {
        alg_id: 1,
        content_type: "application/corim".to_string(),
        meta: CorimMetaMapCbor {
            signer: CorimSignerMapCbor {
                entity_name: EntityNameTypeChoice::Text("Test Signer".to_string()),
                reg_id: None,
            },
            validity: None,
        },
    };
    let mut buf = vec![];
    let _ = into_writer(&pchm, &mut buf);
    let decoded: ProtectedCorimHeaderMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(pchm, decoded);
    assert_eq!(decoded.alg_id, 1);
}

#[test]
fn tag_identity_map_test() {
    // A20050FB51FAC913C546C39390DC306B167F5A0105
    let expected = hex!("A20050FB51FAC913C546C39390DC306B167F5A0105");
    let egl_d: TagIdentityMapCbor = from_reader(expected.clone().as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    assert_eq!(expected.to_vec(), encoded_token);

    let mut encoded_token = vec![];
    let tim = TagIdentityMap {
        tag_id: TagIdTypeChoice::Str("bah".to_string()),
        tag_version: None,
    };
    let _ = into_writer(&tim, &mut encoded_token);
    let tim_d: TagIdentityMap = from_reader(encoded_token.clone().as_slice()).unwrap();
    assert_eq!(tim, tim_d);
    assert_eq!(tim.tag_version, tim_d.tag_version);
    assert_eq!(tim.tag_id, tim_d.tag_id);
}

#[test]
fn triples_map_test() {
    let env = EnvironmentMapCbor {
        class: Some(ClassMapCbor {
            id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType(
                TEST_UUID.as_bytes().to_vec(),
            )))),
            vendor: None,
            model: None,
            layer: None,
            index: None,
        }),
        instance: None,
        group: None,
    };
    let mvm = MeasurementValuesMapCbor {
        version: None,
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
        name: Some("test".to_string()),
        cryptokeys: None,
        int_range: None,
        other: None,
    };
    let mm = MeasurementMapCbor {
        mkey: None,
        value: mvm,
        authorized_by: None,
    };
    use corim::arrays::ReferenceTripleRecordCbor;
    let triple = ReferenceTripleRecordCbor {
        environment_map: env,
        measurement_map: vec![mm],
    };
    let tm = TriplesMapCbor {
        reference_triples: Some(vec![triple]),
        endorsed_triples: None,
        identity_triples: None,
        attest_key_triples: None,
        dependency_triples: None,
        membership_triples: None,
        coswid_triples: None,
        conditional_endorsement_series_triples: None,
        conditional_endorsement_triples: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&tm, &mut buf);
    let decoded: TriplesMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tm, decoded);
    assert!(decoded.reference_triples.is_some());
    assert_eq!(decoded.reference_triples.as_ref().unwrap().len(), 1);
}

#[test]
fn validity_map_test() {
    // encoded value produced using code from the veraison corim repo
    let enc_validity = hex!("a200c11a637cffdc01c11a637d0dec");
    let v: ValidityMapCbor = from_reader(enc_validity.to_vec().as_slice()).unwrap();
    println!("{:?}", v);

    let fab = ValidityMapCbor {
        not_before: Some(common::TimeCbor::T(Required(1669136348))),
        not_after: Some(common::TimeCbor::T(Required(1669139948))),
    };
    let mut encoded_token2 = vec![];
    let _ = into_writer(&fab, &mut encoded_token2);
    assert_eq!(enc_validity.to_vec(), encoded_token2);
}

#[test]
fn version_map_test() {
    let vm = VersionMapCbor {
        version: "2.1.0".to_string(),
        version_scheme: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&vm, &mut buf);
    let decoded: VersionMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(vm, decoded);
    assert_eq!(decoded.version, "2.1.0");

    let json: VersionMap = decoded.try_into().unwrap();
    let cbor_roundtrip: VersionMapCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn veraison_comid_dice_refval_decode() {
    let data = read_cbor(&Some(
        "./tests/examples/comid-dice-refval-veraison.cbor".to_string(),
    ));
    assert!(!data.is_empty(), "test vector file missing");
    let decoded: ConciseMidTagCbor = from_reader(data.as_slice()).unwrap();

    // roundtrip
    let mut buf = vec![];
    let _ = into_writer(&decoded, &mut buf);
    let decoded2: ConciseMidTagCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded, decoded2);
}
