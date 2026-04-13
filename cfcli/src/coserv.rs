//! CoSERV (Concise Service) create, display, sign, verify, and extract operations.

use std::{fs, fs::File, io::Write, path::Path};

use ciborium::{de::from_reader, ser::into_writer, value::Value};

use common::{BinaryOrNil, TextOrInt};
use cose::{arrays::CoseSign1Cbor, maps::HeaderMap};
use cose_crypto::sign::{CoseSign1Builder, verify_sign1};
use coserv::{
    discovery::{CoservWellKnownInfoMap, CoservWellKnownInfoMapCbor},
    maps::{CoservMap, CoservMapCbor},
};

use crate::{
    CoservCommand, CoservCreateSubcommand, CoservExtractSubcommand, CoservSignSubcommand,
    CoservSubCommands, CoservVerifySubcommand, DisplaySubcommand,
    key_utils::{algorithm_from_key, signer_from_key, verifier_from_key},
    utils::find_files,
};

/// Dispatch CoSERV subcommands.
pub fn coserv_main(args: &CoservCommand) {
    match &args.command {
        CoservSubCommands::Create(c) => coserv_create(c),
        CoservSubCommands::Display(c) => coserv_display(c),
        CoservSubCommands::Sign(c) => coserv_sign(c),
        CoservSubCommands::Verify(c) => coserv_verify(c),
        CoservSubCommands::Extract(c) => coserv_extract(c),
        CoservSubCommands::CreateDiscovery(c) => coserv_create_discovery(c),
        CoservSubCommands::DisplayDiscovery(c) => coserv_display_discovery(c),
    }
}

/// Create CBOR-encoded CoSERV files from JSON templates.
fn coserv_create(args: &CoservCreateSubcommand) {
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
        coserv_template_to_cbor(f, output_dir);
    }
}

/// Decode and display a CBOR-encoded CoSERV as JSON.
fn coserv_display(args: &DisplaySubcommand) {
    if matches!(args.format, crate::args::DisplayFormat::Diag) {
        crate::cbor_diag::display_diag(&args.file_to_display);
        return;
    }
    let data = match fs::read(&args.file_to_display) {
        Ok(b) => b,
        Err(e) => {
            println!(
                "Unable to read CoSERV to display from {} with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let cbor: CoservMapCbor = match from_reader(data.as_slice()) {
        Ok(c) => c,
        Err(_) => {
            // Try unwrapping COSE Sign1 envelope
            let payload = crate::utils::unwrap_sign1_payload(&data);
            match from_reader(payload.as_slice()) {
                Ok(c) => c,
                Err(e) => {
                    println!(
                        "Unable to parse data read from {} as a CBOR-encoded CoSERV with error {}",
                        args.file_to_display, e
                    );
                    return;
                }
            }
        }
    };
    let json: CoservMap = match cbor.try_into() {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to convert CBOR CoSERV object to JSON CoSERV object for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };

    let json = match serde_json::to_string_pretty(&json) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to produce JSON CoSERV object for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };
    println!("{}", json);
}

/// Convert a single CoSERV JSON template to a CBOR-encoded file.
fn coserv_template_to_cbor(template_file: &String, output_dir: &Path) {
    let data = match fs::read_to_string(template_file) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to read CoSERV template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let json: CoservMap = match serde_json::from_str(&data) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to parse CoSERV template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let cbor: CoservMapCbor = match json.try_into() {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to convert JSON CoSERV object to CBOR CoSERV object for template {} with error: {}",
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
                "Unable to generate CBOR-encoded CoSERV from template in {} with error {}",
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
        println!("Failed to write CoSERV file {:?}: {}", output_pathbuf, e);
    }
}

// ── COSE Sign1 tag #18 helpers ──

/// Read a signed CoSERV file: expects CBOR tag #18 wrapping a COSE_Sign1.
fn read_signed_coserv(path: &str) -> Result<CoseSign1Cbor, String> {
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

/// Sign an unsigned CoSERV with a JWK key, producing a COSE Sign1 wrapped in CBOR tag #18.
fn coserv_sign(args: &CoservSignSubcommand) {
    // Read unsigned CoSERV
    let coserv_bytes = match fs::read(&args.coserv_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read CoSERV file {}: {}", args.coserv_file, e);
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
        content_type: Some(TextOrInt::Text("application/coserv+cbor".to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };

    // Sign
    let sign1 = match CoseSign1Builder::new()
        .payload(&coserv_bytes)
        .protected(hdr)
        .sign(signer.as_ref())
    {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to sign CoSERV: {}", e);
            return;
        }
    };

    // Write output
    let coserv_path = Path::new(&args.coserv_file);
    let stem = coserv_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("signed-coserv");
    let output_path = Path::new(&args.output_dir).join(format!("signed-{stem}.cbor"));

    if let Err(e) = write_tagged_sign1(&sign1, &output_path) {
        println!("Failed to write signed CoSERV: {}", e);
        return;
    }

    println!("Signed CoSERV written to {:?}", output_path);
}

// ── Verify ──

/// Verify the COSE Sign1 signature on a signed CoSERV using a JWK key.
fn coserv_verify(args: &CoservVerifySubcommand) {
    // Read signed CoSERV
    let sign1 = match read_signed_coserv(&args.signed_coserv_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read signed CoSERV: {}", e);
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

    // Create verifier
    let verifier = match verifier_from_key(&key_bytes) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to create verifier from key: {}", e);
            return;
        }
    };

    // Verify
    match verify_sign1(&sign1, verifier.as_ref(), &[]) {
        Ok(()) => println!("Verification successful"),
        Err(e) => println!("Verification failed: {}", e),
    }
}

// ── Extract ──

/// Extract the payload from a signed CoSERV and write it as an unsigned CoSERV file.
fn coserv_extract(args: &CoservExtractSubcommand) {
    // Read signed CoSERV
    let sign1 = match read_signed_coserv(&args.signed_coserv_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read signed CoSERV: {}", e);
            return;
        }
    };

    // Extract payload
    let payload = match &sign1.payload {
        BinaryOrNil::Binary(p) => p,
        BinaryOrNil::Nil => {
            println!("Signed CoSERV has no payload");
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

    // Derive output filename from input
    let input_path = Path::new(&args.signed_coserv_file);
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("coserv");
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

    println!("Extracted CoSERV payload to {:?}", output_path);
}

// ── Discovery ──

/// Create CBOR-encoded CoSERV discovery documents from JSON templates.
fn coserv_create_discovery(args: &CoservCreateSubcommand) {
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
        discovery_template_to_cbor(f, output_dir);
    }
}

/// Convert a single CoSERV discovery JSON template to a CBOR-encoded file.
fn discovery_template_to_cbor(template_file: &String, output_dir: &Path) {
    let data = match fs::read_to_string(template_file) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to read discovery template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let json: CoservWellKnownInfoMap = match serde_json::from_str(&data) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to parse discovery template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    if let Err(e) = json.validate() {
        println!(
            "Discovery template validation failed for {} with error: {}",
            template_file, e
        );
        return;
    }

    let cbor: CoservWellKnownInfoMapCbor = match (&json).try_into() {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to convert JSON discovery object to CBOR for template {} with error: {}",
                template_file, e
            );
            return;
        }
    };

    let mut encoded = vec![];
    match into_writer(&cbor, &mut encoded) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Unable to generate CBOR-encoded discovery document from template in {} with error {}",
                template_file, e
            );
            return;
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
    if let Err(e) = output_file.write_all(encoded.as_slice()) {
        println!(
            "Failed to write discovery document {:?}: {}",
            output_pathbuf, e
        );
    }
}

/// Decode and display a CBOR-encoded CoSERV discovery document as JSON.
fn coserv_display_discovery(args: &DisplaySubcommand) {
    if matches!(args.format, crate::args::DisplayFormat::Diag) {
        crate::cbor_diag::display_diag(&args.file_to_display);
        return;
    }
    let data = match fs::read(&args.file_to_display) {
        Ok(b) => b,
        Err(e) => {
            println!(
                "Unable to read discovery document from {} with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let cbor: CoservWellKnownInfoMapCbor = match from_reader(data.as_slice()) {
        Ok(c) => c,
        Err(e) => {
            println!(
                "Unable to parse data read from {} as a CBOR-encoded discovery document with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let json: CoservWellKnownInfoMap = match (&cbor).try_into() {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to convert CBOR discovery object to JSON for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };

    let json = match serde_json::to_string_pretty(&json) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to produce JSON discovery document for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };
    println!("{}", json);
}
