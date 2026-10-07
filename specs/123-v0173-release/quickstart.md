# S123 Validation Guide

Prerequisites: branch `codex/s123-v0173-release`, complete specification/analysis, existing pinned documentation/Rust tools. On Windows use the verified hidden `target/s121-run.ps1` launcher for non-Git console tooling, with redirected closed stdin. Use Git Bash at `C:/Program Files/Git/bin/bash.exe`, not the PATH WSL launcher.

1. Before adding Highlights, set `CHANGELOG_HEADING=Unreleased` and run `scripts/release-notes.sh 0.17.3 h8rt3rmin8r/eso-weave`. Expect the missing-Highlights rejection. After editing, expect compact notes with a tag-specific full changelog link.
2. Run `bash scripts/release-notes.test.sh`; existing negative fixtures must pass.
3. Run documentation policy test suites, brand policy, spelling, `mdbook test docs`, `mdbook build docs`, generated-site policy and diagram rendering smoke tests. Archive indexes and migration-ledger destinations must agree.
4. Commit validated preparatory changes, inspect `cargo release 0.17.3`, then execute `cargo release 0.17.3 --execute --no-confirm`. Inspect its commit, all identity fields and unchanged dependency/runtime scope.
5. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked` and `cargo build --release --locked --bin eso-weave`. Run Git whitespace and UTF-8/no-BOM/LF/mojibake checks.
6. Publish the official PR and inspect exact-head CI, review comments/threads and reactions. Resolve every finding within the two-requested-round cap and retain an unmerged, untagged candidate for owner handoff.

See [release contract](contracts/release-candidate.md) for authority boundaries. These are repository/build checks, not installed or game-session observations.
