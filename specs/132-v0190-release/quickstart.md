# S132 Candidate Validation

Run from the repository root with the pinned Rust and documentation tools. On Windows use a hidden noninteractive launcher with redirected output, such as the session's ignored `target/s129-run.ps1`; it waits for each tool to finish and records actual exit status.

1. Inspect `git diff origin/main -- Cargo.toml Cargo.lock` against #284's three dependency changes. Before rollover, `CHANGELOG_HEADING=Unreleased bash scripts/release-notes.sh 0.19.0 h8rt3rmin8r/eso-weave` must reject missing Highlights; after preparatory notes it must pass.
2. On the clean prepared branch run `cargo release 0.19.0` and inspect its dry-run output, then `cargo release 0.19.0 --execute --no-confirm`. The command creates the local identity commit without a tag, push or publication.
3. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings` and `cargo test --all --locked` in the foreground to completion. Build `cargo build --release --locked --bin eso-weave` with the exact mdBook tools available.
4. Run existing release-note and Debian shell contracts, Node documentation/brand/render/trust tests, spelling, `mdbook test docs`, `mdbook build docs`, generated-site policy, brand policy and rendered diagram smoke checks.
5. Audit the six rollover files and unchanged addon manifest versions; check text encoding, LF, punctuation and `git diff --check`.
6. Publish the official PR, inspect every review thread and successful check on its final head, and record evidence in `verification.md`. Deliver the open PR to the owner; tag and publication come later.

See [candidate contract](contracts/release-candidate.md), [data model](data-model.md) and [release procedure](../../docs/project/releasing.md). These are automated repository checks, not installed/game checks.
