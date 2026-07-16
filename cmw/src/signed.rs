//! Signed CBOR CMW ([CMW Section 4.1]).
//!
//! Provides [`SignedCborCmw`], a validated wrapper around [`CoseSign1Cbor`] that
//! enforces the `signed-cbor-cmw` constraints:
//!
//! ```text
//! signed-cbor-cmw = [
//!   protected: bytes .cbor signed-cbor-cmw-protected-hdr
//!   unprotected: signed-cbor-cmw-unprotected-hdr
//!   payload: bytes .cbor cbor-cmw
//!   signature: bytes
//! ]
//!
//! signed-cbor-cmw-protected-hdr = {
//!   1 => int                            ; alg
//!   3 => "application/cmw+cbor" / 10000 ; cty
//!   * cose.label => cose.values
//! }
//! ```
//!
//! When the `crypto` feature is enabled, `SignedCborCmwBuilder` provides
//! construction and signing via `cose_crypto`.
//!
//! [CMW Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-4.1

use alloc::string::{String, ToString};

use serde::{Deserialize, Serialize};

use common::{BinaryOrNil, TextOrInt};
use cose::{arrays::CoseSign1Cbor, choices::EmptyOrSerializedMap, maps::HeaderMapCbor};

use crate::choices::CborCmw;

#[cfg(feature = "crypto")]
use alloc::vec::Vec;

/// Media type for CMW CBOR content.
pub const CMW_CBOR_CONTENT_TYPE: &str = "application/cmw+cbor";

/// CoAP Content-Format number for CMW CBOR content.
pub const CMW_CBOR_CONTENT_FORMAT: i64 = 10000;

/// Errors specific to [`SignedCborCmw`] validation.
#[derive(Clone, Debug, PartialEq)]
pub enum SignedCmwError {
    /// Protected header is empty (must contain alg and cty).
    EmptyProtectedHeader,
    /// Protected header bytes could not be decoded.
    InvalidProtectedHeader(String),
    /// Missing required `alg` (label 1) in protected header.
    MissingAlgorithm,
    /// Missing or invalid `content_type` (label 3) in protected header.
    /// Must be `"application/cmw+cbor"` or `10000`.
    InvalidContentType,
    /// Payload is nil (must be `bytes .cbor cbor-cmw`).
    MissingPayload,
    /// Payload bytes do not decode as a valid `cbor-cmw`.
    InvalidPayload(String),
    /// CBOR serialization/deserialization error.
    CborError(String),
}

impl core::fmt::Display for SignedCmwError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyProtectedHeader => write!(f, "protected header is empty"),
            Self::InvalidProtectedHeader(e) => write!(f, "invalid protected header: {e}"),
            Self::MissingAlgorithm => write!(f, "missing alg (label 1) in protected header"),
            Self::InvalidContentType => write!(
                f,
                "content_type (label 3) must be \"{CMW_CBOR_CONTENT_TYPE}\" or {CMW_CBOR_CONTENT_FORMAT}"
            ),
            Self::MissingPayload => write!(f, "payload is nil"),
            Self::InvalidPayload(e) => write!(f, "payload is not valid cbor-cmw: {e}"),
            Self::CborError(e) => write!(f, "CBOR error: {e}"),
        }
    }
}

/// A validated `signed-cbor-cmw` per [CMW Section 4.1].
///
/// Wraps a [`CoseSign1Cbor`] that has been verified to have:
/// - `alg` (label 1) in the protected header
/// - `content_type` (label 3) = `"application/cmw+cbor"` or `10000`
/// - non-nil payload containing valid `bytes .cbor cbor-cmw`
///
/// [CMW Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-4.1
#[derive(Clone, Debug, PartialEq)]
pub struct SignedCborCmw(CoseSign1Cbor);

impl SignedCborCmw {
    /// Validate a [`CoseSign1Cbor`] as a `signed-cbor-cmw`.
    ///
    /// Returns an error if the protected header is missing required fields
    /// or the payload is not a valid `cbor-cmw`.
    pub fn new(sign1: CoseSign1Cbor) -> Result<Self, SignedCmwError> {
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
    pub fn protected_header(&self) -> Result<HeaderMapCbor, SignedCmwError> {
        deserialize_protected(&self.0.protected)
    }

    /// Decode and return the payload as a [`CborCmw`].
    pub fn payload(&self) -> Result<CborCmw, SignedCmwError> {
        decode_payload(&self.0.payload)
    }

    fn validate(sign1: &CoseSign1Cbor) -> Result<(), SignedCmwError> {
        // Validate protected header
        let hdr = deserialize_protected(&sign1.protected)?;
        validate_header(&hdr)?;

        // Validate payload
        decode_payload(&sign1.payload)?;

        Ok(())
    }
}

/// Check that the protected header has the required fields.
fn validate_header(hdr: &HeaderMapCbor) -> Result<(), SignedCmwError> {
    // alg (label 1) must be present and an integer
    match &hdr.alg_id {
        Some(TextOrInt::Int(_)) => {}
        _ => return Err(SignedCmwError::MissingAlgorithm),
    }

    // content_type (label 3) must be "application/cmw+cbor" or 10000
    match &hdr.content_type {
        Some(TextOrInt::Text(s)) if s == CMW_CBOR_CONTENT_TYPE => {}
        Some(TextOrInt::Int(n)) if *n == CMW_CBOR_CONTENT_FORMAT => {}
        _ => return Err(SignedCmwError::InvalidContentType),
    }

    Ok(())
}

fn deserialize_protected(
    protected: &EmptyOrSerializedMap,
) -> Result<HeaderMapCbor, SignedCmwError> {
    match protected {
        EmptyOrSerializedMap::SerializedMap(bytes) => {
            ciborium::de::from_reader::<HeaderMapCbor, _>(bytes.as_slice())
                .map_err(|e| SignedCmwError::InvalidProtectedHeader(e.to_string()))
        }
        EmptyOrSerializedMap::Empty(_) => Err(SignedCmwError::EmptyProtectedHeader),
    }
}

fn decode_payload(payload: &BinaryOrNil) -> Result<CborCmw, SignedCmwError> {
    match payload {
        BinaryOrNil::Binary(bytes) => ciborium::de::from_reader::<CborCmw, _>(bytes.as_slice())
            .map_err(|e| SignedCmwError::InvalidPayload(e.to_string())),
        BinaryOrNil::Nil => Err(SignedCmwError::MissingPayload),
    }
}

impl Serialize for SignedCborCmw {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SignedCborCmw {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let sign1 = CoseSign1Cbor::deserialize(deserializer)?;
        Self::new(sign1).map_err(serde::de::Error::custom)
    }
}

// ---------- Builder (requires cose_crypto) ----------

#[cfg(feature = "crypto")]
mod builder {
    use super::*;
    use common::TextOrInt;
    use cose::maps::HeaderMap;
    use cose_crypto::crypto::CoseSigner;
    use cose_crypto::error::CoseCryptoError;
    use cose_crypto::sign::CoseSign1Builder;

    /// Builder for [`SignedCborCmw`] messages.
    ///
    /// Wraps [`CoseSign1Builder`] and automatically sets the required
    /// `content_type` header. The payload must be a valid [`CborCmw`].
    pub struct SignedCborCmwBuilder {
        cmw_payload: Vec<u8>,
        protected: HeaderMap,
        unprotected: HeaderMap,
        external_aad: Vec<u8>,
    }

    impl Default for SignedCborCmwBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl SignedCborCmwBuilder {
        /// Create a new builder.
        pub fn new() -> Self {
            Self {
                cmw_payload: Vec::new(),
                protected: cose_crypto::helpers::empty_header(),
                unprotected: cose_crypto::helpers::empty_header(),
                external_aad: Vec::new(),
            }
        }

        /// Set the payload from a [`CborCmw`].
        ///
        /// The CMW is serialized to CBOR bytes and used as the COSE_Sign1 payload.
        pub fn payload(mut self, cmw: &CborCmw) -> Result<Self, SignedCmwError> {
            let mut buf = Vec::new();
            ciborium::ser::into_writer(cmw, &mut buf)
                .map_err(|e| SignedCmwError::CborError(e.to_string()))?;
            self.cmw_payload = buf;
            Ok(self)
        }

        /// Set additional protected header fields.
        ///
        /// The `alg` field should be set here (it will also be inferred from
        /// the signer if not set). The `content_type` field is automatically
        /// set to [`CMW_CBOR_CONTENT_TYPE`] and should not be overridden.
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

        /// Sign the message and produce a [`SignedCborCmw`].
        ///
        /// This sets `content_type` to [`CMW_CBOR_CONTENT_TYPE`] in the
        /// protected header (overriding any previously set value), then
        /// delegates to [`CoseSign1Builder`].
        pub fn sign(
            mut self,
            signer: &dyn CoseSigner,
        ) -> Result<SignedCborCmw, SignedCborCmwError> {
            // Ensure content_type is set
            self.protected.content_type = Some(TextOrInt::Text(CMW_CBOR_CONTENT_TYPE.to_string()));

            let sign1 = CoseSign1Builder::new()
                .payload(&self.cmw_payload)
                .protected(self.protected)
                .unprotected(self.unprotected)
                .external_aad(&self.external_aad)
                .sign(signer)
                .map_err(SignedCborCmwError::Crypto)?;

            SignedCborCmw::new(sign1).map_err(SignedCborCmwError::Validation)
        }
    }

    /// Errors from [`SignedCborCmwBuilder`].
    #[derive(Debug)]
    pub enum SignedCborCmwError {
        /// Cryptographic operation failed.
        Crypto(CoseCryptoError),
        /// Validation of the resulting signed-cbor-cmw failed.
        Validation(SignedCmwError),
    }

    impl core::fmt::Display for SignedCborCmwError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Crypto(e) => write!(f, "signing failed: {e}"),
                Self::Validation(e) => write!(f, "validation failed: {e}"),
            }
        }
    }
}

#[cfg(feature = "crypto")]
pub use builder::{SignedCborCmwBuilder, SignedCborCmwError};
