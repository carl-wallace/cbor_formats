//! COSE signing and verification (CoseSign1, CoseSign).

use alloc::string::ToString;
use alloc::vec::Vec;

use common::{BinaryOrNil, BytesType};
use cose::arrays::{
    CoseSign, CoseSign1, CoseSign1Cbor, CoseSignCbor, CoseSignature, SigStructure, SigStructureCbor,
};
use cose::choices::SignatureOrSignature1;
use cose::maps::HeaderMap;

use crate::crypto::{CoseSigner, CoseVerifier};
use crate::error::CoseCryptoError;
use crate::helpers;

/// Builder for `COSE_Sign1` messages.
pub struct CoseSign1Builder {
    payload: Vec<u8>,
    protected: HeaderMap,
    unprotected: HeaderMap,
    external_aad: Vec<u8>,
}

impl Default for CoseSign1Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl CoseSign1Builder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            payload: Vec::new(),
            protected: helpers::empty_header(),
            unprotected: helpers::empty_header(),
            external_aad: Vec::new(),
        }
    }

    /// Set the payload.
    pub fn payload(mut self, data: &[u8]) -> Self {
        self.payload = data.to_vec();
        self
    }

    /// Set the protected headers.
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

    /// Sign the message and produce a `CoseSign1Cbor`.
    pub fn sign(self, signer: &dyn CoseSigner) -> Result<CoseSign1Cbor, CoseCryptoError> {
        let protected_serialized = helpers::serialize_protected(&self.protected)?;

        let sig_structure = SigStructure {
            context: SignatureOrSignature1::Signature1,
            body_protected: protected_serialized.clone(),
            sign_protected: None,
            external_aad: BytesType::Bytes(self.external_aad),
            payload: BytesType::Bytes(self.payload.clone()),
        };

        let sig_structure_cbor = SigStructureCbor::try_from(sig_structure)
            .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

        let mut tbs = Vec::new();
        ciborium::ser::into_writer(&sig_structure_cbor, &mut tbs)
            .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

        let signature = signer.sign(&tbs)?;

        let cose_sign1 = CoseSign1 {
            protected: protected_serialized,
            unprotected: self.unprotected,
            payload: BinaryOrNil::Binary(self.payload),
            signature: BytesType::Bytes(signature),
        };

        CoseSign1Cbor::try_from(cose_sign1).map_err(|e| CoseCryptoError::CborError(e.to_string()))
    }
}

/// Verify a `CoseSign1Cbor` message.
pub fn verify_sign1(
    msg: &CoseSign1Cbor,
    verifier: &dyn CoseVerifier,
    external_aad: &[u8],
) -> Result<(), CoseCryptoError> {
    let payload = match &msg.payload {
        BinaryOrNil::Binary(p) => p.clone(),
        BinaryOrNil::Nil => {
            return Err(CoseCryptoError::MissingField("payload".to_string()));
        }
    };

    let sig_structure = SigStructure {
        context: SignatureOrSignature1::Signature1,
        body_protected: msg.protected.clone(),
        sign_protected: None,
        external_aad: BytesType::Bytes(external_aad.to_vec()),
        payload: BytesType::Bytes(payload),
    };

    let sig_structure_cbor = SigStructureCbor::try_from(sig_structure)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let mut tbs = Vec::new();
    ciborium::ser::into_writer(&sig_structure_cbor, &mut tbs)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let BytesType::Bytes(signature) = &msg.signature;

    verifier.verify(&tbs, signature)
}

/// Verify a `CoseSign1Cbor` with a detached payload.
pub fn verify_sign1_detached(
    msg: &CoseSign1Cbor,
    payload: &[u8],
    verifier: &dyn CoseVerifier,
    external_aad: &[u8],
) -> Result<(), CoseCryptoError> {
    let sig_structure = SigStructure {
        context: SignatureOrSignature1::Signature1,
        body_protected: msg.protected.clone(),
        sign_protected: None,
        external_aad: BytesType::Bytes(external_aad.to_vec()),
        payload: BytesType::Bytes(payload.to_vec()),
    };

    let sig_structure_cbor = SigStructureCbor::try_from(sig_structure)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let mut tbs = Vec::new();
    ciborium::ser::into_writer(&sig_structure_cbor, &mut tbs)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let BytesType::Bytes(signature) = &msg.signature;

    verifier.verify(&tbs, signature)
}

/// Builder for `COSE_Sign` messages with multiple signers.
pub struct CoseSignBuilder {
    payload: Vec<u8>,
    protected: HeaderMap,
    unprotected: HeaderMap,
    external_aad: Vec<u8>,
    signers: Vec<SignerEntry>,
}

struct SignerEntry {
    protected: HeaderMap,
    unprotected: HeaderMap,
    signer: Box<dyn CoseSigner>,
}

impl Default for CoseSignBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CoseSignBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            payload: Vec::new(),
            protected: helpers::empty_header(),
            unprotected: helpers::empty_header(),
            external_aad: Vec::new(),
            signers: Vec::new(),
        }
    }

    /// Set the payload.
    pub fn payload(mut self, data: &[u8]) -> Self {
        self.payload = data.to_vec();
        self
    }

    /// Set the body-level protected headers.
    pub fn protected(mut self, hdr: HeaderMap) -> Self {
        self.protected = hdr;
        self
    }

    /// Set the body-level unprotected headers.
    pub fn unprotected(mut self, hdr: HeaderMap) -> Self {
        self.unprotected = hdr;
        self
    }

    /// Set the external additional authenticated data.
    pub fn external_aad(mut self, aad: &[u8]) -> Self {
        self.external_aad = aad.to_vec();
        self
    }

    /// Add a signer with per-signer headers.
    pub fn add_signer(
        mut self,
        protected: HeaderMap,
        unprotected: HeaderMap,
        signer: Box<dyn CoseSigner>,
    ) -> Self {
        self.signers.push(SignerEntry {
            protected,
            unprotected,
            signer,
        });
        self
    }

    /// Build the signed message.
    pub fn sign(self) -> Result<CoseSignCbor, CoseCryptoError> {
        let body_protected = helpers::serialize_protected(&self.protected)?;

        let mut signatures = Vec::new();
        for entry in &self.signers {
            let sign_protected = helpers::serialize_protected(&entry.protected)?;

            let sig_structure = SigStructure {
                context: SignatureOrSignature1::Signature,
                body_protected: body_protected.clone(),
                sign_protected: Some(sign_protected.clone()),
                external_aad: BytesType::Bytes(self.external_aad.clone()),
                payload: BytesType::Bytes(self.payload.clone()),
            };

            let sig_structure_cbor = SigStructureCbor::try_from(sig_structure)
                .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

            let mut tbs = Vec::new();
            ciborium::ser::into_writer(&sig_structure_cbor, &mut tbs)
                .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

            let sig = entry.signer.sign(&tbs)?;

            signatures.push(CoseSignature {
                protected: sign_protected,
                unprotected: entry.unprotected.clone(),
                signature: BytesType::Bytes(sig),
            });
        }

        let cose_sign = CoseSign {
            protected: body_protected,
            unprotected: self.unprotected,
            payload: BinaryOrNil::Binary(self.payload),
            signatures,
        };

        CoseSignCbor::try_from(cose_sign).map_err(|e| CoseCryptoError::CborError(e.to_string()))
    }
}

/// Verify one signature in a `CoseSign` message by index.
pub fn verify_sign(
    msg: &CoseSignCbor,
    sig_index: usize,
    verifier: &dyn CoseVerifier,
    external_aad: &[u8],
) -> Result<(), CoseCryptoError> {
    let sig_entry = msg
        .signatures
        .get(sig_index)
        .ok_or_else(|| CoseCryptoError::MissingField("signature index out of range".to_string()))?;

    let payload = match &msg.payload {
        BinaryOrNil::Binary(p) => p.clone(),
        BinaryOrNil::Nil => {
            return Err(CoseCryptoError::MissingField("payload".to_string()));
        }
    };

    let sig_structure = SigStructure {
        context: SignatureOrSignature1::Signature,
        body_protected: msg.protected.clone(),
        sign_protected: Some(sig_entry.protected.clone()),
        external_aad: BytesType::Bytes(external_aad.to_vec()),
        payload: BytesType::Bytes(payload),
    };

    let sig_structure_cbor = SigStructureCbor::try_from(sig_structure)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let mut tbs = Vec::new();
    ciborium::ser::into_writer(&sig_structure_cbor, &mut tbs)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let BytesType::Bytes(signature) = &sig_entry.signature;

    verifier.verify(&tbs, signature)
}
