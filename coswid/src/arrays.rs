//! Array-based structs from the Concise Software Identification Tags (CoSWID) spec ([RFC 9393]).
//!
//! This module re-exports array types defined in the [`common`] crate (e.g., `hash-entry`).
//! See [`common::arrays`] for the CDDL-to-Rust mapping of those types.
//!
//! [RFC 9393]: https://datatracker.ietf.org/doc/html/rfc9393

// defined in corim
// hash-entry = [
//   hash-alg-id: int,
//   hash-value: bytes,
// ]
