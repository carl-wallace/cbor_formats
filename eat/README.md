# Entity Attestation Token (EAT)

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR and JSON encoders and decoders for Entity Attestation Token (EAT) structures
as defined in [RFC 9711](https://www.rfc-editor.org/rfc/rfc9711). EAT provides a
standardized token format for conveying claims about an attesting entity, such as
hardware identity, software measurements, and security state.

The top-level type is `ClaimsSetClaims`/`ClaimsSetClaimsCbor`. Additional types cover
measurements, submodules, hardware and software versions, location, debug status,
DLoA (Device Location of Attestation), and both CBOR-specific and JSON-specific
selector types.

```rust,no_run
use ciborium::de::from_reader;
use eat::maps::{ClaimsSetClaimsCbor, ClaimsSetClaims};

// Decode a CBOR-encoded EAT
let cbor_bytes: &[u8] = &[/* ... */];
let eat_cbor: ClaimsSetClaimsCbor = from_reader(cbor_bytes).unwrap();

// Convert to JSON-friendly form
let eat_json: ClaimsSetClaims = eat_cbor.try_into().unwrap();
let json = serde_json::to_string_pretty(&eat_json).unwrap();
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
