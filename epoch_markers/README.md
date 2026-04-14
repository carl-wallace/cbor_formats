# epoch_markers

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CBOR-focused encoders/decoders for Epoch Marker structures as defined in
[draft-ietf-rats-epoch-markers-03](https://datatracker.ietf.org/doc/html/draft-ietf-rats-epoch-markers-03).

Epoch markers provide a time-synchronization mechanism for attestation
systems. The main types are `EpochMarker` (which can be a simple ID or
a labeled structure), `CborTime` (integer or float seconds since epoch),
and `MessageImprint` (hash algorithm + digest, for timestamping).

```rust,no_run
use ciborium::de::from_reader;
use epoch_markers::choices::EpochMarker;

let cbor_bytes: &[u8] = &[/* ... */];
let marker: EpochMarker = from_reader(cbor_bytes).unwrap();
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
