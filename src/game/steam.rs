//! Small Valve Data Format helpers used by Steam installation discovery.

use std::path::PathBuf;

#[derive(Debug)]
enum Value {
    Str(String),
    Obj(Vec<(String, Value)>),
}

#[derive(Debug)]
enum Tok {
    Str(String),
    Open,
    Close,
}

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

fn parse_object(tokens: &[Tok], pos: &mut usize) -> Vec<(String, Value)> {
    let mut pairs = Vec::new();
    while *pos < tokens.len() {
        match &tokens[*pos] {
            Tok::Close => {
                *pos += 1;
                break;
            }
            Tok::Open => {
                *pos += 1;
                let _ = parse_object(tokens, pos);
            }
            Tok::Str(key) => {
                let key = key.clone();
                *pos += 1;
                match tokens.get(*pos) {
                    Some(Tok::Open) => {
                        *pos += 1;
                        pairs.push((key, Value::Obj(parse_object(tokens, pos))));
                    }
                    Some(Tok::Str(value)) => {
                        pairs.push((key, Value::Str(value.clone())));
                        *pos += 1;
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

fn parse(input: &str) -> Vec<(String, Value)> {
    let tokens = tokenize(input);
    let mut pos = 0;
    parse_object(&tokens, &mut pos)
}

fn collect_libraries(pairs: &[(String, Value)], app_id: &str, out: &mut Vec<PathBuf>) {
    let path = pairs
        .iter()
        .find_map(|(key, value)| match (key.as_str(), value) {
            ("path", Value::Str(path)) => Some(path),
            _ => None,
        });
    let has_app = pairs
        .iter()
        .any(|(key, value)| match (key.as_str(), value) {
            ("apps", Value::Obj(apps)) => apps.iter().any(|(id, _)| id == app_id),
            _ => false,
        });
    if has_app {
        if let Some(path) = path {
            out.push(PathBuf::from(path));
        }
    }
    for (key, value) in pairs {
        if key != "apps" {
            if let Value::Obj(child) = value {
                collect_libraries(child, app_id, out);
            }
        }
    }
}

/// Returns every Steam library whose apps map contains `app_id`.
pub fn library_paths_for_app(vdf: &str, app_id: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect_libraries(&parse(vdf), app_id, &mut out);
    out
}

/// Returns the `installdir` recorded in a Steam app manifest.
pub fn install_dir_from_manifest(vdf: &str) -> Option<PathBuf> {
    fn find(pairs: &[(String, Value)]) -> Option<&str> {
        for (key, value) in pairs {
            match value {
                Value::Str(value) if key.eq_ignore_ascii_case("installdir") => return Some(value),
                Value::Obj(child) => {
                    if let Some(value) = find(child) {
                        return Some(value);
                    }
                }
                Value::Str(_) => {}
            }
        }
        None
    }
    find(&parse(vdf)).map(PathBuf::from)
}
