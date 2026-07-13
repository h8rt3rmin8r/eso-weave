---

description: "Task list for ESO API Version Check Automation"
---

# Tasks: ESO API Version Check Automation

**Input**: Design documents from `specs/018-eso-api-version-check/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api-check.md

**Tests**: REQUIRED. This slice touches a safety-critical surface (marker-gated
manifest writes), which the constitution requires to stay tested. Tests are
written before or alongside the pure logic, red before green.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: US1, US2, US3, or FOUND (foundational) / POLISH

## Phase 1: Setup

- [ ] T001 Add `ureq` (2.x, rustls TLS) to `Cargo.toml` dependencies. Run
  `cargo fetch` to confirm it resolves. Record a dated `### Decisions` entry in
  `CHANGELOG.md` naming the added networked dependency and the GitHub source.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Types, constants, and pure functions every story depends on.

- [ ] T002 [FOUND] In `src/beacon/api_check.rs` (new module; declare `mod api_check;`
  in `src/beacon/mod.rs`), define `GameVersion([u16; 4])` with the derives from
  data-model.md and `parse_commit_message_version(&str) -> Option<GameVersion>`.
- [ ] T003 [P] [FOUND] Unit tests for `parse_commit_message_version`: `"12.0.6"`,
  `"12.0.0 Season Zero Pt.2"`, single-component, over-four-components, empty, and
  non-numeric inputs; plus `GameVersion` ordering.
- [ ] T004 [FOUND] In `src/beacon/mod.rs`, add `DEFAULT_API_VERSION: u32 = 101050`
  and `DEFAULT_GAME_VERSION: GameVersion` (`12.0.6`).
- [ ] T005 [FOUND] In `src/beacon/mod.rs`, add pure `parse_api_version_primary`,
  `rewrite_api_version` (multi-value token rule), and `render_manifest`.
- [ ] T006 [P] [FOUND] Unit tests for the manifest functions: primary parse; token
  rule (primary set, greater kept, lesser dropped); every other line and the
  managed marker preserved; idempotence when already current.
- [ ] T007 [FOUND] In `src/config/state.rs`, add `ApiVersionCache` (Copy) and the
  additive `#[serde(default)] api_version` field on `SessionState`; bump
  `CURRENT_STATE_VERSION` to 2.
- [ ] T008 [P] [FOUND] Unit tests: `SessionState` deserializes a v1 JSON (no
  `api_version` key) to defaults, and round-trips a v2 value.

**Checkpoint**: pure types and functions compile and are tested.

---

## Phase 3: User Story 1 - Installed addon stays API-current (Priority: P1)

**Goal**: On startup, the on-disk manifest is kept at the best known API version,
marker-gated and never downgraded, and a detected game bump is surfaced.

**Independent Test**: quickstart checks 2, 3, 4, and 5.

- [ ] T009 [US1] In `src/beacon/api_check.rs`, define the `GameVersionSource` trait,
  `ApiCheckError`, `ApiCheckOutcome`, and pure `run_check(...)` per
  contracts/api-check.md (resolution, marker-gated rewrite, no downgrade, bump
  notice via `tracing::warn!`). Add a `MockSource` for tests behind `#[cfg(test)]`.
- [ ] T010 [P] [US1] Tests for `run_check` against a `tempfile` AddOns root:
  rewrite when primary `<` effective and marker present; no write when marker
  absent (safety gate); no write when primary `>=` effective (no downgrade/churn);
  fetch error is swallowed; bump surfaced when fetched game version is newer.
- [ ] T011 [US1] Implement `GithubLiveSource` (ureq GET on the GitHub live-commit
  endpoint with `User-Agent` and a short timeout, JSON `.commit.message` parse).
- [ ] T012 [US1] In `src/app/mod.rs`: add the `ApiVersionCache` field to `AppModel`;
  include it in `current_session_state()`; set it in `restore_session()`; add
  `apply_api_check(&mut self, outcome)` that updates the cache and calls
  `scheduler.mark_session(now)` when it changed.
- [ ] T013 [US1] In `src/app/ui.rs`, add an `mpsc::Receiver<ApiCheckOutcome>` to
  `EsoWeaveApp` (constructor parameter beside `toggle_rx`) and drain it each frame
  in `ui`, calling `model.apply_api_check(...)`.
- [ ] T014 [US1] In `src/main.rs`, before `settings`/`session` are moved: clone the
  beacon prefs and the restored `last_known_api_version`/`last_seen_game_version`;
  create the `mpsc::channel::<ApiCheckOutcome>`; spawn a `std::thread` that calls
  `beacon::api_check::run_check` with a `GithubLiveSource`; pass the receiver into
  `EsoWeaveApp::new`.

**Checkpoint**: startup upkeep works end to end; safety gate and no-downgrade hold.

---

## Phase 4: User Story 2 - Correct version ready before install (Priority: P2)

**Goal**: A first install writes the best known API version.

**Independent Test**: install with the addon absent; installed manifest carries the
resolved version.

- [ ] T015 [US2] Change `beacon::install` to render the manifest with a resolved
  `api_version: u32` (via `render_manifest`) instead of writing `MANIFEST`
  verbatim; update the call site `AppModel::install_beacon` to resolve
  `effective = stored.unwrap_or(0).max(DEFAULT_API_VERSION)` and pass it.
- [ ] T016 [P] [US2] Update `tests/beacon.rs`: assert the installed manifest's
  APIVersion primary equals the resolved value; keep the existing marker and
  version assertions; add an install-with-default case.

**Checkpoint**: install path uses the resolved version; US1 still passes.

---

## Phase 5: User Story 3 - A version is always available (Priority: P3)

**Goal**: With no network and no stored value, a valid default is always used.

**Independent Test**: quickstart check 1.

- [ ] T017 [P] [US3] Test: with a `MockSource` returning an error and no stored
  value, `run_check` returns `last_known_api_version == DEFAULT_API_VERSION` and
  any rendered manifest has a valid APIVersion; no panic.

**Checkpoint**: offline first-run is safe.

---

## Phase 6: Polish & Cross-Cutting

- [ ] T018 Run the full CI-parity gate in the foreground:
  `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all --locked`. All green.
- [ ] T019 Add an `[Unreleased] > Added` line in `CHANGELOG.md` for the automation
  (the Decisions entry was added in T001).
- [ ] T020 Run the quickstart manual behavior checks and confirm the observed
  outcomes.

---

## Dependencies & Execution Order

- Phase 1 (T001) first: the dependency must resolve before code compiles.
- Phase 2 (T002 to T008) blocks all stories. Within it, T003/T006/T008 [P] pair
  with their implementation tasks.
- US1 (T009 to T014) is the MVP and depends only on Foundational.
- US2 (T015 to T016) depends on `render_manifest` (T005) and the resolution helper
  used in US1; independently testable.
- US3 (T017) depends on `run_check` (T009).
- Polish (T018 to T020) last.

## Notes

- Write each test red before its implementation turns it green.
- No blocking work is added to the input hook thread; the check runs on its own
  `std::thread`.
- Never write a manifest lacking the managed marker; never downgrade the on-disk
  API version. These are the safety-critical assertions and must stay tested.
