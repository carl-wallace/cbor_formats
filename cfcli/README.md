# cfcli

![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

CLI utility for creating, displaying, signing, and verifying CBOR-encoded RATS and SCITT objects
from the [cbor_formats](https://github.com/carl-wallace/cbor_formats) workspace.

## Status

tl;dr: not ready to use.

This is a work-in-progress implementation which is at an early stage of
development.

## Supported Formats

| Command | Format | Spec |
|---------|--------|------|
| `comid` | CoMID (Concise Module Identifier) | [draft-ietf-rats-corim](https://datatracker.ietf.org/doc/draft-ietf-rats-corim/) |
| `corim` | CoRIM (Concise Reference Integrity Manifest) | [draft-ietf-rats-corim](https://datatracker.ietf.org/doc/draft-ietf-rats-corim/) |
| `coswid` | CoSWID (Concise Software Identifier) | [RFC 9393](https://www.rfc-editor.org/rfc/rfc9393) |
| `cots` | CoTS (Concise Trust Anchor Store) | [draft-ietf-rats-concise-ta-stores](https://datatracker.ietf.org/doc/draft-ietf-rats-concise-ta-stores/) |
| `coserv` | CoSERV (Concise Service) | [draft-ietf-rats-coserv](https://datatracker.ietf.org/doc/draft-ietf-rats-coserv/) |
| `ear` | EAR (EAT Attestation Result) | [draft-ietf-rats-ear](https://datatracker.ietf.org/doc/draft-ietf-rats-ear/) |
| `eat` | EAT (Entity Attestation Token) | [RFC 9711](https://www.rfc-editor.org/rfc/rfc9711) |

## Usage

```
cfcli <COMMAND>

Commands:
  comid   Create and display CoMID objects
  corim   Create, display, sign, verify, and extract CoRIM objects
  coswid  Create and display CoSWID objects
  cots    Create and display CoTS objects
  coserv  Create, display, sign, verify, and extract CoSERV objects
  ear     Create, display, sign, verify, and extract EAR objects
  eat     Create and display EAT objects
```

### Creating objects from JSON templates

Each format supports a `create` subcommand that takes a JSON template and produces
a CBOR-encoded output file. Example templates are in `tests/data/`.

```sh
# Create a CoMID from a JSON template
cfcli comid create --template tests/data/comid_templates/comid-minimal.json --output-dir out/

# Create from all templates in a directory
cfcli comid create --template-dir tests/data/comid_templates/ --output-dir out/

# Create a CoRIM containing CoMID tags
cfcli corim create \
  --template tests/data/corim_templates/corim-minimal.json \
  --comid-dir tests/data/comid_templates/ \
  --output-dir out/

# Create an EAT
cfcli eat create --template tests/data/eat_templates/eat-minimal.json --output-dir out/

# Create an EAR
cfcli ear create --template tests/data/ear_templates/ear-minimal.json --output-dir out/
```

### Displaying CBOR-encoded objects

Each format supports a `display` subcommand that decodes and prints a CBOR file.

```sh
cfcli corim display --file-to-display out/corim-minimal.cbor
```

### Signing and verification

The `corim`, `coserv`, `ear`, and `eat` commands support signing, verifying, and
extracting payloads using COSE Sign1. Key files can be JWK (JSON) or COSE Key
(CBOR) format. Supported algorithms: ES256 (P-256), ES384 (P-384), EdDSA (Ed25519),
and ML-DSA-44/65/87 (with the `pqc` feature).

Example keys are in `tests/data/keys/` (both `.jwk` and `.cosekey` formats).

```sh
# Sign a CoRIM (using a JWK key)
cfcli corim sign \
  --corim-file out/corim-minimal.cbor \
  --key-file tests/data/keys/es256.jwk \
  --meta-file tests/data/corim_templates/meta-minimal.json \
  --output-dir out/

# Sign a CoRIM (using a COSE Key)
cfcli corim sign \
  --corim-file out/corim-minimal.cbor \
  --key-file tests/data/keys/es256.cosekey \
  --meta-file tests/data/corim_templates/meta-full.json \
  --output-dir out/

# Verify a signed CoRIM
cfcli corim verify \
  --signed-corim-file out/signed-corim-minimal.cbor \
  --key-file tests/data/keys/es256.jwk

# Extract payload and tags from a signed CoRIM
cfcli corim extract \
  --signed-corim-file out/signed-corim-minimal.cbor \
  --output-dir out/

# Sign and verify an EAR
cfcli ear sign \
  --ear-file out/ear-minimal.cbor \
  --key-file tests/data/keys/ed25519.cosekey \
  --output-dir out/

cfcli ear verify \
  --signed-ear-file out/signed-ear-minimal.cbor \
  --key-file tests/data/keys/ed25519.cosekey

# Sign and verify an EAT
cfcli eat sign \
  --eat-file out/eat-minimal.cbor \
  --key-file tests/data/keys/es384.jwk \
  --output-dir out/

cfcli eat verify \
  --signed-eat-file out/signed-eat-minimal.cbor \
  --key-file tests/data/keys/es384.jwk

# Sign and verify a CoSERV
cfcli coserv sign \
  --coserv-file out/coserv-query-refval.cbor \
  --key-file tests/data/keys/es256.cosekey \
  --output-dir out/

cfcli coserv verify \
  --signed-coserv-file out/signed-coserv-query-refval.cbor \
  --key-file tests/data/keys/es256.cosekey
```

### CoTS store creation

The `cots create-store` and `cots create-corim` subcommands are **not yet implemented**.
The `cots create` and `cots display` subcommands are available.

## Minimum Supported Rust Version

This crate requires **Rust 1.85** at a minimum.

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
[rustc-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg
