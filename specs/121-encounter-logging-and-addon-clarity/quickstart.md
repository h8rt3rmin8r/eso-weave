# Repository Validation Guide

No live-game or installed-app checks. Windows non-Git console tools use a hidden redirected non-interactive launcher; tests remain foreground and watched to completion.

1. Reconcile [audit.md](audit.md) against [messaging contract](contracts/messaging.md) and all source/state branches.
2. Run focused desktop/history/Lua fixtures, covering setup, current/saved state, recovery, ownership and deletion effects.
3. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked`.
4. Run documentation/trust/issue policy tests, `mdbook build docs`, generated docs policy and headless browser smoke. Include desktop, narrow, 200-percent zoom, text equivalent and offline lineage checks.
5. Check changed text for UTF-8 without BOM, LF, mojibake and prohibited long dashes. Every audited message needs a final disposition.
6. Publish authorized official PR closing #249/#222, handle all findings, at most two Codex rounds and owner merge after green hosted checks.

Actual evidence belongs in [verification.md](verification.md); unperformed checks cannot be passes.
