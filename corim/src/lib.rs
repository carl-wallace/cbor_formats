#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod arrays;
pub mod choices;
pub mod maps;

use alloc::vec::Vec;
use ciborium::tag::Required;

// corim = #6.500($concise-reference-integrity-manifest-type-choice)
//
// $concise-reference-integrity-manifest-type-choice /= #6.501(unsigned-corim-map)
// $concise-reference-integrity-manifest-type-choice /= #6.502(signed-corim)
//
// signed-corim = #6.18(COSE-Sign1-corim)
//
// unprotected-signed-corim-header-map = {
//   * cose-label => cose-values
// }
//
// COSE-Sign1-corim = [
//   protected: bstr .cbor protected-signed-corim-header-map
//   unprotected: unprotected-signed-corim-header-map
//   payload: bstr .cbor unsigned-corim-map
//   signature: bstr
// ]

/// $raw-value-type-choice /= #6.560(bytes)
pub type RawValueTypeChoice = Required<Vec<u8>, 560>;

/// raw-value-mask-type = bytes
pub type RawValueMaskType = Vec<u8>;

//    ip-addr-type-choice = ip4-addr-type / ip6-addr-type
//    ip4-addr-type = bytes .size 4
//    ip6-addr-type = bytes .size 16
//
//    mac-addr-type-choice = eui48-addr-type / eui64-addr-type
//    eui48-addr-type = bytes .size 6
//    eui64-addr-type = bytes .size 8
//
// svn = int
// min-svn = int
// tagged-svn = #6.552(svn)
// tagged-min-svn = #6.553(min-svn)
// svn-type-choice = tagged-svn / tagged-min-svn
//
// flags-type = bytes ;.bits operational-flags
//
// operational-flags = &(
//   not-configured: 0
//   not-secure: 1
//   recovery: 2
//   debug: 3
// )
//
// ip-addr-type-choice = ip4-addr-type / ip6-addr-type
// ip4-addr-type = bytes .size 4
// ip6-addr-type = bytes .size 16
//
// mac-addr-type-choice = eui48-addr-type / eui64-addr-type
// eui48-addr-type = bytes .size 6
// eui64-addr-type = bytes .size 8
//
// serial-number-type = text
//
// digests-type = [ + hash-entry ]
//
// ; non-empty<M> = (M) .within ({ + any => any })
// non-empty<M> = M .within ({ + any => any })
//
// cose-label = int / tstr
// cose-values = any
