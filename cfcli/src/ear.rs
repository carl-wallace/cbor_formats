//! EAR (EAT Attestation Result) create, display, sign, verify, and extract operations.

use crate::key_utils::{algorithm_from_key, signer_from_key, verifier_from_key};
use base64ct::{Base64UrlUnpadded, Encoding};
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::value::Value;
use common::{BinaryOrNil, TextOrInt};
use cose::arrays::CoseSign1Cbor;
use cose::maps::HeaderMap;
use cose_crypto::jwk::algorithm_from_jwk;
use cose_crypto::sign::{CoseSign1Builder, verify_sign1};
use ear::maps::*;
use jose::header::JoseHeader;
use jose::jwk::Jwk;
use jose::jws::{JwsBuilder, verify_compact};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::utils::find_files;
use crate::{
    DisplaySubcommand, EarCommand, EarCreateSubcommand, EarExtractSubcommand, EarSignSubcommand,
    EarSubCommands, EarVerifySubcommand, SigningFormat,
};

/// Dispatch EAR subcommands.
pub fn ear_main(args: &EarCommand) {
    match &args.command {
        EarSubCommands::Create(c) => ear_create(c),
        EarSubCommands::Display(c) => ear_display(c),
        EarSubCommands::Sign(c) => ear_sign(c),
        EarSubCommands::Verify(c) => ear_verify(c),
        EarSubCommands::Extract(c) => ear_extract(c),
    }
}

/// Create CBOR-encoded EAR files from JSON templates.
fn ear_create(args: &EarCreateSubcommand) {
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
        ear_template_to_cbor(f, output_dir);
    }
}

/// Decode and display a CBOR-encoded EAR as JSON.
fn ear_display(args: &DisplaySubcommand) {
    if matches!(args.format, crate::args::DisplayFormat::Diag) {
        crate::cbor_diag::display_diag(&args.file_to_display);
        return;
    }
    let data = match fs::read(&args.file_to_display) {
        Ok(b) => b,
        Err(e) => {
            println!(
                "Unable to read EAR to display from {} with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let cbor: EarCbor = match from_reader(data.as_slice()) {
        Ok(c) => c,
        Err(_) => {
            let payload = crate::utils::unwrap_sign1_payload(&data);
            match from_reader(payload.as_slice()) {
                Ok(c) => c,
                Err(e) => {
                    println!(
                        "Unable to parse data read from {} as a CBOR-encoded EAR with error {}",
                        args.file_to_display, e
                    );
                    return;
                }
            }
        }
    };
    let json: Ear = match cbor.try_into() {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to convert CBOR EAR object to JSON EAR object for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };

    let json = match serde_json::to_string_pretty(&json) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to produce JSON EAR object for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };
    println!("{}", json);
}

/// Convert a single EAR JSON template to a CBOR-encoded file.
fn ear_template_to_cbor(template_file: &String, output_dir: &Path) {
    let data = match fs::read_to_string(template_file) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to read EAR template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let json: Ear = match serde_json::from_str(&data) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to parse EAR template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let cbor: EarCbor = match json.try_into() {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to convert JSON EAR object to CBOR EAR object for template {} with error: {}",
                template_file, e
            );
            return;
        }
    };

    let mut encoded_token = vec![];
    match into_writer(&cbor, &mut encoded_token) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Unable to generate CBOR-encoded EAR from template in {} with error {}",
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
        println!("Failed to write EAR file {:?}: {}", output_pathbuf, e);
    }
}

// ── COSE Sign1 tag #18 helpers ──

/// Read a signed EAR file: expects CBOR tag #18 wrapping a COSE_Sign1.
fn read_signed_ear(path: &str) -> Result<CoseSign1Cbor, String> {
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
            // Try without tag wrapping
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

/// Sign an unsigned EAR with a JWK key.
fn ear_sign(args: &EarSignSubcommand) {
    match args.format {
        SigningFormat::Cose => ear_sign_cose(args),
        SigningFormat::Jws => ear_sign_jws(args),
    }
}

/// Sign an unsigned EAR producing a COSE Sign1 wrapped in CBOR tag #18.
fn ear_sign_cose(args: &EarSignSubcommand) {
    // Read unsigned EAR
    let ear_bytes = match fs::read(&args.ear_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read EAR file {}: {}", args.ear_file, e);
            return;
        }
    };

    // Read JWK key
    let key_bytes = match fs::read(&args.key_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read key file {}: {}", args.key_file, e);
            return;
        }
    };

    // Get algorithm from key (JWK or COSE Key)
    let algorithm = match algorithm_from_key(&key_bytes) {
        Ok(a) => a,
        Err(e) => {
            println!("Failed to determine algorithm from key: {}", e);
            return;
        }
    };

    // Create signer
    let signer = match signer_from_key(&key_bytes) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to create signer from key: {}", e);
            return;
        }
    };

    // Build protected header: alg (label 1) + content_type (label 3)
    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(algorithm.to_i64())),
        criticality: None,
        content_type: Some(TextOrInt::Text(
            "application/eat-cwt; eat_profile=\"tag:ietf.org,2026:rats/ear#03\"".to_string(),
        )),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };

    // Sign
    let sign1 = match CoseSign1Builder::new()
        .payload(&ear_bytes)
        .protected(hdr)
        .sign(signer.as_ref())
    {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to sign EAR: {}", e);
            return;
        }
    };

    // Write output
    let ear_path = Path::new(&args.ear_file);
    let stem = ear_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("signed-ear");
    let output_path = Path::new(&args.output_dir).join(format!("signed-{stem}.cbor"));

    if let Err(e) = write_tagged_sign1(&sign1, &output_path) {
        println!("Failed to write signed EAR: {}", e);
        return;
    }

    println!("Signed EAR written to {:?}", output_path);
}

/// Sign an unsigned EAR producing a JWS compact serialization.
fn ear_sign_jws(args: &EarSignSubcommand) {
    let ear_bytes = match fs::read(&args.ear_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read EAR file {}: {}", args.ear_file, e);
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
    header.set_cty("application/eat-jwt; eat_profile=\"tag:ietf.org,2026:rats/ear#03\"");

    let compact = match JwsBuilder::new(header)
        .payload(&ear_bytes)
        .sign_compact(signer.as_ref())
    {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to sign EAR as JWS: {}", e);
            return;
        }
    };

    let ear_path = Path::new(&args.ear_file);
    let stem = ear_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("signed-ear");
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

    println!("Signed EAR (JWS) written to {:?}", output_path);
}

// ── Verify ──

/// Verify the signature on a signed EAR using a JWK key.
fn ear_verify(args: &EarVerifySubcommand) {
    match args.format {
        SigningFormat::Cose => ear_verify_cose(args),
        SigningFormat::Jws => ear_verify_jws(args),
    }
}

/// Verify a COSE Sign1 signed EAR.
fn ear_verify_cose(args: &EarVerifySubcommand) {
    let sign1 = match read_signed_ear(&args.signed_ear_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read signed EAR: {}", e);
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

/// Verify a JWS compact signed EAR.
fn ear_verify_jws(args: &EarVerifySubcommand) {
    let compact = match fs::read_to_string(&args.signed_ear_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read JWS file {}: {}", args.signed_ear_file, e);
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

/// Extract the payload from a signed EAR.
fn ear_extract(args: &EarExtractSubcommand) {
    match args.format {
        SigningFormat::Cose => ear_extract_cose(args),
        SigningFormat::Jws => ear_extract_jws(args),
    }
}

/// Extract the payload from a COSE Sign1 signed EAR.
fn ear_extract_cose(args: &EarExtractSubcommand) {
    let sign1 = match read_signed_ear(&args.signed_ear_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read signed EAR: {}", e);
            return;
        }
    };

    let payload = match &sign1.payload {
        BinaryOrNil::Binary(p) => p,
        BinaryOrNil::Nil => {
            println!("Signed EAR has no payload");
            return;
        }
    };

    let output_dir = Path::new(&args.output_dir);
    if !output_dir.exists() {
        if let Err(e) = fs::create_dir_all(output_dir) {
            println!("Failed to create output directory {:?}: {}", output_dir, e);
            return;
        }
    }

    let input_path = Path::new(&args.signed_ear_file);
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("ear");
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

    println!("Extracted EAR payload to {:?}", output_path);
}

/// Extract the payload from a JWS compact signed EAR (without verification).
fn ear_extract_jws(args: &EarExtractSubcommand) {
    let compact = match fs::read_to_string(&args.signed_ear_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read JWS file {}: {}", args.signed_ear_file, e);
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
    if !output_dir.exists() {
        if let Err(e) = fs::create_dir_all(output_dir) {
            println!("Failed to create output directory {:?}: {}", output_dir, e);
            return;
        }
    }

    let input_path = Path::new(&args.signed_ear_file);
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("ear");
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

    println!("Extracted EAR payload to {:?}", output_path);
}
