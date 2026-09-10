# Quickstart: S078 Encounter History

1. Configure the correct Live or PTS environment in Settings.
2. Capture one terminal encounter with ESO Weave Encounter and flush SavedVariables.
3. Open File, Encounter History.
4. Choose Import Current Capture.
5. Select the imported summary to calculate observed details against the active catalog.
6. Inspect versions, quality, loss ranges, and unknown IDs before interpreting values.
7. Use Delete Encounter or Delete All only when the corresponding confirmation is open.

## Expected states

- No local encounters: valid empty state; no store is created by viewing it.
- Capture unavailable: the selected environment could not supply the terminal file.
- Store unavailable: the existing raw store is corrupt, locked, or unsupported and is
  left untouched.
- Catalog unavailable or invalid: summaries remain visible but derived details do not.
- Version mismatch: capture channel or API does not match the active catalog.
- Degraded: observed results remain visible with exact declared loss.

## Verification

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```
