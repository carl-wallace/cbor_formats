//! COSE MAC operations (CoseMac0, CoseMac).

use alloc::{string::ToString, vec::Vec};

use common::{BinaryOrNil, BytesType};
use cose::{
    arrays::{CoseMac, CoseMac0, CoseMac0Cbor, CoseMacCbor, MacStructure, MacStructureCbor},
    choices::MacStructureContext,
    maps::HeaderMap,
};

use crate::{crypto::CoseMacAlgorithm, error::CoseCryptoError, helpers};

/// Builder for `COSE_Mac0` messages.
pub struct CoseMac0Builder {
    payload: Vec<u8>,
    protected: HeaderMap,
    unprotected: HeaderMap,
    external_aad: Vec<u8>,
}

impl Default for CoseMac0Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl CoseMac0Builder {
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

    /// Compute the MAC tag and produce a `CoseMac0Cbor`.
    ///
    /// Returns an error if the MAC algorithm does not match the algorithm
    /// in the protected header.
    pub fn tag(self, mac: &dyn CoseMacAlgorithm) -> Result<CoseMac0Cbor, CoseCryptoError> {
        let protected_serialized = helpers::serialize_protected(&self.protected)?;
        let header_alg = helpers::extract_algorithm(&protected_serialized)?;
        if header_alg != mac.algorithm() {
            return Err(CoseCryptoError::AlgorithmMismatch {
                header: header_alg,
                key: mac.algorithm(),
            });
        }

        let mac_structure = MacStructure {
            context: MacStructureContext::Mac0,
            protected: protected_serialized.clone(),
            external_aad: BytesType(self.external_aad),
            payload: BytesType(self.payload.clone()),
        };

        let mac_structure_cbor = MacStructureCbor::try_from(mac_structure)
            .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

        let mut to_mac = Vec::new();
        ciborium::ser::into_writer(&mac_structure_cbor, &mut to_mac)
            .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

        let tag_bytes = mac.compute(&to_mac)?;

        let cose_mac0 = CoseMac0 {
            protected: protected_serialized,
            unprotected: self.unprotected,
            payload: BinaryOrNil::Binary(self.payload),
            tag: BytesType(tag_bytes),
        };

        CoseMac0Cbor::try_from(cose_mac0).map_err(|e| CoseCryptoError::CborError(e.to_string()))
    }
}

/// Verify a `CoseMac0Cbor` message.
pub fn verify_mac0(
    msg: &CoseMac0Cbor,
    mac: &dyn CoseMacAlgorithm,
    external_aad: &[u8],
) -> Result<(), CoseCryptoError> {
    let payload = match &msg.payload {
        BinaryOrNil::Binary(p) => p.clone(),
        BinaryOrNil::Nil => {
            return Err(CoseCryptoError::MissingField("payload".to_string()));
        }
    };

    let mac_structure = MacStructure {
        context: MacStructureContext::Mac0,
        protected: msg.protected.clone(),
        external_aad: BytesType(external_aad.to_vec()),
        payload: BytesType(payload),
    };

    let mac_structure_cbor = MacStructureCbor::try_from(mac_structure)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let mut to_mac = Vec::new();
    ciborium::ser::into_writer(&mac_structure_cbor, &mut to_mac)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let BytesType(tag) = &msg.tag;

    mac.verify(&to_mac, tag)
}

/// Builder for `COSE_Mac` messages (multi-recipient, direct key only).
pub struct CoseMacBuilder {
    payload: Vec<u8>,
    protected: HeaderMap,
    unprotected: HeaderMap,
    external_aad: Vec<u8>,
}

impl Default for CoseMacBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CoseMacBuilder {
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

    /// Compute the MAC tag and produce a `CoseMacCbor` (with empty recipients list).
    ///
    /// Returns an error if the MAC algorithm does not match the algorithm
    /// in the protected header.
    pub fn tag(self, mac: &dyn CoseMacAlgorithm) -> Result<CoseMacCbor, CoseCryptoError> {
        let protected_serialized = helpers::serialize_protected(&self.protected)?;
        let header_alg = helpers::extract_algorithm(&protected_serialized)?;
        if header_alg != mac.algorithm() {
            return Err(CoseCryptoError::AlgorithmMismatch {
                header: header_alg,
                key: mac.algorithm(),
            });
        }

        let mac_structure = MacStructure {
            context: MacStructureContext::Mac,
            protected: protected_serialized.clone(),
            external_aad: BytesType(self.external_aad),
            payload: BytesType(self.payload.clone()),
        };

        let mac_structure_cbor = MacStructureCbor::try_from(mac_structure)
            .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

        let mut to_mac = Vec::new();
        ciborium::ser::into_writer(&mac_structure_cbor, &mut to_mac)
            .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

        let tag_bytes = mac.compute(&to_mac)?;

        let cose_mac = CoseMac {
            protected: protected_serialized,
            unprotected: self.unprotected,
            payload: BinaryOrNil::Binary(self.payload),
            tag: BytesType(tag_bytes),
            recipients: Vec::new(),
        };

        CoseMacCbor::try_from(cose_mac).map_err(|e| CoseCryptoError::CborError(e.to_string()))
    }
}

/// Verify a `CoseMacCbor` message (direct key).
pub fn verify_mac(
    msg: &CoseMacCbor,
    mac: &dyn CoseMacAlgorithm,
    external_aad: &[u8],
) -> Result<(), CoseCryptoError> {
    let payload = match &msg.payload {
        BinaryOrNil::Binary(p) => p.clone(),
        BinaryOrNil::Nil => {
            return Err(CoseCryptoError::MissingField("payload".to_string()));
        }
    };

    let mac_structure = MacStructure {
        context: MacStructureContext::Mac,
        protected: msg.protected.clone(),
        external_aad: BytesType(external_aad.to_vec()),
        payload: BytesType(payload),
    };

    let mac_structure_cbor = MacStructureCbor::try_from(mac_structure)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let mut to_mac = Vec::new();
    ciborium::ser::into_writer(&mac_structure_cbor, &mut to_mac)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let BytesType(tag) = &msg.tag;

    mac.verify(&to_mac, tag)
}
