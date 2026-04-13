//! CoRIM (Concise Reference Integrity Manifest) create, display, sign, verify, and extract operations.

use crate::key_utils::{algorithm_from_key, signer_from_key, verifier_from_key};
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::value::Value;
use common::{TextOrInt, Tuple};
use corim::maps::*;
use cose::arrays::CoseSign1Cbor;
use cose::maps::HeaderMap;
use cose_crypto::sign::{CoseSign1Builder, verify_sign1};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::utils::find_files;
use crate::{
    CorimCommand, CorimCreateSubcommand, CorimExtractSubcommand, CorimSignSubcommand,
    CorimSubCommands, CorimVerifySubcommand, DisplaySubcommand,
};

/// Dispatch CoRIM subcommands.
pub fn corim_main(args: &CorimCommand) {
    match &args.command {
        CorimSubCommands::Create(c) => corim_create(c),
        CorimSubCommands::Display(c) => corim_display(c),
        CorimSubCommands::Sign(c) => corim_sign(c),
        CorimSubCommands::Verify(c) => corim_verify(c),
        CorimSubCommands::Extract(c) => corim_extract(c),
    }
}

/// Create unsigned CoRIM files from JSON templates and optional CoMID/CoSWID inputs.
fn corim_create(args: &CorimCreateSubcommand) {
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
        corim_template_to_cbor(f, output_dir);
    }
}

/// Decode and display a CBOR-encoded CoRIM as JSON.
fn corim_display(args: &DisplaySubcommand) {
    let data = match fs::read(&args.file_to_display) {
        Ok(b) => b,
        Err(e) => {
            println!(
                "Unable to read CoMID to display from {} with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let comid_cbor: CorimMapCbor = match from_reader(data.as_slice()) {
        Ok(c) => c,
        Err(e) => {
            println!(
                "Unable to parse data read from {} as a CBOR-encoded CoMID with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let comid_json: CorimMap = match comid_cbor.try_into() {
        Ok(s) => s,
        Err(_) => {
            println!(
                "Unable to convert CBOR CoMID object to JSON CoMID object for {}",
                args.file_to_display
            );
            return;
        }
    };

    let json = match serde_json::to_string(&comid_json) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to produce JSON CoMID object for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };
    println!("{}", json);
}

/// Convert a single CoRIM JSON template to a CBOR-encoded file.
fn corim_template_to_cbor(template_file: &String, output_dir: &Path) {
    let data = match fs::read_to_string(template_file) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to read CoMID template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let comid_json: CorimMap = match serde_json::from_str(&data) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to parse CoMID template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let comid_cbor: CorimMapCbor = match comid_json.try_into() {
        Ok(s) => s,
        Err(_) => {
            println!(
                "Unable to convert JSON CoMID object to CBOR CoMID object for template {}",
                template_file
            );
            return;
        }
    };

    let mut encoded_token = vec![];
    match into_writer(&comid_cbor, &mut encoded_token) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Unable to generate CBOR-encoded CoMID from template in {} with error {}",
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
        println!("Failed to write CoRIM file {:?}: {}", output_pathbuf, e);
    }
}

// ── Meta JSON parsing (cocli-compatible format) ──

/// Intermediate struct for parsing cocli-compatible meta JSON.
#[derive(Debug, Deserialize)]
struct MetaJson {
    /// Signer information (name and optional URI).
    signer: MetaSignerJson,
    /// Optional validity period (not-before / not-after timestamps).
    validity: Option<MetaValidityJson>,
}

/// Signer identity for the CoRIM meta header.
#[derive(Debug, Deserialize)]
struct MetaSignerJson {
    /// Display name of the signer entity.
    name: String,
    /// Optional registration URI for the signer entity.
    uri: Option<String>,
}

/// Validity period with ISO 8601 timestamps.
#[derive(Debug, Deserialize)]
struct MetaValidityJson {
    /// Optional earliest validity time (ISO 8601, e.g. `2025-01-01T00:00:00Z`).
    #[serde(rename = "not-before")]
    not_before: Option<String>,
    /// Optional latest validity time (ISO 8601, e.g. `2026-01-01T00:00:00Z`).
    #[serde(rename = "not-after")]
    not_after: Option<String>,
}

/// Parse an ISO 8601 datetime string (`YYYY-MM-DDTHH:MM:SSZ`) to a Unix timestamp.
fn parse_time(s: &str) -> Result<i64, String> {
    // Parse ISO 8601 datetime to Unix timestamp.
    // Supports format: YYYY-MM-DDTHH:MM:SSZ
    let s = s.trim();
    if s.len() < 20 || !s.ends_with('Z') {
        return Err(format!("unsupported time format: {s}"));
    }
    let parts: Vec<&str> = s[..19].split('T').collect();
    if parts.len() != 2 {
        return Err(format!("unsupported time format: {s}"));
    }
    let date_parts: Vec<&str> = parts[0].split('-').collect();
    let time_parts: Vec<&str> = parts[1].split(':').collect();
    if date_parts.len() != 3 || time_parts.len() != 3 {
        return Err(format!("unsupported time format: {s}"));
    }
    let year: i64 = date_parts[0]
        .parse()
        .map_err(|_| format!("bad year: {s}"))?;
    let month: i64 = date_parts[1]
        .parse()
        .map_err(|_| format!("bad month: {s}"))?;
    let day: i64 = date_parts[2].parse().map_err(|_| format!("bad day: {s}"))?;
    let hour: i64 = time_parts[0]
        .parse()
        .map_err(|_| format!("bad hour: {s}"))?;
    let min: i64 = time_parts[1].parse().map_err(|_| format!("bad min: {s}"))?;
    let sec: i64 = time_parts[2].parse().map_err(|_| format!("bad sec: {s}"))?;

    if !(1970..=9999).contains(&year) {
        return Err(format!("year out of range: {year}"));
    }
    if !(1..=12).contains(&month) {
        return Err(format!("month out of range: {month}"));
    }
    if !(1..=31).contains(&day) {
        return Err(format!("day out of range: {day}"));
    }
    if !(0..=23).contains(&hour) {
        return Err(format!("hour out of range: {hour}"));
    }
    if !(0..=59).contains(&min) {
        return Err(format!("minute out of range: {min}"));
    }
    if !(0..=59).contains(&sec) {
        return Err(format!("second out of range: {sec}"));
    }

    // Simple days-from-epoch calculation (no leap second handling)
    let mut days = 0i64;
    for y in 1970..year {
        days += if is_leap_year(y) { 366 } else { 365 };
    }
    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let max_day = if month == 2 && is_leap_year(year) {
        29
    } else {
        month_days[(month - 1) as usize]
    };
    if day > max_day as i64 {
        return Err(format!("day {day} out of range for month {month}"));
    }
    for m in 1..month {
        days += month_days[(m - 1) as usize] as i64;
        if m == 2 && is_leap_year(year) {
            days += 1;
        }
    }
    days += day - 1;
    Ok(days * 86400 + hour * 3600 + min * 60 + sec)
}

/// Returns true if the given year is a leap year.
fn is_leap_year(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

/// Convert a [`MetaJson`] into a CBOR-encoded [`CorimMetaMapCbor`] for the protected header.
fn meta_json_to_cbor(meta: &MetaJson) -> Result<CorimMetaMapCbor, String> {
    use corim::choices::EntityNameTypeChoice;

    let signer = CorimSignerMap {
        entity_name: EntityNameTypeChoice::Text(meta.signer.name.clone()),
        reg_id: meta.signer.uri.clone(),
    };

    let validity = match &meta.validity {
        Some(v) => {
            let not_before = match &v.not_before {
                Some(s) => Some(parse_time(s)?),
                None => None,
            };
            let not_after = match &v.not_after {
                Some(s) => Some(parse_time(s)?),
                None => None,
            };
            Some(ValidityMap {
                not_before,
                not_after,
            })
        }
        None => None,
    };

    let meta_map = CorimMetaMap { signer, validity };
    CorimMetaMapCbor::try_from(meta_map)
}

// ── CoRIM tag 501 helper ──

/// Ensure the CoRIM payload is wrapped in CBOR tag 501.
/// If already tagged, return the original bytes; otherwise wrap.
fn ensure_corim_tag(data: &[u8]) -> Result<Vec<u8>, String> {
    let value: Value = from_reader(data).map_err(|e| format!("failed to parse CoRIM CBOR: {e}"))?;
    match &value {
        Value::Tag(501, _) => Ok(data.to_vec()),
        _ => {
            let tagged = Value::Tag(501, Box::new(value));
            let mut buf = Vec::new();
            into_writer(&tagged, &mut buf)
                .map_err(|e| format!("failed to serialize tagged CoRIM: {e}"))?;
            Ok(buf)
        }
    }
}

// ── COSE Sign1 tag #18 helpers ──

/// Read a signed CoRIM file: expects CBOR tag #18 wrapping a COSE_Sign1.
fn read_signed_corim(path: &str) -> Result<CoseSign1Cbor, String> {
    let data = fs::read(path).map_err(|e| format!("failed to read {path}: {e}"))?;
    let value: Value = from_reader(data.as_slice())
        .map_err(|e| format!("failed to parse CBOR from {path}: {e}"))?;

    match value {
        Value::Tag(18, inner) => {
            let sign1: CoseSign1Cbor = from_reader(
                &*cbor_bytes(&inner).map_err(|e| format!("failed to serialize inner CBOR: {e}"))?,
            )
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

/// Serialize a CBOR Value to bytes.
fn cbor_bytes(value: &Value) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
    let mut buf = Vec::new();
    into_writer(value, &mut buf)?;
    Ok(buf)
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

/// Sign an unsigned CoRIM with a JWK key, producing a COSE Sign1 wrapped in CBOR tag #18.
fn corim_sign(args: &CorimSignSubcommand) {
    // Read unsigned CoRIM
    let corim_file_bytes = match fs::read(&args.corim_file) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read CoRIM file {}: {}", args.corim_file, e);
            return;
        }
    };

    // Ensure the payload is wrapped in tag 501 (unsigned CoRIM) for cocli interop.
    // If it's already tagged, use as-is; otherwise wrap it.
    let corim_bytes = match ensure_corim_tag(&corim_file_bytes) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to process CoRIM payload: {}", e);
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

    // Read meta JSON template
    let meta_str = match fs::read_to_string(&args.meta_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read meta file {}: {}", args.meta_file, e);
            return;
        }
    };

    // Parse meta JSON
    let meta_json: MetaJson = match serde_json::from_str(&meta_str) {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to parse meta JSON: {}", e);
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

    // Convert meta to CBOR
    let meta_cbor = match meta_json_to_cbor(&meta_json) {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to convert meta to CBOR: {}", e);
            return;
        }
    };

    // Serialize meta to CBOR bytes for protected header.
    // Per CoRIM spec, label 8 holds a bstr .cbor corim-meta-map.
    let mut meta_buf = Vec::new();
    if let Err(e) = into_writer(&meta_cbor, &mut meta_buf) {
        println!("Failed to serialize meta CBOR: {}", e);
        return;
    }

    // Build protected header matching cocli format:
    // label 1 = alg, label 3 = content_type, label 8 = meta (as bstr)
    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(algorithm.to_i64())),
        criticality: None,
        content_type: Some(TextOrInt::Text("application/rim+cbor".to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: Some(vec![Tuple {
            key: Value::Integer(8.into()),
            value: Value::Bytes(meta_buf),
        }]),
    };

    // Sign
    let sign1 = match CoseSign1Builder::new()
        .payload(&corim_bytes)
        .protected(hdr)
        .sign(signer.as_ref())
    {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to sign CoRIM: {}", e);
            return;
        }
    };

    // Write output
    let corim_path = Path::new(&args.corim_file);
    let stem = corim_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("signed-corim");
    let output_path = Path::new(&args.output_dir).join(format!("signed-{stem}.cbor"));

    if let Err(e) = write_tagged_sign1(&sign1, &output_path) {
        println!("Failed to write signed CoRIM: {}", e);
        return;
    }

    println!("Signed CoRIM written to {:?}", output_path);
}

// ── Verify ──

/// Verify the COSE Sign1 signature on a signed CoRIM using a JWK key.
fn corim_verify(args: &CorimVerifySubcommand) {
    // Read signed CoRIM
    let sign1 = match read_signed_corim(&args.signed_corim_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read signed CoRIM: {}", e);
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

/// Extract the payload and individual tags (CoMID/CoSWID) from a signed CoRIM.
fn corim_extract(args: &CorimExtractSubcommand) {
    // Read signed CoRIM
    let sign1 = match read_signed_corim(&args.signed_corim_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read signed CoRIM: {}", e);
            return;
        }
    };

    // Extract payload
    let payload = match &sign1.payload {
        common::BinaryOrNil::Binary(p) => p,
        common::BinaryOrNil::Nil => {
            println!("Signed CoRIM has no payload");
            return;
        }
    };

    // Parse payload as raw CBOR Value to be resilient to schema variations
    let raw_value: Value = match from_reader(payload.as_slice()) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to parse payload as CBOR: {}", e);
            return;
        }
    };

    // Strip tag 501 wrapper if present
    let corim_value = match raw_value {
        Value::Tag(501, inner) => *inner,
        other => other,
    };

    // Extract the tags array (key 1 in corim-map)
    let tags = match &corim_value {
        Value::Map(entries) => {
            let mut tags = None;
            for (k, v) in entries {
                if let Value::Integer(i) = k {
                    if i64::try_from(*i) == Ok(1) {
                        tags = v.as_array();
                        break;
                    }
                }
            }
            match tags {
                Some(t) => t.clone(),
                None => {
                    println!("No tags array (key 1) found in CoRIM payload");
                    return;
                }
            }
        }
        _ => {
            println!("CoRIM payload is not a CBOR map");
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

    // Each tag in the CoRIM is a bstr containing tagged CBOR:
    // tag 505 for CoSWID, tag 506 for CoMID.
    let mut comid_count = 0u32;
    let mut coswid_count = 0u32;

    for tag_entry in &tags {
        let raw = match tag_entry.as_bytes() {
            Some(b) => b,
            None => {
                println!("Tag entry is not a byte string, skipping");
                continue;
            }
        };

        // Parse the tagged CBOR to determine the type
        let value: Value = match from_reader(raw.as_slice()) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to parse tag bytes as CBOR: {}", e);
                continue;
            }
        };

        let (prefix, count) = match &value {
            Value::Tag(506, _) => {
                comid_count += 1;
                ("comid", comid_count)
            }
            Value::Tag(505, _) => {
                coswid_count += 1;
                ("coswid", coswid_count)
            }
            _ => {
                comid_count += 1;
                ("tag", comid_count)
            }
        };

        let filename = format!("{prefix}-{count}.cbor");
        let output_path = output_dir.join(&filename);
        match File::create(&output_path) {
            Ok(mut f) => {
                if let Err(e) = f.write_all(raw) {
                    println!("Failed to write {filename}: {}", e);
                }
            }
            Err(e) => {
                println!("Failed to create {filename}: {}", e);
            }
        }
    }

    println!(
        "Extracted {} CoMID(s) and {} CoSWID(s) to {:?}",
        comid_count, coswid_count, output_dir
    );
}
