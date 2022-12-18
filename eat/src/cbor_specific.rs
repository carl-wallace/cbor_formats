//! CBOR-specific definitions per section 7.3.2 of EAT specification

// EAT-CBOR-Token = $EAT-CBOR-Tagged-Token / $EAT-CBOR-Untagged-Token
//
// $EAT-CBOR-Tagged-Token /= CWT-Tagged-Message
// $EAT-CBOR-Tagged-Token /= BUNDLE-Tagged-Message
//
// $EAT-CBOR-Untagged-Token /= CWT-Untagged-Message
// $EAT-CBOR-Untagged-Token /= BUNDLE-Untagged-Message
//
// Nested-Token = CBOR-Nested-Token
//
// CBOR-Nested-Token =
//     JSON-Token-Inside-CBOR-Token /
//     CBOR-Token-Inside-CBOR-Token
//
// CBOR-Token-Inside-CBOR-Token = bstr .cbor $EAT-CBOR-Tagged-Token
//
// JSON-Token-Inside-CBOR-Token = tstr
//
// $$Claims-Set-Claims //= (submods-label => { + text => Submodule })
//
// Submodule = Claims-Set / CBOR-Nested-Token /
//             Detached-Submodule-Digest
