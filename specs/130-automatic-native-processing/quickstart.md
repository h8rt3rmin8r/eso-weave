# S130 Validation Guide

Use temporary synthetic roots/logs and SQLite stores. Do not launch ESO or change its logging/settings.

1. Run focused incremental parser tests for meaningful byte splits, v15 vocabulary, completion, recovery, limits and checkpoint validation.
2. Run automatic-store/runtime tests for catchup/live append, exact identities/bytes, restart/rotation/copy/truncation, failed writes, suppression and commit/disable races.
3. Run history/API fixtures with retained old schemas, Catalog swaps, disabled/absent collection and permanently absent viewers. Assert qualified source capabilities and unchanged authentication/read-only bounds.
4. Run interface/accessibility/layout tests across supported themes/scales and regenerate final deterministic captures. Audit active source/help/contracts/assets/generated output against the current-surface inventory.
5. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked` in the foreground. Run repository documentation/render/trust/brand/release-note/package gates and release-profile embedded-guide tests using pinned tooling.
6. Publish the authorized official PR, run at most two Codex rounds, resolve all findings and verify hosted checks on the final head. Stop for owner final review and merge.

Record actual command results/tool versions in verification.md. Fixture coverage is not installed-game proof. Normal/inline and verbose remain unsupported until #296/#297 land.
