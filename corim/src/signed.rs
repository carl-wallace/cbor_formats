//! Signed CoRIM ([CoRIM Section 4.2]).
//!
//! Provides [`SignedCorim`], a validated wrapper around [`CoseSign1Cbor`] that
//! enforces the `COSE-Sign1-corim` constraints:
//!
//! ```text
//! COSE-Sign1-corim = [
//!   protected: bstr .cbor protected-corim-header-map
//!   unprotected: unprotected-corim-header-map
//!   payload: bstr .cbor tagged-unsigned-corim-map
//!   signature: bstr
//! ]
//!
//! protected-corim-header-map-inline = {
//!   &(alg: 1) => int
//!   &(content-type: 3) => "application/rim+cbor"
//!   meta-group
//!   * cose-label => cose-value
//! }
//! ```
//!
//! When the `crypto` feature is enabled, `SignedCorimBuilder` provides
//! construction and signing via `cose_crypto`.
//!
//! [CoRIM Section 4.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.2

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use ciborium::value::Value;
use serde::{Deserialize, Serialize};

use common::{BinaryOrNil, TextOrInt};
use cose::{arrays::CoseSign1Cbor, choices::EmptyOrSerializedMap, maps::HeaderMapCbor};

use crate::maps::CorimMapCbor;

/// Media type for CoRIM CBOR content.
pub const CORIM_CBOR_CONTENT_TYPE: &str = "application/rim+cbor";

/// CBOR tag number for `tagged-unsigned-corim-map`.
pub const TAG_UNSIGNED_CORIM_MAP: u64 = 501;

/// Errors specific to [`SignedCorim`] validation.
#[derive(Clone, Debug, PartialEq)]
pub enum SignedCorimError {
    /// Protected header is empty (must contain alg and cty).
    EmptyProtectedHeader,
    /// Protected header bytes could not be decoded.
    InvalidProtectedHeader(String),
    /// Missing required `alg` (label 1) in protected header.
    MissingAlgorithm,
    /// Missing or invalid `content_type` (label 3) in protected header.
    /// Must be `"application/rim+cbor"`.
    InvalidContentType,
    /// Payload is nil.
    MissingPayload,
    /// Payload bytes do not decode as a valid `tagged-unsigned-corim-map`.
    InvalidPayload(String),
    /// CBOR serialization/deserialization error.
    CborError(String),
}

impl core::fmt::Display for SignedCorimError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyProtectedHeader => write!(f, "protected header is empty"),
            Self::InvalidProtectedHeader(e) => write!(f, "invalid protected header: {e}"),
            Self::MissingAlgorithm => write!(f, "missing alg (label 1) in protected header"),
            Self::InvalidContentType => write!(
                f,
                "content_type (label 3) must be \"{CORIM_CBOR_CONTENT_TYPE}\""
            ),
            Self::MissingPayload => write!(f, "payload is nil"),
            Self::InvalidPayload(e) => {
                write!(f, "payload is not valid tagged-unsigned-corim-map: {e}")
            }
            Self::CborError(e) => write!(f, "CBOR error: {e}"),
        }
    }
}

/// A validated `COSE-Sign1-corim` per [CoRIM Section 4.2].
///
/// Wraps a [`CoseSign1Cbor`] that has been verified to have:
/// - `alg` (label 1) in the protected header
/// - `content_type` (label 3) = `"application/rim+cbor"`
/// - non-nil payload containing a valid `tagged-unsigned-corim-map` (`#6.501`)
///   or bare `unsigned-corim-map`
///
/// [CoRIM Section 4.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.2
#[derive(Clone, Debug, PartialEq)]
pub struct SignedCorim(CoseSign1Cbor);

impl SignedCorim {
    /// Validate a [`CoseSign1Cbor`] as a `COSE-Sign1-corim`.
    ///
    /// Returns an error if the protected header is missing required fields
    /// or the payload is not a valid unsigned CoRIM map.
    pub fn new(sign1: CoseSign1Cbor) -> Result<Self, SignedCorimError> {
        Self::validate(&sign1)?;
        Ok(Self(sign1))
    }

    /// Consume this wrapper and return the inner [`CoseSign1Cbor`].
    pub fn into_inner(self) -> CoseSign1Cbor {
        self.0
    }

    /// Borrow the inner [`CoseSign1Cbor`].
    pub fn as_inner(&self) -> &CoseSign1Cbor {
        &self.0
    }

    /// Decode and return the protected header.
    pub fn protected_header(&self) -> Result<HeaderMapCbor, SignedCorimError> {
        deserialize_protected(&self.0.protected)
    }

    /// Decode and return the payload as a [`CorimMapCbor`].
    ///
    /// Strips the `#6.501` tag if present before decoding.
    pub fn payload(&self) -> Result<CorimMapCbor, SignedCorimError> {
        decode_payload(&self.0.payload)
    }

    fn validate(sign1: &CoseSign1Cbor) -> Result<(), SignedCorimError> {
        let hdr = deserialize_protected(&sign1.protected)?;
        validate_header(&hdr)?;
        decode_payload(&sign1.payload)?;
        Ok(())
    }
}

fn validate_header(hdr: &HeaderMapCbor) -> Result<(), SignedCorimError> {
    // alg (label 1) must be present and an integer
    match &hdr.alg_id {
        Some(TextOrInt::Int(_)) => {}
        _ => return Err(SignedCorimError::MissingAlgorithm),
    }

    // content_type (label 3) must be "application/rim+cbor"
    match &hdr.content_type {
        Some(TextOrInt::Text(s)) if s == CORIM_CBOR_CONTENT_TYPE => {}
        _ => return Err(SignedCorimError::InvalidContentType),
    }

    Ok(())
}

fn deserialize_protected(
    protected: &EmptyOrSerializedMap,
) -> Result<HeaderMapCbor, SignedCorimError> {
    match protected {
        EmptyOrSerializedMap::SerializedMap(bytes) => {
            ciborium::de::from_reader::<HeaderMapCbor, _>(bytes.as_slice())
                .map_err(|e| SignedCorimError::InvalidProtectedHeader(e.to_string()))
        }
        EmptyOrSerializedMap::Empty(_) => Err(SignedCorimError::EmptyProtectedHeader),
    }
}

/// Decode the payload, accepting either `#6.501(unsigned-corim-map)` or a bare
/// `unsigned-corim-map`.
fn decode_payload(payload: &BinaryOrNil) -> Result<CorimMapCbor, SignedCorimError> {
    let bytes = match payload {
        BinaryOrNil::Binary(b) => b,
        BinaryOrNil::Nil => return Err(SignedCorimError::MissingPayload),
    };

    // Parse as generic CBOR value first to handle the optional tag 501
    let value: Value = ciborium::de::from_reader(bytes.as_slice())
        .map_err(|e| SignedCorimError::InvalidPayload(e.to_string()))?;

    // Strip tag 501 if present, then re-decode as CorimMapCbor
    let inner = match &value {
        Value::Tag(TAG_UNSIGNED_CORIM_MAP, inner) => inner.as_ref(),
        other => other,
    };

    let mut buf = Vec::new();
    if ciborium::ser::into_writer(inner, &mut buf).is_err() {
        return Err(SignedCorimError::InvalidPayload(
            "failed to re-serialize inner value".into(),
        ));
    }
    ciborium::de::from_reader::<CorimMapCbor, _>(buf.as_slice()).map_err(|_| {
        SignedCorimError::InvalidPayload("payload is not a valid unsigned-corim-map".into())
    })
}

impl Serialize for SignedCorim {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SignedCorim {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let sign1 = CoseSign1Cbor::deserialize(deserializer)?;
        Self::new(sign1).map_err(serde::de::Error::custom)
    }
}

// ---------- Builder (requires cose_crypto) ----------

#[cfg(feature = "crypto")]
mod builder {
    use super::*;
    use alloc::boxed::Box;
    use common::TextOrInt;
    use cose::maps::HeaderMap;
    use cose_crypto::crypto::CoseSigner;
    use cose_crypto::error::CoseCryptoError;
    use cose_crypto::sign::CoseSign1Builder;

    /// Builder for [`SignedCorim`] messages.
    ///
    /// Wraps [`CoseSign1Builder`] and automatically sets the required
    /// `content_type` header. The payload is a CBOR-encoded
    /// `tagged-unsigned-corim-map` (`#6.501`).
    pub struct SignedCorimBuilder {
        corim_payload: Vec<u8>,
        protected: HeaderMap,
        unprotected: HeaderMap,
        external_aad: Vec<u8>,
    }

    impl Default for SignedCorimBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl SignedCorimBuilder {
        /// Create a new builder.
        pub fn new() -> Self {
            Self {
                corim_payload: Vec::new(),
                protected: cose_crypto::helpers::empty_header(),
                unprotected: cose_crypto::helpers::empty_header(),
                external_aad: Vec::new(),
            }
        }

        /// Set the payload from a [`CorimMapCbor`].
        ///
        /// The CoRIM map is wrapped in CBOR tag 501 and serialized to bytes.
        pub fn payload(mut self, corim: &CorimMapCbor) -> Result<Self, SignedCorimError> {
            // Serialize the CorimMapCbor, then wrap in tag 501
            let mut inner_buf = Vec::new();
            ciborium::ser::into_writer(corim, &mut inner_buf)
                .map_err(|e| SignedCorimError::CborError(e.to_string()))?;

            // Re-read as Value, wrap in tag, re-serialize
            let inner_value: Value = ciborium::de::from_reader(inner_buf.as_slice())
                .map_err(|e| SignedCorimError::CborError(e.to_string()))?;
            let tagged = Value::Tag(TAG_UNSIGNED_CORIM_MAP, Box::new(inner_value));

            let mut buf = Vec::new();
            ciborium::ser::into_writer(&tagged, &mut buf)
                .map_err(|e| SignedCorimError::CborError(e.to_string()))?;
            self.corim_payload = buf;
            Ok(self)
        }

        /// Set the payload from raw bytes.
        ///
        /// The bytes should already be a CBOR-encoded `tagged-unsigned-corim-map`.
        pub fn payload_bytes(mut self, bytes: &[u8]) -> Self {
            self.corim_payload = bytes.to_vec();
            self
        }

        /// Set additional protected header fields.
        ///
        /// The `alg` field should be set here. The `content_type` field is
        /// automatically set to [`CORIM_CBOR_CONTENT_TYPE`] and should not
        /// be overridden. CoRIM metadata can be included via the `other`
        /// field with label 8.
        pub fn protected(mut self, hdr: HeaderMap) -> Self {
            self.protected = hdr;
            self
        }

        /// Set the unprotected headers.
        pub fn unprotected(mut self, hdr: HeaderMap) -> Self {
            self.unprotected = hdr;
            self
        }

        /// Set the external additional authenticated data.
        pub fn external_aad(mut self, aad: &[u8]) -> Self {
            self.external_aad = aad.to_vec();
            self
        }

        /// Sign the message and produce a [`SignedCorim`].
        ///
        /// This sets `content_type` to [`CORIM_CBOR_CONTENT_TYPE`] in the
        /// protected header (overriding any previously set value), then
        /// delegates to [`CoseSign1Builder`].
        pub fn sign(
            mut self,
            signer: &dyn CoseSigner,
        ) -> Result<SignedCorim, SignedCorimBuilderError> {
            self.protected.content_type =
                Some(TextOrInt::Text(CORIM_CBOR_CONTENT_TYPE.to_string()));

            let sign1 = CoseSign1Builder::new()
                .payload(&self.corim_payload)
                .protected(self.protected)
                .unprotected(self.unprotected)
                .external_aad(&self.external_aad)
                .sign(signer)
                .map_err(SignedCorimBuilderError::Crypto)?;

            SignedCorim::new(sign1).map_err(SignedCorimBuilderError::Validation)
        }
    }

    /// Errors from [`SignedCorimBuilder`].
    #[derive(Debug)]
    pub enum SignedCorimBuilderError {
        /// Cryptographic operation failed.
        Crypto(CoseCryptoError),
        /// Validation of the resulting COSE-Sign1-corim failed.
        Validation(SignedCorimError),
    }

    impl core::fmt::Display for SignedCorimBuilderError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Crypto(e) => write!(f, "signing failed: {e}"),
                Self::Validation(e) => write!(f, "validation failed: {e}"),
            }
        }
    }
}

#[cfg(feature = "crypto")]
pub use builder::{SignedCorimBuilder, SignedCorimBuilderError};
