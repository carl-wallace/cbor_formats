use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::tag::Required;
use common::{TimeCbor, UeidType};
use eat::choices::{DebugStatusType, DebugStatusTypeKwown};
use hex_literal::hex;

use eat::arrays::*;
use eat::maps::*;

mod utils;
use utils::*;

#[test]
fn claims_set_claims_test() {
    // this example from veraison uses different keys for some fields than the current EAT spec. let them
    // be as-is and accumulate in the other bucket to test extensibility support.
    let expected = hex!("b0016941636d6520496e632e026772722d74726170036941636d6520496e632e04c10005c10006c1000746ffffffffffff0a4800000000000000000b5101deadbeefdeadbeefdeadbeefdeadbeef0c6941636d6520496e632e0d46ffffffffffff0e030ff5100111a201fb4028ae147ae147ae02fb404c63d70a3d70a413183c");
    println!(
        "Encoded ClaimsSetClaims from veraison: {:?}",
        buffer_to_hex(expected.as_slice())
    );
    let csc_d: ClaimsSetClaimsCbor = from_reader(expected.clone().as_slice()).unwrap();
    //println!("Decoded ConciseMidTag: {:?}", comid_d);
    let mut encoded_token = vec![];
    let _ = into_writer(&csc_d, &mut encoded_token);
    println!(
        "Encoded ClaimsSetClaims: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    assert_eq!(expected.to_vec(), encoded_token);

    let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
    println!("{}", serde_json::to_string(&csc_json).unwrap());

    let mut encoded_token2 = vec![];
    let _ = into_writer(&csc_json, &mut encoded_token2);
    println!(
        "Encoded ClaimsSetClaims with string keys: {:?}",
        buffer_to_hex(encoded_token2.as_slice())
    );

    let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
    let mut encoded_token3 = vec![];
    let _ = into_writer(&csc_cbor, &mut encoded_token3);
    assert_eq!(expected.to_vec(), encoded_token3);
    println!(
        "Re-encoded ClaimsSetClaims with integer keys: {:?}",
        buffer_to_hex(encoded_token3.as_slice())
    );
}

#[test]
fn iss_test() {
    let valid = vec![hex!("A10166497373756572").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.iss.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("810166497373756572").to_vec(), // map not array
        hex!("A101664973737565").to_vec(),   // value too short
        hex!("A101F6").to_vec(),             // not an iss value
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn sub_test() {
    let valid = vec![hex!("A10266497373756572").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.sub.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("810266497373756572").to_vec(), // map not array
        hex!("A102664973737565").to_vec(),   // value too short
        hex!("A102F6").to_vec(),             // not an iss value
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn aud_test() {
    let valid = vec![hex!("A10366497373756572").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.aud.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("810366497373756572").to_vec(), // map not array
        hex!("A103664973737565").to_vec(),   // value too short
        hex!("A103F6").to_vec(),             // not an iss value
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn exp_test() {
    let valid = vec![hex!("A104C11A63921172").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.exp.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8104C11A63921172").to_vec(), // map not array
        hex!("A104C11A639211").to_vec(),   // value too short
        hex!("A104F6").to_vec(),           // not an iss value
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn nbf_test() {
    let valid = vec![hex!("A105C11A63921172").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.nbf.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8105C11A63921172").to_vec(), // map not array
        hex!("A105C11A639211").to_vec(),   // value too short
        hex!("A105F6").to_vec(),           // not an iss value
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}
#[test]
fn iat_test() {
    let valid = vec![hex!("A106C11A63921172").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.iat.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8106C11A63921172").to_vec(), // map not array
        hex!("A106C11A639211").to_vec(),   // value too short
        hex!("A106F6").to_vec(),           // not an iss value
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn cti_test() {
    let valid = vec![hex!("A1074B06092B0601040185BF1004").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.cti.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("81074B06092B0601040185BF1004").to_vec(), // map not array
        hex!("A1074B06092B0601040185BF10").to_vec(),   // value too short
                                                       //hex!("A107F6").to_vec(),    // not an iss value
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn nonce_test() {
    let valid = vec![
        hex!("A10A4B06092B0601040185BF1004").to_vec(),
        hex!("A10A824B06092B0601040185BF10044B06092B0601040185BF1005").to_vec(),
    ];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.nonce.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("810A4B06092B0601040185BF1004").to_vec(), // map not array
        hex!("A10A4B06092B0601040185BF10").to_vec(),   // value too short
        hex!("A10A834B06092B0601040185BF10044B06092B0601040185BF1005").to_vec(), // wrong number of array elements
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn boot_count_test() {
    let valid = vec![hex!("A119011404").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.boot_count.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8119011404").to_vec(), // map not array
        hex!("A1190114").to_vec(),   // value too short
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn bootsy_test() {
    let valid = vec![hex!("A119010C4B06092B0601040185BF1004").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.boot_seed.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8119010C4B06092B0601040185BF1004").to_vec(), // map not array
        hex!("A119010C4B06092B0601040185BF10").to_vec(),   // value too short
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn debug_status_test() {
    let csc = ClaimsSetClaimsCbor {
        iss: None,
        sub: None,
        aud: None,
        exp: None,
        nbf: None,
        iat: None,
        cti: None,
        nonce: None,
        boot_count: None,
        boot_seed: None,
        debug_status: Some(DebugStatusType::Known(DebugStatusTypeKwown::Enabled)),
        dloas: None,
        hardware_model: None,
        hardware_version: None,
        intended_use: None,
        location: None,
        profile: None,
        secure_boot: None,
        sw_name: None,
        sw_version: None,
        ueid: None,
        uptime: None,
        manifests: None,
        measurements: None,
        other: None,
    };
    let mut encoded_token = vec![];
    let _ = into_writer(&csc, &mut encoded_token);
    println!(
        "Encoded ClaimsSetClaims: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );

    let valid = vec![
        hex!("A119010700").to_vec(),
        hex!("A119010701").to_vec(),
        hex!("A119010702").to_vec(),
        hex!("A119010703").to_vec(),
        hex!("A119010704").to_vec(),
    ];
    assert_eq!(encoded_token, valid[0]);
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.debug_status.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8119010701").to_vec(), // map not array
        hex!("A1190107").to_vec(),   // value too short
        hex!("A119010705").to_vec(), // unknown value
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn dloas_test() {
    let csc = ClaimsSetClaimsCbor {
        iss: None,
        sub: None,
        aud: None,
        exp: None,
        nbf: None,
        iat: None,
        cti: None,
        nonce: None,
        boot_count: None,
        boot_seed: None,
        debug_status: None,
        dloas: Some(vec![DloaTypeCbor {
            dloa_registrar: "Registrar".to_string(),
            dloa_platform_label: "Platform Label".to_string(),
            dloa_application_label: Some("Application Label".to_string()),
        }]),
        hardware_model: None,
        hardware_version: None,
        intended_use: None,
        location: None,
        profile: None,
        secure_boot: None,
        sw_name: None,
        sw_version: None,
        ueid: None,
        uptime: None,
        manifests: None,
        measurements: None,
        other: None,
    };
    let mut encoded_token = vec![];
    let _ = into_writer(&csc, &mut encoded_token);
    println!(
        "Encoded ClaimsSetClaims: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
    let valid = vec![
        hex!("A119010E8183695265676973747261726E506C6174666F726D204C6162656C714170706C69636174696F6E204C6162656C").to_vec(),
        hex!("A119010E8182695265676973747261726E506C6174666F726D204C6162656C").to_vec(),
        hex!("A119010E8283695265676973747261726E506C6174666F726D204C6162656C714170706C69636174696F6E204C6162656C826A526567697374726172326F506C6174666F726D204C6162656C32").to_vec(),
    ];
    assert_eq!(encoded_token, valid[0]);
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.dloas.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8119010E8183695265676973747261726E506C6174666F726D204C6162656C714170706C69636174696F6E204C6162656C").to_vec(), // map not array
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn hardware_model_test() {
    let valid = vec![hex!("A11901034B06092B0601040185BF1004").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.hardware_model.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("811901034B06092B0601040185BF1004").to_vec(), // map not array
        hex!("A11901034B06092B0601040185BF10").to_vec(),   // value too short
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn hardware_version_test() {
    let valid = vec![
        hex!("A11901048265312E312E3101").to_vec(),
        hex!("A11901048165312E312E31").to_vec(),
        hex!("A11901048266312E312E316102").to_vec(),
        hex!("A11901048268414243312E312E3103").to_vec(),
        hex!("A119010482613104").to_vec(),
        hex!("A11901048265312E322E33194000").to_vec(),
        hex!("A11901048263466F6F63426172").to_vec(),
        hex!("A11901048263466F6F1863").to_vec(),
    ];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.hardware_version.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("811901048265312E312E3101").to_vec(), // map not array
        hex!("A11901048165312E312E").to_vec(),     // value too short
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn intended_use_test() {
    let valid = vec![
        hex!("A119010D01").to_vec(),
        hex!("A119010D02").to_vec(),
        hex!("A119010D03").to_vec(),
        hex!("A119010D04").to_vec(),
        hex!("A119010D05").to_vec(),
    ];

    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.intended_use.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8119010D01").to_vec(), // map not array
        hex!("A119010D").to_vec(),   // value too short
        hex!("A119010D00").to_vec(), // unknown value
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn location_test() {
    let valid = vec![
        hex!("A1190108A9010002010302040305040605070608C11A63923B9A0907").to_vec(),
        hex!("A1190108A70100020103020504060508C11A63923B9A0907").to_vec(),
    ];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.location.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("81190108A9010002010302040305040605070608C11A63923B9A0907").to_vec(), // map not array
        hex!("A1190108A9010002010302040305040605070608C11A63923B9A").to_vec(), // value too short
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn profile_test() {
    let valid = vec![
        hex!("A11901094B06092B0601040185BF1004").to_vec(),
        hex!("A11901097818687474703A2F2F61726D2E636F6D2F7073612F322E302E30").to_vec(),
    ];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    // todo probably ought fail (has same non-standard key twice)
    let other = vec![hex!("a212496086480165030402011249608648016503040201").to_vec()];
    for v in other {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        match csc_d {
            Ok(v) => match v.other {
                Some(v) => {
                    println!("{:?}", v);
                }
                _ => {
                    panic!()
                }
            },
            Err(_) => panic!(),
        }
    }

    //todo third one ought fail as a duplicate but does not
    let invalid = vec![
        hex!("A119010902").to_vec(),
        hex!("a1121f").to_vec(),
        //hex!("A21901094960864801650304020119010949608648016503040201").to_vec(),
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn secboot_test() {
    let valid = vec![hex!("A1190106F5").to_vec(), hex!("A1190106F4").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(v.to_vec(), encoded_token2);
    }

    let invalid = vec![
        hex!("A1190106F6").to_vec(),
        hex!("A119010600").to_vec(),
        hex!("A11901061F").to_vec(),
        hex!("A11901066566616C7365").to_vec(),
        hex!("A11901066474727565").to_vec(),
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn sw_name_test() {
    let valid = vec![hex!("A119010F6C537472696E672056616C7565").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.sw_name.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8119010F6C537472696E672056616C7565").to_vec(), // map not array
        hex!("A119010F6C537472696E672056616C75").to_vec(),   // value too short
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn sw_version_test() {
    let valid = vec![
        hex!("A11901108265312E312E3101").to_vec(),
        hex!("A11901108165312E312E31").to_vec(),
        hex!("A11901108266312E312E316102").to_vec(),
        hex!("A11901108268414243312E312E3103").to_vec(),
        hex!("A119011082613104").to_vec(),
        hex!("A11901108265312E322E33194000").to_vec(),
        hex!("A11901108263466F6F63426172").to_vec(),
        hex!("A11901108263466F6F1863").to_vec(),
    ];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.sw_version.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("811901108265312E312E3101").to_vec(), // map not array
        hex!("A11901108165312E312E").to_vec(),     // value too short
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn ueid_test() {
    let csc = ClaimsSetClaimsCbor {
        iss: None,
        sub: None,
        aud: None,
        exp: None,
        nbf: None,
        iat: None,
        cti: None,
        nonce: None,
        boot_count: None,
        boot_seed: None,
        debug_status: None,
        dloas: None,
        hardware_model: None,
        hardware_version: None,
        intended_use: None,
        location: None,
        profile: None,
        secure_boot: None,
        sw_name: None,
        sw_version: None,
        ueid: Some(UeidType::Ueid(hex!("02deadbeefdead").to_vec())),
        uptime: None,
        manifests: None,
        measurements: None,
        other: None,
    };
    let mut encoded_token = vec![];
    let _ = into_writer(&csc, &mut encoded_token);
    println!(
        "Encoded ClaimsSetClaims: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
    assert_eq!(encoded_token, hex!("A11901004702DEADBEEFDEAD").to_vec());
    let valid = vec![hex!("A11901004702DEADBEEFDEAD").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.ueid.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("811901004702DEADBEEFDEAD").to_vec(), // map not array
        hex!("A11901004702DEADBEEFDE").to_vec(),   // value too short
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn uptime_test() {
    let valid = vec![hex!("A119010B04").to_vec()];
    for v in valid {
        let csc_d: ClaimsSetClaimsCbor = from_reader(v.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&csc_d, &mut encoded_token);
        assert_eq!(v.to_vec(), encoded_token);
        assert!(csc_d.uptime.is_some());
        let csc_json: ClaimsSetClaims = csc_d.try_into().unwrap();
        let csc_cbor: ClaimsSetClaimsCbor = csc_json.try_into().unwrap();
        let mut encoded_token2 = vec![];
        let _ = into_writer(&csc_cbor, &mut encoded_token2);
        assert_eq!(encoded_token2, v.to_vec());
    }

    let invalid = vec![
        hex!("8119010B04").to_vec(), // map not array
        hex!("A119010B").to_vec(),   // value too short
    ];
    for v in invalid {
        let csc_d: Result<ClaimsSetClaimsCbor, _> = from_reader(v.clone().as_slice());
        assert!(csc_d.is_err());
    }
}

#[test]
fn other_test() {
    // todo!("other_test")
}

#[test]
fn location_type_test() {
    let csc = ClaimsSetClaimsCbor {
        iss: None,
        sub: None,
        aud: None,
        exp: None,
        nbf: None,
        iat: None,
        cti: None,
        nonce: None,
        boot_count: None,
        boot_seed: None,
        debug_status: None,
        dloas: None,
        hardware_model: None,
        hardware_version: None,
        intended_use: None,
        location: Some(LocationTypeCbor {
            latitude: 0,
            longitude: 1,
            altitude: Some(2),
            accuracy: Some(3),
            altitude_accuracy: Some(4),
            heading: Some(5),
            speed: Some(6),
            timestamp: Some(TimeCbor::T(Required(1670527898))),
            age: Some(7),
        }),
        profile: None,
        secure_boot: None,
        sw_name: None,
        sw_version: None,
        ueid: None,
        uptime: None,
        manifests: None,
        measurements: None,
        other: None,
    };
    let mut encoded_token = vec![];
    let _ = into_writer(&csc, &mut encoded_token);
    println!(
        "Encoded ClaimsSetClaims: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
    assert_eq!(
        encoded_token,
        hex!("A1190108A9010002010302040305040605070608C11A63923B9A0907").to_vec()
    );
}

#[test]
fn sueids_type_test() {
    // todo!("sueids_type_test")
}
