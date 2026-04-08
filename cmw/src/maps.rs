//! Collection types from the CMW specification (draft-ietf-rats-msg-wrap-23 Section 3)

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::fmt;
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::choices::{CborCmw, CborCollectionKey, JsonCmw};

/// The `cbor-collection` type from [CMW Section 3].
///
/// ```text
/// cbor-collection = {
///   ? "__cmwc_t": ~uri / oid
///   + &(label: (int / text)) => cbor-cmw
/// }
/// ```
///
/// [CMW Section 3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3
#[derive(Clone, Debug, PartialEq)]
pub struct CborCollection {
    /// Optional collection type identifier (`__cmwc_t`), a URI or OID string.
    pub collection_type: Option<String>,
    /// One or more entries keyed by integer or text labels.
    pub entries: BTreeMap<CborCollectionKey, CborCmw>,
}

const CMWC_T_KEY: &str = "__cmwc_t";

impl Serialize for CborCollection {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = self.entries.len() + if self.collection_type.is_some() { 1 } else { 0 };
        let mut map = serializer.serialize_map(Some(len))?;
        if let Some(ref ct) = self.collection_type {
            map.serialize_entry(CMWC_T_KEY, ct)?;
        }
        for (k, v) in &self.entries {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for CborCollection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CborCollectionVisitor;

        impl<'de> Visitor<'de> for CborCollectionVisitor {
            type Value = CborCollection;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a cbor-collection map")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut collection_type = None;
                let mut entries = BTreeMap::new();

                // In CBOR, keys can be int or text. We use ciborium::Value as the key
                // to handle both cases during deserialization.
                while let Some(key) = map.next_key::<ciborium::Value>()? {
                    match &key {
                        ciborium::Value::Text(s) if s == CMWC_T_KEY => {
                            let val: String = map.next_value()?;
                            collection_type = Some(val);
                        }
                        ciborium::Value::Text(s) => {
                            let val: CborCmw = map.next_value()?;
                            entries.insert(CborCollectionKey::Text(s.clone()), val);
                        }
                        ciborium::Value::Integer(i) => {
                            let ival: i64 = (*i).try_into().map_err(de::Error::custom)?;
                            let val: CborCmw = map.next_value()?;
                            entries.insert(CborCollectionKey::Int(ival), val);
                        }
                        _ => {
                            return Err(de::Error::custom(
                                "cbor-collection keys must be int or text",
                            ));
                        }
                    }
                }

                if entries.is_empty() {
                    return Err(de::Error::custom(
                        "cbor-collection must have at least one entry",
                    ));
                }

                Ok(CborCollection {
                    collection_type,
                    entries,
                })
            }
        }

        deserializer.deserialize_map(CborCollectionVisitor)
    }
}

/// The `json-collection` type from [CMW Section 3].
///
/// ```text
/// json-collection = {
///   ? "__cmwc_t": ~uri / oid
///   + &(label: text) => json-cmw
/// }
/// ```
///
/// [CMW Section 3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3
#[derive(Clone, Debug, PartialEq)]
pub struct JsonCollection {
    /// Optional collection type identifier (`__cmwc_t`), a URI or OID string.
    pub collection_type: Option<String>,
    /// One or more entries keyed by text labels.
    pub entries: BTreeMap<String, JsonCmw>,
}

impl Serialize for JsonCollection {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = self.entries.len() + if self.collection_type.is_some() { 1 } else { 0 };
        let mut map = serializer.serialize_map(Some(len))?;
        if let Some(ref ct) = self.collection_type {
            map.serialize_entry(CMWC_T_KEY, ct)?;
        }
        for (k, v) in &self.entries {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for JsonCollection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct JsonCollectionVisitor;

        impl<'de> Visitor<'de> for JsonCollectionVisitor {
            type Value = JsonCollection;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a json-collection map")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut collection_type = None;
                let mut entries = BTreeMap::new();

                while let Some(key) = map.next_key::<String>()? {
                    if key == CMWC_T_KEY {
                        let val: String = map.next_value()?;
                        collection_type = Some(val);
                    } else {
                        let val: JsonCmw = map.next_value()?;
                        entries.insert(key, val);
                    }
                }

                if entries.is_empty() {
                    return Err(de::Error::custom(
                        "json-collection must have at least one entry",
                    ));
                }

                Ok(JsonCollection {
                    collection_type,
                    entries,
                })
            }
        }

        deserializer.deserialize_map(JsonCollectionVisitor)
    }
}
