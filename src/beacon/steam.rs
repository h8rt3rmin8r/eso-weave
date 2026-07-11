//! Steam `libraryfolders.vdf` extraction.
//!
//! Only the slice needed for discovery is parsed: for each library block, its
//! `path` and whether its `apps` map contains the target app id. The parser is
//! pure and compiled on all targets so the Linux discovery logic has host test
//! coverage.

use std::path::PathBuf;

/// A parsed VDF value: either a string or a nested object of key-value pairs.
enum Value {
    Str(String),
    Obj(Vec<(String, Value)>),
}

/// A VDF token.
enum Tok {
    Str(String),
    Open,
    Close,
}

/// Tokenizes VDF text into quoted strings and braces; all other characters
/// (whitespace, unquoted tokens) are ignored.
fn tokenize(input: &str) -> Vec<Tok> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(Tok::Open),
            '}' => tokens.push(Tok::Close),
            '"' => {
                let mut buf = String::new();
                while let Some(c2) = chars.next() {
                    match c2 {
                        '"' => break,
                        '\\' => {
                            if let Some(escaped) = chars.next() {
                                match escaped {
                                    'n' => buf.push('\n'),
                                    't' => buf.push('\t'),
                                    other => buf.push(other),
                                }
                            }
                        }
                        other => buf.push(other),
                    }
                }
                tokens.push(Tok::Str(buf));
            }
            _ => {}
        }
    }
    tokens
}

/// Parses key-value pairs until a closing brace or the end of the token stream.
fn parse_object(tokens: &[Tok], pos: &mut usize) -> Vec<(String, Value)> {
    let mut pairs = Vec::new();
    while *pos < tokens.len() {
        match &tokens[*pos] {
            Tok::Close => {
                *pos += 1;
                break;
            }
            Tok::Open => {
                // Stray open brace with no key; consume its object and discard.
                *pos += 1;
                let _ = parse_object(tokens, pos);
            }
            Tok::Str(key) => {
                let key = key.clone();
                *pos += 1;
                match tokens.get(*pos) {
                    Some(Tok::Open) => {
                        *pos += 1;
                        let obj = parse_object(tokens, pos);
                        pairs.push((key, Value::Obj(obj)));
                    }
                    Some(Tok::Str(val)) => {
                        let val = val.clone();
                        *pos += 1;
                        pairs.push((key, Value::Str(val)));
                    }
                    Some(Tok::Close) => {
                        *pos += 1;
                        break;
                    }
                    None => break,
                }
            }
        }
    }
    pairs
}

/// Recursively collects, in file order, the `path` of every object that looks
/// like a library (has a `path` field) whose `apps` map contains `app_id`.
fn collect(pairs: &[(String, Value)], app_id: &str, out: &mut Vec<PathBuf>) {
    let path = pairs.iter().find_map(|(k, v)| match (k.as_str(), v) {
        ("path", Value::Str(s)) => Some(s.clone()),
        _ => None,
    });
    let has_app = pairs.iter().any(|(k, v)| match (k.as_str(), v) {
        ("apps", Value::Obj(apps)) => apps.iter().any(|(app_key, _)| app_key == app_id),
        _ => false,
    });
    if let Some(path) = path {
        if has_app {
            out.push(PathBuf::from(path));
        }
    }
    for (key, value) in pairs {
        if let Value::Obj(child) = value {
            if key != "apps" {
                collect(child, app_id, out);
            }
        }
    }
}

/// Returns, in file order, the `path` of each Steam library whose `apps` map
/// lists `app_id`.
pub fn library_paths_for_app(vdf: &str, app_id: &str) -> Vec<PathBuf> {
    let tokens = tokenize(vdf);
    let mut pos = 0;
    let root = parse_object(&tokens, &mut pos);
    let mut out = Vec::new();
    collect(&root, app_id, &mut out);
    out
}
