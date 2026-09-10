# Quickstart: User-Initiated Catalog Updates

## Reviewed candidate path

1. Copy an unmodified S073 candidate directory into the user-data catalog import
   root under its declared channel and candidate SHA-256.
2. Start ESO Weave. Startup remains immediate while candidate inspection runs on
   the background check thread.
3. Open the catalog update notice or choose Catalog Update from the application
   menu.
4. Review the Live or PTS label, version tuple, size, source summary, coverage
   changes, icon placeholders, and compatibility.
5. Confirm Install for a compatible Live candidate. PTS remains preview-only.
6. Follow Validating, Installing, Opening, and Complete. The previous catalog is
   preserved as the rollback point.

## Collector-assisted path

1. Choose Collector-assisted build in the update modal.
2. Review selected public-API categories and excluded or pseudonymized fields.
3. Install or update the separately managed collector when offered.
4. In ESO, run the explicit collection and finish with `/reloadui`, logout, or
   exit so SavedVariables reaches disk.
5. Return to ESO Weave and continue only after the modal recognizes a later
   stable flushed capture.
6. The app parses the restricted export, builds an S073 candidate with the
   current Live catalog as a zero-removal baseline, verifies it, and uses the
   same install boundary as a reviewed candidate.
7. Delete the capture or uninstall the collector from the cleanup actions if
   desired.

## Recovery and rollback

- Cancel is available only before the selection commit boundary.
- A cancelled, corrupt, incompatible, incomplete, or failed candidate never
  changes the active selection.
- On restart, incomplete staging is marked interrupted and ignored.
- Roll back re-verifies the previous Live target before changing selection.
- If a user selection is invalid, startup falls back to the bundled catalog and
  reports the diagnostic without deleting evidence.

## Developer verification

Run focused S074 tests first, then the complete merge gates:

```text
cargo test --locked --test catalog_update
cargo test --locked --test app_ui_sizing
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Also build the optimized binaries, test and build mdBook, run the documentation
policy, typo checker, JSON validation, and UTF-8/no-BOM hygiene checks.
