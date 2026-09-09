//! Explicit maintainer command for catalog build, verification, and diffing.

use std::collections::BTreeMap;
use std::path::PathBuf;

use eso_weave::catalog::compiler::{build_catalog, diff_catalogs, verify_catalog, BuildRequest};
use eso_weave::catalog::{CatalogError, Channel};

fn main() {
    if let Err(error) = run(std::env::args().skip(1).collect()) {
        eprintln!("catalog-compiler: {error}");
        std::process::exit(2);
    }
}

fn run(arguments: Vec<String>) -> Result<(), CatalogError> {
    let Some(command) = arguments.first() else {
        return Err(usage());
    };
    let flags = flags(&arguments[1..])?;
    match command.as_str() {
        "build" => {
            let input = required(&flags, "--input")?;
            let output = required(&flags, "--output")?;
            let channel = parse_channel(required(&flags, "--channel")?)?;
            let mut request = BuildRequest::new(input, output, channel);
            request.report_path = flags.get("--report").map(PathBuf::from);
            let report = build_catalog(&request)?;
            print_json(&report)?;
        }
        "verify" => {
            let report = verify_catalog(required(&flags, "--catalog")?)?;
            print_json(&report)?;
        }
        "diff" => {
            let report = diff_catalogs(required(&flags, "--old")?, required(&flags, "--new")?)?;
            if let Some(path) = flags.get("--output") {
                write_json(PathBuf::from(path), &report)?;
            }
            print_json(&report)?;
        }
        _ => return Err(usage()),
    }
    Ok(())
}

fn flags(arguments: &[String]) -> Result<BTreeMap<String, String>, CatalogError> {
    if !arguments.len().is_multiple_of(2) {
        return Err(usage());
    }
    let mut result = BTreeMap::new();
    for pair in arguments.chunks_exact(2) {
        if !pair[0].starts_with("--") || result.insert(pair[0].clone(), pair[1].clone()).is_some() {
            return Err(usage());
        }
    }
    Ok(result)
}

fn required<'a>(flags: &'a BTreeMap<String, String>, name: &str) -> Result<&'a str, CatalogError> {
    flags.get(name).map(String::as_str).ok_or_else(usage)
}

fn parse_channel(value: &str) -> Result<Channel, CatalogError> {
    match value {
        "live" => Ok(Channel::Live),
        "pts" => Ok(Channel::Pts),
        _ => Err(CatalogError::Validation(
            "--channel must be live or pts".to_string(),
        )),
    }
}

fn print_json(value: &impl serde::Serialize) -> Result<(), CatalogError> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn write_json(path: PathBuf, value: &impl serde::Serialize) -> Result<(), CatalogError> {
    if let Some(parent) = path.parent().filter(|value| !value.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    std::fs::write(path, bytes)?;
    Ok(())
}

fn usage() -> CatalogError {
    CatalogError::Validation(
        "usage: catalog-compiler build --input PATH --output PATH --channel live|pts [--report PATH]; catalog-compiler verify --catalog PATH; catalog-compiler diff --old PATH --new PATH [--output PATH]"
            .to_string(),
    )
}
