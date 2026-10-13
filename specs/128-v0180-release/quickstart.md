# S128 Candidate Validation Guide

Use the existing toolchain and pinned docs tools. On Windows, run non-Git tools through a hidden noninteractive launcher; run git/gh directly. All Rust gates run in the foreground to completion.

1. Preview notes before rollover: `CHANGELOG_HEADING=Unreleased bash scripts/release-notes.sh 0.18.0 h8rt3rmin8r/eso-weave`. Expect three bounded highlights and a version-specific full-history link.
2. Run release-note/Debian fixtures and the existing documentation/render/brand/trust policy suites. Run spelling, mdBook test/build and generated-site policy. Expect every existing assertion to remain intact.
3. Inspect Plan053 archive, Plan054 current index and migration ledger. Run `git diff --check` and strict UTF-8/no-BOM/LF/mojibake checks before the clean preparation commit.
4. On `codex/s128-v0180-release`, inspect `cargo release 0.18.0`, then execute `cargo release 0.18.0 --execute --no-confirm`. Expect one local identity commit, no tag/push/publication and exactly the six identity files from [data-model.md](data-model.md).
5. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked` and `cargo build --release --locked --bin eso-weave`.
6. Rebuild mdBook and run `node .github/scripts/docs-policy.mjs docs target/docs-site/html` and `node .github/scripts/docs-render-smoke.mjs --site target/docs-site/html`. Extract the versioned notes/section; inspect synchronized identity and unchanged dependency graph.
7. Publish the official PR with `Closes #279`; wait for checks on its exact final head and all returned reviews. Answer findings, resolve completed threads and stay within two rounds before owner handoff.

These are repository/build checks. Package installation, real-game behavior, owner merge and later tag/publication are outside this candidate slice.
