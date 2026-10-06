# S122 Automated Validation

Use the repository's verified hidden launcher on Windows. Commands below run
noninteractively with redirected I/O; no game or installed application is opened.

1. `node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs .github/scripts/brand-kit-policy.test.mjs`
2. `node .github/scripts/brand-kit-policy.mjs`
3. `typos docs/src docs/README.md README.md`
4. `mdbook test docs` then `mdbook build docs`.
5. `node .github/scripts/docs-policy.mjs docs target/docs-site/html`
6. `node .github/scripts/docs-render-smoke.mjs --site target/docs-site/html`
7. `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
   and `cargo test --all --locked`, watched to completion through the hidden launcher.
8. Review generated figures and source-contract matrix; check changed text for
   BOM, CRLF, long dashes and mojibake. `git diff --check` must pass.

Expected: exact eight-figure inventory, 64 paint cells, eight topology probes,
both new complete equivalents and readable 200 percent zoom/no-script evidence.
Mutation tests must reject missing gates or equivalent boundaries. Full Rust
suite retains local-service/database/player-state and all safety-critical tests.
Record results in `verification.md`; push/PR and at most one additional review
round follow the kickoff authorization. Owner performs final review and merge.
