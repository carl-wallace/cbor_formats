use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::tag::Required;
use ciborium::value::Value;
use common::arrays::*;
use common::*;

// --- BytesType ---

#[test]
fn bytes_type_from_value_ref() {
    let v = Value::Bytes(vec![1, 2, 3]);
    let bt = BytesType::try_from(&v).unwrap();
    assert_eq!(bt, BytesType::Bytes(vec![1, 2, 3]));
}

#[test]
fn bytes_type_from_value_owned() {
    let v = Value::Bytes(vec![4, 5, 6]);
    let bt = BytesType::try_from(v).unwrap();
    assert_eq!(bt, BytesType::Bytes(vec![4, 5, 6]));
}

#[test]
fn bytes_type_from_non_bytes_fails() {
    let v = Value::Text("not bytes".to_string());
    assert!(BytesType::try_from(&v).is_err());
    assert!(BytesType::try_from(v).is_err());
}

#[test]
fn bytes_type_roundtrip() {
    let bt = BytesType::Bytes(vec![0xDE, 0xAD]);
    let mut buf = vec![];
    into_writer(&bt, &mut buf).unwrap();
    let decoded: BytesType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(bt, decoded);
}

// --- NonceType ---

#[test]
fn nonce_type_single_valid() {
    let nonce_bytes = vec![0u8; 8]; // minimum valid size
    let v = Value::Bytes(nonce_bytes.clone());
    let nt = NonceType::try_from(&v).unwrap();
    assert_eq!(nt, NonceType::One(BytesType::Bytes(nonce_bytes)));
}

#[test]
fn nonce_type_single_max_valid() {
    let nonce_bytes = vec![0u8; 64]; // maximum valid size
    let v = Value::Bytes(nonce_bytes.clone());
    let nt = NonceType::try_from(&v).unwrap();
    assert_eq!(nt, NonceType::One(BytesType::Bytes(nonce_bytes)));
}

#[test]
fn nonce_type_too_short() {
    let v = Value::Bytes(vec![0u8; 7]);
    assert!(NonceType::try_from(&v).is_err());
}

#[test]
fn nonce_type_too_long() {
    let v = Value::Bytes(vec![0u8; 65]);
    assert!(NonceType::try_from(&v).is_err());
}

#[test]
fn nonce_type_array_valid() {
    let v = Value::Array(vec![
        Value::Bytes(vec![0u8; 8]),
        Value::Bytes(vec![0u8; 16]),
    ]);
    let nt = NonceType::try_from(&v).unwrap();
    match nt {
        NonceType::More(items) => assert_eq!(items.len(), 2),
        _ => panic!("expected More variant"),
    }
}

#[test]
fn nonce_type_array_with_invalid_size() {
    let v = Value::Array(vec![
        Value::Bytes(vec![0u8; 8]),
        Value::Bytes(vec![0u8; 3]), // too short
    ]);
    assert!(NonceType::try_from(&v).is_err());
}

#[test]
fn nonce_type_array_with_non_bytes() {
    let v = Value::Array(vec![Value::Text("not bytes".to_string())]);
    assert!(NonceType::try_from(&v).is_err());
}

#[test]
fn nonce_type_from_non_bytes_non_array() {
    let v = Value::Integer(42.into());
    assert!(NonceType::try_from(&v).is_err());
}

#[test]
fn nonce_type_roundtrip() {
    let nt = NonceType::One(BytesType::Bytes(vec![0xAA; 16]));
    let mut buf = vec![];
    into_writer(&nt, &mut buf).unwrap();
    let decoded: NonceType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(nt, decoded);
}

// --- UeidType ---

#[test]
fn ueid_type_valid_min() {
    let v = Value::Bytes(vec![0u8; 7]);
    let ut = UeidType::try_from(&v).unwrap();
    assert_eq!(ut, UeidType::Ueid(vec![0u8; 7]));
}

#[test]
fn ueid_type_valid_max() {
    let v = Value::Bytes(vec![0u8; 33]);
    let ut = UeidType::try_from(&v).unwrap();
    assert_eq!(ut, UeidType::Ueid(vec![0u8; 33]));
}

#[test]
fn ueid_type_too_short() {
    let v = Value::Bytes(vec![0u8; 6]);
    assert!(UeidType::try_from(&v).is_err());
}

#[test]
fn ueid_type_too_long() {
    let v = Value::Bytes(vec![0u8; 34]);
    assert!(UeidType::try_from(&v).is_err());
}

#[test]
fn ueid_type_non_bytes_fails() {
    let v = Value::Text("not bytes".to_string());
    assert!(UeidType::try_from(&v).is_err());
}

#[test]
fn ueid_type_roundtrip() {
    let ut = UeidType::Ueid(vec![0x01; 16]);
    let mut buf = vec![];
    into_writer(&ut, &mut buf).unwrap();
    let decoded: UeidType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ut, decoded);
}

// --- UuidType ---

#[test]
fn uuid_type_valid() {
    let v = Value::Bytes(vec![0u8; 16]);
    let ut = UuidType::try_from(&v).unwrap();
    assert_eq!(ut, UuidType::Uuid(vec![0u8; 16]));
}

#[test]
fn uuid_type_wrong_size() {
    let v = Value::Bytes(vec![0u8; 15]);
    assert!(UuidType::try_from(&v).is_err());
    let v = Value::Bytes(vec![0u8; 17]);
    assert!(UuidType::try_from(&v).is_err());
}

#[test]
fn uuid_type_non_bytes_fails() {
    let v = Value::Integer(42.into());
    assert!(UuidType::try_from(&v).is_err());
}

#[test]
fn uuid_type_roundtrip() {
    let ut = UuidType::Uuid(vec![0xAB; 16]);
    let mut buf = vec![];
    into_writer(&ut, &mut buf).unwrap();
    let decoded: UuidType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ut, decoded);
}

// --- OidType ---

#[test]
fn oid_type_roundtrip() {
    let ot = OidType::Oid(vec![0x2B, 0x06, 0x01]); // 1.3.6.1 prefix
    let mut buf = vec![];
    into_writer(&ot, &mut buf).unwrap();
    let decoded: OidType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ot, decoded);
}

// --- TextOrBinary ---

#[test]
fn text_or_binary_text_from_value() {
    let v = Value::Text("hello".to_string());
    let tob = TextOrBinary::try_from(&v).unwrap();
    assert_eq!(tob, TextOrBinary::Text("hello".to_string()));

    let tob2 = TextOrBinary::try_from(v).unwrap();
    assert_eq!(tob2, TextOrBinary::Text("hello".to_string()));
}

#[test]
fn text_or_binary_binary_from_value() {
    let v = Value::Bytes(vec![1, 2, 3]);
    let tob = TextOrBinary::try_from(&v).unwrap();
    assert_eq!(tob, TextOrBinary::Binary(vec![1, 2, 3]));

    let v2 = Value::Bytes(vec![1, 2, 3]);
    let tob2 = TextOrBinary::try_from(v2).unwrap();
    assert_eq!(tob2, TextOrBinary::Binary(vec![1, 2, 3]));
}

#[test]
fn text_or_binary_invalid_type() {
    let v = Value::Integer(42.into());
    assert!(TextOrBinary::try_from(&v).is_err());
    assert!(TextOrBinary::try_from(v).is_err());
}

#[test]
fn text_or_binary_roundtrip() {
    let t = TextOrBinary::Text("test".to_string());
    let mut buf = vec![];
    into_writer(&t, &mut buf).unwrap();
    let decoded: TextOrBinary = from_reader(buf.as_slice()).unwrap();
    assert_eq!(t, decoded);

    let b = TextOrBinary::Binary(vec![0xFF]);
    let mut buf2 = vec![];
    into_writer(&b, &mut buf2).unwrap();
    let decoded2: TextOrBinary = from_reader(buf2.as_slice()).unwrap();
    assert_eq!(b, decoded2);
}

// --- BinaryOrNil ---

#[test]
fn binary_or_nil_binary_from_value() {
    let v = Value::Bytes(vec![1, 2]);
    let bon = BinaryOrNil::try_from(&v).unwrap();
    assert_eq!(bon, BinaryOrNil::Binary(vec![1, 2]));

    let v2 = Value::Bytes(vec![1, 2]);
    let bon2 = BinaryOrNil::try_from(v2).unwrap();
    assert_eq!(bon2, BinaryOrNil::Binary(vec![1, 2]));
}

#[test]
fn binary_or_nil_nil_from_value() {
    let v = Value::Null;
    let bon = BinaryOrNil::try_from(&v).unwrap();
    assert_eq!(bon, BinaryOrNil::Nil);

    let bon2 = BinaryOrNil::try_from(Value::Null).unwrap();
    assert_eq!(bon2, BinaryOrNil::Nil);
}

#[test]
fn binary_or_nil_invalid_type() {
    let v = Value::Text("nope".to_string());
    assert!(BinaryOrNil::try_from(&v).is_err());
    assert!(BinaryOrNil::try_from(v).is_err());
}

// --- PkixBase64Type ---

#[test]
fn pkix_base64_from_value() {
    let v = Value::Text("MIIB...".to_string());
    let p = PkixBase64Type::try_from(&v).unwrap();
    assert_eq!(p, PkixBase64Type::Base64("MIIB...".to_string()));

    let v2 = Value::Text("MIIB...".to_string());
    let p2 = PkixBase64Type::try_from(v2).unwrap();
    assert_eq!(p2, PkixBase64Type::Base64("MIIB...".to_string()));
}

#[test]
fn pkix_base64_invalid_type() {
    let v = Value::Bytes(vec![1]);
    assert!(PkixBase64Type::try_from(&v).is_err());
    assert!(PkixBase64Type::try_from(v).is_err());
}

#[test]
fn pkix_base64_roundtrip() {
    let p = PkixBase64Type::Base64("dGVzdA==".to_string());
    let mut buf = vec![];
    into_writer(&p, &mut buf).unwrap();
    let decoded: PkixBase64Type = from_reader(buf.as_slice()).unwrap();
    assert_eq!(p, decoded);
}

// --- PkixCa ---

#[test]
fn pkix_ca_from_value() {
    let v = Value::Bytes(vec![0x30, 0x82]);
    let p = PkixCa::try_from(&v).unwrap();
    assert_eq!(p, PkixCa::Binary(vec![0x30, 0x82]));

    let v2 = Value::Bytes(vec![0x30, 0x82]);
    let p2 = PkixCa::try_from(v2).unwrap();
    assert_eq!(p2, PkixCa::Binary(vec![0x30, 0x82]));
}

#[test]
fn pkix_ca_invalid_type() {
    let v = Value::Text("nope".to_string());
    assert!(PkixCa::try_from(&v).is_err());
    assert!(PkixCa::try_from(v).is_err());
}

// --- TextOrInt ---

#[test]
fn text_or_int_text_from_value() {
    let v = Value::Text("hello".to_string());
    let ti = TextOrInt::try_from(&v).unwrap();
    assert_eq!(ti, TextOrInt::Text("hello".to_string()));

    let v2 = Value::Text("hello".to_string());
    let ti2 = TextOrInt::try_from(v2).unwrap();
    assert_eq!(ti2, TextOrInt::Text("hello".to_string()));
}

#[test]
fn text_or_int_int_from_value() {
    let v = Value::Integer(42.into());
    let ti = TextOrInt::try_from(&v).unwrap();
    assert_eq!(ti, TextOrInt::Int(42));

    let v2 = Value::Integer((-1).into());
    let ti2 = TextOrInt::try_from(v2).unwrap();
    assert_eq!(ti2, TextOrInt::Int(-1));
}

#[test]
fn text_or_int_invalid_type() {
    let v = Value::Bytes(vec![1]);
    assert!(TextOrInt::try_from(&v).is_err());
    assert!(TextOrInt::try_from(v).is_err());
}

// --- TimeCbor ---

#[test]
fn time_cbor_from_i64() {
    let tc = TimeCbor::try_from(1234567890i64).unwrap();
    let val: i64 = i64::try_from(&tc).unwrap();
    assert_eq!(val, 1234567890);
}

#[test]
fn time_cbor_from_i64_ref() {
    let t = 1234567890i64;
    let tc = TimeCbor::try_from(&t).unwrap();
    assert_eq!(tc, TimeCbor::T(Required(1234567890)));
}

#[test]
fn time_cbor_from_value() {
    let v = Value::Tag(1, Box::new(Value::Integer(1234567890.into())));
    let tc = TimeCbor::try_from(&v).unwrap();
    let val: i64 = i64::try_from(&tc).unwrap();
    assert_eq!(val, 1234567890);
}

#[test]
fn time_cbor_non_tag_fails() {
    let v = Value::Integer(42.into());
    assert!(TimeCbor::try_from(&v).is_err());
}

#[test]
fn time_cbor_roundtrip() {
    let tc = TimeCbor::try_from(1700000000i64).unwrap();
    let mut buf = vec![];
    into_writer(&tc, &mut buf).unwrap();
    // TimeCbor uses #[serde(untagged)] with a CBOR tag, so decode via Value
    let value: Value = from_reader(buf.as_slice()).unwrap();
    let decoded = TimeCbor::try_from(&value).unwrap();
    assert_eq!(tc, decoded);
}

// --- TaggedUriTypeCbor ---

#[test]
fn tagged_uri_type_cbor_from_value() {
    let v = Value::Tag(32, Box::new(Value::Text("https://example.com".to_string())));
    let u = TaggedUriTypeCbor::try_from(&v).unwrap();
    let s: String = String::try_from(&u).unwrap();
    assert_eq!(s, "https://example.com");
}

#[test]
fn tagged_uri_type_cbor_from_string() {
    let u = TaggedUriTypeCbor::try_from("https://example.com".to_string()).unwrap();
    let s: String = u.try_into().unwrap();
    assert_eq!(s, "https://example.com");
}

#[test]
fn tagged_uri_type_cbor_from_string_ref() {
    let s = "https://example.com".to_string();
    let u = TaggedUriTypeCbor::try_from(&s).unwrap();
    let s2: String = String::try_from(&u).unwrap();
    assert_eq!(s2, "https://example.com");
}

#[test]
fn tagged_uri_type_cbor_wrong_tag_fails() {
    let v = Value::Tag(33, Box::new(Value::Text("x".to_string())));
    assert!(TaggedUriTypeCbor::try_from(&v).is_err());
}

#[test]
fn tagged_uri_type_cbor_non_text_in_tag_fails() {
    let v = Value::Tag(32, Box::new(Value::Bytes(vec![1])));
    assert!(TaggedUriTypeCbor::try_from(&v).is_err());
}

#[test]
fn tagged_uri_type_cbor_roundtrip() {
    let u = TaggedUriTypeCbor::try_from("https://example.com".to_string()).unwrap();
    let mut buf = vec![];
    into_writer(&u, &mut buf).unwrap();
    let value: Value = from_reader(buf.as_slice()).unwrap();
    let decoded = TaggedUriTypeCbor::try_from(&value).unwrap();
    assert_eq!(u, decoded);
}

// --- OidOrUri ---

#[test]
fn oid_or_uri_uri_from_value() {
    let v = Value::Tag(32, Box::new(Value::Text("https://example.com".to_string())));
    let ou = OidOrUri::try_from(&v).unwrap();
    assert_eq!(ou, OidOrUri::U("https://example.com".to_string()));
}

#[test]
fn oid_or_uri_oid_from_value() {
    let v = Value::Tag(111, Box::new(Value::Bytes(vec![0x2B, 0x06])));
    let ou = OidOrUri::try_from(&v).unwrap();
    assert_eq!(ou, OidOrUri::O(OidType::Oid(vec![0x2B, 0x06])));
}

#[test]
fn oid_or_uri_invalid_tag() {
    let v = Value::Tag(99, Box::new(Value::Text("x".to_string())));
    assert!(OidOrUri::try_from(&v).is_err());
}

#[test]
fn oid_or_uri_non_text_in_tag32() {
    let v = Value::Tag(32, Box::new(Value::Bytes(vec![1])));
    assert!(OidOrUri::try_from(&v).is_err());
}

#[test]
fn oid_or_uri_non_bytes_in_tag111() {
    let v = Value::Tag(111, Box::new(Value::Text("x".to_string())));
    assert!(OidOrUri::try_from(&v).is_err());
}

// --- OidOrUriCbor ---

#[test]
fn oid_or_uri_cbor_uri_from_value() {
    let v = Value::Tag(32, Box::new(Value::Text("https://example.com".to_string())));
    let ou = OidOrUriCbor::try_from(&v).unwrap();
    match ou {
        OidOrUriCbor::U(_) => {}
        _ => panic!("expected U variant"),
    }
}

#[test]
fn oid_or_uri_cbor_oid_from_value() {
    let v = Value::Tag(111, Box::new(Value::Bytes(vec![0x2B])));
    let ou = OidOrUriCbor::try_from(&v).unwrap();
    match ou {
        OidOrUriCbor::O(_) => {}
        _ => panic!("expected O variant"),
    }
}

#[test]
fn oid_or_uri_cbor_invalid_tag() {
    let v = Value::Tag(99, Box::new(Value::Text("x".to_string())));
    assert!(OidOrUriCbor::try_from(&v).is_err());
}

#[test]
fn oid_or_uri_cbor_roundtrip_uri() {
    let ou = OidOrUriCbor::U(TaggedUriTypeCbor::U(Required(
        "https://example.com".to_string(),
    )));
    let mut buf = vec![];
    into_writer(&ou, &mut buf).unwrap();
    let value: Value = from_reader(buf.as_slice()).unwrap();
    let decoded = OidOrUriCbor::try_from(&value).unwrap();
    assert_eq!(ou, decoded);
}

#[test]
fn oid_or_uri_cbor_roundtrip_oid() {
    let ou = OidOrUriCbor::O(Required(OidType::Oid(vec![0x2B, 0x06, 0x01])));
    let mut buf = vec![];
    into_writer(&ou, &mut buf).unwrap();
    let value: Value = from_reader(buf.as_slice()).unwrap();
    let decoded = OidOrUriCbor::try_from(&value).unwrap();
    assert_eq!(ou, decoded);
}

// --- Tagged type aliases ---

#[test]
fn tagged_svn_roundtrip() {
    let svn: TaggedSvn = Required(42);
    let mut buf = vec![];
    into_writer(&svn, &mut buf).unwrap();
    let decoded: TaggedSvn = from_reader(buf.as_slice()).unwrap();
    assert_eq!(svn, decoded);
}

#[test]
fn tagged_min_svn_roundtrip() {
    let svn: TaggedMinSvn = Required(10);
    let mut buf = vec![];
    into_writer(&svn, &mut buf).unwrap();
    let decoded: TaggedMinSvn = from_reader(buf.as_slice()).unwrap();
    assert_eq!(svn, decoded);
}

#[test]
fn tagged_uuid_roundtrip() {
    let uuid: TaggedUuidType = Required(UuidType::Uuid(vec![0xAB; 16]));
    let mut buf = vec![];
    into_writer(&uuid, &mut buf).unwrap();
    let decoded: TaggedUuidType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(uuid, decoded);
}

#[test]
fn tagged_ueid_roundtrip() {
    let ueid: TaggedUeidType = Required(UeidType::Ueid(vec![0x01; 16]));
    let mut buf = vec![];
    into_writer(&ueid, &mut buf).unwrap();
    let decoded: TaggedUeidType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ueid, decoded);
}

#[test]
fn tagged_bytes_roundtrip() {
    let tb: TaggedBytes = Required(BytesType::Bytes(vec![0xDE, 0xAD]));
    let mut buf = vec![];
    into_writer(&tb, &mut buf).unwrap();
    let decoded: TaggedBytes = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tb, decoded);
}

#[test]
fn tagged_cose_key_type_roundtrip() {
    let tk: TaggedCoseKeyType = Required(BytesType::Bytes(vec![0x01, 0x02]));
    let mut buf = vec![];
    into_writer(&tk, &mut buf).unwrap();
    let decoded: TaggedCoseKeyType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tk, decoded);
}

#[test]
fn tagged_pkix_asn1_der_cert_type_roundtrip() {
    let tp: TaggedPkixAsn1DerCertType = Required(BytesType::Bytes(vec![0x30, 0x82]));
    let mut buf = vec![];
    into_writer(&tp, &mut buf).unwrap();
    let decoded: TaggedPkixAsn1DerCertType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tp, decoded);
}

#[test]
fn tagged_pkix_base64_key_type_roundtrip() {
    let tp: TaggedPkixBase64KeyType = Required("MIIBIjAN...".to_string());
    let mut buf = vec![];
    into_writer(&tp, &mut buf).unwrap();
    let decoded: TaggedPkixBase64KeyType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tp, decoded);
}

#[test]
fn tagged_pkix_base64_cert_type_roundtrip() {
    let tp: TaggedPkixBase64CertType = Required("MIIBIjAN...".to_string());
    let mut buf = vec![];
    into_writer(&tp, &mut buf).unwrap();
    let decoded: TaggedPkixBase64CertType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tp, decoded);
}

#[test]
fn tagged_pkix_base64_cert_path_type_roundtrip() {
    let tp: TaggedPkixBase64CertPathType = Required("MIIBIjAN...".to_string());
    let mut buf = vec![];
    into_writer(&tp, &mut buf).unwrap();
    let decoded: TaggedPkixBase64CertPathType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tp, decoded);
}

#[test]
fn tagged_key_thumbprint_type_roundtrip() {
    let he = HashEntry {
        hash_alg_id: 1,
        hash_value: vec![0xAA; 32],
    };
    let tp: TaggedKeyThumbprintType = Required(he);
    let mut buf = vec![];
    into_writer(&tp, &mut buf).unwrap();
    let decoded: TaggedKeyThumbprintType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tp, decoded);
}

#[test]
fn tagged_cert_thumbprint_type_roundtrip() {
    let he = HashEntry {
        hash_alg_id: 2,
        hash_value: vec![0xBB; 32],
    };
    let tp: TaggedCertThumbprintType = Required(he);
    let mut buf = vec![];
    into_writer(&tp, &mut buf).unwrap();
    let decoded: TaggedCertThumbprintType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tp, decoded);
}

#[test]
fn tagged_cert_path_thumbprint_type_roundtrip() {
    let he = HashEntry {
        hash_alg_id: 7,
        hash_value: vec![0xCC; 48],
    };
    let tp: TaggedCertPathThumbprintType = Required(he);
    let mut buf = vec![];
    into_writer(&tp, &mut buf).unwrap();
    let decoded: TaggedCertPathThumbprintType = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tp, decoded);
}

// --- MaskedRawValue and IntRange (from arrays.rs) ---

#[test]
fn masked_raw_value_roundtrip() {
    let mrv = MaskedRawValueCbor {
        value: vec![0xFF, 0x00],
        mask: vec![0xFF, 0xFF],
    };
    let mut buf = vec![];
    into_writer(&mrv, &mut buf).unwrap();
    let decoded: MaskedRawValueCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(mrv, decoded);

    let json_mrv: MaskedRawValue = decoded.try_into().unwrap();
    assert_eq!(json_mrv.value, vec![0xFF, 0x00]);
    assert_eq!(json_mrv.mask, vec![0xFF, 0xFF]);
}

#[test]
fn tagged_masked_raw_value_roundtrip() {
    let mrv = MaskedRawValueCbor {
        value: vec![0x01],
        mask: vec![0xFF],
    };
    let tagged: TaggedMaskedRawValue = Required(mrv);
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: TaggedMaskedRawValue = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tagged, decoded);
}

#[test]
fn int_range_roundtrip() {
    let ir = IntRangeCbor { min: -10, max: 100 };
    let mut buf = vec![];
    into_writer(&ir, &mut buf).unwrap();
    let decoded: IntRangeCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ir, decoded);

    let json_ir: IntRange = decoded.try_into().unwrap();
    assert_eq!(json_ir.min, -10);
    assert_eq!(json_ir.max, 100);
}

#[test]
fn tagged_int_range_roundtrip() {
    let ir = IntRangeCbor { min: 0, max: 255 };
    let tagged: TaggedIntRange = Required(ir);
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: TaggedIntRange = from_reader(buf.as_slice()).unwrap();
    assert_eq!(tagged, decoded);
}
