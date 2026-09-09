# S073 Quickstart

## Build an offline review candidate

From the repository root:

~~~console
cargo run --locked --bin catalog-compiler -- pipeline-build --request specs/073-reviewed-catalog-pipeline/fixtures/offline-live-request.json --workspace . --source-cache target/catalog-pipeline/sources --icon-cache target/catalog-pipeline/icons --candidates target/catalog-pipeline/candidates
~~~

The command prints a redacted receipt containing the immutable candidate ID and
relative directory. It does not modify assets/catalog/catalog.sqlite.

Verify the resulting candidate:

~~~console
cargo run --locked --bin catalog-compiler -- pipeline-verify --candidate target/catalog-pipeline/candidates/live/CANDIDATE_SHA256
~~~

## Other modes

Use mode live or mode pts with an exact-channel normalized bundle. Use mode
user-capture with one collector-capture input source. Offline mode permits
either channel but performs no network request.

For approved immutable remote source pins, enable request network policy and
pass --network enabled. Both gates are required. Remote access remains
restricted to pinned raw GitHub content and every response must match the
declared SHA-256.

The icon source is optional and relative to the workspace. Its S072 cache is a
separate local output. Candidate artifacts contain only a redacted receipt, not
icon bytes.

## Review

1. Inspect manifest.json for the exact version tuple and artifact allowlist.
2. Inspect sources.json for revisions, hashes, policy, and acquisition mode.
3. Inspect validation.json for warnings and no blocking findings.
4. Inspect build-report.json and verify-report.json for integrity and hashes.
5. Inspect diff.json for stable additions, removals, changes, and coverage.
6. Inspect icons.json for placeholder and ready counts.
7. Inspect checksums.json, then run pipeline-verify.

Acceptance, active installation, repository updates, release publication, and
PTS promotion are separate human-controlled operations outside S073.

## Verification commands

~~~console
cargo test --locked --test catalog_pipeline
cargo test --locked --test catalog_pipeline_workflow
cargo test --locked --test catalog_compiler
cargo test --locked --test collector_import
cargo test --locked --test icon_cache
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --all --locked
cargo build --locked --release --bin eso-weave
cargo build --locked --release --bin catalog-compiler
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
git diff --check
~~~

## Expected evidence

- invented Live, PTS, offline, and capture requests succeed only in their exact
  channels;
- repeated candidates have identical canonical identities and semantic reports;
- source limits, hashes, host rules, links, aliases, and cache corruption fail
  closed;
- threshold, baseline, and interrupted publication failures preserve prior
  candidates;
- candidate files match the exact allowlist and contain no local paths, source
  bytes, capture bytes, credentials, or user-local image bytes;
- the workflow has read-only permissions, pinned actions, and no acceptance or
  release authority.
