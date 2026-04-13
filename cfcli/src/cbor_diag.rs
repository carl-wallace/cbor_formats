//! CBOR diagnostic notation formatter (RFC 8949 Section 8).

use ciborium::value::Value;
use std::fs;

/// Read a CBOR file and print it in diagnostic notation. Returns true if handled.
pub fn display_diag(path: &str) -> bool {
    let data = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            println!("Unable to read file {}: {}", path, e);
            return true;
        }
    };
    let value: Value = match ciborium::de::from_reader(data.as_slice()) {
        Ok(v) => v,
        Err(e) => {
            println!("Unable to parse CBOR from {}: {}", path, e);
            return true;
        }
    };
    println!("{}", to_diag(&value));
    true
}

/// Format a CBOR `Value` as diagnostic notation.
pub fn to_diag(value: &Value) -> String {
    let mut buf = String::new();
    format_value(value, &mut buf, 0);
    buf
}

fn format_value(value: &Value, buf: &mut String, indent: usize) {
    match value {
        Value::Integer(i) => {
            let n: i128 = (*i).into();
            buf.push_str(&n.to_string());
        }
        Value::Bytes(b) => {
            buf.push_str("h'");
            for byte in b {
                buf.push_str(&format!("{byte:02x}"));
            }
            buf.push('\'');
        }
        Value::Text(s) => {
            buf.push('"');
            for c in s.chars() {
                match c {
                    '"' => buf.push_str("\\\""),
                    '\\' => buf.push_str("\\\\"),
                    '\n' => buf.push_str("\\n"),
                    '\r' => buf.push_str("\\r"),
                    '\t' => buf.push_str("\\t"),
                    c => buf.push(c),
                }
            }
            buf.push('"');
        }
        Value::Bool(b) => {
            buf.push_str(if *b { "true" } else { "false" });
        }
        Value::Null => {
            buf.push_str("null");
        }
        Value::Tag(tag, inner) => {
            buf.push_str(&tag.to_string());
            buf.push('(');
            format_value(inner, buf, indent);
            buf.push(')');
        }
        Value::Array(items) => {
            if items.is_empty() {
                buf.push_str("[]");
            } else if is_compact(value) {
                buf.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        buf.push_str(", ");
                    }
                    format_value(item, buf, indent);
                }
                buf.push(']');
            } else {
                buf.push_str("[\n");
                let child_indent = indent + 2;
                for (i, item) in items.iter().enumerate() {
                    push_indent(buf, child_indent);
                    format_value(item, buf, child_indent);
                    if i < items.len() - 1 {
                        buf.push(',');
                    }
                    buf.push('\n');
                }
                push_indent(buf, indent);
                buf.push(']');
            }
        }
        Value::Map(entries) => {
            if entries.is_empty() {
                buf.push_str("{}");
            } else {
                buf.push_str("{\n");
                let child_indent = indent + 2;
                for (i, (k, v)) in entries.iter().enumerate() {
                    push_indent(buf, child_indent);
                    format_value(k, buf, child_indent);
                    buf.push_str(": ");
                    format_value(v, buf, child_indent);
                    if i < entries.len() - 1 {
                        buf.push(',');
                    }
                    buf.push('\n');
                }
                push_indent(buf, indent);
                buf.push('}');
            }
        }
        Value::Float(f) => {
            if f.fract() == 0.0 && f.is_finite() {
                buf.push_str(&format!("{f:.1}"));
            } else {
                buf.push_str(&f.to_string());
            }
        }
        _ => {
            buf.push_str("undefined");
        }
    }
}

/// Decide if an array is short enough to print on one line.
fn is_compact(value: &Value) -> bool {
    match value {
        Value::Array(items) => {
            items.len() <= 4
                && items
                    .iter()
                    .all(|v| matches!(v, Value::Integer(_) | Value::Bool(_) | Value::Null))
        }
        _ => false,
    }
}

fn push_indent(buf: &mut String, indent: usize) {
    for _ in 0..indent {
        buf.push(' ');
    }
}
