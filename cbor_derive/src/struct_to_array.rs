//! Code supporting StructToArray procedural macro

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Ident, Lifetime};

use crate::cbor_derive_utils::{extract_type, is_option, is_option_vec, is_vec};
use crate::default_lifetime;
use crate::field::StructField;

/// Derive the `StructToMap` trait for a struct
pub(crate) struct DeriveStructToArray {
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

impl DeriveStructToArray {
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
        self.alt_struct_name = format!("{}Cbor", self.ident);
        let sname = syn::Ident::new(&self.alt_struct_name, self.ident.span());

        let mut fields = TokenStream::new();

        let comment = format!("CBOR encoding/decoding of [{}]", self.ident);

        for (_field_count, field) in (self.fields).iter().enumerate() {
            let name = &field.ident;

            let ty = field.field_type.clone();

            let f = if Some(true) == field.attrs.cbor {
                let alt_ty = match extract_type(&ty) {
                    Some(t) => format!("{}Cbor", t),
                    None => panic!("Failed to determine type for field {}", name),
                };

                let ty2 = syn::Ident::new(&alt_ty, self.ident.span());
                if is_option_vec(&ty) {
                    quote! {
                         /// Defer decoded field
                         pub #name: Option<Vec<#ty2>>,
                    }
                } else if is_vec(&ty) {
                    quote! {
                         /// Defer decoded field
                         pub #name: Vec<#ty2>,
                    }
                } else if is_option(&ty) {
                    quote! {
                         /// Defer decoded field
                         pub #name: Option<#ty2>,
                    }
                } else {
                    quote! {
                         /// Defer decoded field
                         pub #name: #ty2,
                    }
                }
            } else {
                quote! {
                     /// Decoded field
                     pub #name: #ty,
                }
            };
            f.to_tokens(&mut fields);
        }

        let struct_def = quote! {
            #[doc = #comment]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, Debug, PartialEq)]
            pub struct #sname {
                #fields
            }
        };
        struct_def.to_tokens(&mut self.alt_struct);
    }

    /// Lower the derived output into a [`TokenStream`].
    pub fn to_tokens(&self) -> TokenStream {
        let orig_ident = &self.ident;
        let alt_struct_name = format!("{}Cbor", self.ident);
        let alt_ident = syn::Ident::new(&alt_struct_name, self.ident.span());
        let alt_ident_name = format!("{}", alt_ident);

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

        for (index, field) in self.fields.iter().enumerate() {
            decode_body.push(field.to_decode_tokens_array());
            encode_body.push(field.to_encode_tokens_array(index));
            to_cbor.push(field.to_try_from_tokens(true));
            from_cbor.push(field.to_try_from_tokens(false));
        }

        let alt_struct = &self.alt_struct;

        quote! {
            macro_rules! val {
                ($x:expr) => {
                    match cbor!($x) {
                        Ok(v) => v,
                        Err(e) => return Err(format!("CBOR parsing error: {:?}", e))
                    }
                };
            }

            #alt_struct

            impl TryFrom<#alt_ident> for #orig_ident<#lt_params> {
                type Error = String;
                fn try_from(value: #alt_ident) -> Result<Self, Self::Error> {
                    Ok(#orig_ident {
                      #(#from_cbor)*
                    })
                }
            }
            impl TryFrom<#orig_ident> for #alt_ident<#lt_params> {
                type Error = String;
                fn try_from(value: #orig_ident) -> Result<Self, Self::Error> {
                    Ok(#alt_ident {
                      #(#to_cbor)*
                    })
                }
            }
            impl TryFrom<&#alt_ident> for #orig_ident<#lt_params> {
                type Error = String;
                fn try_from(value: &#alt_ident) -> Result<Self, Self::Error> {
                    Ok(#orig_ident {
                      #(#from_cbor)*
                    })
                }
            }
            impl TryFrom<&#orig_ident> for #alt_ident<#lt_params> {
                type Error = String;
                fn try_from(value: &#orig_ident) -> Result<Self, Self::Error> {
                    Ok(#alt_ident {
                      #(#to_cbor)*
                    })
                }
            }
            impl TryFrom<Value> for #alt_ident<#lt_params> {
                type Error = String;
                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    match &value {
                        Value::Array(s) => match Self::try_from(s.clone()) {
                            Ok(val) => Ok(val),
                            Err(e) => Err(format!("Failed to parse {} value from array. Error: {:?}", #alt_ident_name, e))
                        },
                        _ => Err(format!("Expected array while parsing {} and found: {:?}", #alt_ident_name, &value))
                    }
                }
            }
            impl TryFrom<&Value> for #alt_ident<#lt_params> {
                type Error = String;
                fn try_from(value: &Value) -> Result<Self, Self::Error> {
                    match &value {
                        Value::Array(s) => match Self::try_from(s.clone()) {
                            Ok(val) => Ok(val),
                            Err(e) => Err(format!("Failed to parse {} value from array. Error: {:?}", #alt_ident_name, e))
                        },
                        _ => Err(format!("Expected array while parsing {} and found: {:?}", #alt_ident_name, &value))
                    }
                }
            }
            impl TryFrom<&#alt_ident> for Vec<Value> {
                type Error = String;

                fn try_from(value: &#alt_ident) -> Result<Self, Self::Error> {
                    let mut v = vec![];
                    #(#decode_body)*
                    Ok(v)
                }
            }
            impl TryFrom<Vec<Value>> for #alt_ident<#lt_params> {
                type Error = String;

                fn try_from(v: Vec<Value>) -> Result<Self, Self::Error> {
                    Ok(#alt_ident {
                      #(#encode_body)*
                    })
                }
            }

            impl serde::Serialize for #alt_ident<#lt_params>  {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: serde::Serializer,
                {
                    let mut v :Vec<Value> = match self.try_into() {
                        Ok(r) => r,
                        Err(e) => { return Err(__S::Error::custom(e)) },
                    };
                    // todo - what about fields that are encoded as NULL?
                    v.retain(|x| *x != Value::Null);
                    let m = Value::Array(v);
                    m.serialize(__serializer)
                }
            }

            impl<'de> Deserialize<'de> for #alt_ident<#lt_params> {
                fn deserialize<__D>(
                    deserializer: __D,
                ) -> serde::__private::Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    struct MapVisitor {
                        marker: PhantomData<Value>,
                    }
                    impl<'de> Visitor<'de> for MapVisitor
                    {
                        type Value = Vec<Value>;
                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("an array")
                        }
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::SeqAccess<'de>,
                        {
                            let i = __seq.size_hint().unwrap_or_else(|| 0);
                            let mut values = Vec::with_capacity(size_hint::cautious(__seq.size_hint()));
                            while let Some(value) = __seq.next_element()? {
                                values.push(value);
                            }
                            Ok(values)
                        }
                    }

                    let visitor = MapVisitor {
                        marker: PhantomData,
                    };
                    match deserializer.deserialize_seq(visitor) {
                        Ok(v) => {
                            match #alt_ident::try_from(v) {
                                Ok(r) => Ok(r),
                                Err(e) => Err(__D::Error::custom(e)),
                            }
                        }
                        Err(e) => Err(__D::Error::custom(e)),
                    }
                }
            }
        }
    }
}
