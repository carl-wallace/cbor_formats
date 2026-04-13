# cose_crypto

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

Cryptographic operations for COSE ([RFC 9052](https://www.rfc-editor.org/rfc/rfc9052))
message types, built on the [`cose`](../cose/index.html) crate's data structures.
This crate provides builders for constructing and verifying COSE Sign1, MAC0, and
Encrypt0 messages, as well as JWK key parsing for interoperability.

```rust,no_run
use cose_crypto::algorithm::CoseAlgorithm;
use cose_crypto::crypto::ecdsa::{Es256Signer, Es256Verifier};
use cose_crypto::helpers::header_with_algorithm;
use cose_crypto::sign::{CoseSign1Builder, verify_sign1};

// Generate or load a P-256 key pair
// let signer = Es256Signer::from_bytes(&private_key_bytes).unwrap();
// let verifier = Es256Verifier::from_xy(&x, &y).unwrap();

// Sign a payload
// let sign1 = CoseSign1Builder::new()
//     .payload(b"Hello, COSE!")
//     .protected(header_with_algorithm(CoseAlgorithm::Es256))
//     .sign(&signer)
//     .unwrap();

// Verify
// verify_sign1(&sign1, &verifier, &[]).unwrap();
```

Key parsing is available via the `jwk` module (JSON Web Key files) and the `keys` module
(COSE Key CBOR), supporting EC P-256, P-384, OKP Ed25519, and ML-DSA (with the `pqc` feature).

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

## Supported Algorithms

- **Signing:** ES256 (P-256), ES384 (P-384), EdDSA (Ed25519)
- **Signing (PQC):** ML-DSA-44, ML-DSA-65, ML-DSA-87 (requires `pqc` feature)
- **MAC:** HMAC-SHA-256, HMAC-SHA-256/64, HMAC-SHA-384, HMAC-SHA-512
- **AEAD:** AES-128-GCM, AES-256-GCM

## Crate Feature Flags

- `pqc` enables ML-DSA-44, ML-DSA-65, and ML-DSA-87 post-quantum signing and verification per [draft-ietf-cose-dilithium-11](https://datatracker.ietf.org/doc/html/draft-ietf-cose-dilithium-11), using the `ml-dsa` crate. This feature is not enabled by default.

## Examples

### Key Generation

The `gen_keys` example generates sample JWK key material (ES256, ES384, EdDSA):

```sh
cargo run --example gen_keys -p cose_crypto
```

The `gen_cose_keys` example generates COSE Key CBOR files and converts existing JWK keys:

```sh
cargo run --example gen_cose_keys -p cose_crypto
```

Pre-generated key files in both formats are in `cfcli/tests/data/keys/` (`.jwk` and `.cosekey`).

## Minimum Supported Rust Version

This crate requires **Rust 1.85** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a minor
version bump.

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg
