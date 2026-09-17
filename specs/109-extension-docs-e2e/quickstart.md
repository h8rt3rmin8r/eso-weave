# Quickstart: Verify S109 Local Extension Documentation

This contributor path verifies the S109 documentation and production adapters without a live ESO installation.

## 1. Run focused contract tests

```powershell
cargo test --locked --test local_extension_contract
cargo test --locked --test local_service
```

Expected result: field inventory, documented examples, HTTP and MCP parity, lifecycle recovery, and shutdown checks pass.

## 2. Run the documentation policy and site

```powershell
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

Expected result: the new reference chapter is indexed, all local links resolve, semantic tables and examples match the exact policy inventory, and the generated site passes policy.

## 3. Run repository parity

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
cargo build --release --locked --bin eso-weave
node --test .github/scripts/trust-policy.test.mjs
node .github/scripts/trust-policy.mjs .
```

Expected result: all commands complete successfully with no weakened safety or trust assertion.

## 4. Inspect hygiene and scope

- Confirm every changed text file is UTF-8 without BOM and LF-only.
- Confirm no em dash, en dash, mojibake, real credential, or user-specific absolute path appears.
- Confirm the implementation adds no remote listener, write API, telemetry, agent orchestration, or new observation.
- Confirm Plan 045 identifies S109 as its final slice and issue #180 remains the only closing reference.
