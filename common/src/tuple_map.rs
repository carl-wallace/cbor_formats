//! General-purpose TupleMap and TupleMapCbor types

use crate::tuple::*;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use ciborium::value::Value;
use core::{fmt, marker::PhantomData};
use serde::de::MapAccess;
use serde::ser::Error as OtherError;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};
use serde::{__private::size_hint, de::Error, de::Visitor};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct TupleMap {
    pub tuples: Vec<Tuple>,
}
///Supports CBOR encoding/decoding of the corresponding array type, which is described in [TupleMap]
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct TupleMapCbor {
    pub tuples: Vec<TupleCbor>,
}

impl TryFrom<TupleMapCbor> for TupleMap {
    type Error = String;
    fn try_from(value: TupleMapCbor) -> Result<Self, Self::Error> {
        Ok(TupleMap {
            tuples: value
                .tuples
                .iter()
                .map(|m| Tuple::try_from(m).unwrap())
                .collect(),
        })
    }
}
impl TryFrom<TupleMap> for TupleMapCbor {
    type Error = String;
    fn try_from(value: TupleMap) -> Result<Self, Self::Error> {
        Ok(TupleMapCbor {
            tuples: value
                .tuples
                .iter()
                .map(|m| TupleCbor::try_from(m).unwrap())
                .collect(),
        })
    }
}
impl TryFrom<&TupleMapCbor> for TupleMap {
    type Error = String;
    fn try_from(value: &TupleMapCbor) -> Result<Self, Self::Error> {
        Ok(TupleMap {
            tuples: value
                .tuples
                .iter()
                .map(|m| Tuple::try_from(m).unwrap())
                .collect(),
        })
    }
}
impl TryFrom<&TupleMap> for TupleMapCbor {
    type Error = String;
    fn try_from(value: &TupleMap) -> Result<Self, Self::Error> {
        Ok(TupleMapCbor {
            tuples: value
                .tuples
                .iter()
                .map(|m| TupleCbor::try_from(m).unwrap())
                .collect(),
        })
    }
}
impl TryFrom<Value> for TupleMapCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(s) => match Self::try_from(s) {
                Ok(val) => Ok(val),
                Err(_) => Err("Failed to parse TupleCbor".to_string()),
            },
            _ => Err("Failed to parse TupleCbor".to_string()),
        }
    }
}
impl TryFrom<&Value> for TupleMapCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(s) => match Self::try_from(s.clone()) {
                Ok(val) => Ok(val),
                Err(_) => Err("Failed to parse TupleCbor".to_string()),
            },
            _ => Err("Failed to parse TupleCbor".to_string()),
        }
    }
}
impl TryFrom<&TupleMapCbor> for Vec<(Value, Value)> {
    type Error = String;
    fn try_from(value: &TupleMapCbor) -> Result<Self, Self::Error> {
        let mut v = ::alloc::vec::Vec::new();
        for i in &value.tuples {
            let v1 = match {
                #[allow(unused_imports)]
                use ::ciborium::value::Value::Null as null;
                ::ciborium::value::Value::serialized(&i.key)
            } {
                Ok(v) => v,
                Err(_) => return Err("Failed to parse TupleCbor".to_string()),
            };
            let v2 = match {
                #[allow(unused_imports)]
                use ::ciborium::value::Value::Null as null;
                ::ciborium::value::Value::serialized(&i.value)
            } {
                Ok(v) => v,
                Err(_) => return Err("Failed to parse TupleCbor".to_string()),
            };
            v.push((v1, v2));
        }
        Ok(v)
    }
}
impl TryFrom<Vec<Value>> for TupleMapCbor {
    type Error = String;
    fn try_from(v: Vec<Value>) -> Result<Self, Self::Error> {
        Ok(TupleMapCbor {
            tuples: v.iter().map(|m| TupleCbor::try_from(m).unwrap()).collect(),
        })
    }
}
impl serde::Serialize for TupleMapCbor {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let v: Vec<(Value, Value)> = match self.try_into() {
            Ok(r) => r,
            Err(e) => {
                return Err(__S::Error::custom(e));
            }
        };
        let mut m = __serializer.serialize_map(Some(v.len())).unwrap();
        for i in &v {
            match TupleCbor::try_from(i.clone()) {
                Ok(tmc) => {
                    if m.serialize_entry(&tmc.key, &tmc.value).is_err() {
                        return Err(__S::Error::custom("Failed to serialize tuple element"));
                    }
                }
                Err(e) => {
                    return Err(__S::Error::custom(e));
                }
            }
        }
        m.end()
    }
}
impl<'de> Deserialize<'de> for TupleMapCbor {
    fn deserialize<__D>(deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        struct MapVisitor {
            marker: PhantomData<Value>,
        }
        impl<'de> Visitor<'de> for MapVisitor {
            type Value = Vec<(Value, Value)>;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = Vec::with_capacity(size_hint::cautious(map.size_hint()));
                while let Some(value) = map.next_entry()? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        let visitor = MapVisitor {
            marker: PhantomData,
        };
        match deserializer.deserialize_map(visitor) {
            Ok(v) => {
                let mut retval = vec![];
                for i in v {
                    let iv = match TupleCbor::try_from(i) {
                        Ok(r) => r,
                        Err(e) => return Err(__D::Error::custom(e)),
                    };
                    retval.push(iv);
                }
                Ok(TupleMapCbor { tuples: retval })
            }
            Err(e) => Err(__D::Error::custom(e)),
        }
    }
}
