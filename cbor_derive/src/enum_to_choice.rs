//! Derive macro for CBOR choice types (CDDL `/` operator).
//!
//! Generates `TryFrom<Value>`, `TryFrom<&Value>`, and `Serialize` implementations
//! for enums where each variant dispatches on a CBOR tag or value type.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

use crate::attributes::FieldAttrs;

pub(crate) struct DeriveEnumToChoice {
    ident: Ident,
    variants: Vec<ChoiceVariant>,
}

struct ChoiceVariant {
    ident: Ident,
    inner_type: syn::Type,
    dispatch: ChoiceDispatch,
}

enum ChoiceDispatch {
    /// Match on `Value::Tag(N, inner)`, construct variant via `TryFrom` on inner
    Tag(u64),
    /// Match on `Value::Tag(N, inner)`, wrap result in `Required(parsed)`
    TagWrapped(u64),
    /// Match on a specific `Value` variant (Text, Bytes, Integer, Bool, Map, Array)
    ValueType(String),
    /// Try `TryFrom<Value>` on the variant's inner type (fallback)
    TryFrom,
    /// Catch-all: match any `Value::Tag(t, b)` not matched by earlier arms,
    /// construct a TupleCbor with tag as key and content as value.
    Socket,
}

impl DeriveEnumToChoice {
    pub fn new(input: DeriveInput) -> Self {
        let ident = input.ident;
        let data = match input.data {
            Data::Enum(e) => e,
            _ => panic!("EnumToChoice can only be derived for enums"),
        };

        let variants: Vec<_> = data
            .variants
            .iter()
            .map(|v| {
                let variant_ident = v.ident.clone();

                // Get inner type from the single unnamed field
                let inner_type = match &v.fields {
                    Fields::Unnamed(f) if f.unnamed.len() == 1 => {
                        f.unnamed.first().expect("checked len").ty.clone()
                    }
                    _ => panic!(
                        "EnumToChoice variant `{}` must have exactly one unnamed field",
                        variant_ident
                    ),
                };

                // Parse cbor attributes on the variant
                let attrs = FieldAttrs::parse(&v.attrs);

                let dispatch = if attrs.socket {
                    ChoiceDispatch::Socket
                } else if let Some(tag) = attrs.tag {
                    if attrs.cbor == Some(true) {
                        ChoiceDispatch::TagWrapped(tag as u64)
                    } else {
                        ChoiceDispatch::Tag(tag as u64)
                    }
                } else if !attrs.value.is_empty() {
                    ChoiceDispatch::ValueType(attrs.value.clone())
                } else {
                    ChoiceDispatch::TryFrom
                };

                ChoiceVariant {
                    ident: variant_ident,
                    inner_type,
                    dispatch,
                }
            })
            .collect();

        Self { ident, variants }
    }

    pub fn to_tokens(&self) -> TokenStream {
        let ident = &self.ident;
        let ident_name = format!("{}", ident);

        let match_arms = self.gen_match_arms();

        quote! {
            impl TryFrom<Value> for #ident {
                type Error = String;
                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    #ident::try_from(&value)
                }
            }
            impl TryFrom<&Value> for #ident {
                type Error = String;
                fn try_from(value: &Value) -> Result<Self, Self::Error> {
                    #(#match_arms)*
                    Err(alloc::format!("failed to parse value as {}", #ident_name))
                }
            }
        }
    }

    fn gen_match_arms(&self) -> Vec<TokenStream> {
        let ident = &self.ident;
        let mut arms = Vec::new();

        for v in &self.variants {
            let var_ident = &v.ident;
            let inner_type = &v.inner_type;

            match &v.dispatch {
                ChoiceDispatch::Tag(tag) => {
                    arms.push(quote! {
                        if let Value::Tag(#tag, inner) = value {
                            return #inner_type::try_from(inner.as_ref())
                                .map(#ident::#var_ident);
                        }
                    });
                }
                ChoiceDispatch::TagWrapped(tag) => {
                    // Re-serialize the tagged value and deserialize via ciborium's
                    // serde support for Required<T, N>, which handles tag unwrapping.
                    arms.push(quote! {
                        if let Value::Tag(#tag, _) = value {
                            let mut buf = alloc::vec::Vec::new();
                            ciborium::ser::into_writer(value, &mut buf)
                                .map_err(|e| alloc::format!("failed to re-encode tag {}: {}", #tag, e))?;
                            let parsed: #inner_type = ciborium::de::from_reader(buf.as_slice())
                                .map_err(|e| alloc::format!("failed to decode tag {}: {}", #tag, e))?;
                            return Ok(#ident::#var_ident(parsed));
                        }
                    });
                }
                ChoiceDispatch::ValueType(vtype) => {
                    let arm = match vtype.as_str() {
                        "Text" => quote! {
                            if let Value::Text(s) = value {
                                return Ok(#ident::#var_ident(s.clone()));
                            }
                        },
                        "Bytes" => quote! {
                            if let Value::Bytes(_) = value {
                                return Ok(#ident::#var_ident(
                                    #inner_type::try_from(value)?
                                ));
                            }
                        },
                        "Integer" => quote! {
                            if let Value::Integer(i) = value {
                                let val: #inner_type = (*i).try_into().map_err(|_|
                                    alloc::format!("integer out of range for {}", stringify!(#inner_type))
                                )?;
                                return Ok(#ident::#var_ident(val));
                            }
                        },
                        "Bool" => quote! {
                            if let Value::Bool(b) = value {
                                return Ok(#ident::#var_ident(*b));
                            }
                        },
                        "Map" | "Array" => quote! {
                            if let Ok(v) = #inner_type::try_from(value) {
                                return Ok(#ident::#var_ident(v));
                            }
                        },
                        other => panic!("unsupported value type in EnumToChoice: {}", other),
                    };
                    arms.push(arm);
                }
                ChoiceDispatch::TryFrom => {
                    arms.push(quote! {
                        if let Ok(v) = #inner_type::try_from(value) {
                            return Ok(#ident::#var_ident(v));
                        }
                    });
                }
                ChoiceDispatch::Socket => {
                    arms.push(quote! {
                        if let Value::Tag(t, b) = value {
                            return Ok(#ident::#var_ident(#inner_type {
                                key: Value::Integer(ciborium::value::Integer::from(*t)),
                                value: *b.clone(),
                            }));
                        }
                    });
                }
            }
        }

        arms
    }
}
