//! General-purpose Tuple and TupleCbor types

use alloc::string::String;
use alloc::vec::Vec;
use ciborium::value::{Integer, Value};
use core::{fmt, marker::PhantomData};
use serde::de::Error;
use serde::de::VariantAccess;
use serde::de::Visitor;
use serde::ser::Error as OtherError;
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Tuple {
    pub key: Value,
    pub value: Value,
}
#[derive(Clone, Debug, PartialEq)]
///Supports CBOR encoding/decoding of the corresponding array type, which is described in [Tuple]
pub struct TupleCbor {
    /// Decoded field
    pub key: Value,
    /// Decoded field
    pub value: Value,
}

impl TryFrom<TupleCbor> for Tuple {
    type Error = String;
    fn try_from(value: TupleCbor) -> Result<Self, Self::Error> {
        Ok(Tuple {
            key: value.key.clone(),
            value: value.value,
        })
    }
}
impl TryFrom<Tuple> for TupleCbor {
    type Error = String;
    fn try_from(value: Tuple) -> Result<Self, Self::Error> {
        Ok(TupleCbor {
            key: value.key.clone(),
            value: value.value,
        })
    }
}
impl TryFrom<&TupleCbor> for Tuple {
    type Error = String;
    fn try_from(value: &TupleCbor) -> Result<Self, Self::Error> {
        Ok(Tuple {
            key: value.key.clone(),
            value: value.value.clone(),
        })
    }
}
impl TryFrom<&Tuple> for TupleCbor {
    type Error = String;
    fn try_from(value: &Tuple) -> Result<Self, Self::Error> {
        Ok(TupleCbor {
            key: value.key.clone(),
            value: value.value.clone(),
        })
    }
}

impl TryFrom<(Value, Value)> for TupleCbor {
    type Error = String;
    fn try_from(value: (Value, Value)) -> Result<Self, Self::Error> {
        Ok(TupleCbor {
            key: value.0.clone(),
            value: value.1,
        })
    }
}

impl TryFrom<Value> for TupleCbor {
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
impl TryFrom<&Value> for TupleCbor {
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
impl TryFrom<&TupleCbor> for Vec<Value> {
    type Error = String;
    fn try_from(value: &TupleCbor) -> Result<Self, Self::Error> {
        let mut v = ::alloc::vec::Vec::new();
        v.push(
            match {
                #[allow(unused_imports)]
                use ::ciborium::value::Value::Null as null;
                ::ciborium::value::Value::serialized(&value.key)
            } {
                Ok(v) => v,
                Err(_) => return Err("Failed to parse TupleCbor".to_string()),
            },
        );
        v.push(
            match {
                #[allow(unused_imports)]
                use ::ciborium::value::Value::Null as null;
                ::ciborium::value::Value::serialized(&value.value)
            } {
                Ok(v) => v,
                Err(_) => return Err("Failed to parse TupleCbor".to_string()),
            },
        );
        Ok(v)
    }
}
impl TryFrom<Vec<Value>> for TupleCbor {
    type Error = String;
    fn try_from(v: Vec<Value>) -> Result<Self, Self::Error> {
        Ok(TupleCbor {
            key: v[0usize].clone(),
            value: v[1usize].clone(),
        })
    }
}
impl serde::Serialize for TupleCbor {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let v: Vec<Value> = match self.try_into() {
            Ok(r) => r,
            Err(e) => {
                return Err(__S::Error::custom(e));
            }
        };
        // let t: u64 = v[0].as_integer().unwrap().try_into().unwrap();
        // let val = Value::Tag(t, Box::new(v[1].clone()));
        // __serializer.serialize_some(&val)
        let i = match v[0].as_integer() {
            Some(i) => i,
            None => {
                return Err(__S::Error::custom(
                    "Failed to parse tag value as an integer",
                ))
            }
        };

        let t: u64 = match i.try_into() {
            Ok(t) => t,
            Err(e) => return Err(__S::Error::custom(e)),
        };

        let val = Value::Tag(t, Box::new(v[1].clone()));
        __serializer.serialize_some(&val)
    }
}
impl<'de> Deserialize<'de> for TupleCbor {
    fn deserialize<__D>(deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        struct MapVisitor {
            marker: PhantomData<Value>,
        }
        impl<'de> Visitor<'de> for MapVisitor {
            type Value = Value;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an array")
            }
            fn visit_enum<A: de::EnumAccess<'de>>(self, acc: A) -> Result<Self::Value, A::Error> {
                struct Inner;

                impl<'de> serde::de::Visitor<'de> for Inner {
                    type Value = Value;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "a valid CBOR item")
                    }

                    #[inline]
                    fn visit_seq<A: de::SeqAccess<'de>>(
                        self,
                        mut acc: A,
                    ) -> Result<Self::Value, A::Error> {
                        let tag: u64 = acc
                            .next_element()?
                            .ok_or_else(|| de::Error::custom("expected tag"))?;
                        let val = acc
                            .next_element()?
                            .ok_or_else(|| de::Error::custom("expected val"))?;
                        Ok(Value::Tag(tag, Box::new(val)))
                    }
                }

                let (name, data): (String, _) = acc.variant()?;
                assert_eq!("@@TAGGED@@", name);
                data.tuple_variant(2, Inner)
            }
        }
        let visitor = MapVisitor {
            marker: PhantomData,
        };
        match deserializer.deserialize_any(visitor) {
            Ok(v) => {
                let t = match v.as_tag() {
                    Some(t) => t,
                    None => return Err(__D::Error::custom("Failed to parse tag value")),
                };
                match Integer::try_from(t.0) {
                    Ok(i) => {
                        let v0 = Value::Integer(i);
                        let vals = vec![v0, t.1.clone()];
                        match TupleCbor::try_from(vals) {
                            Ok(r) => Ok(r),
                            Err(e) => Err(__D::Error::custom(e)),
                        }
                    }
                    Err(e) => Err(__D::Error::custom(e)),
                }
            }
            Err(e) => Err(__D::Error::custom(e)),
        }
    }
}
