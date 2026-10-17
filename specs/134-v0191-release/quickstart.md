# S134 Validation Guide

Use the hidden, noninteractive Windows launcher and redirected I/O required by
project directives. No package installation or game session is required.

1. From the selected release branch, preview Unreleased notes with
   CHANGELOG_HEADING=Unreleased bash scripts/release-notes.sh 0.19.1 h8rt3rmin8r/eso-weave.
   Missing Highlights must fail before preparation; valid five-outcome notes must pass.
2. Run bash scripts/release-notes.test.sh and bash scripts/validate-debian-package.test.sh.
   Real dpkg cases require Linux and are verified by hosted Linux CI.
3. Validate documentation/brand/trust tests and source policy; commit preparation
   and confirm a clean tree before cargo release 0.19.1 --no-confirm dry run.
4. Inspect expected replacements, then cargo release 0.19.1 --execute --no-confirm.
   Configuration must retain tag=false, push=false and publish=false.
5. Run cargo fmt --all -- --check, cargo clippy --all-targets --all-features -- -D warnings,
   cargo test --all --locked and cargo build --release --locked --bin eso-weave.
6. Run mdbook test/build docs, generated documentation policy, spelling and the
   headless documentation render smoke gate; run the release-profile embedded-doc tests.
7. Compare all identity surfaces and historical/addon/dependency preservation.
   Inspect final-head hosted checks and review threads, then stop with the PR
   open and no v0.19.1 tag or published release. Record observations in verification.md.
