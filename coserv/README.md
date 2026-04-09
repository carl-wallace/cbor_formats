# coserv

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR and JSON encoders and decoders for the Concise Service (CoSERV) protocol
as defined in [draft-ietf-rats-coserv](https://datatracker.ietf.org/doc/draft-ietf-rats-coserv/).
CoSERV defines a request/response protocol for querying and retrieving attestation
reference values, endorsed values, trust anchors, and attestation keys from a
verification service.

Key types include `CoservMap`/`CoservMapCbor` (the top-level message), `QueryMap`,
`EnvironmentSelectorMap`, `ResultsMap`, and quad types (`RefvalQuadMap`, `EndvalQuadMap`,
`CondEndvalQuadMap`, `AkQuadMap`) for associating authorities with attestation data.
The `arrays` module provides `StatefulClass`, `StatefulInstance`, and `StatefulGroup`
for environment selectors with optional measurements. The `signed` module provides
`SignedCoserv` for validating COSE Sign1-wrapped CoSERV objects.

```rust,no_run
use ciborium::de::from_reader;
use coserv::maps::{CoservMapCbor, CoservMap};

// Decode a CBOR-encoded CoSERV message
let cbor_bytes: &[u8] = &[/* ... */];
let coserv_cbor: CoservMapCbor = from_reader(cbor_bytes).unwrap();

// Convert to JSON-friendly form
let coserv_json: CoservMap = coserv_cbor.try_into().unwrap();
let json = serde_json::to_string_pretty(&coserv_json).unwrap();
```

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

## Crate Feature Flags

- `crypto` enables `SignedCoservBuilder` for creating signed CoSERV objects using COSE Sign1 via the `cose_crypto` crate (ES256, ES384, EdDSA). This feature is not enabled by default.

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

[draft-ietf-rats-coserv-05]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05
