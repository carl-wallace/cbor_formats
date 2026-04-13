//! Derive macro for CBOR choice types (CDDL `/` operator).
//!
//! Generates `TryFrom<Value>`, `TryFrom<&Value>` implementations for the enum,
//! and optionally a `*Cbor` companion enum with `Required<T, N>` wrappers for
//! tagged variants, plus bidirectional `TryFrom` conversions between the two.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident};

use crate::attributes::FieldAttrs;

pub(crate) struct DeriveEnumToChoice {
    ident: Ident,
    variants: Vec<ChoiceVariant>,
    type_attrs: crate::attributes::TypeAttrs,
}

struct ChoiceVariant {
    ident: Ident,
    /// `None` for unit variants (IntegerUnit dispatch).
    inner_type: Option<syn::Type>,
    dispatch: ChoiceDispatch,
}

enum ChoiceDispatch {
    /// Match on `Value::Tag(N, inner)`, construct variant via `TryFrom` on inner
    Tag(u64),
    /// Match on `Value::Tag(N, inner)`, re-serialize/deserialize via ciborium serde
    /// for `Required<T, N>` types. In the Cbor companion, wraps in `Required<T, N>`.
    TagWrapped(u64),
    /// Match on a specific `Value` variant (Text, Bytes, Integer, Bool, Map, Array)
    ValueType(String),
    /// Try `TryFrom<Value>` on the variant's inner type (fallback)
    TryFrom,
    /// Catch-all for CDDL `$` sockets: match any `Value::Tag(t, b)` not matched
    /// by earlier arms.
    Socket,
    /// Unit variant mapped to a specific integer value (for `&(name: N)` CDDL groups).
    IntegerUnit(i64),
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

                let attrs = FieldAttrs::parse(&v.attrs);

                let (inner_type, dispatch) = match &v.fields {
                    Fields::Unnamed(f) if f.unnamed.len() == 1 => {
                        let ty = f.unnamed.first().expect("checked len").ty.clone();
                        let d = if attrs.socket {
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
                        (Some(ty), d)
                    }
                    Fields::Unit => {
                        // Unit variant — must have a tag attribute giving the integer value
                        let int_val = attrs.tag.unwrap_or_else(|| {
                            panic!(
                                "EnumToChoice unit variant `{}` requires #[cbor(tag = \"N\")] for its integer value",
                                variant_ident
                            );
                        });
                        (None, ChoiceDispatch::IntegerUnit(int_val as i64))
                    }
                    _ => panic!(
                        "EnumToChoice variant `{}` must have one unnamed field or be a unit variant",
                        variant_ident
                    ),
                };

                ChoiceVariant {
                    ident: variant_ident,
                    inner_type,
                    dispatch,
                }
            })
            .collect();

        let type_attrs = crate::attributes::TypeAttrs::parse(&input.attrs);

        Self {
            ident,
            variants,
            type_attrs,
        }
    }

    pub fn to_tokens(&self) -> TokenStream {
        let try_from_impls = self.gen_try_from_value();
        let cbor_companion = self.gen_cbor_companion();

        quote! {
            #try_from_impls
            #cbor_companion
        }
    }

    /// Generate `TryFrom<Value>` and `TryFrom<&Value>` for the enum itself.
    fn gen_try_from_value(&self) -> TokenStream {
        let ident = &self.ident;
        let ident_name = format!("{}", ident);
        let match_arms = self.gen_match_arms(ident);

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

    /// Generate the `*Cbor` companion enum if any variant uses `TagWrapped`.
    /// Also generates `TryFrom` conversions between the JSON and CBOR enums.
    fn gen_cbor_companion(&self) -> TokenStream {
        // Only generate a Cbor companion when explicitly requested via #[cbor(companion = "true")]
        if !self.type_attrs.companion {
            return TokenStream::new();
        }

        let ident = &self.ident;
        let cbor_ident = format_ident!("{}Cbor", ident);
        let cbor_ident_name = format!("{}", cbor_ident);

        // Check if this is an integer-unit enum (all variants are IntegerUnit or other(String))
        let is_integer_unit = self.variants.iter().all(|v| {
            matches!(v.dispatch, ChoiceDispatch::IntegerUnit(_))
                || (v.inner_type.is_some() && format!("{}", v.ident) == "other")
        });

        if is_integer_unit {
            return self.gen_integer_unit_companion();
        }

        // Generate the Cbor enum variants
        let mut cbor_variants = Vec::new();
        for v in &self.variants {
            let var_ident = &v.ident;
            let inner_type = v.inner_type.as_ref();

            match &v.dispatch {
                ChoiceDispatch::TagWrapped(tag) => {
                    let inner = inner_type.expect("TagWrapped must have inner type");
                    cbor_variants.push(quote! {
                        #var_ident(ciborium::tag::Required<#inner, #tag>)
                    });
                }
                ChoiceDispatch::Socket => {
                    cbor_variants.push(quote! {
                        Other(common::TupleCbor)
                    });
                }
                _ => {
                    let inner = inner_type.expect("non-socket variant must have inner type");
                    cbor_variants.push(quote! {
                        #var_ident(#inner)
                    });
                }
            }
        }

        // Generate TryFrom<Value> match arms for the Cbor enum
        let cbor_match_arms = self.gen_cbor_match_arms(&cbor_ident);

        // Generate JSON -> CBOR conversion
        let json_to_cbor_arms = self.gen_json_to_cbor_arms(&cbor_ident);

        // Generate CBOR -> JSON conversion
        let cbor_to_json_arms = self.gen_cbor_to_json_arms(&cbor_ident);

        let comment = format!(
            "Auto-generated CBOR companion for [{}]. Tagged variants use `Required<T, N>` wrappers.",
            ident
        );

        quote! {
            #[doc = #comment]
            #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
            #[serde(untagged)]
            #[allow(missing_docs, non_camel_case_types)]
            pub enum #cbor_ident {
                #(#cbor_variants),*
            }

            impl TryFrom<Value> for #cbor_ident {
                type Error = String;
                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    #cbor_ident::try_from(&value)
                }
            }
            impl TryFrom<&Value> for #cbor_ident {
                type Error = String;
                fn try_from(value: &Value) -> Result<Self, Self::Error> {
                    #(#cbor_match_arms)*
                    Err(alloc::format!("failed to parse value as {}", #cbor_ident_name))
                }
            }

            impl TryFrom<#ident> for #cbor_ident {
                type Error = String;
                fn try_from(value: #ident) -> Result<Self, Self::Error> {
                    #cbor_ident::try_from(&value)
                }
            }
            impl TryFrom<&#ident> for #cbor_ident {
                type Error = String;
                fn try_from(value: &#ident) -> Result<Self, Self::Error> {
                    match value {
                        #(#json_to_cbor_arms)*
                    }
                }
            }

            impl TryFrom<#cbor_ident> for #ident {
                type Error = String;
                fn try_from(value: #cbor_ident) -> Result<Self, Self::Error> {
                    #ident::try_from(&value)
                }
            }
            impl TryFrom<&#cbor_ident> for #ident {
                type Error = String;
                fn try_from(value: &#cbor_ident) -> Result<Self, Self::Error> {
                    match value {
                        #(#cbor_to_json_arms)*
                    }
                }
            }
        }
    }

    /// Generate match arms for `TryFrom<&Value>` on the source enum.
    fn gen_match_arms(&self, ident: &Ident) -> Vec<TokenStream> {
        let mut arms = Vec::new();

        // Collect IntegerUnit arms into a single match block
        let mut int_unit_arms = Vec::new();

        for v in &self.variants {
            let var_ident = &v.ident;

            match &v.dispatch {
                ChoiceDispatch::IntegerUnit(val) => {
                    let val_i64 = *val;
                    int_unit_arms.push(quote! {
                        #val_i64 => return Ok(#ident::#var_ident),
                    });
                }
                _ => {
                    let inner_type = v
                        .inner_type
                        .as_ref()
                        .expect("non-unit variant must have inner type");

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
                            arms.push(quote! {
                                if let Value::Tag(#tag, _) = value {
                                    let mut buf = alloc::vec::Vec::new();
                                    ciborium::ser::into_writer(value, &mut buf)
                                        .map_err(|e| alloc::format!("failed to encode tag {}: {}", #tag, e))?;
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
                                other => {
                                    panic!("unsupported value type in EnumToChoice: {}", other)
                                }
                            };
                            arms.push(arm);
                        }
                        ChoiceDispatch::TryFrom => {
                            // Skip `other(String)` in integer-unit enums —
                            // handled by the integer match fallthrough
                            if !int_unit_arms.is_empty() && format!("{}", var_ident) == "other" {
                                // will be handled in integer match block
                            } else {
                                arms.push(quote! {
                                    if let Ok(v) = #inner_type::try_from(value) {
                                        return Ok(#ident::#var_ident(v));
                                    }
                                });
                            }
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
                        ChoiceDispatch::IntegerUnit(_) => unreachable!(),
                    }
                }
            }
        }

        // If there are IntegerUnit variants, emit a match block for integers
        if !int_unit_arms.is_empty() {
            // Check if there's an `other(String)` variant for unknown integers
            let other_variant = self
                .variants
                .iter()
                .find(|v| v.inner_type.is_some() && format!("{}", v.ident) == "other");
            let other_arm = if let Some(ov) = other_variant {
                let ov_ident = &ov.ident;
                quote! {
                    other => return Ok(#ident::#ov_ident(alloc::format!("{}", other))),
                }
            } else {
                quote! { _ => {} }
            };
            arms.insert(
                0,
                quote! {
                    if let Value::Integer(i) = value {
                        let val: i64 = (*i).try_into().map_err(|_|
                            alloc::format!("integer out of range")
                        )?;
                        match val {
                            #(#int_unit_arms)*
                            #other_arm
                        }
                    }
                },
            );
        }

        arms
    }

    /// Generate match arms for `TryFrom<&Value>` on the Cbor companion enum.
    fn gen_cbor_match_arms(&self, cbor_ident: &Ident) -> Vec<TokenStream> {
        let mut arms = Vec::new();

        for v in &self.variants {
            let var_ident = &v.ident;
            let inner_type = v.inner_type.as_ref();

            match &v.dispatch {
                ChoiceDispatch::IntegerUnit(_) => {
                    // Handled by gen_integer_unit_companion
                }
                ChoiceDispatch::TagWrapped(tag) => {
                    let inner = inner_type.expect("TagWrapped must have inner type");
                    // Re-serialize/deserialize to let ciborium handle Required<T, N>
                    let required_type = quote! { ciborium::tag::Required<#inner, #tag> };
                    arms.push(quote! {
                        if let Value::Tag(#tag, _) = value {
                            let mut buf = alloc::vec::Vec::new();
                            ciborium::ser::into_writer(value, &mut buf)
                                .map_err(|e| alloc::format!("failed to re-encode tag {}: {}", #tag, e))?;
                            let parsed: #required_type = ciborium::de::from_reader(buf.as_slice())
                                .map_err(|e| alloc::format!("failed to decode tag {}: {}", #tag, e))?;
                            return Ok(#cbor_ident::#var_ident(parsed));
                        }
                    });
                }
                ChoiceDispatch::Socket => {
                    arms.push(quote! {
                        if let Value::Tag(t, b) = value {
                            return Ok(#cbor_ident::Other(common::TupleCbor {
                                key: Value::Integer(ciborium::value::Integer::from(*t)),
                                value: *b.clone(),
                            }));
                        }
                    });
                }
                ChoiceDispatch::ValueType(vtype) => {
                    let arm = match vtype.as_str() {
                        "Text" => quote! {
                            if let Value::Text(s) = value {
                                return Ok(#cbor_ident::#var_ident(s.clone()));
                            }
                        },
                        "Bytes" => quote! {
                            if let Value::Bytes(_) = value {
                                return Ok(#cbor_ident::#var_ident(
                                    #inner_type::try_from(value)?
                                ));
                            }
                        },
                        "Integer" => quote! {
                            if let Value::Integer(i) = value {
                                let val: #inner_type = (*i).try_into().map_err(|_|
                                    alloc::format!("integer out of range for {}", stringify!(#inner_type))
                                )?;
                                return Ok(#cbor_ident::#var_ident(val));
                            }
                        },
                        "Bool" => quote! {
                            if let Value::Bool(b) = value {
                                return Ok(#cbor_ident::#var_ident(*b));
                            }
                        },
                        "Map" | "Array" => quote! {
                            if let Ok(v) = #inner_type::try_from(value) {
                                return Ok(#cbor_ident::#var_ident(v));
                            }
                        },
                        other => panic!("unsupported value type: {}", other),
                    };
                    arms.push(arm);
                }
                ChoiceDispatch::Tag(_) | ChoiceDispatch::TryFrom => {
                    arms.push(quote! {
                        if let Ok(v) = #inner_type::try_from(value) {
                            return Ok(#cbor_ident::#var_ident(v));
                        }
                    });
                }
            }
        }

        arms
    }

    /// Generate JSON enum -> CBOR enum conversion arms.
    fn gen_json_to_cbor_arms(&self, cbor_ident: &Ident) -> Vec<TokenStream> {
        let ident = &self.ident;
        let mut arms = Vec::new();

        for v in &self.variants {
            let var_ident = &v.ident;

            match &v.dispatch {
                ChoiceDispatch::TagWrapped(_) => {
                    arms.push(quote! {
                        #ident::#var_ident(inner) => {
                            Ok(#cbor_ident::#var_ident(ciborium::tag::Required(inner.clone())))
                        }
                    });
                }
                ChoiceDispatch::Socket => {
                    arms.push(quote! {
                        #ident::#var_ident(inner) => {
                            Ok(#cbor_ident::Other(common::TupleCbor::try_from(inner.clone())?))
                        }
                    });
                }
                _ => {
                    arms.push(quote! {
                        #ident::#var_ident(inner) => {
                            Ok(#cbor_ident::#var_ident(inner.clone()))
                        }
                    });
                }
            }
        }

        arms
    }

    /// Generate CBOR enum -> JSON enum conversion arms.
    fn gen_cbor_to_json_arms(&self, cbor_ident: &Ident) -> Vec<TokenStream> {
        let ident = &self.ident;
        let mut arms = Vec::new();

        for v in &self.variants {
            let var_ident = &v.ident;

            match &v.dispatch {
                ChoiceDispatch::TagWrapped(_) => {
                    arms.push(quote! {
                        #cbor_ident::#var_ident(ciborium::tag::Required(inner)) => {
                            Ok(#ident::#var_ident(inner.clone()))
                        }
                    });
                }
                ChoiceDispatch::Socket => {
                    let inner_type = &v.inner_type;
                    arms.push(quote! {
                        #cbor_ident::Other(inner) => {
                            Ok(#ident::#var_ident(#inner_type::try_from(inner.clone())?))
                        }
                    });
                }
                _ => {
                    arms.push(quote! {
                        #cbor_ident::#var_ident(inner) => {
                            Ok(#ident::#var_ident(inner.clone()))
                        }
                    });
                }
            }
        }

        arms
    }

    /// Generate a flat `Value(i8)` companion for integer-unit enums.
    fn gen_integer_unit_companion(&self) -> TokenStream {
        let ident = &self.ident;
        let cbor_ident = format_ident!("{}Cbor", ident);
        let cbor_ident_name = format!("{}", cbor_ident);

        // Generate associated constants
        let mut constants = Vec::new();
        let mut json_to_cbor_arms = Vec::new();
        let mut cbor_to_json_arms = Vec::new();
        let mut has_other = false;

        for v in &self.variants {
            let var_ident = &v.ident;

            match &v.dispatch {
                ChoiceDispatch::IntegerUnit(val) => {
                    let val_i8 = *val as i8;
                    let const_name =
                        format_ident!("{}", to_screaming_snake_case(&format!("{}", var_ident)));
                    constants.push(quote! {
                        /// Known value constant.
                        pub const #const_name: i8 = #val_i8;
                    });
                    json_to_cbor_arms.push(quote! {
                        #ident::#var_ident => Ok(#cbor_ident::Value(#cbor_ident::#const_name)),
                    });
                    cbor_to_json_arms.push(quote! {
                        #cbor_ident::#const_name => Ok(#ident::#var_ident),
                    });
                }
                _ => {
                    // `other(String)` fallback
                    has_other = true;
                    json_to_cbor_arms.push(quote! {
                        #ident::#var_ident(s) => Ok(#cbor_ident::Value(
                            s.parse::<i8>().map_err(|e| alloc::format!("{}", e))?
                        )),
                    });
                }
            }
        }

        let other_cbor_to_json = if has_other {
            let other_ident = self
                .variants
                .iter()
                .find(|v| !matches!(v.dispatch, ChoiceDispatch::IntegerUnit(_)))
                .map(|v| &v.ident);
            if let Some(oi) = other_ident {
                quote! {
                    other => Ok(#ident::#oi(alloc::format!("{}", other))),
                }
            } else {
                quote! {
                    other => Err(alloc::format!("unknown value: {}", other)),
                }
            }
        } else {
            quote! {
                other => Err(alloc::format!("unknown value: {}", other)),
            }
        };

        let comment = format!(
            "Auto-generated CBOR companion for [{}]. Carries the raw integer value.",
            ident
        );

        quote! {
            #[doc = #comment]
            #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
            #[serde(untagged)]
            #[allow(missing_docs)]
            pub enum #cbor_ident {
                Value(i8),
            }

            impl #cbor_ident {
                #(#constants)*
            }

            impl TryFrom<Value> for #cbor_ident {
                type Error = String;
                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    #cbor_ident::try_from(&value)
                }
            }
            impl TryFrom<&Value> for #cbor_ident {
                type Error = String;
                fn try_from(value: &Value) -> Result<Self, Self::Error> {
                    if let Value::Integer(i) = value {
                        let val: i8 = (*i).try_into().map_err(|_|
                            alloc::format!("integer out of range for {}", #cbor_ident_name)
                        )?;
                        return Ok(Self::Value(val));
                    }
                    Err(alloc::format!("failed to parse value as {}", #cbor_ident_name))
                }
            }

            impl TryFrom<#ident> for #cbor_ident {
                type Error = String;
                fn try_from(value: #ident) -> Result<Self, Self::Error> {
                    #cbor_ident::try_from(&value)
                }
            }
            impl TryFrom<&#ident> for #cbor_ident {
                type Error = String;
                fn try_from(value: &#ident) -> Result<Self, Self::Error> {
                    match value {
                        #(#json_to_cbor_arms)*
                    }
                }
            }

            impl TryFrom<#cbor_ident> for #ident {
                type Error = String;
                fn try_from(value: #cbor_ident) -> Result<Self, Self::Error> {
                    #ident::try_from(&value)
                }
            }
            impl TryFrom<&#cbor_ident> for #ident {
                type Error = String;
                fn try_from(value: &#cbor_ident) -> Result<Self, Self::Error> {
                    match value {
                        #cbor_ident::Value(v) => match *v {
                            #(#cbor_to_json_arms)*
                            #other_cbor_to_json
                        },
                    }
                }
            }
        }
    }
}

/// Convert CamelCase to SCREAMING_SNAKE_CASE.
fn to_screaming_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_uppercase());
    }
    result
}
