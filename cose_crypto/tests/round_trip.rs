use p256::elliptic_curve::Generate;

use cose_crypto::{
    algorithm::CoseAlgorithm,
    crypto::CoseAead,
    crypto::aes_gcm::AesGcmKey,
    crypto::ecdsa::{Es256Signer, Es256Verifier, Es384Signer, Es384Verifier},
    crypto::eddsa::{Ed25519Signer, Ed25519Verifier},
    crypto::hmac::{HmacSha256Key, HmacSha384Key, HmacSha512Key},
    encrypt::{CoseEncrypt0Builder, decrypt_encrypt0},
    helpers,
    mac::{CoseMac0Builder, verify_mac0},
    sign::{CoseSign1Builder, verify_sign1},
};

// ── ES256 Sign1 round-trip ──

#[test]
fn sign1_es256_round_trip() {
    // Generate a P-256 key pair
    let signing_key = p256::ecdsa::SigningKey::generate();
    let d = signing_key.to_bytes();
    let verifying_key = signing_key.verifying_key();
    let point = verifying_key.to_sec1_point(false);
    let x = point.x().unwrap().to_vec();
    let y = point.y().unwrap().to_vec();

    let signer = Es256Signer::from_bytes(&d).unwrap();
    let verifier = Es256Verifier::from_xy(&x, &y).unwrap();

    let payload = b"This is the content.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es256);

    let msg = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(&signer)
        .unwrap();

    verify_sign1(&msg, &verifier, &[]).unwrap();
}

// ── ES384 Sign1 round-trip ──

#[test]
fn sign1_es384_round_trip() {
    let signing_key = p384::ecdsa::SigningKey::generate();
    let d = signing_key.to_bytes();
    let verifying_key = signing_key.verifying_key();
    let point = verifying_key.to_sec1_point(false);
    let x = point.x().unwrap().to_vec();
    let y = point.y().unwrap().to_vec();

    let signer = Es384Signer::from_bytes(&d).unwrap();
    let verifier = Es384Verifier::from_xy(&x, &y).unwrap();

    let payload = b"This is the content.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es384);

    let msg = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(&signer)
        .unwrap();

    verify_sign1(&msg, &verifier, &[]).unwrap();
}

// ── EdDSA Sign1 round-trip ──

#[test]
fn sign1_eddsa_round_trip() {
    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rng());
    let d = signing_key.to_bytes();
    let x = signing_key.verifying_key().to_bytes();

    let signer = Ed25519Signer::from_bytes(&d).unwrap();
    let verifier = Ed25519Verifier::from_bytes(&x).unwrap();

    let payload = b"This is the content.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Eddsa);

    let msg = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(&signer)
        .unwrap();

    verify_sign1(&msg, &verifier, &[]).unwrap();
}

// ── Verification with wrong key fails ──

#[test]
fn sign1_es256_wrong_key_fails() {
    let signing_key = p256::ecdsa::SigningKey::generate();
    let d = signing_key.to_bytes();

    // Different key pair for verification
    let wrong_key = p256::ecdsa::SigningKey::generate();
    let wrong_vk = wrong_key.verifying_key();
    let point = wrong_vk.to_sec1_point(false);
    let x = point.x().unwrap().to_vec();
    let y = point.y().unwrap().to_vec();

    let signer = Es256Signer::from_bytes(&d).unwrap();
    let verifier = Es256Verifier::from_xy(&x, &y).unwrap();

    let payload = b"This is the content.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es256);

    let msg = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(&signer)
        .unwrap();

    assert!(verify_sign1(&msg, &verifier, &[]).is_err());
}

// ── HMAC MAC0 round-trip ──

#[test]
fn mac0_hmac_sha256_round_trip() {
    let key_bytes = [0x42u8; 32];
    let mac_key = HmacSha256Key::from_bytes(&key_bytes, CoseAlgorithm::Hs256).unwrap();

    let payload = b"MAC this data.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Hs256);

    let msg = CoseMac0Builder::new()
        .payload(payload)
        .protected(protected)
        .tag(&mac_key)
        .unwrap();

    verify_mac0(&msg, &mac_key, &[]).unwrap();
}

#[test]
fn mac0_hmac_sha384_round_trip() {
    let key_bytes = [0x43u8; 48];
    let mac_key = HmacSha384Key::from_bytes(&key_bytes).unwrap();

    let payload = b"MAC this data.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Hs384);

    let msg = CoseMac0Builder::new()
        .payload(payload)
        .protected(protected)
        .tag(&mac_key)
        .unwrap();

    verify_mac0(&msg, &mac_key, &[]).unwrap();
}

#[test]
fn mac0_hmac_sha512_round_trip() {
    let key_bytes = [0x44u8; 64];
    let mac_key = HmacSha512Key::from_bytes(&key_bytes).unwrap();

    let payload = b"MAC this data.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Hs512);

    let msg = CoseMac0Builder::new()
        .payload(payload)
        .protected(protected)
        .tag(&mac_key)
        .unwrap();

    verify_mac0(&msg, &mac_key, &[]).unwrap();
}

#[test]
fn mac0_hmac_wrong_key_fails() {
    let key_bytes = [0x42u8; 32];
    let mac_key = HmacSha256Key::from_bytes(&key_bytes, CoseAlgorithm::Hs256).unwrap();

    let payload = b"MAC this data.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Hs256);

    let msg = CoseMac0Builder::new()
        .payload(payload)
        .protected(protected)
        .tag(&mac_key)
        .unwrap();

    let wrong_key = HmacSha256Key::from_bytes(&[0x99u8; 32], CoseAlgorithm::Hs256).unwrap();
    assert!(verify_mac0(&msg, &wrong_key, &[]).is_err());
}

// ── AES-GCM Encrypt0 round-trip ──

#[test]
fn encrypt0_aes128gcm_round_trip() {
    let key_bytes = [0x55u8; 16];
    let aead = AesGcmKey::from_bytes(&key_bytes).unwrap();

    let plaintext = b"Encrypt this data.";
    let iv = [0x01u8; 12];

    let protected = helpers::header_with_algorithm(CoseAlgorithm::A128Gcm);
    let mut unprotected = helpers::empty_header();
    unprotected.iv = Some(iv.to_vec());

    let msg = CoseEncrypt0Builder::new()
        .plaintext(plaintext)
        .protected(protected)
        .unprotected(unprotected)
        .encrypt(&aead)
        .unwrap();

    let decrypted = decrypt_encrypt0(&msg, &aead, &[]).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypt0_aes256gcm_round_trip() {
    let key_bytes = [0x66u8; 32];
    let aead = AesGcmKey::from_bytes(&key_bytes).unwrap();

    let plaintext = b"Encrypt this data with 256-bit key.";
    let iv = [0x02u8; 12];

    let protected = helpers::header_with_algorithm(CoseAlgorithm::A256Gcm);
    let mut unprotected = helpers::empty_header();
    unprotected.iv = Some(iv.to_vec());

    let msg = CoseEncrypt0Builder::new()
        .plaintext(plaintext)
        .protected(protected)
        .unprotected(unprotected)
        .encrypt(&aead)
        .unwrap();

    let decrypted = decrypt_encrypt0(&msg, &aead, &[]).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypt0_wrong_key_fails() {
    let key_bytes = [0x55u8; 16];
    let aead = AesGcmKey::from_bytes(&key_bytes).unwrap();

    let plaintext = b"Encrypt this data.";
    let iv = [0x01u8; 12];

    let protected = helpers::header_with_algorithm(CoseAlgorithm::A128Gcm);
    let mut unprotected = helpers::empty_header();
    unprotected.iv = Some(iv.to_vec());

    let msg = CoseEncrypt0Builder::new()
        .plaintext(plaintext)
        .protected(protected)
        .unprotected(unprotected)
        .encrypt(&aead)
        .unwrap();

    let wrong_aead = AesGcmKey::from_bytes(&[0x99u8; 16]).unwrap();
    assert!(decrypt_encrypt0(&msg, &wrong_aead, &[]).is_err());
}

// ── AES-GCM invalid nonce length ──

#[test]
fn encrypt0_wrong_nonce_length_returns_error() {
    let key_bytes = [0x55u8; 16];
    let aead = AesGcmKey::from_bytes(&key_bytes).unwrap();

    // Too short
    assert!(aead.encrypt(&[0u8; 8], &[], b"data").is_err());
    // Too long
    assert!(aead.encrypt(&[0u8; 16], &[], b"data").is_err());
    // Empty
    assert!(aead.encrypt(&[], &[], b"data").is_err());
    // Correct length works
    assert!(aead.encrypt(&[0u8; 12], &[], b"data").is_ok());
}

// ── Sign1 with external AAD ──

#[test]
fn sign1_with_external_aad() {
    let signing_key = p256::ecdsa::SigningKey::generate();
    let d = signing_key.to_bytes();
    let verifying_key = signing_key.verifying_key();
    let point = verifying_key.to_sec1_point(false);
    let x = point.x().unwrap().to_vec();
    let y = point.y().unwrap().to_vec();

    let signer = Es256Signer::from_bytes(&d).unwrap();
    let verifier = Es256Verifier::from_xy(&x, &y).unwrap();

    let payload = b"This is the content.";
    let external_aad = b"additional context";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es256);

    let msg = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .external_aad(external_aad)
        .sign(&signer)
        .unwrap();

    // Correct AAD
    verify_sign1(&msg, &verifier, external_aad).unwrap();

    // Wrong AAD should fail
    assert!(verify_sign1(&msg, &verifier, b"wrong aad").is_err());
}

// ── CBOR serialization round-trip ──

#[test]
fn sign1_cbor_serialization_round_trip() {
    let signing_key = p256::ecdsa::SigningKey::generate();
    let d = signing_key.to_bytes();
    let verifying_key = signing_key.verifying_key();
    let point = verifying_key.to_sec1_point(false);
    let x = point.x().unwrap().to_vec();
    let y = point.y().unwrap().to_vec();

    let signer = Es256Signer::from_bytes(&d).unwrap();
    let verifier = Es256Verifier::from_xy(&x, &y).unwrap();

    let payload = b"This is the content.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es256);

    let msg = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(&signer)
        .unwrap();

    // Serialize to CBOR
    let mut cbor_bytes = Vec::new();
    ciborium::ser::into_writer(&msg, &mut cbor_bytes).unwrap();

    // Deserialize from CBOR
    let msg2: cose::arrays::CoseSign1Cbor =
        ciborium::de::from_reader(cbor_bytes.as_slice()).unwrap();

    // Verify the deserialized message
    verify_sign1(&msg2, &verifier, &[]).unwrap();
}
