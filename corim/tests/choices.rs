use crate::utils::buffer_to_hex;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::tag::Required;
use ciborium::value::{Integer, Value};
use common::choices::*;
use common::{IntType, OidType, UeidType, UuidType};
use corim::choices::*;

mod utils;

#[test]
fn class_id_type_choice_test() {
    let v = vec![0x01, 0x02, 0x03];
    let fab = ClassIdTypeChoiceCbor::Oid(Required(OidType::Oid(v)));
    let mut encoded_token = vec![];
    let _ = into_writer(&fab, &mut encoded_token).unwrap();

    let fab_j: ClassIdTypeChoice = fab.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j).unwrap();
    let fab_c: ClassIdTypeChoiceCbor = fab_j.try_into().unwrap();
    assert_eq!(fab, fab_c);

    let fab2 = ClassIdTypeChoiceCbor::Uuid(Required(UuidType::Uuid(vec![
        104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 112,
    ])));
    let mut encoded_token2 = vec![];
    let _ = into_writer(&fab2, &mut encoded_token2).unwrap();

    let fab2_j: ClassIdTypeChoice = fab2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab2_j).unwrap();
    let fab2_c: ClassIdTypeChoiceCbor = fab2_j.try_into().unwrap();
    assert_eq!(fab2, fab2_c);

    let fab3 = ClassIdTypeChoiceCbor::Int(Required(IntType::Int([0x01].to_vec())));
    let mut encoded_token3 = vec![];
    let _ = into_writer(&fab3, &mut encoded_token3).unwrap();

    let fab3_j: ClassIdTypeChoice = fab3.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab3_j).unwrap();
    let fab3_c: ClassIdTypeChoiceCbor = fab3_j.try_into().unwrap();
    assert_eq!(fab3, fab3_c);
}

#[test]
fn corim_id_type_choice_test() {
    use common::*;
    let mut encoded_token = vec![];
    let citc = CorimIdTypeChoice::Str("bah".to_string());
    let _ = into_writer(&citc, &mut encoded_token);
    let citc_d: CorimIdTypeChoice = from_reader(encoded_token.clone().as_slice()).unwrap();
    assert_eq!(citc, citc_d);

    let mut encoded_token2 = vec![];
    let citc2 = CorimIdTypeChoice::Uuid(UuidType::Uuid(vec![
        104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 112,
    ]));
    let _ = into_writer(&citc2, &mut encoded_token2);
    let citc_d2: CorimIdTypeChoice = from_reader(encoded_token2.clone().as_slice()).unwrap();
    assert_eq!(citc2, citc_d2);
}

#[test]
fn corim_role_type_choice_test() {
    let cr = CorimRoleTypeChoiceKnownCbor::Creator;
    let mut encoded_token = vec![];
    let _ = into_writer(&cr, &mut encoded_token);
    println!(
        "Encoded CorimRoleTypeChoiceCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
    let cr_d: CorimRoleTypeChoiceKnownCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    assert_eq!(cr, cr_d);

    let vcr = vec![
        CorimRoleTypeChoiceCbor::Known(CorimRoleTypeChoiceKnownCbor::Creator),
        CorimRoleTypeChoiceCbor::Known(CorimRoleTypeChoiceKnownCbor::TagCreator),
        CorimRoleTypeChoiceCbor::Known(CorimRoleTypeChoiceKnownCbor::Maintainer),
        CorimRoleTypeChoiceCbor::Extensions(55),
    ];
    let mut encoded_token2 = vec![];
    let _ = into_writer(&vcr, &mut encoded_token2);
    println!(
        "Encoded Vec<CorimRoleTypeChoiceCbor>: {:?}",
        buffer_to_hex(encoded_token2.as_slice())
    );
    let vcr_d: Vec<CorimRoleTypeChoiceCbor> =
        from_reader(encoded_token2.clone().as_slice()).unwrap();
    assert_eq!(vcr, vcr_d);
}

#[test]
fn crypto_key_type_choice_test() {
    let base64_key = "-----BEGIN PUBLIC KEY-----
MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAEn1LlwLN/KBYQRVH6HfIMTzfEqJOVztLe
kLchp2hi78cCaMY81FBlYs8J9l7krc+M4aBeCGYFjba+hiXttJWPL7ydlE+5UG4U
Nkn3Eos8EiZByi9DVsyfy9eejh+8AXgp
-----END PUBLIC KEY-----";
    let base64_cert = "-----BEGIN CERTIFICATE-----
MIICLDCCAdKgAwIBAgIBADAKBggqhkjOPQQDAjB9MQswCQYDVQQGEwJCRTEPMA0G
A1UEChMGR251VExTMSUwIwYDVQQLExxHbnVUTFMgY2VydGlmaWNhdGUgYXV0aG9y
aXR5MQ8wDQYDVQQIEwZMZXV2ZW4xJTAjBgNVBAMTHEdudVRMUyBjZXJ0aWZpY2F0
ZSBhdXRob3JpdHkwHhcNMTEwNTIzMjAzODIxWhcNMTIxMjIyMDc0MTUxWjB9MQsw
CQYDVQQGEwJCRTEPMA0GA1UEChMGR251VExTMSUwIwYDVQQLExxHbnVUTFMgY2Vy
dGlmaWNhdGUgYXV0aG9yaXR5MQ8wDQYDVQQIEwZMZXV2ZW4xJTAjBgNVBAMTHEdu
dVRMUyBjZXJ0aWZpY2F0ZSBhdXRob3JpdHkwWTATBgcqhkjOPQIBBggqhkjOPQMB
BwNCAARS2I0jiuNn14Y2sSALCX3IybqiIJUvxUpj+oNfzngvj/Niyv2394BWnW4X
uQ4RTEiywK87WRcWMGgJB5kX/t2no0MwQTAPBgNVHRMBAf8EBTADAQH/MA8GA1Ud
DwEB/wQFAwMHBgAwHQYDVR0OBBYEFPC0gf6YEr+1KLlkQAPLzB9mTigDMAoGCCqG
SM49BAMCA0gAMEUCIDGuwD1KPyG+hRf88MeyMQcqOFZD0TbVleF+UsAGQ4enAiEA
l4wOuDwKQa+upc8GftXE2C//4mKANBC6It01gUaTIpo=
-----END CERTIFICATE-----";

    let fab = CryptoKeyTypeChoice::Key(Required(base64_key.to_string()));
    let mut encoded_token = vec![];
    let _ = into_writer(&fab, &mut encoded_token).unwrap();

    let fab_j: CryptoKeyTypeChoice = fab.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j).unwrap();
    let fab_c: CryptoKeyTypeChoice = fab_j.try_into().unwrap();
    assert_eq!(fab, fab_c);

    let fab2 = CryptoKeyTypeChoice::Cert(Required(base64_cert.to_string()));
    let mut encoded_token2 = vec![];
    let _ = into_writer(&fab2, &mut encoded_token2).unwrap();

    let fab_j2: CryptoKeyTypeChoice = fab2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j2).unwrap();
    let fab_c2: CryptoKeyTypeChoice = fab_j2.try_into().unwrap();
    assert_eq!(fab2, fab_c2);

    let fab3 = CryptoKeyTypeChoice::Cert(Required(base64_cert.to_string()));
    let mut encoded_token3 = vec![];
    let _ = into_writer(&fab3, &mut encoded_token3).unwrap();

    let fab_j3: CryptoKeyTypeChoice = fab3.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j3).unwrap();
    let fab_c3: CryptoKeyTypeChoice = fab_j3.try_into().unwrap();
    assert_eq!(fab3, fab_c3);
}

#[test]
fn domain_type_choice_test() {
    let fab = DomainTypeChoice::U64(5);
    let mut encoded_token = vec![];
    let _ = into_writer(&fab, &mut encoded_token).unwrap();

    let fab_j: DomainTypeChoice = fab.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j).unwrap();
    let fab_c: DomainTypeChoice = fab_j.try_into().unwrap();
    assert_eq!(fab, fab_c);

    let fab2 = DomainTypeChoice::Uuid(Required(UuidType::Uuid(vec![
        104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 112,
    ])));
    let mut encoded_token2 = vec![];
    let _ = into_writer(&fab2, &mut encoded_token2).unwrap();

    let fab2_j: DomainTypeChoice = fab2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab2_j).unwrap();
    let fab2_c: DomainTypeChoice = fab2_j.try_into().unwrap();
    assert_eq!(fab2, fab2_c);

    let fab3 = DomainTypeChoice::Text("bah".to_string());
    let mut encoded_token3 = vec![];
    let _ = into_writer(&fab3, &mut encoded_token3).unwrap();

    let fab3_j: DomainTypeChoice = fab3.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab3_j).unwrap();
    let fab3_c: DomainTypeChoice = fab3_j.try_into().unwrap();
    assert_eq!(fab3, fab3_c);
}

#[test]
fn entity_name_type_choice_test() {
    let mut encoded_token = vec![];
    let entc = EntityNameTypeChoice::Text("bah".to_string());
    let _ = into_writer(&entc, &mut encoded_token);
    let entc_d: EntityNameTypeChoice = from_reader(encoded_token.clone().as_slice()).unwrap();
    assert_eq!(entc, entc_d);
}

#[test]
fn group_id_type_choice_test() {
    let fab2 = GroupIdTypeChoice::Uuid(Required(UuidType::Uuid(vec![
        104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 112,
    ])));
    let mut encoded_token2 = vec![];
    let _ = into_writer(&fab2, &mut encoded_token2).unwrap();

    let fab2_j: GroupIdTypeChoice = fab2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab2_j).unwrap();
    let fab2_c: GroupIdTypeChoice = fab2_j.try_into().unwrap();
    assert_eq!(fab2, fab2_c);
}

#[test]
fn instance_id_type_choice_test() {
    let fab = InstanceIdTypeChoice::Uuid(Required(UuidType::Uuid(vec![
        104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 112,
    ])));
    let mut encoded_token = vec![];
    let _ = into_writer(&fab, &mut encoded_token).unwrap();

    let fab_j: InstanceIdTypeChoice = fab.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j).unwrap();
    let fab_c: InstanceIdTypeChoice = fab_j.try_into().unwrap();
    assert_eq!(fab, fab_c);

    let fab2 = InstanceIdTypeChoice::Ueid(Required(UeidType::Ueid(vec![
        104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 112,
    ])));
    let mut encoded_token2 = vec![];
    let _ = into_writer(&fab2, &mut encoded_token2).unwrap();

    let fab2_j: InstanceIdTypeChoice = fab2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab2_j).unwrap();
    let fab2_c: InstanceIdTypeChoice = fab2_j.try_into().unwrap();
    assert_eq!(fab2, fab2_c);
}

#[test]
fn measured_element_type_choice_test() {
    let v = vec![0x01, 0x02, 0x03];
    let fab = MeasuredElementTypeChoiceCbor::Oid(Required(OidType::Oid(v)));
    let mut encoded_token = vec![];
    let _ = into_writer(&fab, &mut encoded_token).unwrap();

    let fab_j: MeasuredElementTypeChoice = fab.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j).unwrap();
    let fab_c: MeasuredElementTypeChoiceCbor = fab_j.try_into().unwrap();
    assert_eq!(fab, fab_c);

    let fab2 = MeasuredElementTypeChoiceCbor::Uuid(Required(UuidType::Uuid(vec![
        104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 112,
    ])));
    let mut encoded_token2 = vec![];
    let _ = into_writer(&fab2, &mut encoded_token2).unwrap();

    let fab2_j: MeasuredElementTypeChoice = fab2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab2_j).unwrap();
    let fab2_c: MeasuredElementTypeChoiceCbor = fab2_j.try_into().unwrap();
    assert_eq!(fab2, fab2_c);
}

#[test]
fn profile_type_choice_test() {
    let v = vec![0x01, 0x02, 0x03];
    let fab = ProfileTypeChoiceCbor::Oid(Required(OidType::Oid(v)));
    let mut encoded_token = vec![];
    let _ = into_writer(&fab, &mut encoded_token).unwrap();

    let fab_j: ProfileTypeChoice = fab.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j).unwrap();
    let fab_c: ProfileTypeChoiceCbor = fab_j.try_into().unwrap();
    assert_eq!(fab, fab_c);

    let fab2 = ProfileTypeChoiceCbor::Uri("www.example.com".to_string());
    let mut encoded_token = vec![];
    let _ = into_writer(&fab2, &mut encoded_token).unwrap();

    let fab2_j: ProfileTypeChoice = fab2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab2_j).unwrap();
    let fab2_c: ProfileTypeChoiceCbor = fab2_j.try_into().unwrap();
    assert_eq!(fab2, fab2_c);
}

#[test]
fn svn_type_choice_test() {
    let fab = SvnTypeChoice::TaggedSvn(Required(2));
    let mut encoded_token = vec![];
    let _ = into_writer(&fab, &mut encoded_token).unwrap();

    let fab_j: SvnTypeChoice = fab.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j).unwrap();
    let fab_c: SvnTypeChoice = fab_j.try_into().unwrap();
    assert_eq!(fab, fab_c);

    let fab2 = SvnTypeChoice::TaggedMinSvn(Required(1));
    let mut encoded_token = vec![];
    let _ = into_writer(&fab2, &mut encoded_token).unwrap();

    let fab2_j: SvnTypeChoice = fab2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab2_j).unwrap();
    let fab2_c: SvnTypeChoice = fab2_j.try_into().unwrap();
    assert_eq!(fab2, fab2_c);
}

#[test]
fn tag_id_type_choice_test() {
    use common::*;
    let titc = TagIdTypeChoiceCbor::Str("bah".to_string());
    let mut encoded_token = vec![];
    let _ = into_writer(&titc, &mut encoded_token).unwrap();

    let titc_j: TagIdTypeChoice = titc.clone().try_into().unwrap();
    let _ = serde_json::to_string(&titc_j).unwrap();
    let titc_cbor: TagIdTypeChoiceCbor = titc_j.try_into().unwrap();
    assert_eq!(titc, titc_cbor);

    let mut encoded_token2 = vec![];
    let titc2 = TagIdTypeChoiceCbor::Uuid(UuidType::Uuid(vec![
        104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 112,
    ]));
    let _ = into_writer(&titc2, &mut encoded_token2).unwrap();

    let titc2_j: TagIdTypeChoice = titc2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&titc2_j).unwrap();
    let titc2_cbor: TagIdTypeChoiceCbor = titc2_j.try_into().unwrap();
    assert_eq!(titc2, titc2_cbor);
}

#[test]
fn tag_rel_type_choice_test() {
    let rtc = TagRelTypeChoice::Known(TagRelTypeChoiceKnown::Replaces);
    let mut encoded_token = vec![];
    let _ = into_writer(&rtc, &mut encoded_token).unwrap();
    assert_eq!([0x01], encoded_token.as_slice());

    let rtc_j: TagRelTypeChoice = rtc.clone().try_into().unwrap();
    let _ = serde_json::to_string(&rtc_j).unwrap();
    let rtc_cbor: TagRelTypeChoice = rtc_j.try_into().unwrap();
    assert_eq!(rtc, rtc_cbor);

    let mut encoded_token2 = vec![];
    let _ = into_writer(&rtc_cbor, &mut encoded_token2).unwrap();
    assert_eq!([0x01], encoded_token2.as_slice());

    let rtc2 = TagRelTypeChoice::Extensions(32);
    let mut encoded_token = vec![];
    let _ = into_writer(&rtc2, &mut encoded_token).unwrap();
    assert_eq!([0x18, 0x20], encoded_token.as_slice());

    let rtc2_j: TagRelTypeChoice = rtc2.clone().try_into().unwrap();
    let _ = serde_json::to_string(&rtc2_j).unwrap();
    let rtc2_cbor: TagRelTypeChoice = rtc2_j.try_into().unwrap();
    assert_eq!(rtc2, rtc2_cbor);

    let mut encoded_token2 = vec![];
    let _ = into_writer(&rtc2_cbor, &mut encoded_token2).unwrap();
    assert_eq!([0x18, 0x20], encoded_token2.as_slice());
}

#[test]
fn tag_version_type_test() {
    let v = TagVersionType::U64(1);
    let mut encoded_token = vec![];
    let _ = into_writer(&v, &mut encoded_token).unwrap();
    assert_eq!([0x01], encoded_token.as_slice());

    let v_j: TagVersionType = v.clone().try_into().unwrap();
    let _ = serde_json::to_string(&v_j).unwrap();
    let v_cbor: TagVersionType = v_j.try_into().unwrap();
    assert_eq!(v, v_cbor);

    let mut encoded_token2 = vec![];
    let _ = into_writer(&v_cbor, &mut encoded_token2).unwrap();
    assert_eq!([0x01], encoded_token2.as_slice());
}

#[test]
fn version_scheme_test() {
    let vs = VersionSchemeCbor::Known(VersionSchemeKnownCbor::AlphaNumeric);
    let mut encoded_token = vec![];
    let _ = into_writer(&vs, &mut encoded_token).unwrap();
    assert_eq!([0x03], encoded_token.as_slice());

    let vs_j: VersionScheme = vs.clone().try_into().unwrap();
    let _ = serde_json::to_string(&vs_j).unwrap();
    let vs_cbor: VersionSchemeCbor = vs_j.try_into().unwrap();
    assert_eq!(vs, vs_cbor);

    let vs2 = vs.clone();
    assert_eq!(vs2, vs);

    let vs3 = VersionSchemeCbor::Text("custom".to_string());
    let mut encoded_token3 = vec![];
    let _ = into_writer(&vs3, &mut encoded_token3).unwrap();
    let vs3b: VersionSchemeCbor = from_reader(encoded_token3.as_slice()).unwrap();
    assert_eq!(vs3b, vs3);

    let vs_j3: VersionScheme = vs3.clone().try_into().unwrap();
    let _ = serde_json::to_string(&vs_j3).unwrap();
    let vs_cbor3: VersionSchemeCbor = vs_j3.try_into().unwrap();
    assert_eq!(vs3, vs_cbor3);

    let unknown = 99999;
    let unknown_as_int: Integer = unknown.try_into().unwrap();
    let unknown_as_value: Value = unknown_as_int.try_into().unwrap();
    let vs4: VersionSchemeCbor = unknown_as_value.try_into().unwrap();
    assert_eq!(vs4, VersionSchemeCbor::IntExtensions(99999));
    let mut encoded_token4 = vec![];
    let _ = into_writer(&vs4, &mut encoded_token4).unwrap();
    let vs4b: VersionSchemeCbor = from_reader(encoded_token4.as_slice()).unwrap();
    assert_eq!(vs4, vs4b);

    let vs_j4: VersionScheme = vs4.clone().try_into().unwrap();
    let _ = serde_json::to_string(&vs_j4).unwrap();
    let vs_cbor4: VersionSchemeCbor = vs_j4.try_into().unwrap();
    assert_eq!(vs4, vs_cbor4);
}
