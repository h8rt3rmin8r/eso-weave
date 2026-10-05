# S120 Automated Validation

Run through a hidden, redirected foreground launcher on Windows. Do not launch ESO or the desktop application.

1. `cargo test --locked --test beacon` covers the exact Lua discovery module, four slots, desktop/controller coexistence, actual desktop conflicts, unsupported and absent sources, and complete light attack routing/synthesis.
2. `cargo test --locked --lib beacon::api_check` covers merge-head history, numeric API parsing, source age, malformed/offline states, cache behavior, and ownership-safe manifest handling.
3. `cargo test --locked --test app_view_model --test app_ui_sizing --test app_strings` covers fixed dashboard presentation, transition logs, retained values, and addon compatibility wording.
4. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked` to completion before committing Rust changes.
5. Review UTF-8 without BOM, LF, no mojibake, complete spec requirement coverage, and relevant canonical documentation.
6. After publication, require green hosted Windows/Linux checks and every external review finding resolved. Count at most two Codex rounds. The owner merges; no release cut or field verification is included.
