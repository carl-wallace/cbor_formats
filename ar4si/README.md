# ar4si

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR-focused encoders/decoders for Attestation Results for Secure Interactions (AR4SI)
structures as defined in [draft-ietf-rats-ar4si-09](https://datatracker.ietf.org/doc/html/draft-ietf-rats-ar4si-09).

AR4SI defines a `TrustworthinessVector` containing per-category trustworthiness claims
(instance identity, configuration, executables, hardware, etc.) and a `VerifierId`
identifying the entity that produced the attestation result.

```rust,no_run
use ciborium::de::from_reader;
use ar4si::maps::TrustworthinessVectorCbor;

let cbor_bytes: &[u8] = &[/* ... */];
let tv: TrustworthinessVectorCbor = from_reader(cbor_bytes).unwrap();
```

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

## Minimum Supported Rust Version (MSRV)

This crate requires **Rust 1.85.0** or later.

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg
