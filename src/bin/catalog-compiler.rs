//! Explicit maintainer command for catalog and local data maintenance.

use std::collections::BTreeMap;
use std::path::PathBuf;

use eso_weave::catalog::compiler::{build_catalog, diff_catalogs, verify_catalog, BuildRequest};
use eso_weave::catalog::{CatalogError, Channel};
use eso_weave::catalog_pipeline::{build_candidate, verify_candidate, PipelineError, PipelineRun};
use eso_weave::collector::lifecycle::{
    install as install_collector, status as collector_status, uninstall as remove_collector,
    RunningState as CollectorRunningState,
};
use eso_weave::collector::{
    import_capture, CollectorError, ImportRequest as CollectorImportRequest,
};
use eso_weave::encounter::{
    backup_store, delete_all, delete_encounter, import_encounter, list_encounters,
    project_encounter, EncounterError, ImportRequest as EncounterImportRequest, ProjectionRequest,
};

#[derive(thiserror::Error, Debug)]
enum CliError {
    #[error("{0}")]
    Catalog(#[from] CatalogError),
    #[error("{0}")]
    Collector(#[from] CollectorError),
    #[error("{0}")]
    Pipeline(#[from] PipelineError),
    #[error("{0}")]
    Encounter(#[from] EncounterError),
}

fn main() {
    if let Err(error) = run(std::env::args().skip(1).collect()) {
        eprintln!("catalog-compiler: {error}");
        std::process::exit(2);
    }
}

fn run(arguments: Vec<String>) -> Result<(), CliError> {
    let Some(command) = arguments.first() else {
        return Err(usage().into());
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
        "import-collector" => {
            let input = required(&flags, "--input")?;
            let output = required(&flags, "--output")?;
            let channel = parse_channel(required(&flags, "--channel")?)?;
            let catalog_version = required(&flags, "--catalog-version")?;
            let report = import_capture(&CollectorImportRequest::new(
                input,
                output,
                channel,
                catalog_version,
            ))?;
            print_json(&report)?;
        }
        "encounter-import" => {
            let report = import_encounter(&EncounterImportRequest::new(
                required(&flags, "--input")?,
                required(&flags, "--store")?,
                parse_channel(required(&flags, "--channel")?)?,
            ))?;
            print_json(&report)?;
        }
        "encounter-list" => {
            print_json(&list_encounters(required(&flags, "--store")?)?)?;
        }
        "encounter-project" => {
            print_json(&project_encounter(&ProjectionRequest::new(
                required(&flags, "--store")?,
                required(&flags, "--catalog")?,
                required(&flags, "--output")?,
                required(&flags, "--session")?,
                required(&flags, "--encounter")?,
            ))?)?;
        }
        "encounter-backup" => {
            print_json(&backup_store(
                required(&flags, "--store")?,
                required(&flags, "--output")?,
            )?)?;
        }
        "encounter-delete" => {
            let store = required(&flags, "--store")?;
            let delete_everything = flags.get("--all").is_some_and(|value| value == "true");
            let has_identity = flags.contains_key("--session") || flags.contains_key("--encounter");
            let report = if delete_everything {
                if has_identity {
                    return Err(CatalogError::Validation(
                        "encounter-delete accepts either --all or one complete identity".into(),
                    )
                    .into());
                }
                delete_all(store)?
            } else {
                delete_encounter(
                    store,
                    required(&flags, "--session")?,
                    required(&flags, "--encounter")?,
                )?
            };
            print_json(&report)?;
        }
        "pipeline-build" => {
            let network = flags
                .get("--network")
                .map(String::as_str)
                .unwrap_or("disabled");
            if !matches!(network, "enabled" | "disabled") {
                return Err(CatalogError::Validation(
                    "--network must be enabled or disabled".to_string(),
                )
                .into());
            }
            let report = build_candidate(&PipelineRun::new(
                required(&flags, "--request")?,
                required(&flags, "--workspace")?,
                required(&flags, "--source-cache")?,
                required(&flags, "--icon-cache")?,
                required(&flags, "--candidates")?,
                network == "enabled",
            ))?;
            print_json(&report)?;
        }
        "pipeline-verify" => {
            let report = verify_candidate(required(&flags, "--candidate")?)?;
            print_json(&report)?;
        }
        "collector-status" => {
            print_json(&collector_status(
                PathBuf::from(required(&flags, "--addons")?).as_path(),
            ))?;
        }
        "collector-install" => {
            let addons = PathBuf::from(required(&flags, "--addons")?);
            let api_version = required(&flags, "--api-version")?
                .parse::<u32>()
                .map_err(|_| {
                    CatalogError::Validation("--api-version must be a positive integer".to_string())
                })?;
            if api_version == 0 {
                return Err(CatalogError::Validation(
                    "--api-version must be a positive integer".to_string(),
                )
                .into());
            }
            print_json(&install_collector(
                &addons,
                collector_running_state(),
                api_version,
            )?)?;
        }
        "collector-remove" => {
            let addons = PathBuf::from(required(&flags, "--addons")?);
            print_json(&remove_collector(&addons, collector_running_state())?)?;
        }
        _ => return Err(usage().into()),
    }
    Ok(())
}

fn flags(arguments: &[String]) -> Result<BTreeMap<String, String>, CatalogError> {
    let mut result = BTreeMap::new();
    let mut index = 0;
    while index < arguments.len() {
        let name = &arguments[index];
        if name == "--all" {
            if result.insert(name.clone(), "true".into()).is_some() {
                return Err(usage());
            }
            index += 1;
            continue;
        }
        let Some(value) = arguments.get(index + 1) else {
            return Err(usage());
        };
        if !name.starts_with("--")
            || value.starts_with("--")
            || result.insert(name.clone(), value.clone()).is_some()
        {
            return Err(usage());
        }
        index += 2;
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

fn print_json(value: &impl serde::Serialize) -> Result<(), CliError> {
    println!(
        "{}",
        serde_json::to_string_pretty(value).map_err(CatalogError::from)?
    );
    Ok(())
}

fn collector_running_state() -> CollectorRunningState {
    match eso_weave::beacon::probe_game_running() {
        eso_weave::beacon::RunningState::Running => CollectorRunningState::Running,
        eso_weave::beacon::RunningState::NotRunning => CollectorRunningState::NotRunning,
        eso_weave::beacon::RunningState::Unknown => CollectorRunningState::Unknown,
    }
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
        "usage: catalog-compiler build --input PATH --output PATH --channel live|pts [--report PATH]; catalog-compiler verify --catalog PATH; catalog-compiler diff --old PATH --new PATH [--output PATH]; catalog-compiler import-collector --input PATH --output PATH --channel live|pts --catalog-version VERSION; catalog-compiler encounter-import --input PATH --store PATH --channel live|pts; catalog-compiler encounter-list --store PATH; catalog-compiler encounter-project --store PATH --catalog PATH --output PATH --session ID --encounter ID; catalog-compiler encounter-backup --store PATH --output PATH; catalog-compiler encounter-delete --store PATH (--session ID --encounter ID | --all); catalog-compiler pipeline-build --request PATH --workspace PATH --source-cache PATH --icon-cache PATH --candidates PATH [--network enabled|disabled]; catalog-compiler pipeline-verify --candidate PATH; catalog-compiler collector-status --addons PATH; catalog-compiler collector-install --addons PATH --api-version VERSION; catalog-compiler collector-remove --addons PATH"
            .to_string(),
    )
}
