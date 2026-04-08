//! Internal helper functions for COSE structure construction.

use alloc::string::ToString;
use alloc::vec::Vec;
use common::TextOrInt;
use cose::choices::EmptyOrSerializedMap;
use cose::maps::{HeaderMap, HeaderMapCbor};

use crate::algorithm::CoseAlgorithm;
use crate::error::CoseCryptoError;

/// Serialize a `HeaderMap` into `EmptyOrSerializedMap`.
/// If the header map has no fields set, returns `Empty`.
pub fn serialize_protected(hdr: &HeaderMap) -> Result<EmptyOrSerializedMap, CoseCryptoError> {
    let cbor: HeaderMapCbor =
        HeaderMapCbor::try_from(hdr).map_err(|e| CoseCryptoError::CborError(e.to_string()))?;
    let mut buf = Vec::new();
    ciborium::ser::into_writer(&cbor, &mut buf)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;
    Ok(EmptyOrSerializedMap::SerializedMap(buf))
}

/// Create an empty protected header.
pub fn empty_protected() -> EmptyOrSerializedMap {
    EmptyOrSerializedMap::Empty(Vec::new())
}

/// Deserialize protected header bytes into a `HeaderMapCbor`.
pub fn deserialize_protected(
    protected: &EmptyOrSerializedMap,
) -> Result<Option<HeaderMapCbor>, CoseCryptoError> {
    match protected {
        EmptyOrSerializedMap::SerializedMap(bytes) => {
            let hdr: HeaderMapCbor = ciborium::de::from_reader(bytes.as_slice())
                .map_err(|e| CoseCryptoError::InvalidProtectedHeader(e.to_string()))?;
            Ok(Some(hdr))
        }
        EmptyOrSerializedMap::Empty(_) => Ok(None),
    }
}

/// Extract the algorithm from an `EmptyOrSerializedMap` protected header.
pub fn extract_algorithm(
    protected: &EmptyOrSerializedMap,
) -> Result<CoseAlgorithm, CoseCryptoError> {
    let hdr = deserialize_protected(protected)?.ok_or(CoseCryptoError::MissingAlgorithm)?;
    match &hdr.alg_id {
        Some(toi) => CoseAlgorithm::from_text_or_int(toi),
        None => Err(CoseCryptoError::MissingAlgorithm),
    }
}

/// Extract IV from protected and unprotected headers.
pub fn extract_iv(
    protected: &EmptyOrSerializedMap,
    unprotected: &HeaderMapCbor,
) -> Result<Vec<u8>, CoseCryptoError> {
    // Check unprotected first (more common location for IV)
    if let Some(iv) = &unprotected.iv {
        return Ok(iv.clone());
    }
    if let Some(piv) = &unprotected.partial_iv {
        return Ok(piv.clone());
    }
    // Check protected
    if let Some(hdr) = deserialize_protected(protected)? {
        if let Some(iv) = hdr.iv {
            return Ok(iv);
        }
        if let Some(piv) = hdr.partial_iv {
            return Ok(piv);
        }
    }
    Err(CoseCryptoError::MissingField(
        "IV or Partial IV".to_string(),
    ))
}

/// Get the raw bytes from an `EmptyOrSerializedMap`.
pub fn protected_bytes(protected: &EmptyOrSerializedMap) -> &[u8] {
    match protected {
        EmptyOrSerializedMap::SerializedMap(b) => b.as_slice(),
        EmptyOrSerializedMap::Empty(b) => b.as_slice(),
    }
}

/// Create a `HeaderMap` with just the algorithm set.
pub fn header_with_algorithm(alg: CoseAlgorithm) -> HeaderMap {
    HeaderMap {
        alg_id: Some(TextOrInt::Int(alg.to_i64())),
        criticality: None,
        content_type: None,
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    }
}

/// Create an empty `HeaderMap`.
pub fn empty_header() -> HeaderMap {
    HeaderMap {
        alg_id: None,
        criticality: None,
        content_type: None,
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    }
}
