# Quickstart: Deterministic Screenshot Sandbox

Run from the repository root after implementation.

## Validate without rendering

```powershell
cargo test --locked --test documentation_capture
```

Expected: the catalog and isolation contracts pass without initializing a renderer or retaining generated files. Synthetic fixture files exist only in an automatically removed temporary directory.

## Generate the complete capture matrix

```powershell
cargo test --locked --test documentation_capture -- --capture-to target/documentation-captures
```

Expected: 28 PNGs and `capture-manifest.json` appear under `target/documentation-captures`. No native window or prompt appears.

## Reproducibility check

Run the generator into two distinct target subdirectories. Compare the two manifests byte for byte and compare PNG hashes on the same machine and renderer adapter.

## CI parity

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all --locked
```

Expected: formatting, linting, all integration targets, and all tests pass. The custom capture target runs validation only during the full suite.

## Release isolation

```powershell
cargo build --locked --release
```

Expected: only the production application binary is built. No capture option, feature, or alternate application entry point exists.

## Text and repository checks

Run `git diff --check`, strict UTF-8 and LF validation, forbidden-dash and mojibake scans, JSON parsing, and verify that generated captures remain untracked below `target/`.
