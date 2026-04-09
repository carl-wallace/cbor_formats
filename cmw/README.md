# cmw

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR and JSON encoders and decoders for the RATS Conceptual Message Wrapper (CMW)
as defined in [draft-ietf-rats-msg-wrap](https://datatracker.ietf.org/doc/draft-ietf-rats-msg-wrap/).
CMW provides a uniform envelope for wrapping attestation evidence, attestation
results, and other RATS conceptual messages, with support for CBOR tags, CBOR
record arrays, JSON collections, and signed wrappers.

Key types include [`CborCmw`](choices::CborCmw) (CBOR-encoded CMW with variants for
tagged, record, and collection forms), `CborCollection`/`CborCollectionCbor`,
`JsonCollection`, and `SignedCborCmw` for validating COSE Sign1-wrapped CMW objects.
Application-defined CBOR tags in the CMW range (per the RFC 9277 TN() transform) are
captured by the [`CborCmw::TagData`](choices::CborCmw::TagData) variant, which carries
the tag number and opaque payload bytes.

```rust,no_run
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use cmw::arrays::{CborRecord, CborRecordCbor};

// Decode a CBOR-encoded CMW record
let cbor_bytes: &[u8] = &[/* your CBOR bytes here */];
let record_cbor: CborRecordCbor = from_reader(cbor_bytes).unwrap();

// Convert to JSON-friendly form
let record_json: CborRecord = record_cbor.clone().try_into().unwrap();

// Convert back and re-encode
let roundtrip: CborRecordCbor = record_json.try_into().unwrap();
let mut buf = vec![];
into_writer(&roundtrip, &mut buf).unwrap();
```

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

## Crate Feature Flags

- `crypto` enables `SignedCborCmwBuilder` for creating signed CBOR CMW objects using COSE Sign1 via the `cose_crypto` crate (ES256, ES384, EdDSA). This feature is not enabled by default.

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

[draft-ietf-rats-msg-wrap-23]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23
