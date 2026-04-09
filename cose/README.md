# CBOR Object Signing and Encryption (COSE)

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR and JSON encoders and decoders for CBOR Object Signing and Encryption (COSE)
data structures as defined in [RFC 9052](https://www.rfc-editor.org/rfc/rfc9052).

This crate handles the structural representation of COSE messages — it does not
perform cryptographic operations. For signing, verification, MAC, and AEAD see the
companion [`cose_crypto`](../cose_crypto/index.html) crate.

Supported structures:

- **Signing** — `CoseSign1`/`CoseSign1Cbor`, `CoseSign`/`CoseSignCbor`, `CoseSignature`/`CoseSignatureCbor`
- **Encryption** — `CoseEncrypt0`/`CoseEncrypt0Cbor`, `CoseEncrypt`/`CoseEncryptCbor`
- **MAC** — `CoseMac0`/`CoseMac0Cbor`, `CoseMac`/`CoseMacCbor`
- **Headers** — `HeaderMap`/`HeaderMapCbor`
- **Keys** — `CoseKey`/`CoseKeyCbor`, `CoseKeySet`

```rust,no_run
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use cose::arrays::CoseSign1Cbor;

// Decode a CBOR-encoded COSE_Sign1 message
let cbor_bytes: &[u8] = &[/* ... */];
let sign1: CoseSign1Cbor = from_reader(cbor_bytes).unwrap();

// Re-encode
let mut buf = vec![];
into_writer(&sign1, &mut buf).unwrap();
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
