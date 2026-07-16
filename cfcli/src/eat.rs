//! EAT (Entity Attestation Token) create, display, sign, verify, and extract operations.

use std::{fs, fs::File, io::Write, path::Path};

use ciborium::{de::from_reader, ser::into_writer, value::Value};

use base64ct::{Base64UrlUnpadded, Encoding};

use common::{BinaryOrNil, TextOrInt};
use cose::{arrays::CoseSign1Cbor, maps::HeaderMap};
use cose_crypto::{
    jwk::algorithm_from_jwk,
    sign::{CoseSign1Builder, verify_sign1},
};
use eat::maps::{ClaimsSetClaims, ClaimsSetClaimsCbor};
use jose::{
    header::JoseHeader,
    jwk::Jwk,
    jws::{JwsBuilder, verify_compact},
};

use crate::{
    DisplaySubcommand, EatCommand, EatCreateSubcommand, EatExtractSubcommand, EatSignSubcommand,
    EatSubCommands, EatVerifySubcommand, SigningFormat,
    key_utils::{algorithm_from_key, signer_from_key, verifier_from_key},
    utils::find_files,
};

/// Dispatch EAT subcommands.
pub fn eat_main(args: &EatCommand) {
    match &args.command {
        EatSubCommands::Create(c) => eat_create(c),
        EatSubCommands::Display(c) => eat_display(c),
        EatSubCommands::Sign(c) => eat_sign(c),
        EatSubCommands::Verify(c) => eat_verify(c),
        EatSubCommands::Extract(c) => eat_extract(c),
    }
}

/// Create CBOR-encoded EAT files from JSON templates.
fn eat_create(args: &EatCreateSubcommand) {
    if args.template.is_none() && args.template_dir.as_ref().is_none_or(|d| d.is_empty()) {
        println!("No templates supplied");
        return;
    }

    let mut files = vec![];
    if let Some(f) = &args.template {
        files.push(f.clone());
    }

    if let Some(f) = args.template_dir.as_ref() {
        find_files(f, "json", &mut files)
    }

    let output_dir = Path::new(&args.output_dir);

    for f in &files {
        eat_template_to_cbor(f, output_dir);
    }
}

/// Decode and display a CBOR-encoded EAT as JSON.
fn eat_display(args: &DisplaySubcommand) {
    if matches!(args.format, crate::args::DisplayFormat::Diag) {
        crate::cbor_diag::display_diag(&args.file_to_display);
        return;
    }
    let data = match fs::read(&args.file_to_display) {
        Ok(b) => b,
        Err(e) => {
            println!(
                "Unable to read EAT to display from {} with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let cbor: ClaimsSetClaimsCbor = match from_reader(data.as_slice()) {
        Ok(c) => c,
        Err(_) => {
            let payload = crate::utils::unwrap_sign1_payload(&data);
            match from_reader(payload.as_slice()) {
                Ok(c) => c,
                Err(e) => {
                    println!(
                        "Unable to parse data read from {} as a CBOR-encoded EAT with error {}",
                        args.file_to_display, e
                    );
                    return;
                }
            }
        }
    };
    let json: ClaimsSetClaims = match cbor.try_into() {
        Ok(s) => s,
        Err(_) => {
            println!(
                "Unable to convert CBOR EAT object to JSON EAT object for {}",
                args.file_to_display
            );
            return;
        }
    };

    let json = match serde_json::to_string(&json) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to produce JSON EAT object for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };
    println!("{}", json);
}

/// Convert a single EAT JSON template to a CBOR-encoded file.
fn eat_template_to_cbor(template_file: &String, output_dir: &Path) {
    let data = match fs::read_to_string(template_file) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to read EAT template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let json: ClaimsSetClaims = match serde_json::from_str(&data) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to parse EAT template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let cbor: ClaimsSetClaimsCbor = match json.try_into() {
        Ok(s) => s,
        Err(_) => {
            println!(
                "Unable to convert JSON EAT object to CBOR EAT object for template {}",
                template_file
            );
            return;
        }
    };

    let mut encoded_token = vec![];
    match into_writer(&cbor, &mut encoded_token) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Unable to generate CBOR-encoded EAT from template in {} with error {}",
                template_file, e
            )
        }
    };

    let template_path = Path::new(template_file);
    let template_filename = match template_path.file_name() {
        Some(s) => s,
        None => {
            println!("Failed to read file name from template {}", template_file);
            return;
        }
    };

    let output_path = Path::new(output_dir);
    let filename_str = match template_filename.to_str() {
        Some(s) => s,
        None => {
            println!("Failed to convert filename to string");
            return;
        }
    };
    let mut output_pathbuf = output_path.join(filename_str);
    output_pathbuf.set_extension("cbor");

    let mut output_file = match File::create(&output_pathbuf) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to create output file {:?}: {}", output_pathbuf, e);
            return;
        }
    };
    if let Err(e) = output_file.write_all(encoded_token.as_slice()) {
        println!("Failed to write EAT file {:?}: {}", output_pathbuf, e);
    }
}

// ── COSE Sign1 tag #18 helpers ──

/// Read a signed EAT file: expects CBOR tag #18 wrapping a COSE_Sign1.
fn read_signed_eat(path: &str) -> Result<CoseSign1Cbor, String> {
    let data = fs::read(path).map_err(|e| format!("failed to read {path}: {e}"))?;
    let value: Value = from_reader(data.as_slice())
        .map_err(|e| format!("failed to parse CBOR from {path}: {e}"))?;

    match value {
        Value::Tag(18, inner) => {
            let mut buf = Vec::new();
            into_writer(&*inner, &mut buf)
                .map_err(|e| format!("failed to serialize inner CBOR: {e}"))?;
            let sign1: CoseSign1Cbor = from_reader(buf.as_slice())
                .map_err(|e| format!("failed to deserialize CoseSign1Cbor: {e}"))?;
            Ok(sign1)
        }
        _ => {
            let sign1: CoseSign1Cbor = from_reader(data.as_slice())
                .map_err(|e| format!("failed to parse as CoseSign1Cbor: {e}"))?;
            Ok(sign1)
        }
    }
}

/// Write a CoseSign1Cbor wrapped in CBOR tag #18.
fn write_tagged_sign1(sign1: &CoseSign1Cbor, path: &Path) -> Result<(), String> {
    let mut sign1_bytes = Vec::new();
    into_writer(sign1, &mut sign1_bytes)
        .map_err(|e| format!("failed to serialize CoseSign1Cbor: {e}"))?;

    let inner: Value = from_reader(sign1_bytes.as_slice())
        .map_err(|e| format!("failed to round-trip CoseSign1Cbor as Value: {e}"))?;

    let tagged = Value::Tag(18, Box::new(inner));
    let mut output = Vec::new();
    into_writer(&tagged, &mut output)
        .map_err(|e| format!("failed to serialize tagged CoseSign1: {e}"))?;

    let mut file = File::create(path).map_err(|e| format!("failed to create {path:?}: {e}"))?;
    file.write_all(&output)
        .map_err(|e| format!("failed to write {path:?}: {e}"))?;
    Ok(())
}

// ── Sign ──

/// Sign an unsigned EAT with a JWK key.
fn eat_sign(args: &EatSignSubcommand) {
    match args.format {
        SigningFormat::Cose => eat_sign_cose(args),
        SigningFormat::Jws => eat_sign_jws(args),
    }
}

/// Sign an unsigned EAT producing a COSE Sign1 wrapped in CBOR tag #18.
fn eat_sign_cose(args: &EatSignSubcommand) {
    let eat_bytes = match fs::read(&args.eat_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read EAT file {}: {}", args.eat_file, e);
            return;
        }
    };

    let key_bytes = match fs::read(&args.key_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read key file {}: {}", args.key_file, e);
            return;
        }
    };

    let algorithm = match algorithm_from_key(&key_bytes) {
        Ok(a) => a,
        Err(e) => {
            println!("Failed to determine algorithm from key: {}", e);
            return;
        }
    };

    let signer = match signer_from_key(&key_bytes) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to create signer from key: {}", e);
            return;
        }
    };

    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(algorithm.to_i64())),
        criticality: None,
        content_type: Some(TextOrInt::Text("application/eat+cwt".to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };

    let sign1 = match CoseSign1Builder::new()
        .payload(&eat_bytes)
        .protected(hdr)
        .sign(signer.as_ref())
    {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to sign EAT: {}", e);
            return;
        }
    };

    let eat_path = Path::new(&args.eat_file);
    let stem = eat_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("signed-eat");
    let output_path = Path::new(&args.output_dir).join(format!("signed-{stem}.cbor"));

    if let Err(e) = write_tagged_sign1(&sign1, &output_path) {
        println!("Failed to write signed EAT: {}", e);
        return;
    }

    println!("Signed EAT written to {:?}", output_path);
}

/// Sign an unsigned EAT producing a JWS compact serialization.
fn eat_sign_jws(args: &EatSignSubcommand) {
    let eat_bytes = match fs::read(&args.eat_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read EAT file {}: {}", args.eat_file, e);
            return;
        }
    };

    let key_bytes = match fs::read(&args.key_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read key file {}: {}", args.key_file, e);
            return;
        }
    };

    let algorithm = match algorithm_from_jwk(&key_bytes) {
        Ok(a) => a,
        Err(e) => {
            println!("Failed to determine algorithm from JWK: {}", e);
            return;
        }
    };

    let jwk = match Jwk::from_json(&key_bytes) {
        Ok(j) => j,
        Err(e) => {
            println!("Failed to parse JWK: {}", e);
            return;
        }
    };

    let signer = match jwk.to_signer() {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to create signer from JWK: {}", e);
            return;
        }
    };

    let mut header = JoseHeader::new(algorithm.to_jose_alg());
    header.set_typ("JWT");
    header.set_cty("application/eat+jwt");

    let compact = match JwsBuilder::new(header)
        .payload(&eat_bytes)
        .sign_compact(signer.as_ref())
    {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to sign EAT as JWS: {}", e);
            return;
        }
    };

    let eat_path = Path::new(&args.eat_file);
    let stem = eat_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("signed-eat");
    let output_path = Path::new(&args.output_dir).join(format!("signed-{stem}.jws"));

    match File::create(&output_path) {
        Ok(mut f) => {
            if let Err(e) = f.write_all(compact.as_bytes()) {
                println!("Failed to write JWS: {}", e);
                return;
            }
        }
        Err(e) => {
            println!("Failed to create output file {:?}: {}", output_path, e);
            return;
        }
    }

    println!("Signed EAT (JWS) written to {:?}", output_path);
}

// ── Verify ──

/// Verify the signature on a signed EAT using a JWK key.
fn eat_verify(args: &EatVerifySubcommand) {
    match args.format {
        SigningFormat::Cose => eat_verify_cose(args),
        SigningFormat::Jws => eat_verify_jws(args),
    }
}

/// Verify a COSE Sign1 signed EAT.
fn eat_verify_cose(args: &EatVerifySubcommand) {
    let sign1 = match read_signed_eat(&args.signed_eat_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read signed EAT: {}", e);
            return;
        }
    };

    let key_bytes = match fs::read(&args.key_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read key file {}: {}", args.key_file, e);
            return;
        }
    };

    let verifier = match verifier_from_key(&key_bytes) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to create verifier from key: {}", e);
            return;
        }
    };

    match verify_sign1(&sign1, verifier.as_ref(), &[]) {
        Ok(()) => println!("Verification successful"),
        Err(e) => println!("Verification failed: {}", e),
    }
}

/// Verify a JWS compact signed EAT.
fn eat_verify_jws(args: &EatVerifySubcommand) {
    let compact = match fs::read_to_string(&args.signed_eat_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read JWS file {}: {}", args.signed_eat_file, e);
            return;
        }
    };

    let key_bytes = match fs::read(&args.key_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read key file {}: {}", args.key_file, e);
            return;
        }
    };

    let jwk = match Jwk::from_json(&key_bytes) {
        Ok(j) => j,
        Err(e) => {
            println!("Failed to parse JWK: {}", e);
            return;
        }
    };

    let verifier = match jwk.to_verifier() {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to create verifier from JWK: {}", e);
            return;
        }
    };

    match verify_compact(compact.trim(), verifier.as_ref()) {
        Ok(_) => println!("Verification successful"),
        Err(e) => println!("Verification failed: {}", e),
    }
}

// ── Extract ──

/// Extract the payload from a signed EAT.
fn eat_extract(args: &EatExtractSubcommand) {
    match args.format {
        SigningFormat::Cose => eat_extract_cose(args),
        SigningFormat::Jws => eat_extract_jws(args),
    }
}

/// Extract the payload from a COSE Sign1 signed EAT.
fn eat_extract_cose(args: &EatExtractSubcommand) {
    let sign1 = match read_signed_eat(&args.signed_eat_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read signed EAT: {}", e);
            return;
        }
    };

    let payload = match &sign1.payload {
        BinaryOrNil::Binary(p) => p,
        BinaryOrNil::Nil => {
            println!("Signed EAT has no payload");
            return;
        }
    };

    let output_dir = Path::new(&args.output_dir);
    if !output_dir.exists()
        && let Err(e) = fs::create_dir_all(output_dir)
    {
        println!("Failed to create output directory {:?}: {}", output_dir, e);
        return;
    }

    let input_path = Path::new(&args.signed_eat_file);
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("eat");
    let output_path = output_dir.join(format!("{stem}-payload.cbor"));

    match File::create(&output_path) {
        Ok(mut f) => {
            if let Err(e) = f.write_all(payload) {
                println!("Failed to write payload: {}", e);
                return;
            }
        }
        Err(e) => {
            println!("Failed to create output file {:?}: {}", output_path, e);
            return;
        }
    }

    println!("Extracted EAT payload to {:?}", output_path);
}

/// Extract the payload from a JWS compact signed EAT (without verification).
fn eat_extract_jws(args: &EatExtractSubcommand) {
    let compact = match fs::read_to_string(&args.signed_eat_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read JWS file {}: {}", args.signed_eat_file, e);
            return;
        }
    };

    let parts: Vec<&str> = compact.trim().splitn(3, '.').collect();
    if parts.len() != 3 {
        println!("Invalid JWS compact serialization");
        return;
    }

    let payload = match Base64UrlUnpadded::decode_vec(parts[1]) {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to decode JWS payload: {}", e);
            return;
        }
    };

    let output_dir = Path::new(&args.output_dir);
    if !output_dir.exists()
        && let Err(e) = fs::create_dir_all(output_dir)
    {
        println!("Failed to create output directory {:?}: {}", output_dir, e);
        return;
    }

    let input_path = Path::new(&args.signed_eat_file);
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("eat");
    let output_path = output_dir.join(format!("{stem}-payload.cbor"));

    match File::create(&output_path) {
        Ok(mut f) => {
            if let Err(e) = f.write_all(&payload) {
                println!("Failed to write payload: {}", e);
                return;
            }
        }
        Err(e) => {
            println!("Failed to create output file {:?}: {}", output_path, e);
            return;
        }
    }

    println!("Extracted EAT payload to {:?}", output_path);
}
