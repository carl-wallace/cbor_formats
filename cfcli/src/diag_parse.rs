//! Parser for CBOR diagnostic notation (RFC 8949 Section 8) into `ciborium::Value`.

use ciborium::value::Value;

/// Parse a CBOR diagnostic notation string into a `ciborium::Value`.
pub fn from_diag(input: &str) -> Result<Value, String> {
    let tokens = tokenize(input)?;
    let mut pos = 0;
    let value = parse_value(&tokens, &mut pos)?;
    // Skip trailing whitespace tokens
    while pos < tokens.len() {
        if tokens[pos] != Token::Comma {
            return Err(format!("unexpected trailing token: {:?}", tokens[pos]));
        }
        pos += 1;
    }
    Ok(value)
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Integer(i128),
    Float(f64),
    Bytes(Vec<u8>),
    Text(String),
    True,
    False,
    Null,
    Undefined,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Colon,
    Comma,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' | '\r' => i += 1,
            '[' => {
                tokens.push(Token::LBracket);
                i += 1;
            }
            ']' => {
                tokens.push(Token::RBracket);
                i += 1;
            }
            '{' => {
                tokens.push(Token::LBrace);
                i += 1;
            }
            '}' => {
                tokens.push(Token::RBrace);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            'h' if i + 1 < chars.len() && chars[i + 1] == '\'' => {
                i += 2; // skip h'
                let mut hex = String::new();
                while i < chars.len() && chars[i] != '\'' {
                    if !chars[i].is_ascii_whitespace() {
                        hex.push(chars[i]);
                    }
                    i += 1;
                }
                if i >= chars.len() {
                    return Err("unterminated hex byte string".to_string());
                }
                i += 1; // skip closing '
                let bytes = hex_decode(&hex)?;
                tokens.push(Token::Bytes(bytes));
            }
            'b' if i + 1 < chars.len() && chars[i + 1] == '\'' => {
                // b'...' base64url byte string — not common but handle it
                i += 2;
                let mut b64 = String::new();
                while i < chars.len() && chars[i] != '\'' {
                    b64.push(chars[i]);
                    i += 1;
                }
                if i >= chars.len() {
                    return Err("unterminated base64 byte string".to_string());
                }
                // base64url byte strings not yet supported
                return Err(format!(
                    "base64 byte strings (b'...') not yet supported: {b64}"
                ));
            }
            '"' => {
                i += 1;
                let mut s = String::new();
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        i += 1;
                        match chars[i] {
                            '"' => s.push('"'),
                            '\\' => s.push('\\'),
                            'n' => s.push('\n'),
                            'r' => s.push('\r'),
                            't' => s.push('\t'),
                            '/' => s.push('/'),
                            other => {
                                s.push('\\');
                                s.push(other);
                            }
                        }
                    } else {
                        s.push(chars[i]);
                    }
                    i += 1;
                }
                if i >= chars.len() {
                    return Err("unterminated string".to_string());
                }
                i += 1; // skip closing "
                tokens.push(Token::Text(s));
            }
            '-' | '0'..='9' => {
                let start = i;
                if chars[i] == '-' {
                    i += 1;
                }
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                // Check for float
                if i < chars.len() && chars[i] == '.' {
                    i += 1;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                    }
                    // Exponent
                    if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                        i += 1;
                        if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                            i += 1;
                        }
                        while i < chars.len() && chars[i].is_ascii_digit() {
                            i += 1;
                        }
                    }
                    let s: String = chars[start..i].iter().collect();
                    let f: f64 = s.parse().map_err(|_| format!("invalid float: {s}"))?;
                    tokens.push(Token::Float(f));
                } else {
                    // Skip optional _N suffix (cbor-diag encoding hint)
                    if i < chars.len() && chars[i] == '_' {
                        i += 1;
                        while i < chars.len() && chars[i].is_ascii_digit() {
                            i += 1;
                        }
                    }
                    let s: String = chars[start..i].iter().collect();
                    // Remove _N suffix for parsing
                    let num_str = if let Some(idx) = s.find('_') {
                        &s[..idx]
                    } else {
                        &s
                    };
                    let n: i128 = num_str
                        .parse()
                        .map_err(|_| format!("invalid integer: {s}"))?;
                    tokens.push(Token::Integer(n));
                }
            }
            't' if input[i..].starts_with("true") && !is_ident_char(chars.get(i + 4)) => {
                tokens.push(Token::True);
                i += 4;
            }
            'f' if input[i..].starts_with("false") && !is_ident_char(chars.get(i + 5)) => {
                tokens.push(Token::False);
                i += 5;
            }
            'n' if input[i..].starts_with("null") && !is_ident_char(chars.get(i + 4)) => {
                tokens.push(Token::Null);
                i += 4;
            }
            'u' if input[i..].starts_with("undefined") && !is_ident_char(chars.get(i + 9)) => {
                tokens.push(Token::Undefined);
                i += 9;
            }
            '/' if i + 1 < chars.len() && chars[i + 1] == '/' => {
                // Line comment
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            '/' if i + 1 < chars.len() && chars[i + 1] == '*' => {
                // Block comment
                i += 2;
                while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                if i + 1 < chars.len() {
                    i += 2;
                }
            }
            other => {
                return Err(format!("unexpected character: '{other}' at position {i}"));
            }
        }
    }
    Ok(tokens)
}

fn is_ident_char(c: Option<&char>) -> bool {
    matches!(c, Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
}

fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("hex string has odd length: {}", s.len()));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|_| format!("invalid hex at position {i}: {}", &s[i..i + 2]))
        })
        .collect()
}

fn parse_value(tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
    if *pos >= tokens.len() {
        return Err("unexpected end of input".to_string());
    }

    match &tokens[*pos] {
        Token::Integer(n) => {
            let n = *n;
            *pos += 1;
            // Check if this is a tag: integer followed by '('
            if *pos < tokens.len() && tokens[*pos] == Token::LParen {
                *pos += 1; // skip '('
                let inner = parse_value(tokens, pos)?;
                expect_token(tokens, pos, &Token::RParen)?;
                let tag = u64::try_from(n).map_err(|_| format!("tag value out of range: {n}"))?;
                Ok(Value::Tag(tag, Box::new(inner)))
            } else {
                Ok(Value::Integer(
                    n.try_into()
                        .map_err(|_| format!("integer out of CBOR range: {n}"))?,
                ))
            }
        }
        Token::Float(f) => {
            let f = *f;
            *pos += 1;
            Ok(Value::Float(f))
        }
        Token::Bytes(b) => {
            let b = b.clone();
            *pos += 1;
            Ok(Value::Bytes(b))
        }
        Token::Text(s) => {
            let s = s.clone();
            *pos += 1;
            Ok(Value::Text(s))
        }
        Token::True => {
            *pos += 1;
            Ok(Value::Bool(true))
        }
        Token::False => {
            *pos += 1;
            Ok(Value::Bool(false))
        }
        Token::Null => {
            *pos += 1;
            Ok(Value::Null)
        }
        Token::Undefined => {
            *pos += 1;
            // ciborium doesn't have Undefined, use Null
            Ok(Value::Null)
        }
        Token::LBracket => {
            *pos += 1;
            let mut items = Vec::new();
            while *pos < tokens.len() && tokens[*pos] != Token::RBracket {
                items.push(parse_value(tokens, pos)?);
                if *pos < tokens.len() && tokens[*pos] == Token::Comma {
                    *pos += 1;
                }
            }
            expect_token(tokens, pos, &Token::RBracket)?;
            Ok(Value::Array(items))
        }
        Token::LBrace => {
            *pos += 1;
            let mut entries = Vec::new();
            while *pos < tokens.len() && tokens[*pos] != Token::RBrace {
                let key = parse_value(tokens, pos)?;
                expect_token(tokens, pos, &Token::Colon)?;
                let value = parse_value(tokens, pos)?;
                entries.push((key, value));
                if *pos < tokens.len() && tokens[*pos] == Token::Comma {
                    *pos += 1;
                }
            }
            expect_token(tokens, pos, &Token::RBrace)?;
            Ok(Value::Map(entries))
        }
        other => Err(format!("unexpected token: {other:?}")),
    }
}

fn expect_token(tokens: &[Token], pos: &mut usize, expected: &Token) -> Result<(), String> {
    if *pos >= tokens.len() {
        return Err(format!("expected {expected:?}, got end of input"));
    }
    if tokens[*pos] != *expected {
        return Err(format!("expected {expected:?}, got {:?}", tokens[*pos]));
    }
    *pos += 1;
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer() {
        assert_eq!(from_diag("42").unwrap(), Value::Integer(42.into()));
        assert_eq!(from_diag("-7").unwrap(), Value::Integer((-7).into()));
        assert_eq!(from_diag("0").unwrap(), Value::Integer(0.into()));
    }

    #[test]
    fn parse_text() {
        assert_eq!(
            from_diag(r#""hello""#).unwrap(),
            Value::Text("hello".to_string())
        );
        assert_eq!(
            from_diag(r#""with \"quotes\"""#).unwrap(),
            Value::Text("with \"quotes\"".to_string())
        );
    }

    #[test]
    fn parse_bytes() {
        assert_eq!(
            from_diag("h'deadbeef'").unwrap(),
            Value::Bytes(vec![0xde, 0xad, 0xbe, 0xef])
        );
        assert_eq!(from_diag("h''").unwrap(), Value::Bytes(vec![]));
    }

    #[test]
    fn parse_bool_null() {
        assert_eq!(from_diag("true").unwrap(), Value::Bool(true));
        assert_eq!(from_diag("false").unwrap(), Value::Bool(false));
        assert_eq!(from_diag("null").unwrap(), Value::Null);
    }

    #[test]
    fn parse_array() {
        assert_eq!(from_diag("[]").unwrap(), Value::Array(vec![]));
        assert_eq!(
            from_diag("[1, 2, 3]").unwrap(),
            Value::Array(vec![
                Value::Integer(1.into()),
                Value::Integer(2.into()),
                Value::Integer(3.into()),
            ])
        );
    }

    #[test]
    fn parse_map() {
        assert_eq!(from_diag("{}").unwrap(), Value::Map(vec![]));
        assert_eq!(
            from_diag(r#"{1: "hello", 2: h'ff'}"#).unwrap(),
            Value::Map(vec![
                (Value::Integer(1.into()), Value::Text("hello".to_string())),
                (Value::Integer(2.into()), Value::Bytes(vec![0xff])),
            ])
        );
    }

    #[test]
    fn parse_tag() {
        assert_eq!(
            from_diag("1(1234567890)").unwrap(),
            Value::Tag(1, Box::new(Value::Integer(1234567890.into())))
        );
        assert_eq!(
            from_diag(r#"32("https://example.com")"#).unwrap(),
            Value::Tag(32, Box::new(Value::Text("https://example.com".to_string())))
        );
    }

    #[test]
    fn parse_nested() {
        let input = r#"{1: 37(h'deadbeef'), 2: [1, "two", true]}"#;
        let value = from_diag(input).unwrap();
        match value {
            Value::Map(entries) => {
                assert_eq!(entries.len(), 2);
                match &entries[0].1 {
                    Value::Tag(37, inner) => {
                        assert_eq!(**inner, Value::Bytes(vec![0xde, 0xad, 0xbe, 0xef]));
                    }
                    other => panic!("expected tag, got: {other:?}"),
                }
            }
            other => panic!("expected map, got: {other:?}"),
        }
    }

    #[test]
    fn parse_cbor_diag_encoding_hint() {
        // cbor-diag uses _N suffix to indicate encoding width — should be ignored
        assert_eq!(
            from_diag("560_1(h'00112233')").unwrap(),
            Value::Tag(560, Box::new(Value::Bytes(vec![0x00, 0x11, 0x22, 0x33])))
        );
    }

    #[test]
    fn parse_trailing_commas() {
        // cbor-diag outputs trailing commas
        assert_eq!(
            from_diag("{1: 2,}").unwrap(),
            Value::Map(vec![(Value::Integer(1.into()), Value::Integer(2.into()))])
        );
        assert_eq!(
            from_diag("[1, 2,]").unwrap(),
            Value::Array(vec![Value::Integer(1.into()), Value::Integer(2.into())])
        );
    }

    #[test]
    fn roundtrip_diag() {
        use crate::cbor_diag::to_diag;

        let input = r#"{0: "test", 1: 37(h'0102030405060708090a0b0c0d0e0f10'), 4: {0: [{0: {0: 560(h'aabbccdd')}, 1: [{1: {2: [[1, h'0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20']]}}]}]}}"#;
        let value = from_diag(input).unwrap();
        let diag = to_diag(&value);
        let roundtripped = from_diag(&diag).unwrap();
        // Serialize both to CBOR and compare
        let mut buf1 = vec![];
        let mut buf2 = vec![];
        ciborium::ser::into_writer(&value, &mut buf1).unwrap();
        ciborium::ser::into_writer(&roundtripped, &mut buf2).unwrap();
        assert_eq!(buf1, buf2);
    }
}
