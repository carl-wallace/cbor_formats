//! COSE encryption and decryption (CoseEncrypt0).

use alloc::{string::ToString, vec::Vec};

use common::{BinaryOrNil, BytesType};
use cose::{
    arrays::{CoseEncrypt0, CoseEncrypt0Cbor, EncStructure, EncStructureCbor},
    choices::EncStructureContext,
    maps::{HeaderMap, HeaderMapCbor},
};

use crate::{crypto::CoseAead, error::CoseCryptoError, helpers};

/// Builder for `COSE_Encrypt0` messages.
pub struct CoseEncrypt0Builder {
    plaintext: Vec<u8>,
    protected: HeaderMap,
    unprotected: HeaderMap,
    external_aad: Vec<u8>,
}

impl Default for CoseEncrypt0Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl CoseEncrypt0Builder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            plaintext: Vec::new(),
            protected: helpers::empty_header(),
            unprotected: helpers::empty_header(),
            external_aad: Vec::new(),
        }
    }

    /// Set the plaintext to encrypt.
    pub fn plaintext(mut self, data: &[u8]) -> Self {
        self.plaintext = data.to_vec();
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

    /// Encrypt the plaintext and produce a `CoseEncrypt0Cbor`.
    ///
    /// The IV/nonce must be provided in either the protected or unprotected headers.
    pub fn encrypt(self, aead: &dyn CoseAead) -> Result<CoseEncrypt0Cbor, CoseCryptoError> {
        let protected_serialized = helpers::serialize_protected(&self.protected)?;

        let unprotected_cbor = HeaderMapCbor::try_from(&self.unprotected)
            .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

        let iv = helpers::extract_iv(&protected_serialized, &unprotected_cbor)?;

        let enc_structure = EncStructure {
            context: EncStructureContext::Encrypt0,
            protected: protected_serialized.clone(),
            external_aad: BytesType(self.external_aad),
        };

        let enc_structure_cbor = EncStructureCbor::try_from(enc_structure)
            .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

        let mut aad_bytes = Vec::new();
        ciborium::ser::into_writer(&enc_structure_cbor, &mut aad_bytes)
            .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

        let ciphertext = aead.encrypt(&iv, &aad_bytes, &self.plaintext)?;

        let cose_enc0 = CoseEncrypt0 {
            protected: protected_serialized,
            unprotected: self.unprotected,
            ciphertext: BinaryOrNil::Binary(ciphertext),
        };

        CoseEncrypt0Cbor::try_from(cose_enc0).map_err(|e| CoseCryptoError::CborError(e.to_string()))
    }
}

/// Decrypt a `CoseEncrypt0Cbor` message.
pub fn decrypt_encrypt0(
    msg: &CoseEncrypt0Cbor,
    aead: &dyn CoseAead,
    external_aad: &[u8],
) -> Result<Vec<u8>, CoseCryptoError> {
    let ciphertext = match &msg.ciphertext {
        BinaryOrNil::Binary(c) => c,
        BinaryOrNil::Nil => {
            return Err(CoseCryptoError::MissingField("ciphertext".to_string()));
        }
    };

    let iv = helpers::extract_iv(&msg.protected, &msg.unprotected)?;

    let enc_structure = EncStructure {
        context: EncStructureContext::Encrypt0,
        protected: msg.protected.clone(),
        external_aad: BytesType(external_aad.to_vec()),
    };

    let enc_structure_cbor = EncStructureCbor::try_from(enc_structure)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    let mut aad_bytes = Vec::new();
    ciborium::ser::into_writer(&enc_structure_cbor, &mut aad_bytes)
        .map_err(|e| CoseCryptoError::CborError(e.to_string()))?;

    aead.decrypt(&iv, &aad_bytes, ciphertext)
}
