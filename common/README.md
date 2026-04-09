# common

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

Shared CBOR and JSON types used across the `cbor_formats` workspace. This crate provides
the foundational data types that appear in multiple IETF specifications including
[CoRIM](https://datatracker.ietf.org/doc/draft-ietf-rats-corim/),
[COSE (RFC 9052)](https://www.rfc-editor.org/rfc/rfc9052),
[EAT (RFC 9711)](https://www.rfc-editor.org/rfc/rfc9711), and
[CoSWID (RFC 9393)](https://www.rfc-editor.org/rfc/rfc9393).

Each type has a JSON-friendly form and a CBOR-friendly form (with a `Cbor` suffix) connected
by `TryFrom` conversions. CBOR encoding and decoding is handled by
[ciborium](https://crates.io/crates/ciborium).

Key types include:

- **Inline types** — `UeidType`, `UuidType`, `OidType`, `Uri`, `Time`/`TimeCbor`, `TextOrBinary`, `BinaryOrNil`, `TextOrInt`
- **Tagged types** — `TaggedUuidType`, `TaggedUeidType`, `TaggedBytes`, `TaggedSvn`, `TaggedMinSvn`, `TaggedUriType`/`TaggedUriTypeCbor`, and various tagged key and certificate types
- **Array types** — `HashEntry`/`HashEntryCbor`, `MaskedRawValue`/`MaskedRawValueCbor`, `IntRange`/`IntRangeCbor`
- **Choice types** — `VersionScheme`/`VersionSchemeCbor`
- **Tuple types** — `Tuple`/`TupleCbor` for generic key-value pairs, `TupleMap`/`TupleMapCbor` for maps of pairs

```rust
use common::TimeCbor;

// Create a Time value (Unix timestamp) and convert to CBOR form (tag 1)
let t: i64 = 1700000000;
let t_cbor: TimeCbor = t.try_into().unwrap();

// Convert back
let t_roundtrip: i64 = (&t_cbor).try_into().unwrap();
assert_eq!(t, t_roundtrip);
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
