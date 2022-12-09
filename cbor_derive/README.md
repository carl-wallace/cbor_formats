# cbor_derive

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

The cbor_derive crate provided procedural macros that can be used to derive support for marshaling data 
between structures and map and array structures used by the [ciborium](https://crates.io/crates/ciborium) library. 
The `StructToMap` macro maps the fields of a structure onto a `Vec<(Value,Value)>`.
The `StructToArray` macro maps the fields of a structure onto a `Vec<Value>`. 

The mappings are relative to auto-generated structures that are named by appending `Cbor` to the name of 
the structure. This allows original structure definitions to be used with [serde-json](https://crates.io/crates/serde_json)
for JSON-encodings or with ciborium for maps that feature text-based indices with the alternative structures used with 
ciborium for maps with integer-based indices. The generated code includes TryFrom support to move between the defined 
structures and the generated related structures.

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

## Minimum Supported Rust Version

This crate requires **Rust 1.56** at a minimum.

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
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[RFC 5280]: https://datatracker.ietf.org/doc/html/rfc5280
[RFC 5937]: https://datatracker.ietf.org/doc/html/rfc5937
