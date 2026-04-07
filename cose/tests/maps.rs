use common::TextOrInt;
use cose::maps::*;

#[test]
fn header_map_validate_iv_only() {
    let hm = HeaderMap {
        alg_id: None,
        criticality: None,
        content_type: None,
        key_id: None,
        iv: Some(vec![0x01, 0x02, 0x03]),
        partial_iv: None,
        other: None,
    };
    assert!(hm.validate().is_ok());
}

#[test]
fn header_map_validate_partial_iv_only() {
    let hm = HeaderMap {
        alg_id: None,
        criticality: None,
        content_type: None,
        key_id: None,
        iv: None,
        partial_iv: Some(vec![0x04, 0x05]),
        other: None,
    };
    assert!(hm.validate().is_ok());
}

#[test]
fn header_map_validate_neither_iv() {
    let hm = HeaderMap {
        alg_id: Some(TextOrInt::Int(-7)),
        criticality: None,
        content_type: None,
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };
    assert!(hm.validate().is_ok());
}

#[test]
fn header_map_validate_both_iv_fails() {
    let hm = HeaderMap {
        alg_id: None,
        criticality: None,
        content_type: None,
        key_id: None,
        iv: Some(vec![0x01, 0x02, 0x03]),
        partial_iv: Some(vec![0x04, 0x05]),
        other: None,
    };
    let result = hm.validate();
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("must not contain both IV (label 5) and Partial IV (label 6)")
    );
}
