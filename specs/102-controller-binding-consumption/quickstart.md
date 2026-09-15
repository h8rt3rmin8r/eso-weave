# Quickstart: Controller Binding Consumption

1. Start from `codex/s102-controller-binding-consumption` with issue #208 linked.
2. Add failing tests for Interact and Quickslot native action attempts, invalidation, modifiers, failure cleanup, and retry accounting.
3. Add failing legacy settings and read-only presentation tests.
4. Implement shared autonomous authority and complete chord execution in `src/input/mod.rs`.
5. Move the Fishing and Auto Potion real sinks to their respective native actions.
6. Remove the duplicate config fields and settings controls.
7. Update docs, captures, Plan 043, and `CHANGELOG.md`.
8. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked`, followed by the repository's extended verification commands.
9. Push and open the official PR, resolve every review, request exactly one second Codex round, and wait for green CI.
