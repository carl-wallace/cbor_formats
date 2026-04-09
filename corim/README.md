# corim

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR and JSON encoders and decoders for Concise Reference Integrity Manifest (CoRIM)
structures as defined in [draft-ietf-rats-corim](https://datatracker.ietf.org/doc/draft-ietf-rats-corim/).
CoRIM provides a framework for conveying reference values, endorsed values, and
other attestation-related information.

Key types include `CorimMap`/`CorimMapCbor` (the top-level manifest),
`ConciseMidTag`/`ConciseMidTagCbor` (CoMID tags), `ClassMap`, `EnvironmentMap`,
`MeasurementMap`, `EntityMap`, and various triple record types for reference values,
endorsed values, and attestation keys. The `signed` module provides `SignedCorim` for
validating COSE Sign1-wrapped CoRIM objects.

```rust,no_run
use ciborium::de::from_reader;
use corim::maps::{CorimMapCbor, CorimMap};

// Decode a CBOR-encoded CoRIM
let cbor_bytes: &[u8] = &[/* ... */];
let corim_cbor: CorimMapCbor = from_reader(cbor_bytes).unwrap();

// Convert to JSON-friendly form
let corim_json: CorimMap = corim_cbor.try_into().unwrap();
let json = serde_json::to_string_pretty(&corim_json).unwrap();
```

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

## Crate Feature Flags

- `crypto` enables `SignedCorimBuilder` for creating signed CoRIM objects using COSE Sign1 via the `cose_crypto` crate (ES256, ES384, EdDSA). This feature is not enabled by default.

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

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[RFC 5280]: https://datatracker.ietf.org/doc/html/rfc5280
[RFC 5937]: https://datatracker.ietf.org/doc/html/rfc5937
