# S126 Repository Validation

Use the existing Rust toolchain and synthetic Lua harness; no game installation
or owner field-verification task is required.

1. Run cargo test --test encounter_addon retained to exercise two single sessions,
   mixed environments, reload, resume, capacity and confirmed discard.
2. Run cargo test --test encounter_import and cargo test --test encounter_history
   for strict parsing, atomic multi-session import, migration, snapshot collisions,
   repeated import and deletion/reimport order.
3. Run cargo test --test app_encounter_history and cargo test --test app_ui_sizing
   for selected-environment source summaries, stale workers and bounded workflow UI.
4. Run the complete fmt, Clippy, locked-test and documentation/rendering gates
   recorded in verification.md before committing.
5. After automatic PR publication, inspect every external review thread and
   final-head checks. Use no more than the authorized second review round.

Expected: all terminal identities/payloads and catalog bytes survive ordinary
start/reload/resume; confirmed deletion affects exactly its target; invalid
sources write no history. Repository results do not prove installed game behavior.
