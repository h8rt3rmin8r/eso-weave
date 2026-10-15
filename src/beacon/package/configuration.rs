//! Preference writes own only the bounded generated startup representation.
//! A compatible manual package is never adopted for replacement or removal.

use super::*;

const PREFIX: &str = ".eso-weave-startup-config-";
const REPLACEMENT: &str = "replacement.lua";
const PREVIOUS: &str = "previous.lua";

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigurationReceipt {
    identity: String,
    root: PathBuf,
    before: StartupConfig,
    after: StartupConfig,
    sources: Vec<String>,
}

fn target(root: &Path) -> PathBuf {
    root.join(super::super::SUBFOLDER).join(STARTUP_CONFIG_FILE)
}

fn sources(root: &Path) -> std::io::Result<Vec<Vec<u8>>> {
    confined_parent(root)?;
    INVENTORY
        .iter()
        .filter(|name| **name != STARTUP_CONFIG_FILE)
        .map(|name| {
            read_regular(
                root,
                &root.join(super::super::SUBFOLDER).join(name),
                MAX_FILE_BYTES,
            )
        })
        .collect()
}

fn confined_parent(root: &Path) -> std::io::Result<()> {
    for directory in [root.to_owned(), root.join(super::super::SUBFOLDER)] {
        let value = fs::symlink_metadata(&directory)?;
        let canonical = fs::canonicalize(&directory)?;
        let same_path = crate::bounded_file::is_same_or_nested(&canonical, &directory)
            && crate::bounded_file::is_same_or_nested(&directory, &canonical);
        if !value.is_dir() || is_link_like(&value) || !same_path {
            return Err(invalid_data(
                "startup configuration parent changed or escaped its root",
            ));
        }
    }
    Ok(())
}

fn verify_sources(root: &Path, expected: &[String]) -> std::io::Result<()> {
    if sources(root)?
        .iter()
        .map(|bytes| digest(bytes))
        .collect::<Vec<_>>()
        != expected
    {
        return Err(invalid_data(
            "startup configuration package identity changed",
        ));
    }
    Ok(())
}

fn outcome(root: &Path) -> Result<LifecycleOutcome, LifecycleError> {
    Ok(LifecycleOutcome {
        status: inspect(root)?,
        reload_required: false,
    })
}

pub(super) fn update(
    root: &Path,
    desired: StartupConfig,
) -> Result<LifecycleOutcome, LifecycleError> {
    update_with(root, desired, |_, _| Ok(()))
}

fn update_with(
    root: &Path,
    desired: StartupConfig,
    mut boundary: impl FnMut(&str, &Path) -> std::io::Result<()>,
) -> Result<LifecycleOutcome, LifecycleError> {
    let presence = inspect_presence(root)?;
    if !presence.eligible() {
        return Err(LifecycleError::Unmanaged);
    }
    let before = presence.startup_config.ok_or(LifecycleError::Unmanaged)?;
    if before == desired {
        return outcome(root);
    }
    let target = target(root);
    let target_metadata = fs::symlink_metadata(&target)?;
    if !target_metadata.is_file() || is_link_like(&target_metadata) {
        return Err(LifecycleError::Unmanaged);
    }
    if target_metadata.permissions().readonly() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "startup configuration is read-only",
        )
        .into());
    }
    let previous_bytes = read_regular(root, &target, MAX_CONFIG_BYTES)?;
    if parse_startup_config(&previous_bytes) != Some(before) {
        return Err(LifecycleError::Unmanaged);
    }
    let package_sources = sources(root)?;
    let transaction = tempfile::Builder::new()
        .prefix(PREFIX)
        .tempdir_in(root)?
        .keep();
    let receipt = ConfigurationReceipt {
        identity: "PixelBeacon startup configuration 1".into(),
        root: root.to_owned(),
        before,
        after: desired,
        sources: package_sources.iter().map(|bytes| digest(bytes)).collect(),
    };
    write_synced(
        &transaction.join(RECEIPT_FILE),
        &serde_json::to_vec(&receipt).map_err(invalid_data)?,
    )?;
    let mut committed = false;
    let result = (|| -> Result<(), LifecycleError> {
        write_synced(
            &transaction.join(REPLACEMENT),
            render_startup_config(desired).as_bytes(),
        )?;
        boundary("before-retirement", &transaction)?;
        verify_sources(root, &receipt.sources)?;
        if read_regular(root, &target, MAX_CONFIG_BYTES)? != previous_bytes {
            return Err(LifecycleError::Integrity(
                "startup configuration changed before retirement".into(),
            ));
        }
        rename_no_replace(&target, &transaction.join(PREVIOUS))?;
        boundary("after-retirement", &transaction)?;
        if read_regular(root, &transaction.join(PREVIOUS), MAX_CONFIG_BYTES)? != previous_bytes
            || sources(root)? != package_sources
        {
            return Err(LifecycleError::Integrity(
                "package or startup configuration changed before preference commit".into(),
            ));
        }
        boundary("before-commit", &transaction)?;
        verify_sources(root, &receipt.sources)?;
        if read_regular(root, &transaction.join(PREVIOUS), MAX_CONFIG_BYTES)? != previous_bytes {
            return Err(LifecycleError::Integrity(
                "retired startup configuration changed before commit".into(),
            ));
        }
        rename_no_replace(&transaction.join(REPLACEMENT), &target)?;
        committed = true;
        write_synced(&transaction.join(COMMITTED_FILE), COMMITTED_BYTES)?;
        Ok(())
    })();
    if let Err(error) = result {
        if committed {
            return Err(LifecycleError::CleanupPending {
                status: inspect(root).unwrap_or(BeaconStatus::Unmanaged),
                recovery: transaction,
                message: error.to_string(),
            });
        }
        if let Err(recovery) = recover_one(root, &transaction) {
            return Err(LifecycleError::RecoveryPending {
                recovery: transaction,
                message: format!("{error}; recovery failed: {recovery}"),
            });
        }
        return Err(error);
    }
    if let Err(error) = boundary("before-cleanup", &transaction)
        .and_then(|()| cleanup(root, &transaction, &receipt))
    {
        return Err(LifecycleError::CleanupPending {
            status: inspect(root).unwrap_or(BeaconStatus::Unmanaged),
            recovery: transaction,
            message: error.to_string(),
        });
    }
    outcome(root)
}

pub(super) fn recover_all(root: &Path) -> Result<(), LifecycleError> {
    let mut transactions = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if !entry.file_name().to_string_lossy().starts_with(PREFIX) {
            continue;
        }
        let value = fs::symlink_metadata(entry.path())?;
        if value.is_dir()
            && !is_link_like(&value)
            && metadata(&entry.path().join(RECEIPT_FILE))?.is_some()
        {
            transactions.push(entry.path());
        }
    }
    transactions.sort();
    for transaction in transactions {
        recover_one(root, &transaction)?;
    }
    Ok(())
}

fn recover_one(root: &Path, transaction: &Path) -> Result<(), LifecycleError> {
    let value = fs::symlink_metadata(transaction)?;
    if !value.is_dir() || is_link_like(&value) {
        return Err(LifecycleError::Unmanaged);
    }
    let receipt: ConfigurationReceipt = serde_json::from_slice(&read_regular(
        root,
        &transaction.join(RECEIPT_FILE),
        MAX_RECEIPT_BYTES,
    )?)
    .map_err(invalid_data)?;
    if receipt.identity != "PixelBeacon startup configuration 1"
        || receipt.root != root
        || receipt.before == receipt.after
        || receipt.sources.len() != INVENTORY.len() - 1
        || receipt
            .sources
            .iter()
            .any(|value| value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()))
    {
        return Err(LifecycleError::Integrity(
            "startup configuration receipt does not belong to this root".into(),
        ));
    }
    let explicit_commit = if metadata(&transaction.join(COMMITTED_FILE))?.is_some() {
        read_regular(
            root,
            &transaction.join(COMMITTED_FILE),
            COMMITTED_BYTES.len() as u64,
        )? == COMMITTED_BYTES
    } else {
        false
    };
    let inferred_commit = metadata(&transaction.join(REPLACEMENT))?.is_none()
        && metadata(&transaction.join(PREVIOUS))?.is_some()
        && read_regular(root, &target(root), MAX_CONFIG_BYTES)
            .ok()
            .as_deref()
            == Some(render_startup_config(receipt.after).as_bytes());
    if explicit_commit || inferred_commit {
        return cleanup(root, transaction, &receipt).map_err(|error| {
            LifecycleError::CleanupPending {
                status: inspect(root).unwrap_or(BeaconStatus::Unmanaged),
                recovery: transaction.to_owned(),
                message: error.to_string(),
            }
        });
    }
    if metadata(&transaction.join(PREVIOUS))?.is_some() {
        // Restore the entire moved entry, including unexpected raced contents,
        // without following it or replacing a new target.
        verify_sources(root, &receipt.sources).map_err(|error| {
            LifecycleError::RecoveryPending {
                recovery: transaction.to_owned(),
                message: format!(
                    "startup configuration rollback parent cannot be verified: {error}"
                ),
            }
        })?;
        rename_no_replace(&transaction.join(PREVIOUS), &target(root)).map_err(|error| {
            LifecycleError::RecoveryPending {
                recovery: transaction.to_owned(),
                message: format!(
                    "startup configuration rollback preserves a conflicting target: {error}"
                ),
            }
        })?;
    }
    cleanup(root, transaction, &receipt)?;
    Ok(())
}

fn cleanup(root: &Path, transaction: &Path, receipt: &ConfigurationReceipt) -> std::io::Result<()> {
    for entry in fs::read_dir(transaction)? {
        let name = entry?.file_name();
        if ![RECEIPT_FILE, COMMITTED_FILE, REPLACEMENT, PREVIOUS]
            .iter()
            .any(|expected| name == *expected)
        {
            return Err(invalid_data(
                "unexpected startup configuration recovery entry",
            ));
        }
    }
    for (file, expected) in [(REPLACEMENT, receipt.after), (PREVIOUS, receipt.before)] {
        let file = transaction.join(file);
        if metadata(&file)?.is_none() {
            continue;
        }
        if read_regular(root, &file, MAX_CONFIG_BYTES)?
            != render_startup_config(expected).as_bytes()
        {
            return Err(invalid_data("startup configuration recovery bytes changed"));
        }
        fs::remove_file(file)?;
    }
    if metadata(&transaction.join(COMMITTED_FILE))?.is_some() {
        if read_regular(
            root,
            &transaction.join(COMMITTED_FILE),
            COMMITTED_BYTES.len() as u64,
        )? != COMMITTED_BYTES
        {
            return Err(invalid_data("startup configuration commit marker changed"));
        }
        fs::remove_file(transaction.join(COMMITTED_FILE))?;
    }
    fs::remove_file(transaction.join(RECEIPT_FILE))?;
    fs::remove_dir(transaction)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manual(root: &Path) -> Vec<Vec<u8>> {
        install_configured(
            root,
            RunningState::Unknown,
            101051,
            5,
            false,
            Environment::Live,
        )
        .unwrap();
        let manifest = root
            .join(super::super::super::SUBFOLDER)
            .join(super::super::super::MANIFEST_FILE);
        fs::write(
            &manifest,
            fs::read_to_string(&manifest)
                .unwrap()
                .replace(super::super::super::MANAGED_MARKER, ""),
        )
        .unwrap();
        sources(root).unwrap()
    }
    fn desired() -> StartupConfig {
        StartupConfig {
            catalog_disabled: true,
            environment: Environment::Live,
        }
    }

    #[test]
    fn compatible_manual_config_has_separate_authority_and_preserves_every_other_byte() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        let before = manual(&root);
        fs::write(
            root.join("PixelBeacon/custom-helper.lua"),
            b"-- retain manual helper",
        )
        .unwrap();
        let result = update(&root, desired()).unwrap();
        assert_eq!(result.status, BeaconStatus::Unmanaged);
        assert!(!result.reload_required);
        assert_eq!(sources(&root).unwrap(), before);
        assert_eq!(
            fs::read(root.join("PixelBeacon/custom-helper.lua")).unwrap(),
            b"-- retain manual helper"
        );
        assert_eq!(
            inspect_presence(&root).unwrap().startup_config,
            Some(desired())
        );
        assert!(uninstall(&root, RunningState::Unknown).is_err());
    }

    #[test]
    fn config_precommit_failures_restore_original_and_never_change_manual_sources() {
        for failed in ["before-retirement", "after-retirement", "before-commit"] {
            let sandbox = tempfile::tempdir().unwrap();
            let root = fs::canonicalize(sandbox.path()).unwrap();
            let before = manual(&root);
            let config = fs::read(target(&root)).unwrap();
            assert!(update_with(&root, desired(), |boundary, _| {
                if boundary == failed {
                    Err(std::io::Error::other("injected config failure"))
                } else {
                    Ok(())
                }
            })
            .is_err());
            assert_eq!(fs::read(target(&root)).unwrap(), config);
            assert_eq!(sources(&root).unwrap(), before);
            assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
        }
    }

    #[test]
    fn raced_config_target_is_preserved_and_rollback_recovers_after_conflict_is_moved() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        let before = manual(&root);
        let config = fs::read(target(&root)).unwrap();
        let result = update_with(&root, desired(), |boundary, _| {
            if boundary == "before-commit" {
                fs::write(target(&root), b"foreign raced config")?;
            }
            Ok(())
        });
        assert!(matches!(
            result,
            Err(LifecycleError::RecoveryPending { .. })
        ));
        assert_eq!(fs::read(target(&root)).unwrap(), b"foreign raced config");
        fs::rename(target(&root), root.join("foreign-preserved.lua")).unwrap();
        recover_all(&root).unwrap();
        assert_eq!(fs::read(target(&root)).unwrap(), config);
        assert_eq!(sources(&root).unwrap(), before);
        assert_eq!(
            fs::read(root.join("foreign-preserved.lua")).unwrap(),
            b"foreign raced config"
        );
    }

    #[test]
    fn committed_manual_config_cleanup_failure_is_distinct_and_retry_finishes() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        let before = manual(&root);
        assert!(matches!(
            update_with(&root, desired(), |boundary, _| {
                if boundary == "before-cleanup" {
                    Err(std::io::Error::other("injected cleanup failure"))
                } else {
                    Ok(())
                }
            }),
            Err(LifecycleError::CleanupPending {
                status: BeaconStatus::Unmanaged,
                ..
            })
        ));
        assert_eq!(
            inspect_presence(&root).unwrap().startup_config,
            Some(desired())
        );
        recover_all(&root).unwrap();
        assert_eq!(sources(&root).unwrap(), before);
        assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
    }

    #[test]
    fn interrupted_manual_configuration_recovers_before_or_after_file_commit() {
        for commit in [false, true] {
            let sandbox = tempfile::tempdir().unwrap();
            let root = fs::canonicalize(sandbox.path()).unwrap();
            let before_sources = manual(&root);
            let before = inspect_presence(&root).unwrap().startup_config.unwrap();
            let transaction = tempfile::Builder::new()
                .prefix(PREFIX)
                .tempdir_in(&root)
                .unwrap()
                .keep();
            let receipt = ConfigurationReceipt {
                identity: "PixelBeacon startup configuration 1".into(),
                root: root.clone(),
                before,
                after: desired(),
                sources: before_sources.iter().map(|bytes| digest(bytes)).collect(),
            };
            write_synced(
                &transaction.join(RECEIPT_FILE),
                &serde_json::to_vec(&receipt).unwrap(),
            )
            .unwrap();
            write_synced(
                &transaction.join(REPLACEMENT),
                render_startup_config(desired()).as_bytes(),
            )
            .unwrap();
            rename_no_replace(&target(&root), &transaction.join(PREVIOUS)).unwrap();
            if commit {
                rename_no_replace(&transaction.join(REPLACEMENT), &target(&root)).unwrap();
            }
            // The ordinary startup recovery entry point includes config journals.
            recover(&root).unwrap();
            assert_eq!(
                inspect_presence(&root).unwrap().startup_config,
                Some(if commit { desired() } else { before })
            );
            assert_eq!(sources(&root).unwrap(), before_sources);
            assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
        }
    }

    #[test]
    fn directory_swap_before_configuration_commit_preserves_foreign_and_moved_original_bytes() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        let before_sources = manual(&root);
        let before_config = fs::read(target(&root)).unwrap();
        let moved = root.join("moved-original");
        let result = update_with(&root, desired(), |boundary, _| {
            if boundary == "before-commit" {
                fs::rename(root.join(super::super::super::SUBFOLDER), &moved)?;
                fs::create_dir(root.join(super::super::super::SUBFOLDER))?;
                fs::write(target(&root), b"foreign replacement startup")?;
                fs::write(
                    root.join("PixelBeacon/foreign-note"),
                    b"retain foreign note",
                )?;
            }
            Ok(())
        });
        let recovery = match result {
            Err(LifecycleError::RecoveryPending { recovery, .. }) => recovery,
            other => panic!("unexpected {other:?}"),
        };
        assert_eq!(
            fs::read(target(&root)).unwrap(),
            b"foreign replacement startup"
        );
        assert_eq!(
            fs::read(root.join("PixelBeacon/foreign-note")).unwrap(),
            b"retain foreign note"
        );
        let retained = INVENTORY
            .iter()
            .filter(|name| **name != STARTUP_CONFIG_FILE)
            .map(|name| fs::read(moved.join(name)).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(retained, before_sources);
        assert_eq!(fs::read(recovery.join(PREVIOUS)).unwrap(), before_config);
        assert!(recover(&root).is_err());
        assert_eq!(
            fs::read(target(&root)).unwrap(),
            b"foreign replacement startup"
        );
        fs::rename(
            root.join(super::super::super::SUBFOLDER),
            root.join("foreign-preserved"),
        )
        .unwrap();
        fs::rename(moved, root.join(super::super::super::SUBFOLDER)).unwrap();
        recover(&root).unwrap();
        assert_eq!(fs::read(target(&root)).unwrap(), before_config);
        assert_eq!(sources(&root).unwrap(), before_sources);
    }

    #[cfg(unix)]
    #[test]
    fn linked_parent_before_configuration_commit_and_recovery_never_touches_outside_files() {
        use std::os::unix::fs::symlink;
        let sandbox = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        let before_sources = manual(&root);
        let before_config = fs::read(target(&root)).unwrap();
        let moved = root.join("moved-original");
        fs::write(
            outside.path().join(STARTUP_CONFIG_FILE),
            b"outside startup bytes",
        )
        .unwrap();
        let result = update_with(&root, desired(), |boundary, _| {
            if boundary == "before-commit" {
                fs::rename(root.join(super::super::super::SUBFOLDER), &moved)?;
                symlink(outside.path(), root.join(super::super::super::SUBFOLDER))?;
            }
            Ok(())
        });
        let recovery = match result {
            Err(LifecycleError::RecoveryPending { recovery, .. }) => recovery,
            other => panic!("unexpected {other:?}"),
        };
        assert_eq!(
            fs::read(outside.path().join(STARTUP_CONFIG_FILE)).unwrap(),
            b"outside startup bytes"
        );
        assert_eq!(fs::read(recovery.join(PREVIOUS)).unwrap(), before_config);
        assert!(recover(&root).is_err());
        assert_eq!(
            fs::read(outside.path().join(STARTUP_CONFIG_FILE)).unwrap(),
            b"outside startup bytes"
        );
        fs::remove_file(root.join(super::super::super::SUBFOLDER)).unwrap();
        fs::rename(moved, root.join(super::super::super::SUBFOLDER)).unwrap();
        recover(&root).unwrap();
        assert_eq!(fs::read(target(&root)).unwrap(), before_config);
        assert_eq!(sources(&root).unwrap(), before_sources);
    }
}
