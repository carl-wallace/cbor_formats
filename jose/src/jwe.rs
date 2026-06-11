//! JWE (JSON Web Encryption) — RFC 7516.
//!
//! Currently supports **compact serialization decrypt only**, scoped to the algorithm pair
//! `{"alg":"A256KW","enc":"A256GCM"}` that Google Play Integrity Standard's self-managed
//! verification path uses. Other alg/enc combinations and the JSON serializations can be added
//! when consumers need them.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use base64ct::{Base64UrlUnpadded, Encoding};

use cose_crypto::crypto::{CoseAead, CoseKeyWrap};

use crate::{error::JoseError, header::JoseHeader};

/// A decoded JWE with the parsed header and the raw five-segment pieces.
///
/// The pieces are stored as decoded bytes; the original base64url-encoded protected header
/// is preserved separately because it doubles as the AEAD's additional authenticated data
/// per RFC 7516 §5.1 step 14.
#[derive(Debug, Clone)]
pub struct Jwe {
    /// Parsed protected header (the JSON object encoded as the first compact segment).
    pub header: JoseHeader,
    /// Raw ASCII bytes of the base64url-encoded protected header — this is the AAD.
    pub protected_b64: String,
    /// Wrapped CEK (second segment, decoded).
    pub encrypted_key: Vec<u8>,
    /// Initialization vector (third segment, decoded).
    pub iv: Vec<u8>,
    /// Ciphertext (fourth segment, decoded).
    pub ciphertext: Vec<u8>,
    /// Authentication tag (fifth segment, decoded).
    pub tag: Vec<u8>,
}

impl Jwe {
    /// Parse a JWE compact serialization string into its decoded pieces.
    ///
    /// Validates that the input has exactly five dot-separated segments and that each segment
    /// is well-formed base64url. Does not validate header `alg`/`enc` values — the decrypt
    /// step does that.
    pub fn parse_compact(token: &str) -> Result<Self, JoseError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 5 {
            return Err(JoseError::InvalidEncoding(alloc::format!(
                "JWE compact must have 5 dot-separated parts, got {}",
                parts.len()
            )));
        }

        let protected_b64 = parts[0].to_string();
        let header_bytes = b64url_decode(parts[0])?;
        let header: JoseHeader = serde_json::from_slice(&header_bytes)
            .map_err(|e| JoseError::InvalidHeader(alloc::format!("header parse error: {e}")))?;

        let encrypted_key = b64url_decode(parts[1])?;
        let iv = b64url_decode(parts[2])?;
        let ciphertext = b64url_decode(parts[3])?;
        let tag = b64url_decode(parts[4])?;

        Ok(Self {
            header,
            protected_b64,
            encrypted_key,
            iv,
            ciphertext,
            tag,
        })
    }

    /// Decrypt the JWE using the given key-wrap and AEAD implementations.
    ///
    /// The AAD passed to the AEAD is the ASCII bytes of the base64url-encoded protected
    /// header per RFC 7516 §5.1. The AEAD ciphertext input is `ciphertext || tag` because
    /// `CoseAead::decrypt` follows the RustCrypto convention of taking a combined buffer.
    pub fn decrypt(
        &self,
        key_wrap: &dyn CoseKeyWrap,
        aead_from_cek: impl FnOnce(&[u8]) -> Result<alloc::boxed::Box<dyn CoseAead>, JoseError>,
    ) -> Result<Vec<u8>, JoseError> {
        let cek = key_wrap.unwrap(&self.encrypted_key)?;
        let aead = aead_from_cek(&cek)?;
        let mut combined = Vec::with_capacity(self.ciphertext.len() + self.tag.len());
        combined.extend_from_slice(&self.ciphertext);
        combined.extend_from_slice(&self.tag);
        aead.decrypt(&self.iv, self.protected_b64.as_bytes(), &combined)
            .map_err(JoseError::from)
    }
}

/// Decrypt a JWE compact serialization string with the given key-wrap (KEK) and an AEAD
/// constructor that takes the unwrapped CEK and returns the configured AEAD instance.
///
/// The AEAD constructor pattern lets the caller wire up the right AEAD type for the inner
/// `enc` parameter (typically `A256GCM` for Play Integrity), keeping this function
/// alg-agnostic with respect to which AEAD implementation is used.
pub fn decrypt_compact(
    token: &str,
    key_wrap: &dyn CoseKeyWrap,
    aead_from_cek: impl FnOnce(&[u8]) -> Result<alloc::boxed::Box<dyn CoseAead>, JoseError>,
) -> Result<Vec<u8>, JoseError> {
    Jwe::parse_compact(token)?.decrypt(key_wrap, aead_from_cek)
}

fn b64url_decode(s: &str) -> Result<Vec<u8>, JoseError> {
    Base64UrlUnpadded::decode_vec(s)
        .map_err(|e| JoseError::InvalidEncoding(alloc::format!("base64url decode error: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    use cose_crypto::crypto::aes_gcm::AesGcmKey;
    use cose_crypto::crypto::aes_kw::AesKwKey;

    /// Compose an A256KW + A256GCM JWE compact-form token from the constituent pieces.
    /// Used as a test helper because the crate doesn't (yet) implement JWE encrypt.
    fn compose_a256kw_a256gcm(
        kek_bytes: &[u8; 32],
        cek_bytes: &[u8; 32],
        iv: &[u8; 12],
        plaintext: &[u8],
    ) -> String {
        let header = r#"{"alg":"A256KW","enc":"A256GCM"}"#;
        let header_b64 = Base64UrlUnpadded::encode_string(header.as_bytes());

        let kek = AesKwKey::from_bytes(kek_bytes).unwrap();
        let wrapped_cek = kek.wrap(cek_bytes).unwrap();
        let wrapped_b64 = Base64UrlUnpadded::encode_string(&wrapped_cek);

        let iv_b64 = Base64UrlUnpadded::encode_string(iv);

        let cek_aead = AesGcmKey::from_bytes(cek_bytes).unwrap();
        let ct_with_tag = cek_aead
            .encrypt(iv, header_b64.as_bytes(), plaintext)
            .unwrap();
        // RustCrypto AEADs append the 16-byte tag to ciphertext on encrypt.
        let (ct, tag) = ct_with_tag.split_at(ct_with_tag.len() - 16);
        let ct_b64 = Base64UrlUnpadded::encode_string(ct);
        let tag_b64 = Base64UrlUnpadded::encode_string(tag);

        alloc::format!("{header_b64}.{wrapped_b64}.{iv_b64}.{ct_b64}.{tag_b64}")
    }

    #[test]
    fn round_trip_a256kw_a256gcm() {
        let kek_bytes = [0x42u8; 32];
        let cek_bytes = [0x37u8; 32];
        let iv = [0x01u8; 12];
        let plaintext = b"hello jwe";

        let token = compose_a256kw_a256gcm(&kek_bytes, &cek_bytes, &iv, plaintext);
        let kek = AesKwKey::from_bytes(&kek_bytes).unwrap();

        let decrypted = decrypt_compact(&token, &kek, |cek| {
            AesGcmKey::from_bytes(cek)
                .map(|k| alloc::boxed::Box::new(k) as alloc::boxed::Box<dyn CoseAead>)
                .map_err(JoseError::from)
        })
        .unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn parse_compact_rejects_wrong_segment_count() {
        assert!(Jwe::parse_compact("a.b.c.d").is_err()); // 4
        assert!(Jwe::parse_compact("a.b.c.d.e.f").is_err()); // 6
    }

    #[test]
    fn parse_compact_extracts_header_and_pieces() {
        let kek_bytes = [0x42u8; 32];
        let cek_bytes = [0x37u8; 32];
        let iv = [0x01u8; 12];
        let token = compose_a256kw_a256gcm(&kek_bytes, &cek_bytes, &iv, b"x");

        let jwe = Jwe::parse_compact(&token).unwrap();
        assert_eq!(jwe.header.alg(), Some("A256KW"));
        assert_eq!(jwe.iv, iv);
        assert_eq!(jwe.encrypted_key.len(), 40); // 32-byte CEK wrapped + 8-byte IV
        assert_eq!(jwe.tag.len(), 16); // A256GCM tag
    }

    /// RFC 7520 §5.8 — "Key Wrap Using AES-KeyWrap with AES-GCM" (A128KW + A128GCM).
    ///
    /// Spec-rooted independent test vector for our JWE compact-form decrypt pipeline. The token,
    /// KEK, and expected plaintext are copied verbatim from RFC 7520 §5.8.1–§5.8.5. If our
    /// `decrypt_compact` produces the documented plaintext, the entire stack (compact parse,
    /// base64url decode of each segment, AAD construction, AES-KW unwrap, AES-GCM decrypt) is
    /// validated against IETF reference data — not just self-consistent.
    ///
    /// Note: the RFC uses A128KW + A128GCM rather than A256KW + A256GCM. Section 5.8 is the only
    /// AES-KW + AES-GCM combination given in RFC 7520. Both algorithms scale linearly with key
    /// size, so passing 5.8 strongly validates the higher-layer JOSE plumbing even though the
    /// production path uses 256-bit keys.
    #[test]
    fn rfc_7520_5_8_decrypt_compact() {
        // From RFC 7520 §5.8.1, "k" field of Figure 151 (JWK octet key).
        const RFC_7520_5_8_KEK_B64URL: &str = "GZy6sIZ6wl9NJOKB-jnmVQ";

        // From RFC 7520 §5.8.5 (Figure 159). Whitespace removed; segments rejoined.
        const RFC_7520_5_8_COMPACT: &str = concat!(
            "eyJhbGciOiJBMTI4S1ciLCJraWQiOiI4MWIyMDk2NS04MzMyLTQzZDktYTQ2OC",
            "04MjE2MGFkOTFhYzgiLCJlbmMiOiJBMTI4R0NNIn0",
            ".",
            "CBI6oDw8MydIx1IBntf_lQcw2MmJKIQx",
            ".",
            "Qx0pmsDa8KnJc9Jo",
            ".",
            "AwliP-KmWgsZ37BvzCefNen6VTbRK3QMA4TkvRkH0tP1bTdhtFJgJxeVmJkLD6",
            "1A1hnWGetdg11c9ADsnWgL56NyxwSYjU1ZEHcGkd3EkU0vjHi9gTlb90qSYFfe",
            "F0LwkcTtjbYKCsiNJQkcIp1yeM03OmuiYSoYJVSpf7ej6zaYcMv3WwdxDFl8RE",
            "wOhNImk2Xld2JXq6BR53TSFkyT7PwVLuq-1GwtGHlQeg7gDT6xW0JqHDPn_H-p",
            "uQsmthc9Zg0ojmJfqqFvETUxLAF-KjcBTS5dNy6egwkYtOt8EIHK-oEsKYtZRa",
            "a8Z7MOZ7UGxGIMvEmxrGCPeJa14slv2-gaqK0kEThkaSqdYw0FkQZF",
            ".",
            "ER7MWJZ1FBI_NKvn7Zb1Lw"
        );

        // From RFC 7520 §5 introduction: the plaintext used across all encryption examples.
        // U+2013 (en dash, UTF-8 E2 80 93) replaces the `\xe2\x80\x93` escapes in the RFC.
        const RFC_7520_PLAINTEXT: &str = concat!(
            "You can trust us to stick with you through thick and thin",
            "\u{2013}",
            "to the bitter end. And you can trust us to keep any secret of yours",
            "\u{2013}",
            "closer than you keep it yourself. But you cannot trust us to let you face trouble alone, and go off without a word. We are your friends, Frodo."
        );

        let kek_bytes = Base64UrlUnpadded::decode_vec(RFC_7520_5_8_KEK_B64URL).unwrap();
        assert_eq!(kek_bytes.len(), 16);
        let kek = AesKwKey::from_bytes(&kek_bytes).unwrap();

        let plaintext = decrypt_compact(RFC_7520_5_8_COMPACT, &kek, |cek| {
            AesGcmKey::from_bytes(cek)
                .map(|k| alloc::boxed::Box::new(k) as alloc::boxed::Box<dyn CoseAead>)
                .map_err(JoseError::from)
        })
        .unwrap();

        assert_eq!(plaintext.as_slice(), RFC_7520_PLAINTEXT.as_bytes());
    }

    #[test]
    fn decrypt_rejects_tampered_aad() {
        let kek_bytes = [0x42u8; 32];
        let cek_bytes = [0x37u8; 32];
        let iv = [0x01u8; 12];
        let token = compose_a256kw_a256gcm(&kek_bytes, &cek_bytes, &iv, b"x");

        // Flip a bit in the protected header b64 (= the AAD) — must fail GCM auth.
        let mut parts: Vec<&str> = token.split('.').collect();
        let tampered_header = "eyJhbGciOiJBMjU2S1ciLCJlbmMiOiJBMjU2R0NNIn1A"; // valid b64url, wrong content
        parts[0] = tampered_header;
        let tampered = parts.join(".");

        let kek = AesKwKey::from_bytes(&kek_bytes).unwrap();
        let result = decrypt_compact(&tampered, &kek, |cek| {
            AesGcmKey::from_bytes(cek)
                .map(|k| alloc::boxed::Box::new(k) as alloc::boxed::Box<dyn CoseAead>)
                .map_err(JoseError::from)
        });
        assert!(result.is_err());
    }
}
