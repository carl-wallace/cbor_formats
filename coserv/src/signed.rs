//! Signed CoSERV ([CoSERV Section 4.6]).
//!
//! Provides [`SignedCoserv`], a validated wrapper around [`CoseSign1Cbor`] that
//! enforces the `signed-coserv` constraints:
//!
//! ```text
//! signed-coserv = #6.18([
//!   protected: bytes .cbor signed-coserv-protected-hdr
//!   unprotected: signed-coserv-unprotected-hdr
//!   payload: bytes .cbor coserv
//!   signature: bytes
//! ])
//! ```
//!
//! The protected header must contain:
//! - `alg` (label 1): signature algorithm identifier
//! - `content_type` (label 3): `"application/coserv+cbor"`
//!
//! When the `crypto` feature is enabled, `SignedCoservBuilder` provides
//! construction and signing via `cose_crypto`.
//!
//! [CoSERV Section 4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.6

use alloc::string::{String, ToString};

use serde::{Deserialize, Serialize};

use common::{BinaryOrNil, TextOrInt};
use cose::{arrays::CoseSign1Cbor, choices::EmptyOrSerializedMap, maps::HeaderMapCbor};

use crate::maps::CoservMapCbor;

#[cfg(feature = "crypto")]
use alloc::vec::Vec;

/// Media type for CoSERV CBOR content.
pub const COSERV_CBOR_CONTENT_TYPE: &str = "application/coserv+cbor";

/// Errors specific to [`SignedCoserv`] validation.
#[derive(Clone, Debug, PartialEq)]
pub enum SignedCoservError {
    /// Protected header is empty (must contain alg and cty).
    EmptyProtectedHeader,
    /// Protected header bytes could not be decoded.
    InvalidProtectedHeader(String),
    /// Missing required `alg` (label 1) in protected header.
    MissingAlgorithm,
    /// Missing or invalid `content_type` (label 3) in protected header.
    /// Must be `"application/coserv+cbor"`.
    InvalidContentType,
    /// Payload is nil (must be `bytes .cbor coserv`).
    MissingPayload,
    /// Payload bytes do not decode as a valid `coserv`.
    InvalidPayload(String),
    /// CBOR serialization/deserialization error.
    CborError(String),
}

impl core::fmt::Display for SignedCoservError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyProtectedHeader => write!(f, "protected header is empty"),
            Self::InvalidProtectedHeader(e) => write!(f, "invalid protected header: {e}"),
            Self::MissingAlgorithm => write!(f, "missing alg (label 1) in protected header"),
            Self::InvalidContentType => write!(
                f,
                "content_type (label 3) must be \"{COSERV_CBOR_CONTENT_TYPE}\""
            ),
            Self::MissingPayload => write!(f, "payload is nil"),
            Self::InvalidPayload(e) => write!(f, "payload is not valid coserv: {e}"),
            Self::CborError(e) => write!(f, "CBOR error: {e}"),
        }
    }
}

/// A validated `signed-coserv` per [CoSERV Section 4.6].
///
/// Wraps a [`CoseSign1Cbor`] that has been verified to have:
/// - `alg` (label 1) in the protected header
/// - `content_type` (label 3) = `"application/coserv+cbor"`
/// - non-nil payload containing valid `bytes .cbor coserv`
///
/// [CoSERV Section 4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.6
#[derive(Clone, Debug, PartialEq)]
pub struct SignedCoserv(CoseSign1Cbor);

impl SignedCoserv {
    /// Validate a [`CoseSign1Cbor`] as a `signed-coserv`.
    ///
    /// Returns an error if the protected header is missing required fields
    /// or the payload is not a valid `coserv`.
    pub fn new(sign1: CoseSign1Cbor) -> Result<Self, SignedCoservError> {
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
    pub fn protected_header(&self) -> Result<HeaderMapCbor, SignedCoservError> {
        deserialize_protected(&self.0.protected)
    }

    /// Decode and return the payload as a [`CoservMapCbor`].
    pub fn payload(&self) -> Result<CoservMapCbor, SignedCoservError> {
        decode_payload(&self.0.payload)
    }

    fn validate(sign1: &CoseSign1Cbor) -> Result<(), SignedCoservError> {
        let hdr = deserialize_protected(&sign1.protected)?;
        validate_header(&hdr)?;
        decode_payload(&sign1.payload)?;
        Ok(())
    }
}

fn validate_header(hdr: &HeaderMapCbor) -> Result<(), SignedCoservError> {
    // alg (label 1) must be present and an integer
    match &hdr.alg_id {
        Some(TextOrInt::Int(_)) => {}
        _ => return Err(SignedCoservError::MissingAlgorithm),
    }

    // content_type (label 3) must be "application/coserv+cbor"
    match &hdr.content_type {
        Some(TextOrInt::Text(s)) if s == COSERV_CBOR_CONTENT_TYPE => {}
        _ => return Err(SignedCoservError::InvalidContentType),
    }

    Ok(())
}

fn deserialize_protected(
    protected: &EmptyOrSerializedMap,
) -> Result<HeaderMapCbor, SignedCoservError> {
    match protected {
        EmptyOrSerializedMap::SerializedMap(bytes) => {
            ciborium::de::from_reader::<HeaderMapCbor, _>(bytes.as_slice())
                .map_err(|e| SignedCoservError::InvalidProtectedHeader(e.to_string()))
        }
        EmptyOrSerializedMap::Empty(_) => Err(SignedCoservError::EmptyProtectedHeader),
    }
}

fn decode_payload(payload: &BinaryOrNil) -> Result<CoservMapCbor, SignedCoservError> {
    match payload {
        BinaryOrNil::Binary(bytes) => {
            ciborium::de::from_reader::<CoservMapCbor, _>(bytes.as_slice())
                .map_err(|e| SignedCoservError::InvalidPayload(e.to_string()))
        }
        BinaryOrNil::Nil => Err(SignedCoservError::MissingPayload),
    }
}

impl Serialize for SignedCoserv {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SignedCoserv {
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

    /// Builder for [`SignedCoserv`] messages.
    ///
    /// Wraps [`CoseSign1Builder`] and automatically sets the required
    /// `content_type` header. The payload must be a valid [`CoservMapCbor`].
    pub struct SignedCoservBuilder {
        coserv_payload: Vec<u8>,
        protected: HeaderMap,
        unprotected: HeaderMap,
        external_aad: Vec<u8>,
    }

    impl Default for SignedCoservBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl SignedCoservBuilder {
        /// Create a new builder.
        pub fn new() -> Self {
            Self {
                coserv_payload: Vec::new(),
                protected: cose_crypto::helpers::empty_header(),
                unprotected: cose_crypto::helpers::empty_header(),
                external_aad: Vec::new(),
            }
        }

        /// Set the payload from a [`CoservMapCbor`].
        ///
        /// The CoSERV map is serialized to CBOR bytes and used as the
        /// COSE_Sign1 payload.
        pub fn payload(mut self, coserv: &CoservMapCbor) -> Result<Self, SignedCoservError> {
            let mut buf = Vec::new();
            ciborium::ser::into_writer(coserv, &mut buf)
                .map_err(|e| SignedCoservError::CborError(e.to_string()))?;
            self.coserv_payload = buf;
            Ok(self)
        }

        /// Set additional protected header fields.
        ///
        /// The `alg` field should be set here. The `content_type` field is
        /// automatically set to [`COSERV_CBOR_CONTENT_TYPE`] and should not
        /// be overridden.
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

        /// Sign the message and produce a [`SignedCoserv`].
        ///
        /// This sets `content_type` to [`COSERV_CBOR_CONTENT_TYPE`] in the
        /// protected header (overriding any previously set value), then
        /// delegates to [`CoseSign1Builder`].
        pub fn sign(
            mut self,
            signer: &dyn CoseSigner,
        ) -> Result<SignedCoserv, SignedCoservBuilderError> {
            self.protected.content_type =
                Some(TextOrInt::Text(COSERV_CBOR_CONTENT_TYPE.to_string()));

            let sign1 = CoseSign1Builder::new()
                .payload(&self.coserv_payload)
                .protected(self.protected)
                .unprotected(self.unprotected)
                .external_aad(&self.external_aad)
                .sign(signer)
                .map_err(SignedCoservBuilderError::Crypto)?;

            SignedCoserv::new(sign1).map_err(SignedCoservBuilderError::Validation)
        }
    }

    /// Errors from [`SignedCoservBuilder`].
    #[derive(Debug)]
    pub enum SignedCoservBuilderError {
        /// Cryptographic operation failed.
        Crypto(CoseCryptoError),
        /// Validation of the resulting signed-coserv failed.
        Validation(SignedCoservError),
    }

    impl core::fmt::Display for SignedCoservBuilderError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Crypto(e) => write!(f, "signing failed: {e}"),
                Self::Validation(e) => write!(f, "validation failed: {e}"),
            }
        }
    }
}

#[cfg(feature = "crypto")]
pub use builder::{SignedCoservBuilder, SignedCoservBuilderError};
