# cose_crypto

Cryptographic operations for COSE (RFC 9052) message types, built on the `cose` crate's data structures.

## Supported Algorithms

- **Signing:** ES256 (P-256), ES384 (P-384), EdDSA (Ed25519)
- **MAC:** HMAC-SHA-256, HMAC-SHA-256/64, HMAC-SHA-384, HMAC-SHA-512
- **AEAD:** AES-128-GCM, AES-256-GCM

## Minimum Supported Rust Version (MSRV)

This crate requires **Rust 1.85.0** or later.
