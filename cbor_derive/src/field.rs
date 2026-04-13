//! Structure field processing code adapted from the RustCrypto formats library.

use proc_macro_error2::abort;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident, Type};

use crate::{
    attributes::FieldAttrs,
    cbor_derive_utils::{extract_type, extract_type_from_option, is_option, is_option_vec, is_vec},
};

pub type TagNumber = i32;

/// "IR" for a field of a derived `StructToMap` or `StructToArray`.
pub(super) struct StructField {
    /// Variant name.
    pub(super) ident: Ident,

    /// Field-level attributes.
    pub(super) attrs: FieldAttrs,

    /// Field type
    pub(super) field_type: Type,
}

impl StructField {
    /// Create a new [`StructField`] from the input [`Field`].
    pub(super) fn new(field: &Field) -> Self {
        let ident = field.ident.as_ref().cloned().unwrap_or_else(|| {
            abort!(
                field,
                "no name on struct field i.e. tuple structs unsupported"
            )
        });

        let attrs = FieldAttrs::parse(&field.attrs);
        Self {
            ident,
            attrs,
            field_type: field.ty.clone(),
        }
    }

    /// Derive code for decoding a field of a sequence as an instance in a Vec<(Value, Value)
    /// construction named `v` for use with Ciborium.
    pub(super) fn to_decode_tokens_map(&self) -> TokenStream {
        let t = self.attrs.tag.unwrap_or(66666);
        let field_ident = &self.ident;

        if is_option(&self.field_type) {
            // where a field is optional, refrain from contributing to the vector when absent
            if 66666 == t {
                quote! {
                    match &value.#field_ident {
                        Some(value) => {
                            for i in value {
                                v.push((cval!(i.key), val!(i.value)));
                            }
                        },
                        None => {}
                    };
                }
            } else if "Bytes" == self.attrs.value {
                // Vec<u8> will be serialized as an array of bytes unless packed as a Value::Bytes.
                // Use the Bytes attribute value to signal this.
                quote! {
                    match &value.#field_ident {
                        Some(val) => v.push((cval!(#t), val!(Value::Bytes(val.clone())))),
                        None => {}
                    };
                }
            } else {
                quote! {
                    match &value.#field_ident {
                        Some(val) => v.push((cval!(#t), val!(value.#field_ident))),
                        None => {}
                    };
                }
            }
        } else if 66666 == t {
            quote! {
                {
                    for i in value {
                        v.push((cval!(i.key), val!(i.value)));
                    }
                }
            }
        } else if "Bytes" == self.attrs.value {
            // Vec<u8> will be serialized as an array of bytes unless packed as a Value::Bytes.
            // Use the Bytes attribute value to signal this.
            quote! {
                v.push((cval!(#t), val!(Value::Bytes(value.#field_ident.clone()))));
            }
        } else {
            quote! {
                v.push((cval!(#t), val!(value.#field_ident)));
            }
        }
    }

    /// Derive code for use in try_from implementations to marshal data from a struct to its
    /// related Cbor structure.
    pub(super) fn to_try_from_tokens(&self, to_cbor: bool) -> TokenStream {
        let field_ident = &self.ident;
        let mut field_type = &self.field_type;
        if let Some(ty_val) = extract_type_from_option(field_type) {
            field_type = ty_val;
        }

        let field_ident_str = format!("{}", field_ident);
        let field_type_str = match extract_type(field_type) {
            Some(t) => t,
            None => panic!("Failed to determine type for field {}", field_ident),
        };

        // get the type for use in constructing a <type>::try_from statement. This may be the inner
        // type of an Option<>, Vec<> or Option<Vec<>>.
        let mut try_from_type = Ident::new(&field_type_str, field_ident.span());

        if to_cbor {
            // if we are generating a try_from into CBOR types and the field is a CBOR type (i.e.,
            // it is a type derived using StructToMap or StructToArray) then fix up the type name.
            if self.attrs.cbor.is_some() {
                let x = match extract_type(field_type) {
                    Some(t) => format!("{}Cbor", t),
                    None => panic!("Failed to determine type for field {}", field_ident),
                };
                try_from_type = Ident::new(&x, field_ident.span())
            };
        }

        if self.attrs.cbor.is_some() {
            if is_option_vec(&self.field_type) {
                quote! {
                    #field_ident: match &value.#field_ident {
                        Some(o) => {
                            let items: Result<Vec<_>, String> = o.iter().map(|oo| #try_from_type::try_from(oo).map(|v| v.clone())).collect();
                            Some(items?)
                        }
                        None => None,
                    },
                }
            } else if is_option(&self.field_type) {
                quote! {
                    #field_ident: match &value.#field_ident {
                        Some(o) => Some(#try_from_type::try_from(o)?.clone()),
                        None => None,
                    },
                }
            } else if is_vec(&self.field_type) {
                quote! {
                    #field_ident: {
                        let items: Result<Vec<_>, String> = value.#field_ident.iter().map(|oo| #try_from_type::try_from(oo).map(|v| v.clone())).collect();
                        items?
                    },
                }
            } else {
                quote! {
                    #field_ident: match #try_from_type::try_from(&value.#field_ident) {
                        Ok(val) => val.clone(),
                        Err(_) => return Err(format!("Failed to to convert {} to {}", #field_ident_str, #field_type_str))
                    },
                }
            }
        } else if self.attrs.value.is_empty() || "Map" != self.attrs.value {
            quote! {
                #field_ident: value.#field_ident.clone(),
            }
        } else {
            quote! {
                #field_ident: match #try_from_type::try_from(&value.#field_ident) {
                    Ok(val) => val.clone(),
                    Err(_) => return Err(format!("Failed to to convert {} to {}", #field_ident_str, #field_type_str))
                },
            }
        }
    }

    pub(super) fn get_tag_number(&self) -> Option<TokenStream> {
        self.attrs.tag.map(|t| {
            quote! {
              #t,
            }
        })
    }

    /// Derive code for encoding a field of a sequence.
    pub(super) fn to_encode_tokens(&self) -> TokenStream {
        let field_ident = &self.ident;
        let field_ident_str = format!("{}", field_ident);
        let mut field_type = &self.field_type;
        let field_option_type = extract_type_from_option(field_type);
        let is_option = is_option(field_type);
        if let Some(ty_val) = field_option_type {
            field_type = ty_val;
        }
        let field_nested_type = extract_type(field_type);
        let field_adjusted_nested_type = if let Some(ttt) = field_nested_type {
            if self.attrs.cbor.is_some() {
                let x = format!("{}Cbor", ttt);
                Some(Ident::new(&x, field_ident.span()))
            } else {
                Some(Ident::new(&ttt, field_ident.span()))
            }
        } else {
            None
        };

        let f2 = if self.attrs.cbor.is_some() {
            let x = match extract_type(field_type) {
                Some(t) => format!("{}Cbor", t),
                None => panic!("Failed to determine type for field {}", field_ident),
            };

            Ident::new(&x, field_ident.span())
        } else {
            let ty_str = match extract_type(field_type) {
                Some(t) => t,
                None => panic!("Failed to determine type for field {}", field_ident),
            };
            Ident::new(&ty_str, field_ident.span())
        };

        let is_tuple_cbor = "TupleCbor" == format!("{}", f2);

        // assign a garbage tag that is not used when dealing with a tuple CBOR field
        let t = match self.attrs.tag {
            Some(t) => t,
            None => {
                if !is_tuple_cbor {
                    panic!(
                        "At present, only maps with integer indices are supported by StructToMap (default Ciborium support enables text indices)"
                    );
                }
                66666
            }
        };

        if is_tuple_cbor {
            quote! {
                #field_ident: match vt.is_empty() {
                    false => Some(vt),
                    true => None
                },
            }
        } else if "Bytes" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => match v.as_bytes() {
                            Some(b) => Some(b.clone()),
                            None => return Err(format!("Failed to process {} as bytes", #field_ident_str))
                        },
                        None => None
                    },
                }
            } else {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => match v.as_bytes() {
                            Some(b) => b.clone(),
                            None => return Err(format!("Failed to process {} as bytes", #field_ident_str))
                        },
                        None => return Err(format!("Failed to process {}", #field_ident_str))
                    },
                }
            }
        } else if "Map" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => match #f2::try_from(
                            match v.as_map() {
                            Some(val) => val.clone(),
                            None => return Err(format!("Failed to to process {} as a map: {:?}", #field_ident_str, v))
                        }
                    ) {
                        Ok(val) => Some(val),
                        Err(e) => return Err(format!("Failed to to process {} with error: {}", #field_ident_str, e))
                    },
                        None => None
                    },
                }
            } else {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => match #f2::try_from(
                            match v.as_map() {
                            Some(val) => val.clone(),
                            None => return Err(format!("Failed to to process {} as a map: {:?}", #field_ident_str, v))
                        }
                    ) {
                        Ok(val) => val,
                        Err(e) => return Err(format!("Failed to to process {} with error: {}", #field_ident_str, e))
                    },
                        None => return Err(format!("Missing required field {} (label {})", #field_ident_str, #t))
                    },
                }
            }
        } else if "Array" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => match v.as_array() {
                            Some(a) => {
                                let items: Result<Vec<_>, String> = a.into_iter().map(|v| #field_adjusted_nested_type::try_from(v.clone())).collect();
                                Some(items?)
                            },
                            None => return Err(format!("Failed to process {} as an array: {:?}", #field_ident_str, v))
                        },
                        None => None
                    },
                }
            } else {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => match v.as_array() {
                            Some(a) => {
                                let items: Result<Vec<_>, String> = a.into_iter().map(|v| #field_adjusted_nested_type::try_from(v.clone())).collect();
                                items?
                            },
                            None => return Err(format!("Failed to process {} as an array: {:?}", #field_ident_str, v))
                        },
                        None => return Err(format!("Missing required field {} (label {})", #field_ident_str, #t))
                    },
                }
            }
        } else if "Text" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => Some(
                            match v.as_text() {
                                Some(val) => val.to_string(),
                                None => return Err(format!("Failed to to process {} as text: {:?}", #field_ident_str, v))
                            }),
                        None => None,
                    },
                }
            } else {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => match v.as_text() {
                            Some(val) => val.to_string(),
                            None => return Err(format!("Failed to to process {} as text: {:?}", #field_ident_str, v))
                        },
                        None => return Err(format!("Missing required field {} (label {})", #field_ident_str, #t))
                    },
                }
            }
        } else if "Integer" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => Some(
                            match v.as_integer() {
                                Some(i) => {
                                    match i.try_into() {
                                        Ok(val) => val,
                                        Err(e) => return Err(format!("Failed to to process {} with error: {}", #field_ident_str, e))
                                    }
                                }
                                None => return Err(format!("Failed to to process {} as an integer", #field_ident_str))
                            }),
                        None => None,
                    },
                }
            } else {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => match v.as_integer() {
                            Some(i) => {
                                match i.try_into() {
                                    Ok(val) => val,
                                    Err(e) => return Err(format!("Failed to to process {} with error: {}", #field_ident_str, e))
                                }
                            }
                            None => return Err(format!("Failed to to process {} as an integer", #field_ident_str))
                        },
                        None => return Err(format!("Missing required field {} (label {})", #field_ident_str, #t))
                    },
                }
            }
        } else if "Bool" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => Some(
                            match v.as_bool() {
                                Some(val) => val,
                                None => return Err(format!("Failed to to process {} as a boolean", #field_ident_str))
                            }),
                        None => None,
                    },
                }
            } else {
                quote! {
                    #field_ident: match m.get(&#t) {
                        Some(v) => match v.as_bool() {
                            Some(val) => val,
                            None => return Err(format!("Failed to to process {} as a boolean", #field_ident_str))
                        },
                        None => return Err(format!("Missing required field {} (label {})", #field_ident_str, #t))
                    },
                }
            }
        } else if is_option {
            quote! {
                #field_ident: match m.get(&#t) {
                    Some(v) => {
                        match #f2::try_from(v) {
                            Ok(val) => Some(val),
                            Err(e) => return Err(format!("Failed to to process {} with error: {}", #field_ident_str, e))
                        }
                    },
                    None => None,
                },
            }
        } else {
            quote! {
                #field_ident: match m.get(&#t) {
                    Some(v) => match #f2::try_from(v) {
                        Ok(val) => val,
                        Err(e) => return Err(format!("Failed to to process {} with error: {}", #field_ident_str, e))
                    },
                    None => return Err(format!("Missing required field {} (label {})", #field_ident_str, #t))
                },
            }
        }
    }

    /// Derive code for decoding a field of a sequence.
    pub(super) fn to_decode_tokens_array(&self) -> TokenStream {
        let f = &self.ident;

        if is_option(&self.field_type) {
            if "Bytes" == self.attrs.value {
                quote! {
                    match &value.#f {
                        Some(val) => v.push(val!(Value::Bytes(val.clone()))),
                        None => {},
                    };
                }
            } else {
                quote! {
                    match &value.#f {
                        Some(val) => v.push(val!(val)),
                        None => {},
                    };
                }
            }
        } else if "Bytes" == self.attrs.value {
            quote! {
                v.push(val!(Value::Bytes(value.#f.clone())));
            }
        } else {
            quote! {
                v.push(val!(value.#f));
            }
        }
    }

    /// Derive code for encoding a field of a sequence.
    pub(super) fn to_encode_tokens_array(&self, index: usize) -> TokenStream {
        let field_ident = &self.ident;
        let field_type = &self.field_type;
        let field_nested_type = extract_type(field_type);
        let is_option = is_option(field_type);
        let field_adjusted_nested_type = if let Some(ttt) = field_nested_type {
            if self.attrs.cbor.is_some() {
                let x = format!("{}Cbor", ttt);
                Some(Ident::new(&x, field_ident.span()))
            } else {
                Some(Ident::new(&ttt, field_ident.span()))
            }
        } else {
            None
        };

        let f2 = if self.attrs.cbor.is_some() {
            let x = match extract_type(field_type) {
                Some(t) => format!("{}Cbor", t),
                None => panic!("Failed to determine type for field {}", field_ident),
            };
            Ident::new(&x, field_ident.span())
        } else {
            let ty_str = match extract_type(field_type) {
                Some(t) => t,
                None => panic!("Failed to determine type for field {}", field_ident),
            };
            Ident::new(&ty_str, field_ident.span())
        };

        let field_name = format!("{}", field_ident);

        if "Bytes" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match v.get(#index) {
                        Some(val) => {
                            match val.as_bytes() {
                                Some(val2) => Some(val2.clone()),
                                None => return Err(format!("failed to decode field {}", #field_name))
                            }
                        },
                        None => None
                    },
                }
            } else {
                quote! {
                    #field_ident: match v.get(#index)
                        .ok_or_else(|| format!("missing required field {} at index {}", #field_name, #index))?
                        .as_bytes() {
                        Some(val) => val.clone(),
                        None => return Err(format!("failed to decode field {}", #field_name))
                    },
                }
            }
        } else if "Map" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match v.get(#index) {
                        Some(val) => {
                            match #f2::try_from(
                                match val.as_map() {
                                    Some(val) => val.clone(),
                                    None => return Err(format!("failed to decode field {}", #field_name))
                                }
                            ){
                                Ok(val) => Some(val),
                                Err(_) => return Err(format!("failed to decode field {}", #field_name))
                            }
                        },
                        None => None
                    },
                }
            } else {
                quote! {
                    #field_ident: match #f2::try_from(
                        match v.get(#index)
                            .ok_or_else(|| format!("missing required field {} at index {}", #field_name, #index))?
                            .as_map() {
                            Some(val) => val.clone(),
                            None => return Err(format!("failed to decode field {}", #field_name))
                        }
                    ){
                        Ok(val) => val,
                        Err(_) => return Err(format!("failed to decode field {}", #field_name))
                    },
                }
            }
        } else if "Array" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match v.get(#index){
                        Some(val) => {
                            match val.as_array(){
                                Some(val) => {
                                    let items: Result<Vec<_>, String> = val.into_iter().map(|v| #field_adjusted_nested_type::try_from(v.clone())).collect();
                                    Some(items?)
                                },
                                None => return Err(format!("failed to decode field {}", #field_name))
                            }
                        },
                        None => None
                    },
                }
            } else {
                quote! {
                    #field_ident: match v.get(#index)
                        .ok_or_else(|| format!("missing required field {} at index {}", #field_name, #index))?
                        .as_array(){
                        Some(val) => {
                            let items: Result<Vec<_>, String> = val.into_iter().map(|v| #field_adjusted_nested_type::try_from(v.clone())).collect();
                            items?
                        },
                        None => return Err(format!("failed to decode field {}", #field_name))
                    },
                }
            }
        } else if "Text" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match v.get(#index) {
                        Some(v) => {
                            match v.as_text() {
                                Some(v) => Some(v.to_string()),
                                None => return Err(format!("failed to decode field {}", #field_name))
                            }
                        },
                        None => None
                    },
                }
            } else {
                quote! {
                    #field_ident: match v.get(#index)
                        .ok_or_else(|| format!("missing required field {} at index {}", #field_name, #index))?
                        .as_text() {
                        Some(v) => v.to_string(),
                        None => return Err(format!("failed to decode field {}", #field_name))
                    },
                }
            }
        } else if "Integer" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match v.get(#index) {
                        Some(i) => {
                            match i.as_integer() {
                                Some(i) => {
                                    match i.try_into() {
                                        Ok(val) => val,
                                        Err(_) => return Err(format!("failed to decode field {}", #field_name))
                                    }
                                },
                                None => return Err(format!("failed to decode field {}", #field_name))
                            }
                        },
                        None => None
                    },
                }
            } else {
                quote! {
                    #field_ident: match v.get(#index)
                        .ok_or_else(|| format!("missing required field {} at index {}", #field_name, #index))?
                        .as_integer() {
                        Some(i) => {
                            match i.try_into() {
                                Ok(val) => val,
                                Err(_) => return Err(format!("failed to decode field {}", #field_name))
                            }
                        },
                        None => return Err(format!("failed to decode field {}", #field_name))
                    },
                }
            }
        } else if "Bool" == self.attrs.value {
            if is_option {
                quote! {
                    #field_ident: match v.get(#index) {
                        Some(v) => {
                            match v.as_bool() {
                                Some(v) => v,
                                None => return Err(format!("failed to decode field {}", #field_name))
                            }
                        },
                        None => None
                    },
                }
            } else {
                quote! {
                    #field_ident: match v.get(#index)
                        .ok_or_else(|| format!("missing required field {} at index {}", #field_name, #index))?
                        .as_bool() {
                        Some(v) => v,
                        None => return Err(format!("failed to decode field {}", #field_name))
                    },
                }
            }
        } else if is_option {
            quote! {
                #field_ident: match v.get(#index) {
                        Some(val) => { match #f2::try_from(val.clone()) {
                            Ok(val2) => Some(val2),
                            Err(_) => return Err(format!("failed to decode field {}", #field_name))
                        }
                    },
                    None => None
                },
            }
        } else {
            quote! {
                #field_ident: match #f2::try_from(
                    v.get(#index)
                        .ok_or_else(|| format!("missing required field {} at index {}", #field_name, #index))?
                        .clone()
                ) {
                    Ok(v) => v,
                    Err(_) => return Err(format!("failed to decode field {}", #field_name))
                },
            }
        }
    }
}
