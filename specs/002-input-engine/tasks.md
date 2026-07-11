# Tasks: Input Engine

**Feature**: `specs/002-input-engine` | **Branch**: `002-input-engine`

Test-first per constitution Principle III. The safety-critical behaviors
(recursion breaking, focused-window-only suppression, non-blocking interception
path) are covered by required tests against `MockBackend` before the code lands.
Paths are repository-relative. `[P]` marks tasks on different files with no
incomplete dependency.

## Phase 1: Setup

- [x] T001 Update `Cargo.toml`: add target-specific dependencies (`windows-sys` under `[target.'cfg(windows)'.dependencies]` with the hook, SendInput, foreground-window, and timer features; `evdev` under `[target.'cfg(target_os = "linux")'.dependencies]`).
- [x] T002 Declare `pub mod input;` in `src/lib.rs` and create compiling stub files `src/input/mod.rs`, `key.rs`, `action.rs`, `bindings.rs`, `mock.rs`, `windows.rs`, `linux.rs` (backends behind their `cfg`), each warning-free.

## Phase 2: Foundational (blocking prerequisites)

- [x] T003 Implement the platform-neutral `Key` identifier with canonical string parse and display in `src/input/key.rs` (at least Digit1..Digit5, R, X, F1, F2).
- [x] T004 Implement the `Action` enum and the section 6.4 default action-to-key table (with suspend-exempt flags) in `src/input/action.rs`.
- [x] T005 Implement `Binding` and `BindingTable` in `src/input/bindings.rs`: `default()`, `lookup(key)`, and `rebind(action, key)` with conflict rejection that leaves the table unchanged (FR-014, FR-015).
- [x] T006 Add an additive `bindings` section to `config::Settings` (serde default to the section 6.4 defaults, backward compatible, no schema bump) plus load/store helpers that return notices for conflicting or unknown entries with per-action fallback (FR-016, FR-017, FR-021).

## Phase 3: User Story 1 - Focused interception with non-blocking hand-off (P1)

**Goal**: FR-001 through FR-007, FR-020, FR-023. Safety-critical.

**Independent test**: Feed synthetic events via `MockBackend` with focus set and
cleared; confirm suppression, single hand-off on new key-down, pass-through of
unbound keys, both-transition suppression, auto-repeat coalescing, and a
non-blocking full-channel drop.

- [x] T007 [P] [US1] Write failing tests in `tests/input_engine.rs`: focused bound key-down suppresses and hands off exactly one action; unbound key passes through; unfocused key is never suppressed or handed off; bound key-up is suppressed; auto-repeat key-downs suppress without extra hand-offs; a full hand-off channel drops the action without blocking.
- [x] T008 [US1] Implement the `InputEngine` core in `src/input/mod.rs`: `KeyEvent`, `Transition`, `Origin`, `Decision`, `InputError`, the `InputBackend` trait, atomics for focused/suspended, a held-key set, a bounded hand-off channel, and `classify` implementing FR-001 to FR-007, FR-020, FR-023.
- [x] T009 [US1] Implement `MockBackend` in `src/input/mock.rs` to feed crafted `KeyEvent`s to `classify` and capture synthesized output (FR-018). Run `tests/input_engine.rs` to green for US1.

## Phase 4: User Story 2 - Self-input never re-intercepted (P1)

**Goal**: FR-008, FR-009, FR-010. Safety-critical.

**Independent test**: A self-originated event for a bound key while focused is not
suppressed and not handed off; a later real press of the same key is intercepted.

- [x] T010 [P] [US2] Write failing tests in `tests/input_engine.rs`: a self-originated bound key-down while focused yields pass and no hand-off; a subsequent real press of the same key is suppressed and handed off; synthesized output is recorded by `MockBackend`. Confirm `classify` in T008 satisfies these (extend the core only if a gap is found).

## Phase 5: User Story 3 - Suspend with suspend-exempt toggles (P2)

**Goal**: FR-011, FR-012, FR-013.

**Independent test**: While suspended, non-exempt bound keys pass through and
suspend-exempt keys remain intercepted; resuming restores interception.

- [x] T011 [P] [US3] Write failing tests in `tests/input_engine.rs`: suspended non-exempt bound key passes through; suspended suspend-exempt key is intercepted and handed off; after resume a non-exempt bound key is intercepted again.
- [x] T012 [US3] Ensure `classify` and `set_suspended` in `src/input/mod.rs` implement the suspend gating and suspend-exempt exemption (FR-011 to FR-013). Run US3 tests to green.

## Phase 6: User Story 4 - Configurable, conflict-free keybindings (P3)

**Goal**: FR-014 through FR-017, FR-021, FR-022.

**Independent test**: Defaults match section 6.4; rebind to a free key persists;
a colliding rebind is rejected leaving both unchanged; a persisted conflicting or
unknown entry falls back to defaults with a notice.

- [x] T013 [P] [US4] Write failing tests in `tests/input_engine.rs`: default table matches section 6.4; `rebind` to a free key is accepted and, after store then load through a temp config dir, persists; a colliding `rebind` is rejected and both bindings are unchanged; loading a settings file whose bindings map two actions to one key (or name an unknown key) falls back for the affected actions and returns a notice.
- [x] T014 [US4] Wire `InputEngine::load_bindings`/`store_bindings` to the config helpers from T006 and expose `rebind` (FR-016, FR-017). Run US4 tests to green.

## Phase 7: Platform backends (thin adapters)

- [x] T015 Implement the Windows backend in `src/input/windows.rs` (`cfg(windows)`): `WH_KEYBOARD_LL` hook calling `classify`, `SendInput` synthesis, injected-input flag mapped to `Origin::SelfOriginated`, `GetForegroundWindow` feeding focus, and `timeBeginPeriod`/`timeEndPeriod` around the worker lifetime. Must compile clippy-clean on the host.
- [x] T016 Implement the Linux backend in `src/input/linux.rs` (`cfg(target_os = "linux")`): evdev grab for reading, uinput virtual device for synthesis, X11 active-window focus with a documented pure-Wayland limitation, and a clear startup error on missing permission (FR-019). Type-check with `cargo check --target x86_64-unknown-linux-gnu` where available.

## Phase 8: Polish and cross-cutting

- [x] T017 [P] Add module and item documentation across `src/input/*.rs`.
- [x] T018 Update `CHANGELOG.md` `[Unreleased]`: an Added line for the Input Engine and a dated Decisions entry for the new target-specific dependencies (`windows-sys`, `evdev`).
- [x] T019 Run CI parity in the foreground: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`; plus `cargo check --target x86_64-unknown-linux-gnu` if the target is installable, and record any Linux verification gap for the pre-push halt.

## Dependencies and order

- Setup (T001, T002) first. Foundational (T003 to T006) next.
- US1 (T007 to T009) is the core; US2 (T010) validates recursion breaking already
  built into `classify`; US3 (T011, T012) and US4 (T013, T014) build on the core.
- Backends (T015, T016) depend on the finished core and trait.
- Polish (T017 to T019) last; T019 is the CI parity gate.

## Parallel opportunities

- The per-story test tasks (T007, T010, T011, T013) can be drafted in parallel;
  they share one test file so land them sequentially to avoid churn.
- T015 (Windows) and T016 (Linux) touch different files and can proceed in
  parallel once the core is done.

## MVP scope

User Story 1 plus Setup and Foundational is the minimum viable increment: correct,
non-blocking, focus-scoped interception with recursion-safe hand-off, testable end
to end through `MockBackend`. Suspend, bindings persistence, and the OS backends
follow.
