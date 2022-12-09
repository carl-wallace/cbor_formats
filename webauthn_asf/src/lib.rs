#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
//#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// attObj = {
//             authData: bytes,
//             $$attStmtType
//          }
//
// attStmtTemplate = (
//                       fmt: text,
//                       attStmt: { * tstr => any } ; Map is filled in by each concrete attStmtType
//                   )
//
// ; Every attestation statement format must have the above fields
// attStmtTemplate .within $$attStmtType

use alloc::string::String;
use alloc::vec::Vec;
use ciborium::value::Value;
use common::BytesType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs, non_snake_case)]
pub struct AttestationObject {
    #[serde(with = "serde_bytes")]
    pub authData: Vec<u8>,
    pub fmt: String,
    pub attStmt: Supported,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs, non_snake_case)]
pub struct AppleAttestationObject {
    pub x5c: Vec<BytesType>,
    #[serde(with = "serde_bytes")]
    pub receipt: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Supported {
    AppleAppAttest(AppleAttestationObject),
    Any(Value),
}
