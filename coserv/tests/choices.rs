use ciborium::{de::from_reader, ser::into_writer, value::Value};

use coserv::choices::{ArtifactType, ResultType};

#[test]
fn artifact_type_roundtrip() {
    for (variant, expected_val) in [
        (ArtifactType::EndorsedValues, 0i64),
        (ArtifactType::TrustAnchors, 1),
        (ArtifactType::ReferenceValues, 2),
    ] {
        let mut buf = vec![];
        into_writer(&variant, &mut buf).unwrap();
        let decoded: ArtifactType = from_reader(buf.as_slice()).unwrap();
        assert_eq!(variant, decoded);

        // test TryFrom<Value>
        let val = Value::Integer(expected_val.into());
        let from_val = ArtifactType::try_from(val.clone()).unwrap();
        assert_eq!(variant, from_val);
        let from_ref = ArtifactType::try_from(&val).unwrap();
        assert_eq!(variant, from_ref);
    }
}

#[test]
fn artifact_type_invalid() {
    let val = Value::Integer(99i64.into());
    assert!(ArtifactType::try_from(val).is_err());

    let val = Value::Text("bad".to_string());
    assert!(ArtifactType::try_from(val).is_err());
}

#[test]
fn result_type_roundtrip() {
    for (variant, expected_val) in [
        (ResultType::CollectedArtifacts, 0i64),
        (ResultType::SourceArtifacts, 1),
        (ResultType::Both, 2),
    ] {
        let mut buf = vec![];
        into_writer(&variant, &mut buf).unwrap();
        let decoded: ResultType = from_reader(buf.as_slice()).unwrap();
        assert_eq!(variant, decoded);

        // test TryFrom<Value>
        let val = Value::Integer(expected_val.into());
        let from_val = ResultType::try_from(val.clone()).unwrap();
        assert_eq!(variant, from_val);
        let from_ref = ResultType::try_from(&val).unwrap();
        assert_eq!(variant, from_ref);
    }
}

#[test]
fn result_type_invalid() {
    let val = Value::Integer(99i64.into());
    assert!(ResultType::try_from(val).is_err());

    let val = Value::Text("bad".to_string());
    assert!(ResultType::try_from(val).is_err());
}
