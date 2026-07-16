# JOSE — JSON Object Signing and Encryption

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

JWS (JSON Web Signature, [RFC 7515](https://www.rfc-editor.org/rfc/rfc7515)) signing,
verification, and serialization built on top of `cose_crypto` signer/verifier
implementations. Also provides JWK ([RFC 7517](https://www.rfc-editor.org/rfc/rfc7517))
and JOSE Header structures.

Supported JWS serializations: Compact, JSON General, and JSON Flattened (including
detached payloads).

```rust,no_run
use jose::{JwsBuilder, JoseHeader, verify_compact};

// Build a compact JWS
let header = JoseHeader::new(serde_json::json!({"alg": "ES256"}));
let jws = JwsBuilder::new()
    .payload(b"hello")
    .build_compact(&signer, &header)
    .unwrap();

// Verify
let payload = verify_compact(&jws, &verifier).unwrap();
```

## Not in scope

JWE (RFC 7516), JWT claims validation, SD-JWT.

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

## Minimum Supported Rust Version

This crate requires **Rust 1.85** at a minimum.

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg
