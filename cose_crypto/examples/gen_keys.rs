//! Generate sample JWK key files for each supported algorithm.
//!
//! Run: cargo run --example gen_keys -p cose_crypto

use base64ct::{Base64UrlUnpadded, Encoding};
use ed25519_dalek::SigningKey as EdSigningKey;
use elliptic_curve::Generate;
use elliptic_curve::sec1::Coordinates;
use p256::ecdsa::SigningKey as P256SigningKey;
use p384::ecdsa::SigningKey as P384SigningKey;

fn main() {
    // ES256 (P-256)
    let sk256 = P256SigningKey::generate();
    let vk256 = sk256.verifying_key();
    let pt256 = vk256.to_sec1_point(false);
    let (x256_bytes, y256_bytes) = match pt256.coordinates() {
        Coordinates::Uncompressed { x, y } => (x, y),
        _ => panic!("unexpected point format"),
    };
    let d256 = Base64UrlUnpadded::encode_string(&sk256.to_bytes());
    let x256 = Base64UrlUnpadded::encode_string(x256_bytes);
    let y256 = Base64UrlUnpadded::encode_string(y256_bytes);

    // ES384 (P-384)
    let sk384 = P384SigningKey::generate();
    let vk384 = sk384.verifying_key();
    let pt384 = vk384.to_sec1_point(false);
    let (x384_bytes, y384_bytes) = match pt384.coordinates() {
        Coordinates::Uncompressed { x, y } => (x, y),
        _ => panic!("unexpected point format"),
    };
    let d384 = Base64UrlUnpadded::encode_string(&sk384.to_bytes());
    let x384 = Base64UrlUnpadded::encode_string(x384_bytes);
    let y384 = Base64UrlUnpadded::encode_string(y384_bytes);

    // EdDSA (Ed25519)
    let mut rng = rand::rng();
    let sk_ed = EdSigningKey::generate(&mut rng);
    let vk_ed = sk_ed.verifying_key();
    let d_ed = Base64UrlUnpadded::encode_string(&sk_ed.to_bytes());
    let x_ed = Base64UrlUnpadded::encode_string(vk_ed.as_bytes());

    println!("ES256_D={d256}");
    println!("ES256_X={x256}");
    println!("ES256_Y={y256}");
    println!("ES384_D={d384}");
    println!("ES384_X={x384}");
    println!("ES384_Y={y384}");
    println!("EDDSA_D={d_ed}");
    println!("EDDSA_X={x_ed}");
}
