//! File discovery and CBOR utilities for cfcli.

use std::ffi::OsStr;
use std::path::Path;
use walkdir::WalkDir;

/// Try to extract the payload bytes from a COSE Sign1 structure (CBOR tag 18).
/// If the input is a Sign1, returns the payload. Otherwise returns the original bytes unchanged.
/// This allows display commands to handle both signed and unsigned inputs.
pub(crate) fn unwrap_sign1_payload(data: &[u8]) -> Vec<u8> {
    use ciborium::de::from_reader;
    use ciborium::value::Value;

    let value: Value = match from_reader(data) {
        Ok(v) => v,
        Err(_) => return data.to_vec(),
    };

    // COSE Sign1 is Tag(18, [protected, unprotected, payload, signature])
    let inner = match &value {
        Value::Tag(18, inner) => inner,
        _ => return data.to_vec(),
    };

    let array = match inner.as_array() {
        Some(a) if a.len() == 4 => a,
        _ => return data.to_vec(),
    };

    // Payload is element 2 — should be a byte string (or null for detached)
    match &array[2] {
        Value::Bytes(payload) => payload.clone(),
        _ => data.to_vec(),
    }
}

/// Recursively find files with the given extension in `dir` and append their paths to `list`.
pub(crate) fn find_files(dir: &String, ext: &str, list: &mut Vec<String>) {
    if !Path::is_dir(Path::new(dir)) {
        return;
    }

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(e) => {
                let path = e.path();
                if e.file_type().is_dir() {
                    if let Some(s) = path.to_str() {
                        if s != dir {
                            let count = list.len();
                            find_files(&s.to_string(), ext, list);
                            if count == list.len() {
                                continue;
                            }
                        }
                    }
                    continue;
                } else {
                    let file_exts = [ext];
                    if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                        if !file_exts.contains(&ext) {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    if let Some(s) = path.to_str() {
                        let s = s.to_string();
                        if !list.contains(&s) {
                            list.push(s);
                        }
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }
}
