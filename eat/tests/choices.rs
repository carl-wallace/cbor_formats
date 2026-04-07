use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use eat::choices::*;

#[test]
fn debug_status_type_test() {
    // Test each variant roundtrips through CBOR
    let variants = [
        (DebugStatusType::Enabled, 0i8),
        (DebugStatusType::Disabled, 1),
        (DebugStatusType::DisabledSinceBoot, 2),
        (DebugStatusType::DisabledPermanently, 3),
        (DebugStatusType::DisabledFullyAndPermanently, 4),
    ];
    for (variant, expected_val) in variants {
        let mut buf = vec![];
        into_writer(&variant, &mut buf).unwrap();
        let decoded: DebugStatusType = from_reader(buf.as_slice()).unwrap();
        assert_eq!(variant, decoded);
        // Verify repr value
        assert_eq!(variant.clone() as i8, expected_val);
    }
}

#[test]
fn intended_use_test() {
    let variants = [
        (IntendedUseType::Generic, 1i8),
        (IntendedUseType::Registration, 2),
        (IntendedUseType::Provisioning, 3),
        (IntendedUseType::Csr, 4),
        (IntendedUseType::Pop, 5),
    ];
    for (variant, expected_val) in variants {
        let mut buf = vec![];
        into_writer(&variant, &mut buf).unwrap();
        let decoded: IntendedUseType = from_reader(buf.as_slice()).unwrap();
        assert_eq!(variant, decoded);
        assert_eq!(variant.clone() as i8, expected_val);
    }
}

#[test]
fn result_type_test() {
    let variants = [
        (ResultType::Success, 1i8),
        (ResultType::Fail, 2),
        (ResultType::NotRun, 3),
        (ResultType::Absent, 4),
    ];
    for (variant, expected_val) in variants {
        let mut buf = vec![];
        into_writer(&variant, &mut buf).unwrap();
        let decoded: ResultType = from_reader(buf.as_slice()).unwrap();
        assert_eq!(variant, decoded);
        assert_eq!(variant.clone() as i8, expected_val);
    }
}
