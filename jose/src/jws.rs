//! JWS (JSON Web Signature) — RFC 7515.
//!
//! Supports compact serialization (§7.1), JSON general serialization (§7.2.1),
//! and JSON flattened serialization (§7.2.2). Algorithm-agnostic: the caller
//! provides a `CoseSigner`/`CoseVerifier` and the `alg` string in the header.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use base64ct::{Base64UrlUnpadded, Encoding};
use cose_crypto::crypto::{CoseSigner, CoseVerifier};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::error::JoseError;
use crate::header::JoseHeader;

// ---------------------------------------------------------------------------
// Decoded JWS
// ---------------------------------------------------------------------------

/// A decoded JWS with the parsed protected header, raw payload, and signature.
#[derive(Debug, Clone)]
pub struct Jws {
    /// The protected (signed) header.
    pub header: JoseHeader,
    /// The decoded payload bytes.
    pub payload: Vec<u8>,
    /// The raw signature bytes.
    pub signature: Vec<u8>,
}

// ---------------------------------------------------------------------------
// JWS JSON Serialization structures
// ---------------------------------------------------------------------------

/// JWS JSON Serialization — general form (RFC 7515 §7.2.1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwsJson {
    /// Base64url-encoded payload.
    pub payload: String,
    /// Array of signature entries.
    pub signatures: Vec<JwsJsonSignature>,
}

/// A single signature entry in the general JWS JSON serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwsJsonSignature {
    /// Base64url-encoded protected header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<String>,
    /// Unprotected header parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Map<String, Value>>,
    /// Base64url-encoded signature.
    pub signature: String,
}

/// JWS JSON Flattened Serialization (RFC 7515 §7.2.2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwsFlatJson {
    /// Base64url-encoded protected header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<String>,
    /// Unprotected header parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Map<String, Value>>,
    /// Base64url-encoded payload.
    pub payload: String,
    /// Base64url-encoded signature.
    pub signature: String,
}

// ---------------------------------------------------------------------------
// Base64url helpers
// ---------------------------------------------------------------------------

fn b64url_encode(data: &[u8]) -> String {
    Base64UrlUnpadded::encode_string(data)
}

fn b64url_decode(s: &str) -> Result<Vec<u8>, JoseError> {
    Base64UrlUnpadded::decode_vec(s)
        .map_err(|e| JoseError::InvalidEncoding(format!("base64url decode error: {e}")))
}

// ---------------------------------------------------------------------------
// JwsBuilder
// ---------------------------------------------------------------------------

/// Builder for JWS messages. Algorithm-agnostic — the caller provides the
/// header (with `alg` set) and a `CoseSigner`.
pub struct JwsBuilder {
    payload: Vec<u8>,
    header: JoseHeader,
    detached: bool,
}

impl JwsBuilder {
    /// Create a new builder with the given protected header.
    ///
    /// The header MUST contain an `alg` parameter.
    pub fn new(header: JoseHeader) -> Self {
        Self {
            payload: Vec::new(),
            header,
            detached: false,
        }
    }

    /// Set the payload.
    pub fn payload(mut self, data: &[u8]) -> Self {
        self.payload = data.to_vec();
        self
    }

    /// If true, produce a detached payload (empty payload field in output).
    pub fn detached(mut self, detached: bool) -> Self {
        self.detached = detached;
        self
    }

    /// Compute the signing input per RFC 7515 §5.1.
    fn signing_input(&self) -> Result<(String, String), JoseError> {
        let header_json = serde_json::to_vec(&self.header)
            .map_err(|e| JoseError::InvalidHeader(format!("header serialize error: {e}")))?;
        let protected_b64 = b64url_encode(&header_json);
        let payload_b64 = b64url_encode(&self.payload);
        Ok((protected_b64, payload_b64))
    }

    /// Produce JWS Compact Serialization (RFC 7515 §7.1).
    ///
    /// Returns `base64url(header).base64url(payload).base64url(signature)`.
    /// If `detached` is set, the payload portion is empty.
    pub fn sign_compact(self, signer: &dyn CoseSigner) -> Result<String, JoseError> {
        let (protected_b64, payload_b64) = self.signing_input()?;

        let signing_input = format!("{protected_b64}.{payload_b64}");
        let signature = signer.sign(signing_input.as_bytes())?;
        let signature_b64 = b64url_encode(&signature);

        if self.detached {
            Ok(format!("{protected_b64}..{signature_b64}"))
        } else {
            Ok(format!("{protected_b64}.{payload_b64}.{signature_b64}"))
        }
    }

    /// Produce JWS JSON Flattened Serialization (RFC 7515 §7.2.2).
    pub fn sign_flat_json(self, signer: &dyn CoseSigner) -> Result<JwsFlatJson, JoseError> {
        let (protected_b64, payload_b64) = self.signing_input()?;

        let signing_input = format!("{protected_b64}.{payload_b64}");
        let signature = signer.sign(signing_input.as_bytes())?;
        let signature_b64 = b64url_encode(&signature);

        let payload_out = if self.detached {
            String::new()
        } else {
            payload_b64
        };

        Ok(JwsFlatJson {
            protected: Some(protected_b64),
            header: None,
            payload: payload_out,
            signature: signature_b64,
        })
    }
}

// ---------------------------------------------------------------------------
// Verification
// ---------------------------------------------------------------------------

/// Verify a JWS Compact Serialization string.
///
/// Returns the decoded `Jws` on success.
pub fn verify_compact(compact: &str, verifier: &dyn CoseVerifier) -> Result<Jws, JoseError> {
    let parts: Vec<&str> = compact.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err(JoseError::InvalidEncoding(
            "JWS compact must have 3 dot-separated parts".to_string(),
        ));
    }

    let protected_b64 = parts[0];
    let payload_b64 = parts[1];
    let signature_b64 = parts[2];

    let header_bytes = b64url_decode(protected_b64)?;
    let header: JoseHeader = serde_json::from_slice(&header_bytes)
        .map_err(|e| JoseError::InvalidHeader(format!("header parse error: {e}")))?;

    let payload = b64url_decode(payload_b64)?;
    let signature = b64url_decode(signature_b64)?;

    let signing_input = format!("{protected_b64}.{payload_b64}");
    verifier
        .verify(signing_input.as_bytes(), &signature)
        .map_err(|_| JoseError::InvalidSignature)?;

    Ok(Jws {
        header,
        payload,
        signature,
    })
}

/// Verify a JWS Compact Serialization with a detached payload.
///
/// The `compact` string should have an empty middle part (`header..signature`).
/// The `payload` is provided separately.
pub fn verify_compact_detached(
    compact: &str,
    payload: &[u8],
    verifier: &dyn CoseVerifier,
) -> Result<Jws, JoseError> {
    let parts: Vec<&str> = compact.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err(JoseError::InvalidEncoding(
            "JWS compact must have 3 dot-separated parts".to_string(),
        ));
    }

    let protected_b64 = parts[0];
    let signature_b64 = parts[2];

    let header_bytes = b64url_decode(protected_b64)?;
    let header: JoseHeader = serde_json::from_slice(&header_bytes)
        .map_err(|e| JoseError::InvalidHeader(format!("header parse error: {e}")))?;

    let signature = b64url_decode(signature_b64)?;
    let payload_b64 = b64url_encode(payload);

    let signing_input = format!("{protected_b64}.{payload_b64}");
    verifier
        .verify(signing_input.as_bytes(), &signature)
        .map_err(|_| JoseError::InvalidSignature)?;

    Ok(Jws {
        header,
        payload: payload.to_vec(),
        signature,
    })
}

/// Verify a JWS JSON Flattened Serialization.
pub fn verify_flat_json(flat: &JwsFlatJson, verifier: &dyn CoseVerifier) -> Result<Jws, JoseError> {
    let protected_b64 = flat
        .protected
        .as_deref()
        .ok_or_else(|| JoseError::MissingField("protected".to_string()))?;

    let header_bytes = b64url_decode(protected_b64)?;
    let header: JoseHeader = serde_json::from_slice(&header_bytes)
        .map_err(|e| JoseError::InvalidHeader(format!("header parse error: {e}")))?;

    let payload = b64url_decode(&flat.payload)?;
    let signature = b64url_decode(&flat.signature)?;

    let signing_input = format!("{protected_b64}.{}", flat.payload);
    verifier
        .verify(signing_input.as_bytes(), &signature)
        .map_err(|_| JoseError::InvalidSignature)?;

    Ok(Jws {
        header,
        payload,
        signature,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::jwk::Jwk;

    // -----------------------------------------------------------------------
    // Test keys from RFC 7520 §3
    // -----------------------------------------------------------------------

    // RFC 7520 §3.1 — EC P-256 private key (also used in cose_crypto tests).
    const RFC7520_EC_P256_JWK: &[u8] = br#"{
        "kty": "EC",
        "kid": "bilbo.baggins@hobbiton.example",
        "use": "sig",
        "crv": "P-256",
        "x": "MKBCTNIcKUSDii11ySs3526iDZ8AiTo7Tu6KPAqv7D4",
        "y": "4Etl6SRW2YiLUrN5vfvVHuhp7x8PxltmWWlbbM4IFyM",
        "d": "870MB6gfuTJ4HtUnUvYMyJpr5eUZNP4Bk43bVdj3eAE"
    }"#;

    // -----------------------------------------------------------------------
    // Payload from RFC 7520 §4
    // -----------------------------------------------------------------------

    // "It\u{2019}s a dangerous business, Frodo, going out your door. You step
    //  onto the road, and if you don\u{2019}t keep your feet, there\u{2019}s no
    //  knowing where you might be swept off to."
    const RFC7520_PAYLOAD_B64: &str = "SXTigJlzIGEgZGFuZ2Vyb3VzIGJ1c2luZXNzLCBGcm9kb\
        ywgZ29pbmcgb3V0IHlvdXIgZG9vci4gWW91IHN0ZXAgb250byB0aGUgcm9hZCwgYW5kIGlmIHlvdS\
        Bkb24ndCBrZWVwIHlvdXIgZmVldCwgdGhlcmXigJlzIG5vIGtub3dpbmcgd2hlcmUgeW91IG1pZ2h0I\
        GJlIHN3ZXB0IG9mZiB0by4";

    // -----------------------------------------------------------------------
    // RFC 7515 §A.1 payload (different from RFC 7520; used in older examples)
    // -----------------------------------------------------------------------

    // {"iss":"joe",\r\n "exp":1300819380,\r\n "http://example.com/is_root":true}
    const RFC7515_PAYLOAD_B64: &str =
        "eyJpc3MiOiJqb2UiLA0KICJleHAiOjEzMDA4MTkzODAsDQogImh0dHA6Ly9leGFtcGxlLmNvbS9pc19yb290Ijp0cnVlfQ";

    fn signer_and_verifier(
        jwk_json: &[u8],
    ) -> (
        alloc::boxed::Box<dyn CoseSigner>,
        alloc::boxed::Box<dyn CoseVerifier>,
    ) {
        let jwk = Jwk::from_json(jwk_json).unwrap();
        (jwk.to_signer().unwrap(), jwk.to_verifier().unwrap())
    }

    // -----------------------------------------------------------------------
    // Encoding tests using RFC 7515 / RFC 7520 known values
    // -----------------------------------------------------------------------

    #[test]
    fn rfc7515_payload_encoding() {
        // RFC 7515 §A.1: verify our base64url codec round-trips the canonical encoding.
        let payload_bytes = b64url_decode(RFC7515_PAYLOAD_B64).unwrap();
        let re_encoded = b64url_encode(&payload_bytes);
        assert_eq!(re_encoded, RFC7515_PAYLOAD_B64);
    }

    #[test]
    fn rfc7520_payload_encoding() {
        // RFC 7520 §4: verify the payload round-trips.
        let payload_bytes = b64url_decode(RFC7520_PAYLOAD_B64).unwrap();
        let text = String::from_utf8(payload_bytes.clone()).unwrap();
        assert!(text.starts_with("It\u{2019}s a dangerous business"));
        let re_encoded = b64url_encode(&payload_bytes);
        assert_eq!(re_encoded, RFC7520_PAYLOAD_B64);
    }

    #[test]
    fn rfc7515_header_encoding() {
        // RFC 7515 §A.3: {"alg":"ES256"} encodes to "eyJhbGciOiJFUzI1NiJ9".
        let header = JoseHeader::new("ES256");
        let header_json = serde_json::to_vec(&header).unwrap();
        let header_b64 = b64url_encode(&header_json);
        assert_eq!(header_b64, "eyJhbGciOiJFUzI1NiJ9");
    }

    // -----------------------------------------------------------------------
    // Sign + verify round-trip tests using RFC 7520 key and payload
    // -----------------------------------------------------------------------

    #[test]
    fn compact_roundtrip_es256_rfc7520() {
        // RFC 7520 §3.1 P-256 key + §4 payload.
        // ECDSA is non-deterministic, so we sign-then-verify.
        let (signer, verifier) = signer_and_verifier(RFC7520_EC_P256_JWK);
        let payload = b64url_decode(RFC7520_PAYLOAD_B64).unwrap();

        let header = JoseHeader::new("ES256");
        let compact = JwsBuilder::new(header)
            .payload(&payload)
            .sign_compact(signer.as_ref())
            .unwrap();

        assert_eq!(compact.matches('.').count(), 2);

        let jws = verify_compact(&compact, verifier.as_ref()).unwrap();
        assert_eq!(jws.payload, payload);
        assert_eq!(jws.header.alg(), Some("ES256"));
    }

    #[test]
    fn compact_detached_roundtrip_rfc7520() {
        // RFC 7520 §4.5 demonstrates detached content with HMAC, but the
        // mechanism is the same for ECDSA. Use §3.1 P-256 key + §4 payload.
        let (signer, verifier) = signer_and_verifier(RFC7520_EC_P256_JWK);
        let payload = b64url_decode(RFC7520_PAYLOAD_B64).unwrap();

        let header = JoseHeader::new("ES256");
        let compact = JwsBuilder::new(header)
            .payload(&payload)
            .detached(true)
            .sign_compact(signer.as_ref())
            .unwrap();

        // Middle part should be empty (RFC 7515 §Appendix F)
        let parts: Vec<&str> = compact.splitn(3, '.').collect();
        assert_eq!(parts[1], "");

        let jws = verify_compact_detached(&compact, &payload, verifier.as_ref()).unwrap();
        assert_eq!(jws.payload, payload);
    }

    #[test]
    fn flat_json_roundtrip_rfc7520() {
        // Flattened JSON serialization per RFC 7515 §7.2.2, using
        // RFC 7520 §3.1 key + §4 payload.
        let (signer, verifier) = signer_and_verifier(RFC7520_EC_P256_JWK);
        let payload = b64url_decode(RFC7520_PAYLOAD_B64).unwrap();

        let header = JoseHeader::new("ES256");
        let flat = JwsBuilder::new(header)
            .payload(&payload)
            .sign_flat_json(signer.as_ref())
            .unwrap();

        assert!(flat.protected.is_some());
        assert!(!flat.payload.is_empty());
        assert!(!flat.signature.is_empty());

        // JSON round-trip
        let json_str = serde_json::to_string(&flat).unwrap();
        let parsed: JwsFlatJson = serde_json::from_str(&json_str).unwrap();
        assert_eq!(flat.payload, parsed.payload);

        let jws = verify_flat_json(&flat, verifier.as_ref()).unwrap();
        assert_eq!(jws.payload, payload);
    }

    // -----------------------------------------------------------------------
    // Negative tests
    // -----------------------------------------------------------------------

    #[test]
    fn verify_rejects_tampered_payload() {
        let (signer, verifier) = signer_and_verifier(RFC7520_EC_P256_JWK);
        let payload = b64url_decode(RFC7520_PAYLOAD_B64).unwrap();

        let header = JoseHeader::new("ES256");
        let compact = JwsBuilder::new(header)
            .payload(&payload)
            .sign_compact(signer.as_ref())
            .unwrap();

        // Replace the payload with different content
        let parts: Vec<&str> = compact.splitn(3, '.').collect();
        let tampered_payload = b64url_encode(b"tampered");
        let tampered = format!("{}.{}.{}", parts[0], tampered_payload, parts[2]);

        let result = verify_compact(&tampered, verifier.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn verify_rejects_tampered_signature() {
        let (signer, verifier) = signer_and_verifier(RFC7520_EC_P256_JWK);

        let header = JoseHeader::new("ES256");
        let compact = JwsBuilder::new(header)
            .payload(b"test data")
            .sign_compact(signer.as_ref())
            .unwrap();

        // Flip the last character of the signature
        let mut tampered = compact.clone();
        let last = tampered.pop().unwrap();
        tampered.push(if last == 'A' { 'B' } else { 'A' });

        let result = verify_compact(&tampered, verifier.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn invalid_compact_format() {
        let (_, verifier) = signer_and_verifier(RFC7520_EC_P256_JWK);
        let result = verify_compact("only.two_parts", verifier.as_ref());
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // Header tests
    // -----------------------------------------------------------------------

    #[test]
    fn header_with_registered_params() {
        // RFC 7515 §4.1 registered header parameter names.
        let mut header = JoseHeader::new("ES256");
        header.set_kid("bilbo.baggins@hobbiton.example");
        header.set_typ("JWT");

        assert_eq!(header.alg(), Some("ES256"));
        assert_eq!(header.kid(), Some("bilbo.baggins@hobbiton.example"));
        assert_eq!(header.typ(), Some("JWT"));

        let json = serde_json::to_string(&header).unwrap();
        assert!(json.contains("\"alg\":\"ES256\""));
        assert!(json.contains("\"kid\":\"bilbo.baggins@hobbiton.example\""));
        assert!(json.contains("\"typ\":\"JWT\""));
    }

    #[test]
    fn header_extension_params() {
        let mut header = JoseHeader::new("ES256");
        header.set("custom_param", "custom_value");

        assert_eq!(header.0.get("custom_param").unwrap(), "custom_value");

        let json = serde_json::to_string(&header).unwrap();
        assert!(json.contains("custom_param"));
    }

    // -----------------------------------------------------------------------
    // JWK tests using RFC 7520 §3.5 symmetric key
    // -----------------------------------------------------------------------

    #[test]
    fn jwk_accessors_rfc7520() {
        // RFC 7520 §3.5 — HMAC-SHA2 symmetric key.
        let jwk_json = br#"{
            "kty": "oct",
            "kid": "018c0ae5-4d9b-471b-bfd6-eef314bc7037",
            "use": "sig",
            "alg": "HS256",
            "k": "hJtXIZ2uSN5kbQfbtTNWbpdmhkV8FJG-Onbc6mxCcYg"
        }"#;
        let jwk = Jwk::from_json(jwk_json).unwrap();
        assert_eq!(jwk.kty(), Some("oct"));
        assert_eq!(jwk.kid(), Some("018c0ae5-4d9b-471b-bfd6-eef314bc7037"));
        assert_eq!(jwk.alg(), Some("HS256"));
        assert_eq!(jwk.use_(), Some("sig"));
    }

    #[test]
    fn jwk_set_find_by_kid() {
        let jwk_set_json = br#"{
            "keys": [
                {"kty": "EC", "kid": "key-1", "crv": "P-256", "x": "a", "y": "b"},
                {"kty": "oct", "kid": "key-2", "k": "c"}
            ]
        }"#;
        let set: crate::jwk::JwkSet = serde_json::from_slice(jwk_set_json).unwrap();
        assert_eq!(set.keys.len(), 2);
        assert!(set.find_by_kid("key-1").is_some());
        assert!(set.find_by_kid("key-2").is_some());
        assert!(set.find_by_kid("key-3").is_none());
    }

    // -----------------------------------------------------------------------
    // ML-DSA / AKP key type tests (draft-ietf-cose-dilithium-11)
    // -----------------------------------------------------------------------

    // ML-DSA-44 JWK from draft-ietf-cose-dilithium-11 (truncated pub for readability;
    // full value used at runtime).
    const DILITHIUM_ML_DSA_44_JWK: &[u8] = br#"{
        "kid": "T4xl70S7MT6Zeq6r9V9fPJGVn76wfnXJ21-gyo0Gu6o",
        "kty": "AKP",
        "alg": "ML-DSA-44",
        "pub": "unH59k4RuutY-pxvu24U5h8YZD2rSVtHU5qRZsoBmBMcRPgmu9VuNOVdteXi1zNIXjnqJg_GAAxepLqA00Vc3lO0bzRIKu39VFD8Lhuk8l0V-cFEJC-zm7UihxiQMMUEmOFxe3x1ixkKZ0jqmqP3rKryx8tSbtcXyfea64QhT6XNje2SoMP6FViBDxLHBQo2dwjRls0k5a-XSQSu2OTOiHLoaWsLe8pQ5FLNfTDqmkrawDEdZyxr3oSWJAsHQxRjcIiVzZuvwxYy1zl2STiP2vy_fTBaPemkleynQzqPg7oPCyXEE8bjnJbrfWkbNNN8438e6tHPIX4l7zTuzz98YPhLjt_d6EBdT4MldsYe-Y4KLyjaGHcAlTkk9oa5RhRwW89T0z_t1DSO3dvfKLUGXh8gd1BD6Fz5MfgpF5NjoafnQEqDjsAAhrCXY4b-Y3yYJEdX4_dp3dRGdHG_rWcPmgX4JG7lCnser4f8QGnDriqiAzJYEXeS8LzUngg_0bx0lqv_KcyU5IaLISFO0xZSU5mmEPvdSoDnyAcV8pV44qhLtAvd29n0ehG259oRihtljTWeiu9V60a1N2tbZVl5mEqSK-6_xZvNYA1TCdzNctvweH24unV7U3wer9XA9Q6kvJWDVJ4oKaQsKMrCSMlteBJMRxWbGK7ddUq6F7GdQw-3j2M-qdJvVKm9UPjY9rc1lPgol25-oJxTu7nxGlbJUH-4m5pevAN6NyZ6lfhbjWTKlxkrEKZvQXs_Yf6cpXEwpI_ZJeriq1UC1XHIpRkDwdOY9MH3an4RdDl2r9vGl_IwlKPNdh_5aF3jLgn7PCit1FNJAwC8fIncAXgAlgcXIpRXdfJk4bBiO89GGccSyDh2EgXYdpG3XvNgGWy7npuSoNTE7WIyblAk13UQuO4sdCbMIuriCdyfE73mvwj15xgb07RZRQtFGlFTmnFcIdZ90zDrWXDbANntv7KCKwNvoTuv64bY3HiGbj-NQ-U9eMylWVpvr4hrXcES8c9K3PqHWADZC0iIOvlzFv4VBoc_wVflcOrL_SIoaNFCNBAZZq-2v5lAgpJTqVOtqJ_HVraoSfcKy5g45p-qULunXj6Jwq21fobQiKubBKKOZwcJFyJD7F4ACKXOrz-HIvSHMCWW_9dVrRuCpJw0s0aVFbRqopDNhu446nqb4_EDYQM1tTHMozPd_jKxRRD0sH75X8ZoToxFSpLBDbtdWcenxj-zBf6IGWfZnmaetjKEBYJWC7QDQx1A91pJVJCEgieCkoIfTqkeQuePpIyu48g2FG3P1zjRF-kumhUTfSjo5qS0YiZQy0E1BMs6M11EvuxXRsHClLHoy5nLYI2Sj4zjVjYyxSHyPRPGGo9hwB34yWxzYNtPPGiqXS_dNCpi_zRZwRY4lCGrQ-hYTEWIK1Dm5OlttvC4_eiQ1dv63NiGkLRJ5kJA3bICN0fzCDY-MBqnd1cWn8YVBijVkgtaoascjL9EywDgJdeHnXK0eeOvUxHHhXJVkNqcibn8O4RQdpVU60TSA-uiu675ytIjcBHC6kTv8A8pmkj_4oypPd-F92YIJC741swkYQoeIHj8rE-ThcMUkF7KqC5VORbZTRp8HsZSqgiJcIPaouuxd1-8Rxrid3fXkE6p8bkrysPYoxWEJgh7ZFsRCPDWX-yTeJwFN0PKFP1j0F6YtlLfK5wv-c4F8ZQHA_-yc_gODicy7KmWDZgbTP07e7gEWzw4MFRrndjbDQ",
        "priv": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
    }"#;

    // ML-DSA-44 JWS compact serialization from draft-ietf-cose-dilithium-11.
    // Header: {"alg":"ML-DSA-44","kid":"T4xl70S7MT6Zeq6r9V9fPJGVn76wfnXJ21-gyo0Gu6o"}
    const DILITHIUM_ML_DSA_44_COMPACT: &str = "eyJhbGciOiJNTC1EU0EtNDQiLCJraWQiOiJUNHhsNzBTN01UNlplcTZyOVY5ZlBKR1ZuNzZ3Zm5YSjIxLWd5bzBHdTZvIn0.SXTigJlzIGEgZGFuZ2Vyb3VzIGJ1c2luZXNzLCBGcm9kbywgZ29pbmcgb3V0IHlvdXIgZG9vci4.knI1Q_9CIzLH5Xy94Kkc7WVKqZcAgtJ3mNf0GUj1uLA6YXAWFJfXkh-zQxUtEl3UIC7zPCiUwKTDR6ZsuUmFj8Ctb_6aH64hElN7weS_1m5okCy8GqHNL2lsfclCH3Y2f4QNP-DLVS1XsuboDA7Dw3ir2IdYKIfWJyIU7ROHgd24nuun1zJbxcLJC2EKt2M8R0wZudcIE9nm5oPzYXq0z-hPsKoXp9leVYkqgMmO9Lo8SP_1YYIEth3B8v-GuP249KDTFRKPjISmK4aPCknjtjihHsQVv2XePXxKExatHl4qhsiiW-y-EJXa1Kfw4WYpLA7B4_5Ids--cIJmIx7f6xxAWKh5qoBWq1QIOaaFuzsAraRW3NOEuzThew1En85gI3GcRTZGp-VDGyxHm0Al04cyWo2bxAVOF0fbDc265iP2mCNw6Qg10jIJeAhGB4OAMYcBUWJAG0l1MN1U_koEmGh5dXKnQTRl461ea_Cq3DLkcA2Dj2woWUFyDTmQ8oO_yheASfJacyRm7_suj88z5XFNo8F53P8OxTG9xUPlrwvH-TAq7AH3NU4SNXApyVKTU3zhx1tJ34nlTILcTujXVJVo_f0DZfUxr6JSCYqvy4z1Kl0wDQzd55aopyFtQxvOPhcCHbAN34g2Ug750Jm835fl7NOxcqoMbuTcgH68kr37M-Pdh2K9WazXUJgCupgdIWW8WjfOjmTiF59CrVtfVtK2qDzF40OENCfqtNPQlZe5cN5p0P8arj4USB8HCPh7NdqQBAeWrw0wsYhdiM39lrSkA8mLRYMhZnqKGCTPCrHXDdEjRKYRNaqIUT44laYl5c27K0v-ozjKPu6tzEhkYSC4XZ3LehEFtmAzOE0mHbhKMgXqjoPJjOrGIPibX3jwK8_Q5RmMOXtXo8R3vXfBaUdQoLeeyywNYE0nIcsl4z5a8_utwEFiVf0VK2pdviyiOPVSi3zOMAmqz6gFhVy8aMMQOWZAEAuTyDw7ZWG6diwptmrgSXZotW63I19S2ZH7keCXRIq_pFLuYhOuG6dD4MkouILRdC9bXZMLrNDq7COpUOO86aQVlYd0pR935WpUw-V6obSRnHlRFZSmUSIB7h1Q0ImciRzojN93Xhw7qpzGzdzDEO3OOTayXaSG_0YHQyy-eH4hBbmgt_LBx120g1eY4XHeHFRfTfetHkL5ZZusX1jQ_nk9ez4XBG_6hRtTNSuVBsYlH8-KUuR5-qTP8dkvRf8Wk2hHoUr2sz5YO_xDFCMMTrt8ahiMyfjo5ih5Fwo3riFbFUGKibniTLXspFd4spcNK_WchlZLRgkPK4jh6Z_X8JJkHxvQhpyouHQFyGxgBrl24x-_EB1zbWMhJthmm8DiKt-nzKaJz8Cju1-HwCpg76CRqRsEz2hyKEpbb4M5KQSj3AsENCroVmQ5QIv3K2XNRkve4vjBmP6sV2b6GSY_UeRvPElA7SUgBGTKbn-c0aYhBuB8plPhRTBa55_cFqAmNmavF1-fdMktJuIaH2f-K0zZCzbHw54998T7kIWgyMsyGCAvynEB_khOqwT7tCjg5HQ8SIjdnRYW0kjZfjt5LJbGA-PnRo8gPVQVGeYDP2vsSXhNJY94AitKCY1srcSsuYDrhNBKrnoJ1uEsMPVHsgFw_ZHMyAEaVQughSNW4fm8q6_1Nv4zLutDITzmAL6a6i6-WS6QRIs_4VUtwr5cXXIFDDeHVWeGcNivQ6W9urEUP4crguiq7z_DTiYaGfUksub-T7mw0zU8ZoOSd5pUTpJLv-IYIUAl6CscHvunnRLEKqpW1Sa1dcFZs5VP4AfR3mg7wX4Vlq1AHnpFxE2L1LZiKoTc9jDEOvTDkxr86gMkwMm6RdyPF_q48AVJ1br8Qp88-4B84X52zZ5cw-IJYe-HiVJ29LpeYm340_rWivpy-UB5i9TKlMrxf94y1okzZTPbP3_v1_XX0nE7RTLz98EA96euJ7l3EpbEqks7mh6i1FJNnvvlM_u29sYobJ6PUT-i1VlQnF_JBARKEz74pBXm1l5Y5Lo15rsIlaQHinUBCO8fHCHI59LAfKusN4JmodDqLYwkWijEL_sfrC6LtrbXqpM1pw09zSrs_tS1RQ-LnWHuPrU5KLCzv53JKrh8lU_cdBowe_F-Ib_Ui4bQ2FME-0mnyG0XijHUsrGMZ9dfowvIkr83JpqwlFOZAwMmSGPNPEJRw9kDshjotndUB5S1UCfv_U4IoVn7WgvxeCS-BBxqyWfh7YTdf73EnmGwVYxVjlXaHCeeTZmUacnT4MQUAcbFjTq6BBlboAQGWP2FZWpd6HNnruv744VeWmfgLk9z5567wFhwuXMkmE2xvDo4wP80xutjUfsePx5YkLxhY1XsWqTZr19tInxJWWq8RLZsWPmtq5wZ5ucBMasCLpOABenYZdSAcQNhC73wLS0Z2s1HQhBoIl7lr1p372LZs_Seu1u_8Fo7DoJqRpKaNoc2_JUMmn7TUZS8zLyzxgeq8R8iNbRP20DwDBNXocsTDBKaQrtB-QiEPySQtJa4G61XeNZyh5aGzfoWZ9OmjZG9pbbehcqwIrt-ESjPyeT6sfSrvOfTZr7fBXwpUs2rS4BrlNse5g_h8CQiik8aaOTOEPkXiyg4s5DewRlgDZHS-3g-YXPUIBNO62_HxknkMpkJvKW-tkvDbgtxvy4nG80ul6W_KeRsoEKDTRYNKZWxXjZITNa0h6agnwNCJKEbFg3Qhre394c0i60mfP9YIgKTXrCX3Yt2eX-6mPzYmLbSbV5jH69v6WZqYV2WAj-9DU0diR4hOfYQaJnBZhTtKb-SQsYiFuN1BDJ3v9eM9K8hq91NBdCHVa-Thk9Dov-JkcTZnZGRRyW5yXHUV4NOEltBXh8GkjjDvs5Yo3u-2rPCXjK1aGPSI1W8BaUJLQY5sbfAVCAuUHBv-Vlh5Qamt-lgeKguhqTSuy-tjabOb5kiBOG7xGQt3z-XYXtnWFDCii-5h11XfZsQ-xQxy8gSfdMz4hDK9Nw_VQt6fzWiQY0Th_dHzVki0MUfVfsDUjgblhD6j0wgbs3zdj-GM3rtt8oit0wXx11bIOaOKgf07tP0wimVXMRqRWe7LCUAKTE5PkRKU1x_h4iusrzi5uwKDhc4SmRwm6KssNrmCAkiNDZCREVKd3yMnrjA4PAGDzdKWVplcHJ6jKmrsbrEztHd9QAAAAAAAAAAAAAAABIfMEQ";

    #[test]
    fn dilithium_akp_jwk_parse() {
        // draft-ietf-cose-dilithium-11 — ML-DSA-44 AKP JWK.
        // Verifies the newtype-over-Map design handles the AKP key type
        // with its non-standard fields (pub, priv) that a fixed struct would miss.
        let jwk = Jwk::from_json(DILITHIUM_ML_DSA_44_JWK).unwrap();
        assert_eq!(jwk.kty(), Some("AKP"));
        assert_eq!(jwk.alg(), Some("ML-DSA-44"));
        assert_eq!(
            jwk.kid(),
            Some("T4xl70S7MT6Zeq6r9V9fPJGVn76wfnXJ21-gyo0Gu6o")
        );
        // AKP-specific fields accessible via the raw map
        assert!(jwk.0.get("pub").is_some());
        assert!(jwk.0.get("priv").is_some());
        // EC-specific fields should be absent
        assert!(jwk.crv().is_none());
    }

    #[test]
    fn dilithium_jws_compact_decode_header() {
        // draft-ietf-cose-dilithium-11 — parse the ML-DSA-44 JWS compact
        // serialization and verify the header decodes correctly.
        let parts: Vec<&str> = DILITHIUM_ML_DSA_44_COMPACT.splitn(3, '.').collect();
        assert_eq!(parts.len(), 3);

        let header_bytes = b64url_decode(parts[0]).unwrap();
        let header: JoseHeader = serde_json::from_slice(&header_bytes).unwrap();
        assert_eq!(header.alg(), Some("ML-DSA-44"));
        assert_eq!(
            header.kid(),
            Some("T4xl70S7MT6Zeq6r9V9fPJGVn76wfnXJ21-gyo0Gu6o")
        );
    }

    #[test]
    fn dilithium_jws_compact_decode_payload() {
        // draft-ietf-cose-dilithium-11 — the payload is the same Tolkien
        // quote used in RFC 7520 §4 (without the smart quotes variant).
        let parts: Vec<&str> = DILITHIUM_ML_DSA_44_COMPACT.splitn(3, '.').collect();
        let payload = b64url_decode(parts[1]).unwrap();
        let text = String::from_utf8(payload).unwrap();
        assert!(text.starts_with("It\u{2019}s a dangerous business"));
    }
}
