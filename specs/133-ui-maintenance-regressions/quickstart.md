# Regression validation

Run in the repository with the pinned Rust toolchain and bundled documentation tools. All fixtures use temporary directories and synthetic egui frames.

1. Run targeted S133 tests for app_ui_sizing, app_view_model, app_settings and s129_package, plus maintenance unit tests. Assert geometry commands, restart round trips, modal dismissal/confirm, compatibility fixtures and captured retry logs.
2. Run cargo fmt --all -- --check, cargo clippy --all-targets --all-features -- -D warnings and cargo test --all --locked to completion before commit.
3. Run applicable docs and trust policy checks, then inspect CI on the exact PR head.

No installed game/owner field check or release is part of this validation.
