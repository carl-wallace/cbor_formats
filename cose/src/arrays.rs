//! Array-based structs

// Since there is not StructToGroup, the contents of groups are simply copied
// into each structure that uses the group.
// Headers = (
//     protected : empty_or_serialized_map,
//     unprotected : header_map
// )

use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::{fmt, marker::PhantomData};

use ciborium::tag::Required;
use ciborium::{cbor, value::Value};
use serde::ser::Error as OtherError;
use serde::{Deserialize, Serialize};
use serde::{__private::size_hint, de::Error, de::Visitor};

use crate::choices::{
    EmptyOrSerializedMap, EncStructureContext, MacStructureContext, SignatureOrSignature1,
};
use crate::maps::*;
use cbor_derive::StructToArray;
use common::{BinaryOrNil, BytesType};

/// CBOR and JSON encoding/decoding of `COSE_Sign`, see [COSE Section 4.1].
///
/// ```text
/// COSE_Sign = [
///     Headers,
///     payload : bstr / nil,
///     signatures : [+ COSE_Signature]
/// ]
/// ```
/// [COSE Section 4.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-signing-with-one-or-more-si
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseSign {
    pub protected: EmptyOrSerializedMap,
    #[cbor(cbor = "true")]
    pub unprotected: HeaderMap,
    pub payload: BinaryOrNil,
    #[cbor(value = "Array", cbor = "true")]
    pub signatures: Vec<CoseSignature>,
}

/// `COSE_Sign_Tagged` support, see [COSE Section 4.1].
///
/// ```text
/// COSE_Sign_Tagged = #6.98(COSE_Sign)
/// ```
/// [COSE Section 4.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-signing-with-one-or-more-si
pub type TaggedCoseSign = Required<CoseSignCbor, 98>;

/// CBOR and JSON encoding/decoding of `COSE_Signature`, see [COSE Section 4.1].
///
/// ```text
/// COSE_Signature =  [
///     Headers,
///     signature : bstr
/// ]
/// ```
/// [COSE Section 4.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-signing-with-one-or-more-si
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseSignature {
    pub protected: EmptyOrSerializedMap,
    #[cbor(cbor = "true")]
    pub unprotected: HeaderMap,
    pub signature: BytesType,
}

/// CBOR and JSON encoding/decoding of `COSE_Sign1`, see [COSE Section 4.2].
///
/// ```text
/// COSE_Sign1 = [
///     Headers,
///     payload : bstr / nil,
///     signature : bstr
/// ]
/// ```
/// [COSE Section 4.2]: https://datatracker.ietf.org/doc/html/rfc9052#name-signing-with-one-signer
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseSign1 {
    pub protected: EmptyOrSerializedMap,
    #[cbor(cbor = "true")]
    pub unprotected: HeaderMap,
    pub payload: BinaryOrNil,
    pub signature: BytesType,
}

/// `COSE_Sign1_Tagged` support, see [COSE Section 4.2].
///
/// ```text
/// COSE_Sign1_Tagged = #6.18(COSE_Sign1)
/// ```
/// [COSE Section 4.2]: https://datatracker.ietf.org/doc/html/rfc9052#name-signing-with-one-signer
pub type TaggedCoseSign1 = Required<CoseSign1Cbor, 18>;

/// CBOR and JSON encoding/decoding of `Sig_structure`, see [COSE Section 4.4].
///
/// ```text
/// Sig_structure = [
///     context : "Signature" / "Signature1",
///     body_protected : empty_or_serialized_map,
///     ? sign_protected : empty_or_serialized_map,
///     external_aad : bstr,
///     payload : bstr
/// ]
/// ```
/// [COSE Section 4.4]: https://datatracker.ietf.org/doc/html/rfc9052#name-signing-and-verification-pr
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SigStructure {
    pub context: SignatureOrSignature1,
    pub body_protected: EmptyOrSerializedMap,
    pub sign_protected: Option<EmptyOrSerializedMap>,
    pub external_aad: BytesType,
    pub payload: BytesType,
}

/// CBOR and JSON encoding/decoding of `COSE_Encrypt`, see [COSE Section 5.1].
///
/// ```text
/// COSE_Encrypt = [
///     Headers,
///     ciphertext : bstr / nil,
///     recipients : [+COSE_recipient]
/// ]
/// ```
/// [COSE Section 5.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-enveloped-cose-structure
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseEncrypt {
    pub protected: EmptyOrSerializedMap,
    #[cbor(cbor = "true")]
    pub unprotected: HeaderMap,
    pub ciphertext: BinaryOrNil,
    #[cbor(value = "Array", cbor = "true")]
    pub recipients: Vec<CoseRecipient>,
}

/// `COSE_Encrypt0_Tagged` support, see [COSE Section 5.1].
///
/// ```text
/// COSE_Encrypt_Tagged = #6.96(COSE_Encrypt)
/// ```
/// [COSE Section 5.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-enveloped-cose-structure
pub type TaggedCoseEncrypt = Required<CoseEncryptCbor, 96>;

/// CBOR and JSON encoding/decoding of `COSE_recipient`, see [COSE Section 5.1].
///
/// ```text
/// COSE_recipient = [
///     Headers,
///     ciphertext : bstr / nil,
///     ? recipients : [+COSE_recipient]
/// ]
/// ```
/// [COSE Section 5.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-enveloped-cose-structure
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseRecipient {
    pub protected: EmptyOrSerializedMap,
    #[cbor(cbor = "true")]
    pub unprotected: HeaderMap,
    pub ciphertext: BinaryOrNil,
    #[cbor(value = "Array", cbor = "true")]
    pub recipients: Option<Vec<CoseRecipient>>,
}

/// CBOR and JSON encoding/decoding of `COSE_Encrypt0`, see [COSE Section 5.2].
///
/// ```text
/// COSE_Encrypt0 = [
///     Headers,
///     ciphertext : bstr / nil,
/// ]
/// ```
/// [COSE Section 5.2]: https://datatracker.ietf.org/doc/html/rfc9052#name-single-recipient-encrypted
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseEncrypt0 {
    pub protected: EmptyOrSerializedMap,
    #[cbor(cbor = "true")]
    pub unprotected: HeaderMap,
    pub ciphertext: BinaryOrNil,
}

/// `COSE_Encrypt0_Tagged` support, see [COSE Section 5.2].
///
/// ```text
/// COSE_Encrypt0_Tagged = #6.16(COSE_Encrypt0)
/// ```
/// [COSE Section 5.2]: https://datatracker.ietf.org/doc/html/rfc9052#name-single-recipient-encrypted
pub type TaggedCoseEncrypt0 = Required<CoseEncrypt0Cbor, 16>;

/// CBOR and JSON encoding/decoding of `Enc_structure`, see [COSE Section 5.3].
///
/// ```text
// Enc_structure = [
///     context : "Encrypt" / "Encrypt0" / "Enc_Recipient" /
///         "Mac_Recipient" / "Rec_Recipient",
///     protected : empty_or_serialized_map,
///     external_aad : bstr
/// ]
/// ```
/// [COSE Section 5.3]: https://datatracker.ietf.org/doc/html/rfc9052#name-how-to-encrypt-and-decrypt-
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EncStructure {
    pub context: EncStructureContext,
    pub protected: EmptyOrSerializedMap,
    pub external_aad: BytesType,
}

/// CBOR and JSON encoding/decoding of `COSE_Mac0`, see [COSE Section 6.1].
///
/// ```text
/// COSE_Mac = [
//    Headers,
//    payload : bstr / nil,
//    tag : bstr,
//    recipients : [+COSE_recipient]
// ]
/// ```
/// [COSE Section 6.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-maced-message-with-recipien
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseMac {
    pub protected: EmptyOrSerializedMap,
    #[cbor(cbor = "true")]
    pub unprotected: HeaderMap,
    pub payload: BinaryOrNil,
    pub tag: BytesType,
    #[cbor(value = "Array", cbor = "true")]
    pub recipients: Vec<CoseRecipient>,
}

/// `COSE_Mac_Tagged` support, see [COSE Section 6.1].
///
/// ```text
/// COSE_Mac_Tagged = #6.97(COSE_Mac)
/// ```
/// [COSE Section 6.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-maced-message-with-recipien
pub type TaggedCoseMac = Required<CoseMacCbor, 97>;

/// CBOR and JSON encoding/decoding of `COSE_Mac0`, see [COSE Section 6.1].
///
/// ```text
/// COSE_Mac0 = [
///    Headers,
///    payload : bstr / nil,
///    tag : bstr,
/// ]
/// ```
/// [COSE Section 6.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-maced-message-with-recipien
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseMac0 {
    pub protected: EmptyOrSerializedMap,
    #[cbor(cbor = "true")]
    pub unprotected: HeaderMap,
    pub payload: BinaryOrNil,
    pub tag: BytesType,
}

/// `COSE_Mac_Tagged` support, see [COSE Section 6.1].
///
/// ```text
/// COSE_Mac0_Tagged = #6.17(COSE_Mac0)
/// ```
/// [COSE Section 6.1]: https://datatracker.ietf.org/doc/html/rfc9052#name-maced-message-with-recipien
pub type TaggedCoseMac0 = Required<CoseMac0Cbor, 17>;

/// CBOR and JSON encoding/decoding of `MAC_structure`, see [COSE Section 6.3].
///
/// ```text
/// MAC_structure = [
///      context : "MAC" / "MAC0",
///      protected : empty_or_serialized_map,
///      external_aad : bstr,
///      payload : bstr
/// ]
/// ```
/// [COSE Section 6.3]: https://datatracker.ietf.org/doc/html/rfc9052#name-how-to-compute-and-verify-a
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MacStructure {
    pub context: MacStructureContext,
    pub protected: EmptyOrSerializedMap,
    pub external_aad: BytesType,
    pub payload: BytesType,
}
