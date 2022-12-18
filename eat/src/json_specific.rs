//! JSON-specific definitions per section 7.3.3 of EAT specification

use alloc::string::String;
use serde::{Deserialize, Serialize};

// EAT-JSON-Token = $EAT-JSON-Token-Formats
//
// $EAT-JSON-Token-Formats /= JWT-Message
// $EAT-JSON-Token-Formats /= BUNDLE-Untagged-Message
//
//
// Nested-Token = JSON-Selector

// $JSON-Selector-Type /= "JWT" / "CBOR" / "BUNDLE" / "DIGEST"

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum JsonSelectorType {
    #[serde(rename = "JWT")]
    Jwt,
    #[serde(rename = "CBOR")]
    Cbor,
    #[serde(rename = "BUNDLE")]
    Bundle,
    #[serde(rename = "DIGEST")]
    Digest,
    //#[serde(other)]
    Other(String),
}

// $JSON-Selector-Value /= JWT-Message /
//                   CBOR-Token-Inside-JSON-Token /
//                   Detached-EAT-Bundle /
//                   Detached-Submodule-Digest
//
// JSON-Selector = [
//    type : $JSON-Selector-Type,
//    nested-token : $JSON-Selector-Value
// ]
//
// CBOR-Token-Inside-JSON-Token = base64-url-text
//
// $$Claims-Set-Claims //= (submods-label => { + text => Submodule })
//
// Submodule = Claims-Set / JSON-Selector
