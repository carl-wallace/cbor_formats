# cots

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR and JSON encoders and decoders for Concise Trust Anchor Store (CoTS)
structures as defined in
[draft-ietf-rats-concise-ta-stores-02](https://datatracker.ietf.org/doc/html/draft-ietf-rats-concise-ta-stores-02).
CoTS provides a compact format for distributing and managing trust anchors used in
remote attestation.

Key types include `ConciseTaStores`/`ConciseTaStoresCbor` (the top-level container),
`ConciseTaStoreMap`/`ConciseTaStoreMapCbor`, `CasAndTasMap`/`CasAndTasMapCbor`,
and `EnvironmentGroupListMap`/`EnvironmentGroupListMapCbor`.

```rust,no_run
use ciborium::de::from_reader;
use cots::arrays::{ConciseTaStoresCbor, ConciseTaStores};

// Decode a CBOR-encoded CoTS
let cbor_bytes: &[u8] = &[/* ... */];
let cots_cbor: ConciseTaStoresCbor = from_reader(cbor_bytes).unwrap();

// Convert to JSON-friendly form
let cots_json: ConciseTaStores = cots_cbor.try_into().unwrap();
let json = serde_json::to_string_pretty(&cots_json).unwrap();
```

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

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
