//! Code supporting StructToArray procedural macro

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Ident, Lifetime};

use crate::default_lifetime;
use crate::field::StructField;

/// Derive the `StructToMap` trait for a struct
pub(crate) struct DeriveStructToOneOrMore {
    /// Name of the sequence struct.
    ident: Ident,

    /// Lifetime of the struct.
    lifetime: Option<Lifetime>,

    /// Fields of the struct.
    fields: Vec<StructField>,

    /// Bound fields of a struct to be returned
    alt_struct: TokenStream,

    /// Name of alternative struct
    alt_struct_name: String,
}

impl DeriveStructToOneOrMore {
    /// Parse [`DeriveInput`].
    pub fn new(input: DeriveInput) -> Self {
        let data = match input.data {
            syn::Data::Struct(data) => data,
            _ => abort!(
                input.ident,
                "can't derive `StructToMap` on this type: only `struct` types are allowed",
            ),
        };

        let lifetime = input
            .generics
            .lifetimes()
            .next()
            .map(|lt| lt.lifetime.clone());

        // let type_attrs = TypeAttrs::parse(&input.attrs);

        let fields = data.fields.iter().map(StructField::new).collect();

        let mut state = Self {
            ident: input.ident,
            lifetime,
            fields,
            alt_struct: TokenStream::new(),
            alt_struct_name: String::new(),
        };

        state.derive_alt_struct();
        state
    }

    fn derive_alt_struct(&mut self) {
        self.alt_struct_name = format!("OneOrMore{}Cbor", self.ident);
        let sname = syn::Ident::new(&self.alt_struct_name, self.ident.span());
        let sname_base_str = format!("{}Cbor", self.ident);
        let sname_base = syn::Ident::new(&sname_base_str, self.ident.span());

        let name_str = format!("OneOrMore{}", self.ident);
        let name = syn::Ident::new(&name_str, self.ident.span());
        let name_base_str = format!("{}", self.ident);
        let name_base = syn::Ident::new(&name_base_str, self.ident.span());

        let struct_def = quote! {
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(untagged)]
            #[allow(missing_docs)]
            pub enum #sname {
                One(#sname_base),
                More(Vec<#sname_base>)
            }
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(untagged)]
            #[allow(missing_docs)]
            pub enum #name {
                One(#name_base),
                More(Vec<#name_base>)
            }
        };

        struct_def.to_tokens(&mut self.alt_struct);
    }

    /// Lower the derived output into a [`TokenStream`].
    pub fn to_tokens(&self) -> TokenStream {
        let sname = syn::Ident::new(&self.alt_struct_name, self.ident.span());
        let sname_base_str = format!("{}Cbor", self.ident);
        let sname_base = syn::Ident::new(&sname_base_str, self.ident.span());

        let name_str = format!("OneOrMore{}", self.ident);
        let name = syn::Ident::new(&name_str, self.ident.span());

        let lifetime = match self.lifetime {
            Some(ref lifetime) => quote!(#lifetime),
            None => default_lifetime(),
        };

        // Lifetime parameters
        let lt_params = self
            .lifetime
            .as_ref()
            .map(|_| lifetime.clone())
            .unwrap_or_default();

        let mut decode_body = Vec::new();
        let mut encode_body = Vec::new();
        let mut to_cbor = Vec::new();
        let mut from_cbor = Vec::new();

        for field in &self.fields {
            decode_body.push(field.to_decode_tokens_map());
            encode_body.push(field.to_encode_tokens());
            to_cbor.push(field.to_try_from_tokens(true));
            from_cbor.push(field.to_try_from_tokens(false));
        }
        let alt_struct = &self.alt_struct;

        quote! {
            #alt_struct

            impl TryFrom<#sname> for #name<#lt_params> {
                type Error = String;
                fn try_from(value: #sname) -> Result<Self, Self::Error> {
                    match value {
                        #sname::One(v) => {
                            Ok(Self::One(v.try_into().unwrap()))
                        }
                        #sname::More(v) => {
                            Ok(Self::More(v.iter().map(|m|m.try_into().unwrap()).collect()))
                        }
                    }
                }
            }
            impl TryFrom<&#sname> for #name<#lt_params> {
                type Error = String;
                fn try_from(value: &#sname) -> Result<Self, Self::Error> {
                    match value {
                        #sname::One(v) => {
                            Ok(Self::One(v.try_into().unwrap()))
                        }
                        #sname::More(v) => {
                            Ok(Self::More(v.iter().map(|m|m.try_into().unwrap()).collect()))
                        }
                    }
                }
            }

            impl TryFrom<Value> for #sname<#lt_params> {
                type Error = String;
                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    match value {
                        Value::Map(m) => Ok(Self::One(#sname_base::try_from(m.to_vec()).unwrap())),
                        Value::Array(a) => {
                            Ok(Self::More(a.iter().map(|v|#sname_base::try_from(v).unwrap()).collect()))
                        },
                        _ => Err("".to_string()),
                    }
                }
            }
            impl TryFrom<&Value> for #sname<#lt_params> {
                type Error = String;
                fn try_from(value: &Value) -> Result<Self, Self::Error> {
                    match value {
                        Value::Map(m) => Ok(Self::One(#sname_base::try_from(m.to_vec()).unwrap())),
                        Value::Array(a) => {
                            Ok(Self::More(a.iter().map(|v|#sname_base::try_from(v).unwrap()).collect()))
                        },
                        _ => Err("".to_string()),
                    }
                }
            }
            impl TryFrom<#name> for #sname<#lt_params> {
                type Error = String;
                fn try_from(value: #name) -> Result<Self, Self::Error> {
                    match value {
                        #name::One(v) => {
                            Ok(Self::One(v.try_into().unwrap()))
                        }
                        #name::More(v) => {
                            Ok(Self::More(v.iter().map(|m|m.try_into().unwrap()).collect()))
                        }
                    }
                }
            }
            impl TryFrom<&#name> for #sname<#lt_params> {
                type Error = String;
                fn try_from(value: &#name) -> Result<Self, Self::Error> {
                    match value {
                        #name::One(v) => {
                            Ok(Self::One(v.try_into().unwrap()))
                        }
                        #name::More(v) => {
                            Ok(Self::More(v.iter().map(|m|m.try_into().unwrap()).collect()))
                        }
                    }
                }
            }
        }
    }
}
