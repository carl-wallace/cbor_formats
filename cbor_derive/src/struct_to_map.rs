//! Code supporting StructToArray procedural macro

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Ident, Lifetime};

use crate::cbor_derive_utils::{extract_type, is_option, is_option_vec, is_vec};
use crate::default_lifetime;
use crate::field::StructField;

/// Derive the `StructToMap` trait for a struct
pub(crate) struct DeriveStructToMap {
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

impl DeriveStructToMap {
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

        let comment = format!("Supports CBOR encoding/decoding of the corresponding map type, which is described in [{}]", self.ident);

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
            #[derive(Clone, Debug, PartialEq)]
            pub struct #sname {
                #fields
            }
        };
        struct_def.to_tokens(&mut self.alt_struct);
    }

    /// Lower the derived output into a [`TokenStream`].
    pub fn to_tokens(&self) -> TokenStream {
        let ident2 = &self.ident;
        let alt_struct_name = format!("{}Cbor", self.ident);
        let ident = syn::Ident::new(&alt_struct_name, self.ident.span());
        let ident_name = format!("{}", ident);

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

        let mut vindices = vec![];
        for field in &self.fields {
            if let Some(v) = field.get_tag_number() {
                vindices.push(v)
            }
            decode_body.push(field.to_decode_tokens_map());
            encode_body.push(field.to_encode_tokens());
            to_cbor.push(field.to_try_from_tokens(true));
            from_cbor.push(field.to_try_from_tokens(false));
        }
        let alt_struct = &self.alt_struct;

        let tsindices = quote! {
            let indices = vec![#(#vindices)*];
        };

        quote! {
            macro_rules! cval {
                ($x:expr) => {
                    Value::from(val!($x))
                };
            }

            macro_rules! val {
                ($x:expr) => {
                    match cbor!($x) {
                        Ok(v) => v,
                        Err(e) => return Err(format!("CBOR parsing error: {:?}", e))
                    }
                };
            }

            #alt_struct

            impl TryFrom<#ident> for #ident2<#lt_params> {
                type Error = String;
                fn try_from(value: #ident) -> Result<Self, Self::Error> {
                    Ok(#ident2 {
                      #(#from_cbor)*
                    })
                }
            }
            impl TryFrom<#ident2> for #ident<#lt_params> {
                type Error = String;
                fn try_from(value: #ident2) -> Result<Self, Self::Error> {
                    Ok(#ident {
                      #(#to_cbor)*
                    })
                }
            }
            impl TryFrom<&#ident> for #ident2<#lt_params> {
                type Error = String;
                fn try_from(value: &#ident) -> Result<Self, Self::Error> {
                    Ok(#ident2 {
                      #(#from_cbor)*
                    })
                }
            }
            impl TryFrom<&#ident2> for #ident<#lt_params> {
                type Error = String;
                fn try_from(value: &#ident2) -> Result<Self, Self::Error> {
                    Ok(#ident {
                      #(#to_cbor)*
                    })
                }
            }

            impl TryFrom<Value> for #ident<#lt_params> {
                type Error = String;
                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    match &value {
                        Value::Map(s) => {
                          match Self::try_from(s.clone()) {
                            Ok(v) => Ok(v),
                            Err(e) => Err(format!("Failed to parse {} value from map. Error: {:?}", #ident_name, e))
                          }
                        }
                        _ => Err(format!("Expected map while parsing {} and found: {:?}", #ident_name, &value))
                    }
                }
            }
            impl TryFrom<&Value> for #ident<#lt_params> {
                type Error = String;
                fn try_from(value: &Value) -> Result<Self, Self::Error> {
                    match &value {
                        Value::Map(s) => {
                          match Self::try_from(s.clone()) {
                            Ok(v) => Ok(v),
                            Err(e) => Err(format!("Failed to parse {} value from map. Error: {:?}", #ident_name, e))
                          }
                        }
                        _ => Err(format!("Expected map while parsing {} and found: {:?}", #ident_name, &value))
                    }
                }
            }
            impl TryFrom<&#ident> for Vec<(Value, Value)> {
                type Error = String;

                fn try_from(value: &#ident) -> Result<Self, Self::Error> {
                    let mut v = vec![];
                    #(#decode_body)*
                    Ok(v)
                }
            }
            impl TryFrom<Vec<(Value, Value)>> for #ident<#lt_params> {
                type Error = String;

                fn try_from(value: Vec<(Value, Value)>) -> Result<Self, Self::Error> {
                    //let m = value.iter().map(|v|(v.0.as_integer().unwrap().try_into().unwrap(), v.1.clone())).collect::<BTreeMap<u32, Value>>();
                    let mut m : BTreeMap<i32, Value> = BTreeMap::new();
                    let mut vt : Vec<TupleCbor> = vec![];
                    #tsindices
                    for v in value {
                        let index : i32 = match v.0.as_integer() {
                            Some(i) => {
                                match i.try_into() {
                                    Ok(ival) => ival,
                                    Err(_) => return Err("".to_string())
                                }
                            }
                            None => return Err("".to_string())
                        };
                        // accumulate duplicates as TupleCbor items
                        if indices.contains(&index) && !m.contains_key(&index) {
                            m.insert(index, v.1.clone());
                        }
                        else {
                            let t = TupleCbor{ key: v.0, value: v.1 };
                            vt.push(t);
                        }
                    }

                    Ok(#ident {
                      #(#encode_body)*
                    })
                }
            }
            impl TryFrom<&Vec<(Value, Value)>> for #ident<#lt_params> {
                type Error = String;

                fn try_from(value: &Vec<(Value, Value)>) -> Result<Self, Self::Error> {
                    //let m = value.iter().map(|v|(v.0.as_integer().unwrap().try_into().unwrap(), v.1.clone())).collect::<BTreeMap<u32, Value>>();
                    let mut m : BTreeMap<i32, Value> = BTreeMap::new();
                    let mut vt : Vec<TupleCbor> = vec![];
                    #tsindices
                    for v in value {
                        let index : i32 = match v.0.as_integer() {
                            Some(i) => {
                                match i.try_into() {
                                    Ok(ival) => ival,
                                    Err(_) => return Err("".to_string())
                                }
                            }
                            None => return Err("".to_string())
                        };
                        // accumulate duplicates as TupleCbor items
                        if indices.contains(&index) && !m.contains_key(&index) {
                            m.insert(index, v.1.clone());
                        }
                        else {
                            let t = TupleCbor{ key: v.0.clone(), value: v.1.clone() };
                            vt.push(t);
                        }
                    }

                    Ok(#ident {
                      #(#encode_body)*
                    })
                }
            }

            impl serde::Serialize for #ident<#lt_params>  {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: serde::Serializer,
                {
                    let v :Vec<(Value, Value)> = match self.try_into() {
                        Ok(r) => r,
                        Err(e) => {return  Err(__S::Error::custom(e));}
                    };

                    let m = Value::Map(v);
                    m.serialize(__serializer)
                }
            }

            impl<'de> Deserialize<'de> for #ident<#lt_params>
            {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: Deserializer<'de>,
                {
                    struct MapVisitor {
                        marker: PhantomData<Value>,
                    }

                    impl<'de> Visitor<'de> for MapVisitor
                    {
                        type Value = Vec<(Value, Value)>;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
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
                            // todo - what about fields that are encoded as NULL?
                            values.retain(|(k,v)| *v != Value::Null);
                            if values.len() == 0 {
                                return Err(A::Error::custom("No non-Null values to serialize"));
                            }

                            Ok(values)
                        }
                    }

                    let visitor = MapVisitor {
                        marker: PhantomData,
                    };

                    match deserializer.deserialize_map(visitor) {
                        Ok(v) => {
                            match #ident::try_from(v) {
                                Ok(r) => Ok(r),
                                Err(e) => Err(D::Error::custom(e)),
                            }
                        }
                        Err(e) => Err(D::Error::custom(e)),
                    }
                }
            }
        }
    }
}
