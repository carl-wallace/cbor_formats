# cbor_formats

The cbor_formats project provides repositories that aim to simplify the implementation
of specifications that use [CBOR](https://datatracker.ietf.org/doc/html/rfc8949) encodings and that are defined 
using [CDDL](https://datatracker.ietf.org/doc/html/rfc8610) (and exists in lieu of a CDDL compiler for Rust). Support for several specifications is provided
along with a command line utility to generate and parse artifacts from these specifications. The following repositories are provided.

- [cbor_derive](./cbor_derive/index.html) provides procedural macros to enable the use of structures to generate maps and arrays
- [cfcli](./cfcli/index.html) provides a command line utility to exercise the other repositories
- [common](./common/index.html) provides a set of definitions that are shared by various specifications
- [corim](./corim/index.html) provides support for the [Concise Reference Integrity Manifest](https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03) specification
- [coswid](./coswid/index.html) provides support for the [Concise Software Identification Tags](https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22) specification
- [cots](./cots/index.html) provides support for the [Concise TA Stores](https://datatracker.ietf.org/doc/html/draft-wallace-rats-concise-ta-stores-01) specification
- [eat](./eat/index.html) provides support for the [Entity Attestation Token](https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat-17) specification
- [webauthn_asf](./webauthn_asf/index.html) provides support for attestation statement formats as defined in [Web Authentication: An API for accessing Public Key Credentials Level 2](https://www.w3.org/TR/webauthn-2/) specification

CBOR support is provided by the [ciborium](https://crates.io/crates/ciborium) library and JSON support is provided by the [serde-json](https://crates.io/crates/serde_json) library.
Both use [serde](https://crates.io/crates/serde) for serialization and deserialization support.

## Goals

The primary goal of the project is to enable the use of structures when working with maps defined with integer keys for CBOR.
Secondary goals include the ability to encode using CBOR or JSON and to avoid maintaining two sets of structure definitions.
Specifications developed in the [RATS working group](https://datatracker.ietf.org/wg/rats/about/) were the primary motivation.
Supporting additional productions common to these specifications, including arrays, OneOrMore<>, and extensions) became primary
goals due to necessity.

## General Approach

Ciborium supports generating maps with integer keys using instances of Vec<(Value, Value)>. The [cbor_derive](./cbor_derive/index.html)
project provides prodcedural macros that take a structure definition and generate a similar corresponding structure named with 
a `Cbor` suffix. The alternative structure features Serde-compatible serialization and deserilization methods that marshal
data between structure representation and Vec<(Value, Value)> representation. A set of TryFrom implementations is provided
as well to marshal between various Value representations and between the original JSON-friendly structure and the alternative structure.

### Example

The [CoRIM](https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03) specification defines the CorimMetaMap as follows:

```text
corim-meta-map = {
   &(signer: 0) => corim-signer-map
   ? &(signature-validity: 1) => validity-map
}
```

The [corim](./corim/index.html) library defines support for this structure as shown below. The `StructToMap`
derive macro is from the [cbor_derive](./cbor_derive/index.html) library and causes generation of a structure named `CorimMetaMapCbor`.
The Serialize and Deserialize macros are from [serde](https://crates.io/crates/serde) and are used with [serde-json](https://crates.io/crates/serde_json) 
to provide JSON support. These macros also enable CBOR encodings that feature maps with text keys using [ciborium](https://crates.io/crates/ciborium). 

The `cbor` derive helper attribute is used by the derive macros defined in [cbor_derive](./cbor_derive/index.html). 
The `tag` attribute indicates the integer key used to represent the associated field in the map production. 
The `value` attribute indicates the type of `Value` used to represent the data. The `cbor` attribute indicates that a CBOR-specific type should be used for the field (i.e., the type name 
features a `Cbor` suffix when generating or decoding a CBOR-encoding).

```rust
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
        buffer_to_hex(enc_with_text_keys.as_slice())
    );

    // convert to CBOR-friendly struct, encode as CBOR (with integer map keys), then compare with expected
    let roundtrip: CorimMetaMapCbor = dec_meta_j.try_into().unwrap();
    let mut actual = vec![];
    let _ = into_writer(&roundtrip, &mut actual);
    println!(
        "CBOR with integer map keys: {:?}",
        buffer_to_hex(actual.as_slice())
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

## Extensibility

It is common practice for specification authors to make many CDDL definitions extensible. In [coswid](https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22) 
many structures are extensible in two ways. For example, the `entity-entry` definition shows extensibility via the `global-attributes` group 
and via the `entity-extension` socket.

```text
   entity-entry = {
     entity-name => text,
     ? reg-id => any-uri,
     role => one-or-more<$role>,
     ? thumbprint => hash-entry,
     * $$entity-extension,
     global-attributes,
   }
   global-attributes = (
     ? lang => text,
     * any-attribute,
   )

   any-attribute = (
     label => one-or-more<text> / one-or-more<int>
   )

   label = text / int
```

Support for extensibility is currently provided by the `Tuple` and `TupleCbor` structures defined in [common](./common/index.html).
This approach does not allow for distinguishing between different adjacent extensibility mechanisms, as shown below. In this case,
the `lang` attribute from `global-attributes` has been included in the `EntityEntry` structure directly and all fields
added via `global-attributes` or `entity-extension` are accumulated in the `other` field (with only fields labeled using
an integer key supported per this definition).

```rust
#[derive(Clone, Debug, PartialEq, StructToMap, StructToOneOrMore, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EntityEntry {
    #[cbor(tag = "31", value = "Text")]
    pub entity_name: String,
    #[cbor(tag = "32", value = "Text")]
    pub reg_id: Option<Uri>,
    #[cbor(tag = "33")]
    pub role: OneOrMoreRole,
    #[cbor(tag = "34", cbor = "true")]
    pub thumbprint: Option<HashEntry>,
    //   * $$entity-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}
```

## Limitations

Known limitations include:

- No effort has been made to ensure support for all valid CDDL definitions or CBOR encodings. Scope was established using the set
of specifications included in the project today. As additional specifications are implemented, it is likely additional features 
will be required in the `cbor_derive` library. 

- Extensibility support is currently limited to fields with integer keys.

- Ideally structures could be used to represent groups (like the `filesystem-item` and similar groups in [coswid](https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22)).
An attempt was made to define a StructToGroup macro, but this was not successful. Fields from groups are simply copied into the 
target structures as a workaround.

- Some structures are not yet implemented

- The cfcli utility is largely incomplete (and JSON output needs work)

## TODO

- Error handling (especially in closures emitted by proc macros)
- More testing (including fuzzing)
- Finish support for currently supported specifications (additional fields/claims, extensibility in some structs, etc)
- Add additional specifications (and align with updates to currently supported specifications)
- Move more manually written code into macros (particularly for choices)
- Enforce not-empty, size limits, defaults, etc. (and add more sanity checks, in general)
- Improve JSON output (i.e., make sure aligns with specs, base64 encode binary fields, etc.)
- Improve extensibility support to allow for text or int keys
- Improve support for groups, if possible
- Finish cfcli utility
- Add COSE/JOSE support
- Setup CI for project
- Document remaining structs (possibly not until specs become final)

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of development. The specifications are also
in relatively early stages of development (and, correspondingly, so are other implementations used for interop testing).

## Rust Version

This crate was developed using **Rust 1.63**.

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
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[RFC 5280]: https://datatracker.ietf.org/doc/html/rfc5280
[RFC 5937]: https://datatracker.ietf.org/doc/html/rfc5937
