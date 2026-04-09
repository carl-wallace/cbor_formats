# webauthn_asf

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR encoders and decoders for [Web Authentication Level 2](https://www.w3.org/TR/webauthn-2/)
attestation and assertion object formats. These structures are used in the WebAuthn
registration and authentication ceremonies to convey authenticator attestation data.

Key types include `AttestationObject` (standard WebAuthn attestation),
`AppleAttestationObject` (Apple App Attest format), `AppleAssertionObject`,
and the `Supported` enum for identifying which attestation statement format
was decoded. These types use serde directly for both CBOR and JSON serialization.

```rust,no_run
use ciborium::de::from_reader;
use webauthn_asf::AttestationObject;

// Decode a CBOR-encoded WebAuthn attestation object
let cbor_bytes: &[u8] = &[/* attestation object bytes */];
let att: AttestationObject = from_reader(cbor_bytes).unwrap();

// Access fields
println!("Format: {}", att.fmt);
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
