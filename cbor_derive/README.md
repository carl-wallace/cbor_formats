# cbor_derive

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

The cbor_derive crate provides procedural macros that can be used to derive support for marshaling data between a Rust 
struct and the vectors used by the [ciborium](https://crates.io/crates/ciborium) library to process CBOR-encoded 
maps and arrays. The `StructToMap` macro maps the fields of a struct onto a `Vec<(Value,Value)>`.
The `StructToArray` macro maps the fields of a struct onto a `Vec<Value>`. 

The mappings are relative to auto-generated structs that are named by appending `Cbor` to the name of 
the struct. This allows original struct definitions to be used with [serde-json](https://crates.io/crates/serde_json)
for JSON-encodings or with ciborium for maps that feature text-based indices with the alternative structures used with 
ciborium for maps with integer-based indices. The generated code includes TryFrom implementations to convert between the 
original structs and the related auto-generated structs.

### Example

The [CoRIM](https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03) specification defines the CorimMetaMap as follows:

```text
corim-meta-map = {
   &(signer: 0) => corim-signer-map
   ? &(signature-validity: 1) => validity-map
}
```

The [corim](../corim/index.html) library defines support for this structure as shown below. The `StructToMap`
derive macro is from the [cbor_derive](../cbor_derive/index.html) library and causes generation of a structure named `CorimMetaMapCbor`.
The Serialize and Deserialize macros are from [serde](https://crates.io/crates/serde) and are used with [serde-json](https://crates.io/crates/serde_json)
to provide JSON support. These macros also enable CBOR encodings that feature maps with text keys using [ciborium](https://crates.io/crates/ciborium).

The `cbor` derive helper attribute is used by the derive macros defined in [cbor_derive](../cbor_derive/index.html).
The `tag` attribute indicates the integer key used to represent the associated field in the map production.
The `value` attribute indicates the type of `Value` used to represent the data. The `cbor` attribute indicates that a CBOR-specific type should be used for the field (i.e., the type name
features a `Cbor` suffix when generating or decoding a CBOR-encoding).

```rust
use core::fmt;
use std::collections::BTreeMap;

use ciborium::{cbor, value::Value};
use serde::{Deserialize, Deserializer, Serialize, __private::size_hint};
use serde::__private::PhantomData;
use serde::de::{Visitor, MapAccess, Error as OtherError};
use serde::ser::Error;

use cbor_derive::StructToMap;
use common::TupleCbor;
use corim::maps::*;

#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
pub struct CorimMetaMap {
    #[cbor(tag = "0", value = "Map", cbor = "true")]
    pub signer: CorimSignerMap,
    #[cbor(tag = "1", value = "Map", cbor = "true")]
    pub validity: Option<ValidityMap>,
}
```

The following snip shows use of the `CorimMetaMap` and `CorimMetaMapCbor` structures per the above definition. The encoded value used
in this snip is from a test included in the [veraison/corim](https://github.com/veraison/corim) repo.

```rust
use ciborium::tag::Required;
use ciborium::ser::into_writer;
use ciborium::de::from_reader;
use hex_literal::hex;

use common::TimeCbor;
use common::TaggedUriTypeCbor;
use corim::choices::EntityNameTypeChoice;
use corim::maps::*;


// {0: {0: "ACME Ltd.", 1: 32("https://acme.example")}, 1: {0: 1(1601424000), 1: 1(1632960000)}}
let enc_meta = hex!("a200a2006941434d45204c74642e01d8207468747470733a2f2f61636d652e6578616d706c6501a200c11a5f73ca8001c11a6154fe00");

// decode the CBOR-encode example using the generated CorimMetaMapCbor structure
let dec: CorimMetaMapCbor = from_reader(enc_meta.to_vec().as_slice()).unwrap();

// re-encode the value and compare with the expected value
let mut encoded_token = vec![];
let _ = into_writer(&dec, &mut encoded_token);
assert_eq!(encoded_token, enc_meta);

// compare decoded result with expectations
match &dec.signer.entity_name {
    EntityNameTypeChoice::Text(v) => assert_eq!(*v, "ACME Ltd.".to_string()),
};
match &dec.signer.reg_id {
    Some(TaggedUriTypeCbor::U(v)) => assert_eq!(v.0, "https://acme.example".to_string()),
    None => panic!(),
};
match &dec.validity {
    Some(v) => {
        match v.not_before {
            Some(TimeCbor::T(t)) => assert_eq!(t.0, 1601424000),
            None => panic!(),
        }
        assert_eq!(v.not_after, TimeCbor::T(Required(1632960000)))
    }
    None => panic!(),
};

// convert to JSON-friendly struct, encode as JSON, and decode from JSON
let meta_j: CorimMetaMap = dec.try_into().unwrap();
let json = serde_json::to_string(&meta_j).unwrap();
println!("JSON: {}", json);
let dec_meta_j: CorimMetaMap = serde_json::from_str(json.as_str()).unwrap();

// encode as CBOR (with textual map keys)
let mut enc_with_text_keys = vec![];
let _ = into_writer(&dec_meta_j, &mut enc_with_text_keys);
println!(
    "CBOR with text map keys: {:?}",
    std::str::from_utf8(&subtle_encoding::hex::encode_upper(enc_with_text_keys.as_slice())).unwrap()
);

// convert to CBOR-friendly struct, encode as CBOR (with integer map keys), then compare with expected
let roundtrip: CorimMetaMapCbor = dec_meta_j.try_into().unwrap();
let mut actual = vec![];
let _ = into_writer(&roundtrip, &mut actual);
println!(
    "CBOR with integer map keys: {:?}",
    std::str::from_utf8(&subtle_encoding::hex::encode_upper(actual.as_slice())).unwrap()
);
assert_eq!(enc_meta.to_vec(), actual);

// scratch build an instance
let scratch = CorimMetaMapCbor {
    signer: CorimSignerMapCbor {
        entity_name: EntityNameTypeChoice::Text("ACME Ltd.".to_string()),
        reg_id: Some(TaggedUriTypeCbor::U(Required(
            "https://acme.example".to_string(),
        ))),
    },
    validity: Some(ValidityMapCbor {
        not_before: Some(TimeCbor::T(Required(1601424000))),
        not_after: TimeCbor::T(Required(1632960000)),
    }),
};
let mut scratch_actual = vec![];
let _ = into_writer(&scratch, &mut scratch_actual);
assert_eq!(enc_meta.to_vec(), scratch_actual);
```

Output from the above snip is as below.

```text
JSON: {"signer":{"entity_name":"ACME Ltd.","reg_id":"https://acme.example"},"validity":{"not_before":1601424000,"not_after":1632960000}}
CBOR with text map keys: "A2667369676E6572A26B656E746974795F6E616D656941434D45204C74642E667265675F69647468747470733A2F2F61636D652E6578616D706C656876616C6964697479A26A6E6F745F6265666F72651A5F73CA80696E6F745F61667465721A6154FE00"
CBOR with integer map keys: "A200A2006941434D45204C74642E01D8207468747470733A2F2F61636D652E6578616D706C6501A200C11A5F73CA8001C11A6154FE00"
```

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

## Minimum Supported Rust Version

This crate requires **Rust 1.63** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a minor
version bump.

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.63+-blue.svg

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[RFC 5280]: https://datatracker.ietf.org/doc/html/rfc5280
[RFC 5937]: https://datatracker.ietf.org/doc/html/rfc5937
