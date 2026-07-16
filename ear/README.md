# ear

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR-focused encoders/decoders for EAT Attestation Result (EAR) structures
as defined in [draft-ietf-rats-ear-03](https://datatracker.ietf.org/doc/html/draft-ietf-rats-ear-03).

EAR extends the EAT Claims-Set with appraisal results produced by a verifier.
The top-level type is `Ear`/`EarCbor`, which contains a profile identifier,
an `iat` timestamp, and a submods map of named `EarAppraisal` entries, each
carrying a `TrustworthinessVector` from the `ar4si` crate.

```rust,no_run
use ciborium::de::from_reader;
use ear::maps::EarCbor;

let cbor_bytes: &[u8] = &[/* ... */];
let ear: EarCbor = from_reader(cbor_bytes).unwrap();
for (name, appraisal) in &ear.submods.0 {
    println!("{}: status={:?}", name, appraisal.status);
}
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
