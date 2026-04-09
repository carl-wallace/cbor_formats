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
  eat     Create and display EAT objects
```

### Creating objects from JSON templates

Each format supports a `create` subcommand that takes a JSON template and produces
a CBOR-encoded output file.

```sh
# Create a CoMID from a JSON template
cfcli comid create --template comid.json --output-dir out/

# Create from all templates in a directory
cfcli comid create --template-dir templates/ --output-dir out/
```

### Displaying CBOR-encoded objects

Each format supports a `display` subcommand that decodes and prints a CBOR file.

```sh
cfcli corim display --file-to-display unsigned-corim.cbor
```

### CoRIM signing and verification

The `corim` command supports signing, verifying, and extracting CoRIM payloads
using COSE Sign1 with JWK keys (EC P-256, P-384, and Ed25519).

```sh
# Sign a CoRIM
cfcli corim sign \
  --corim-file unsigned-corim.cbor \
  --key-file ec-p256.jwk \
  --meta-file meta.json \
  --output-dir out/

# Verify a signed CoRIM
cfcli corim verify \
  --signed-corim-file signed-corim.cbor \
  --key-file ec-p256.jwk

# Extract payload and tags from a signed CoRIM
cfcli corim extract \
  --signed-corim-file signed-corim.cbor \
  --output-dir out/
```

### CoSERV signing and verification

The `coserv` command supports signing, verifying, and extracting CoSERV payloads
using COSE Sign1 with JWK keys (EC P-256, P-384, and Ed25519).

```sh
# Sign a CoSERV
cfcli coserv sign \
  --coserv-file unsigned-coserv.cbor \
  --key-file ec-p256.jwk \
  --output-dir out/

# Verify a signed CoSERV
cfcli coserv verify \
  --signed-coserv-file signed-coserv.cbor \
  --key-file ec-p256.jwk

# Extract payload from a signed CoSERV
cfcli coserv extract \
  --signed-coserv-file signed-coserv.cbor \
  --output-dir out/
```

### CoTS store creation

The `cots` command has additional subcommands for building trust anchor stores
from X.509 certificates.

```sh
# Create a concise-ta-store-map from CA and TA certificates
cfcli cots create-store \
  --cas ca-certs/ \
  --tas ta-certs/ \
  --environment env.json \
  --language en \
  --output store.cbor

# Wrap a CoTS in a CoRIM
cfcli cots create-corim \
  --template corim.json \
  --cots cots.cbor \
  --output corim.cbor
```

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
