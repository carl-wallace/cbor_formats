use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::tag::Required;
use ciborium::value::Value;
use common::{BytesType, IntType, TaggedUriTypeCbor, TimeCbor, UeidType, UuidType};
use corim::choices::*;
use corim::maps::*;
use coswid::maps::*;
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
    let expected = hex!("a500d8255031fb5abf023e4992aa4e95f9c1503bfa016841434d45204c7464026a526f616472756e6e657203010402");
    let e = ClassMapCbor {
        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType::Uuid(
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
    // {0: 600(h'61636D652D696D706C656D656E746174696F6E2D69642D303030303030303031'), 1: "EMCA Ltd", 2: "Rennurdaor", 3: 2, 4: 1}
    let expected = hex!("a500d90258582061636d652d696d706c656d656e746174696f6e2d69642d3030303030303030310168454d4341204c7464026a52656e6e757264616f7203020401");
    let e = ClassMapCbor {
        id: Some(ClassIdTypeChoiceCbor::Int2(Required(IntType::Int(
            TEST_IMPL_ID.to_vec(),
        )))),
        vendor: Some("EMCA Ltd".to_string()),
        model: Some("Rennurdaor".to_string()),
        layer: Some(2),
        index: Some(1),
    };
    let mut actual = vec![];
    let _ = into_writer(&e, &mut actual);
    assert_eq!(expected.to_vec(), actual);
}

#[test]
fn class_map_class_id_only_test() {
    // {0: 37(h'31FB5ABF023E4992AA4E95F9C1503BFA')}
    let expected = hex!("a100d8255031fb5abf023e4992aa4e95f9c1503bfa");
    let e = ClassMapCbor {
        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType::Uuid(
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
fn concise_mid_tag_test() {
    let files = vec![
        "./tests/examples/comid-psa-refval.cbor",
        "./tests/examples/comid-dice-refval.cbor",
        "./tests/examples/comid_1.cbor",
        "./tests/examples/comid_2.cbor",
        "./tests/examples/comid-psa-iakpub.cbor",
        "./tests/examples/comid-psa-integ-iakpub.cbor",
    ];

    for f in files {
        let comid_cbor_bytes = read_cbor(&Some(f.to_string()));
        println!(
            "Encoded ConciseMidTag from veraison ({}): {:?}",
            f,
            buffer_to_hex(comid_cbor_bytes.as_slice())
        );
        let comid_d: ConciseMidTagCbor = from_reader(comid_cbor_bytes.clone().as_slice()).unwrap();
        //println!("Decoded ConciseMidTag: {:?}", comid_d);
        let mut encoded_token = vec![];
        let _ = into_writer(&comid_d, &mut encoded_token);
        assert_eq!(comid_cbor_bytes, encoded_token);

        let comid_json: ConciseMidTag = comid_d.try_into().unwrap();
        println!("{}", serde_json::to_string(&comid_json).unwrap());

        let mut encoded_token2 = vec![];
        let _ = into_writer(&comid_json, &mut encoded_token2);
        println!(
            "Encoded ConciseMidTag with string keys: {:?}",
            buffer_to_hex(encoded_token2.as_slice())
        );

        //todo the roundtrip here does not always yield expected result due to differences in tagged int encoding
        // (tag 600 becomes tag 551 in the ->JSON-> transition)
        // let comid_cbor: ConciseMidTagCbor = comid_json.try_into().unwrap();
        // let mut encoded_token3 = vec![];
        // let _ = into_writer(&comid_cbor, &mut encoded_token3);
        // println!(
        //     "Re-encoded ConciseMidTag with integer keys: {:?}",
        //     buffer_to_hex(encoded_token3.as_slice())
        // );
        // assert_eq!(comid_cbor_bytes, encoded_token3);
    }
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
fn corim_map_test() {
    let comid_cbor_bytes = read_cbor(&Some("./tests/examples/corim_1.cbor".to_string()));
    println!(
        "Encoded CorimMap from veraison: {:?}",
        buffer_to_hex(comid_cbor_bytes.as_slice())
    );
    let comid_d: CorimMapCbor = from_reader(comid_cbor_bytes.as_slice()).unwrap();
    println!("Decoded CorimMapCbor: {:?}", comid_d);
    let mut encoded_token = vec![];
    let _ = into_writer(&comid_d, &mut encoded_token);
    assert_eq!(comid_cbor_bytes, encoded_token);

    //parse the tags now
    for t in comid_d.tags {
        match t {
            BytesType::Bytes(b) => {
                let x: Result<Value, _> = from_reader(b.as_slice());
                match &x {
                    Ok(Value::Tag(505, v)) => {
                        let val: Value = *v.clone();
                        let swid: ConciseSwidTagCbor = val.try_into().unwrap();
                        let tagged_swid = TaggedCoswidCbor(Required(swid));
                        let mut encoded_token_swid = vec![];
                        let _ = into_writer(&tagged_swid, &mut encoded_token_swid);
                        assert_eq!(b, encoded_token_swid);
                    }
                    Ok(Value::Tag(506, v)) => {
                        let val: Value = *v.clone();
                        let comid: ConciseMidTagCbor = val.try_into().unwrap();
                        let tagged_comid = TaggedComidCbor(Required(comid));
                        let mut encoded_token_comid = vec![];
                        let _ = into_writer(&tagged_comid, &mut encoded_token_comid);
                        assert_eq!(b, encoded_token_comid);
                    }
                    Ok(_) => {
                        panic!()
                    }
                    Err(_) => {
                        panic!()
                    }
                }
            }
        }
    }
}

#[test]
fn corim_meta_map_full_test() {
    let mut encoded_token = vec![];
    // {0: {0: "ACME Ltd.", 1: 32("https://acme.example")}, 1: {0: 1(1601424000), 1: 1(1632960000)}}
    let enc_meta = hex!("a200a2006941434d45204c74642e01d8207468747470733a2f2f61636d652e6578616d706c6501a200c11a5f73ca8001c11a6154fe00");
    let dec: CorimMetaMapCbor = from_reader(enc_meta.to_vec().as_slice()).unwrap();
    let _ = into_writer(&dec, &mut encoded_token);
    assert_eq!(encoded_token, enc_meta);
    match &dec.signer.entity_name {
        EntityNameTypeChoice::Text(v) => assert_eq!(*v, "ACME Ltd.".to_string()),
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
            assert_eq!(v.not_after, TimeCbor::T(Required(1632960000)))
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
            not_after: TimeCbor::T(Required(1632960000)),
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
            CorimRoleTypeChoiceCbor::Known(CorimRoleTypeChoiceKnownCbor::Creator),
            CorimRoleTypeChoiceCbor::Known(CorimRoleTypeChoiceKnownCbor::TagCreator),
            CorimRoleTypeChoiceCbor::Known(CorimRoleTypeChoiceKnownCbor::Maintainer),
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
        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType::Uuid(
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
        id: Some(ClassIdTypeChoiceCbor::Uuid(Required(UuidType::Uuid(
            TEST_UUID.as_bytes().to_vec(),
        )))),
        vendor: None,
        model: None,
        layer: None,
        index: None,
    };
    let e = EnvironmentMapCbor {
        class: Some(c),
        instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType::Ueid(
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
        instance: Some(InstanceIdTypeChoice::Ueid(Required(UeidType::Ueid(
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
        group: Some(GroupIdTypeChoice::Uuid(Required(UuidType::Uuid(
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
    //todo
}

#[test]
fn measurement_map_test() {
    //todo
}

#[test]
fn measurement_values_map_test() {
    //todo
}

#[test]
fn protected_corim_header_map_test() {
    //todo
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
    //todo
}

#[test]
fn validity_map_test() {
    // encoded value produced using code from the veraison corim repo
    let enc_validity = hex!("a200c11a637cffdc01c11a637d0dec");
    let v: ValidityMapCbor = from_reader(enc_validity.to_vec().as_slice()).unwrap();
    println!("{:?}", v);

    let fab = ValidityMapCbor {
        not_before: Some(common::TimeCbor::T(Required(1669136348))),
        not_after: common::TimeCbor::T(Required(1669139948)),
    };
    let mut encoded_token2 = vec![];
    let _ = into_writer(&fab, &mut encoded_token2);
    assert_eq!(enc_validity.to_vec(), encoded_token2);
}

#[test]
fn verification_key_map_test() {
    //todo
}

#[test]
fn version_map_test() {
    //todo
}
