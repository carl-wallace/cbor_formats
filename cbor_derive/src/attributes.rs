//! Attribute processing code adapted from the RustCrypto formats library.

use crate::field::TagNumber;
use core::fmt::Debug;
use core::str::FromStr;
use proc_macro_error::abort;
use syn::LitStr;
use syn::MetaList;
use syn::MetaNameValue;
use syn::Path;
use syn::{self, Attribute, Lit, Meta, NestedMeta};

/// Attribute name.
pub(crate) const ATTR_NAME: &str = "cbor";

/// Parsing error message.
const PARSE_ERR_MSG: &str = "error parsing `cbor` attribute";

/// Field-level attributes.
#[derive(Clone, Debug, Default)]
pub(crate) struct FieldAttrs {
    /// Value of the `#[cbor(tag = "...")] attribute if provided. The value is used as the index
    /// of a field in a map.
    pub tag: Option<TagNumber>,

    /// String that indicates the type of ciborium Value to use when processing the associated field
    pub value: String,

    /// Boolean that indicates if the field has CBOR-specific serialization/deserialization
    /// behavior (i.e., if it uses StructToMap or StructToArray).
    pub cbor: Option<bool>,
}

impl FieldAttrs {
    /// Parse attributes from a struct field or enum variant.
    pub fn parse(attrs: &[Attribute]) -> Self {
        let mut tag = None;
        let mut value = None;
        let mut cbor = None;

        let mut parsed_attrs = Vec::new();
        AttrNameValue::from_attributes(attrs, &mut parsed_attrs);

        for attr in parsed_attrs {
            // `context_specific = "..."` attribute
            if let Some(tag_number) = attr.parse_value("tag") {
                if tag.is_some() {
                    abort!(attr.name, "duplicate cbor `tag` attribute");
                }

                tag = Some(tag_number);
                // `value` attribute
            } else if attr.parse_value::<String>("value").is_some() {
                if value.is_some() {
                    abort!(attr.name, "duplicate cbor `value` attribute");
                }

                value = Some(attr.value.value());
            } else if let Some(ty) = attr.parse_value("cbor") {
                if cbor.is_some() {
                    abort!(attr.name, "duplicate cbor `cbor` attribute: {}");
                }

                cbor = Some(ty);
            } else {
                abort!(
                    attr.name,
                    "unknown field-level `cbor` attribute \
                    (valid options are `tag`, `value`, `cbor`)",
                );
            }
        }

        Self {
            tag,
            value: value.unwrap_or_default(),
            cbor,
        }
    }
}

/// Name/value pair attribute.
struct AttrNameValue {
    /// Attribute name.
    pub name: Path,

    /// Attribute value.
    pub value: LitStr,
}

impl AttrNameValue {
    /// Parse a slice of attributes.
    pub fn from_attributes(attrs: &[Attribute], out: &mut Vec<Self>) {
        for attr in attrs {
            if !attr.path.is_ident(ATTR_NAME) {
                continue;
            }

            let nested = match attr.parse_meta().expect(PARSE_ERR_MSG) {
                Meta::List(MetaList { nested, .. }) => nested,
                other => abort!(other, "malformed `cbor` attribute"),
            };

            for meta in &nested {
                match meta {
                    NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                        path,
                        lit: Lit::Str(lit_str),
                        ..
                    })) => out.push(Self {
                        name: path.clone(),
                        value: lit_str.clone(),
                    }),
                    _ => abort!(nested, "malformed `cbor` attribute"),
                }
            }
        }
    }

    /// Parse an attribute value if the name matches the specified one.
    pub fn parse_value<T>(&self, name: &str) -> Option<T>
    where
        T: FromStr + Debug,
        T::Err: Debug,
    {
        if self.name.is_ident(name) {
            Some(
                self.value
                    .value()
                    .parse()
                    .unwrap_or_else(|_| abort!(self.name, "error parsing attribute")),
            )
        } else {
            None
        }
    }
}
