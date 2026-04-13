//! Generate COSE Key CBOR files for each supported asymmetric algorithm.
//!
//! Generates both private and public key files, and also converts the existing
//! JWK test keys from cfcli/tests/data/keys/ into COSE Key CBOR format.
//!
//! Run: cargo run --example gen_cose_keys -p cose_crypto
//!
//! Output goes to cfcli/tests/data/keys/

use std::{fs, path::Path};

use ciborium::{ser::into_writer, value::Value};

use base64ct::{Base64UrlUnpadded, Encoding};
use ed25519_dalek::SigningKey as EdSigningKey;
use elliptic_curve::{Generate, sec1::Coordinates};
use p256::ecdsa::SigningKey as P256SigningKey;
use p384::ecdsa::SigningKey as P384SigningKey;

/// Encode a COSE Key map to CBOR bytes.
fn encode_cose_key(entries: Vec<(Value, Value)>) -> Vec<u8> {
    let map = Value::Map(entries);
    let mut buf = Vec::new();
    into_writer(&map, &mut buf).expect("CBOR encoding failed");
    buf
}

/// Build an EC2 COSE Key (kty=2).
fn ec2_key(crv: i64, x: &[u8], y: &[u8], d: Option<&[u8]>) -> Vec<u8> {
    let mut entries = vec![
        (Value::Integer(1.into()), Value::Integer(2.into())), // kty: EC2
        (Value::Integer((-1i64).into()), Value::Integer(crv.into())), // crv
        (Value::Integer((-2i64).into()), Value::Bytes(x.to_vec())), // x
        (Value::Integer((-3i64).into()), Value::Bytes(y.to_vec())), // y
    ];
    if let Some(d) = d {
        entries.push((Value::Integer((-4i64).into()), Value::Bytes(d.to_vec())));
        // d
    }
    encode_cose_key(entries)
}

/// Build an OKP COSE Key (kty=1).
fn okp_key(crv: i64, x: &[u8], d: Option<&[u8]>) -> Vec<u8> {
    let mut entries = vec![
        (Value::Integer(1.into()), Value::Integer(1.into())), // kty: OKP
        (Value::Integer((-1i64).into()), Value::Integer(crv.into())), // crv
        (Value::Integer((-2i64).into()), Value::Bytes(x.to_vec())), // x
    ];
    if let Some(d) = d {
        entries.push((Value::Integer((-4i64).into()), Value::Bytes(d.to_vec())));
        // d
    }
    encode_cose_key(entries)
}

fn write_key(dir: &Path, name: &str, data: &[u8]) {
    let path = dir.join(name);
    fs::write(&path, data).unwrap_or_else(|e| panic!("Failed to write {}: {}", path.display(), e));
    println!("  wrote {} ({} bytes)", path.display(), data.len());
}

fn main() {
    let out_dir = Path::new("cfcli/tests/data/keys");
    if !out_dir.exists() {
        eprintln!(
            "Output directory {} does not exist — run from workspace root",
            out_dir.display()
        );
        std::process::exit(1);
    }

    println!("=== Generating fresh COSE Key files ===");

    // ES256 (P-256)
    let sk256 = P256SigningKey::generate();
    let vk256 = sk256.verifying_key();
    let pt256 = vk256.to_sec1_point(false);
    let (x256, y256) = match pt256.coordinates() {
        Coordinates::Uncompressed { x, y } => (x.to_vec(), y.to_vec()),
        _ => panic!("unexpected point format"),
    };
    let d256 = sk256.to_bytes().to_vec();

    write_key(
        out_dir,
        "es256.cosekey",
        &ec2_key(1, &x256, &y256, Some(&d256)),
    );
    write_key(
        out_dir,
        "es256-pub.cosekey",
        &ec2_key(1, &x256, &y256, None),
    );

    // ES384 (P-384)
    let sk384 = P384SigningKey::generate();
    let vk384 = sk384.verifying_key();
    let pt384 = vk384.to_sec1_point(false);
    let (x384, y384) = match pt384.coordinates() {
        Coordinates::Uncompressed { x, y } => (x.to_vec(), y.to_vec()),
        _ => panic!("unexpected point format"),
    };
    let d384 = sk384.to_bytes().to_vec();

    write_key(
        out_dir,
        "es384.cosekey",
        &ec2_key(2, &x384, &y384, Some(&d384)),
    );
    write_key(
        out_dir,
        "es384-pub.cosekey",
        &ec2_key(2, &x384, &y384, None),
    );

    // EdDSA (Ed25519)
    let mut rng = rand::rng();
    let sk_ed = EdSigningKey::generate(&mut rng);
    let vk_ed = sk_ed.verifying_key();

    write_key(
        out_dir,
        "ed25519.cosekey",
        &okp_key(6, vk_ed.as_bytes(), Some(&sk_ed.to_bytes())),
    );
    write_key(
        out_dir,
        "ed25519-pub.cosekey",
        &okp_key(6, vk_ed.as_bytes(), None),
    );

    // === Convert existing JWK keys to COSE Key ===
    println!("\n=== Converting existing JWK keys to COSE Key ===");

    convert_jwk_ec(out_dir, "es256.jwk", "es256-from-jwk.cosekey", 1);
    convert_jwk_ec(out_dir, "es384.jwk", "es384-from-jwk.cosekey", 2);
    convert_jwk_okp(out_dir, "ed25519.jwk", "ed25519-from-jwk.cosekey", 6);

    // === cose-wg test vector keys ===
    println!("\n=== Writing cose-wg example keys ===");

    // P-256 key from cose-wg/Examples ecdsa-sig-01
    // base64url: x=usWxHK2PmfnHKwXPS54m0kTcGJ90UiglWiGahtagnv8
    //            y=IBOL-C3BttVivg-lSreASjpkttcsz-1rb7btKLv8EX4
    //            d=V8kgd2ZBRuh2dgyVINBUqpPDr7BOMGcF22CQMIUHtNM
    let cwg_p256_x =
        hex_literal::hex!("bac5b11cad8f99f9c72b05cf4b9e26d244dc189f745228255a219a86d6a09eff");
    let cwg_p256_y =
        hex_literal::hex!("20138bf82dc1b6d562be0fa54ab7804a3a64b6d72ccfed6b6fb6ed28bbfc117e");
    let cwg_p256_d =
        hex_literal::hex!("57c92077664146e876760c9520d054aa93c3afb04e306705db6090308507b4d3");
    write_key(
        out_dir,
        "cose-wg-p256.cosekey",
        &ec2_key(1, &cwg_p256_x, &cwg_p256_y, Some(&cwg_p256_d)),
    );
    write_key(
        out_dir,
        "cose-wg-p256-pub.cosekey",
        &ec2_key(1, &cwg_p256_x, &cwg_p256_y, None),
    );

    // Ed25519 key from cose-wg/Examples eddsa-sig-01
    let cwg_ed_x =
        hex_literal::hex!("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");
    let cwg_ed_d =
        hex_literal::hex!("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
    write_key(
        out_dir,
        "cose-wg-ed25519.cosekey",
        &okp_key(6, &cwg_ed_x, Some(&cwg_ed_d)),
    );
    write_key(
        out_dir,
        "cose-wg-ed25519-pub.cosekey",
        &okp_key(6, &cwg_ed_x, None),
    );

    println!("\nDone.");
}

/// Convert a JWK EC key file to COSE Key CBOR.
fn convert_jwk_ec(dir: &Path, jwk_name: &str, cose_name: &str, crv: i64) {
    let jwk_path = dir.join(jwk_name);
    let jwk_bytes = fs::read(&jwk_path).unwrap_or_else(|e| {
        panic!("Failed to read {}: {}", jwk_path.display(), e);
    });

    #[derive(serde::Deserialize)]
    struct EcJwk {
        x: String,
        y: String,
        d: Option<String>,
    }

    let jwk: EcJwk = serde_json::from_slice(&jwk_bytes).expect("JWK parse failed");
    let x = Base64UrlUnpadded::decode_vec(&jwk.x).expect("x decode");
    let y = Base64UrlUnpadded::decode_vec(&jwk.y).expect("y decode");
    let d = jwk
        .d
        .as_ref()
        .map(|d| Base64UrlUnpadded::decode_vec(d).expect("d decode"));

    write_key(dir, cose_name, &ec2_key(crv, &x, &y, d.as_deref()));
}

/// Convert a JWK OKP key file to COSE Key CBOR.
fn convert_jwk_okp(dir: &Path, jwk_name: &str, cose_name: &str, crv: i64) {
    let jwk_path = dir.join(jwk_name);
    let jwk_bytes = fs::read(&jwk_path).unwrap_or_else(|e| {
        panic!("Failed to read {}: {}", jwk_path.display(), e);
    });

    #[derive(serde::Deserialize)]
    struct OkpJwk {
        x: String,
        d: Option<String>,
    }

    let jwk: OkpJwk = serde_json::from_slice(&jwk_bytes).expect("JWK parse failed");
    let x = Base64UrlUnpadded::decode_vec(&jwk.x).expect("x decode");
    let d = jwk
        .d
        .as_ref()
        .map(|d| Base64UrlUnpadded::decode_vec(d).expect("d decode"));

    write_key(dir, cose_name, &okp_key(crv, &x, d.as_deref()));
}
