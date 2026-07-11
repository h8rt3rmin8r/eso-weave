# Tasks: Beacon Manager

**Feature**: `specs/006-beacon-manager` | **Branch**: `006-beacon-manager`

Test-first per constitution Principle III. The safety-critical surfaces (the
marker-gated uninstall and the write/delete confinement to the resolved
`PixelBeacon` subtree) live in the pure core and are covered by required,
non-weakened tests against a temporary AddOns root before the code lands. Paths
are repository-relative.

## Phase 1: Setup

- [x] T001 Declare `pub mod beacon;` in `src/lib.rs` and create compiling stub files `src/beacon/mod.rs`, `src/beacon/steam.rs`, `src/beacon/windows.rs` (`cfg(windows)`), and `src/beacon/linux.rs` (`cfg(target_os = "linux")`), warning-free.

## Phase 2: Foundational

- [x] T002 Define the core types in `src/beacon/mod.rs`: `Environment` (with `segment()` -> `"live"`/`"pts"`), `BeaconStatus`, `RunningState`, `DiscoveryError`, `LifecycleOutcome`, `LifecycleError`, and embed the addon via `MANIFEST`/`LUA` `include_str!` from `addon/PixelBeacon/`.
- [x] T003 [P] Write failing tests in `tests/beacon.rs`: `has_managed_marker` is true for the exact marker line and false otherwise; `parse_manifest_version` parses the `## Version:` value and returns `None` when absent or unparsable; `embedded_version` equals the embedded manifest's `## Version:` and the embedded manifest carries the marker line; `reload_reminder` is true for `Running` and `Unknown` and false for `NotRunning`.
- [x] T004 Implement the pure helpers `has_managed_marker`, `parse_manifest_version`, `embedded_version`, and `reload_reminder` in `src/beacon/mod.rs` (FR-008, FR-012, FR-018). Run the foundational tests to green.

## Phase 3: User Story 2 - Verify installed status (P1)

- [x] T005 [P] [US2] Write failing tests in `tests/beacon.rs` over a `tempfile` AddOns root for all four states: `NotInstalled` (no `PixelBeacon` folder, and folder with no manifest); `ManagedUpToDate` (marker plus embedded version); `ManagedVersionMismatch` (marker but a differing or unparsable version); `Unmanaged` (manifest without the marker line).
- [x] T006 [US2] Implement `status(addons_root: &Path) -> BeaconStatus` in `src/beacon/mod.rs` (FR-011 to FR-014); reads only. Run US2 tests to green.

## Phase 4: User Story 1 - Install the beacon addon (P1)

- [x] T007 [P] [US1] Write failing tests in `tests/beacon.rs`: install into an existing tempdir root writes `PixelBeacon/PixelBeacon.txt` and `PixelBeacon.lua` byte-equal to the embedded copies with the marker line and embedded version, returns `ManagedUpToDate`; install when the root is not an existing directory returns `Err(AddonsDirMissing)`; over-installing onto an older manifest yields `ManagedUpToDate`; a sibling sentinel elsewhere under the root is untouched (write confinement).
- [x] T008 [US1] Implement `install(addons_root: &Path, running: RunningState) -> Result<LifecycleOutcome, LifecycleError>` in `src/beacon/mod.rs`: require an existing root, create/populate only `PixelBeacon/`, overwrite in place, set `reload_required` via `reload_reminder` (FR-006 to FR-010, FR-018). Run US1 tests to green.

## Phase 5: User Story 3 - Uninstall only what we manage (P1, safety-critical)

- [x] T009 [P] [US3] Write failing safety tests in `tests/beacon.rs`: uninstall of a managed folder (marker present in the on-disk manifest) removes exactly `PixelBeacon/` and `status` then reports `NotInstalled`; uninstall of a folder whose manifest lacks the marker line returns `Err(Unmanaged)` with the folder and its files intact; uninstall of a folder with no manifest returns `Err(Unmanaged)` and deletes nothing; a sibling sentinel under the root survives every uninstall (delete confinement).
- [x] T010 [US3] Implement `uninstall(addons_root: &Path, running: RunningState) -> Result<LifecycleOutcome, LifecycleError>` in `src/beacon/mod.rs`: re-read the on-disk manifest, gate deletion on the verified marker line, remove exactly `PixelBeacon/`, set `reload_required` (FR-015 to FR-018). Run US3 tests to green.

## Phase 6: User Story 4 - Reload reminder while the game runs (P2)

- [x] T011 [US4] Add tests in `tests/beacon.rs` asserting that both `install` and `uninstall` return `LifecycleOutcome.reload_required == true` for `RunningState::Running` and `RunningState::Unknown` and `false` for `RunningState::NotRunning` (FR-018, FR-019), reusing a managed tempdir root. Confirm green (the rule is already wired in T008/T010).

## Phase 7: Discovery and platform backends (thin)

- [x] T012 [P] Write failing tests in `tests/beacon.rs` for `steam::library_paths_for_app(vdf, "306130")` over crafted `libraryfolders.vdf` text: a single library containing the app returns its path; multiple libraries with only one containing the app returns just that path; no library containing the app returns empty.
- [x] T013 Implement `library_paths_for_app` in `src/beacon/steam.rs` (pure, compiled on all targets). Run the VDF tests to green.
- [x] T014 Implement `resolve_addons_dir(prefs) -> Result<PathBuf, DiscoveryError>` dispatch in `src/beacon/mod.rs` with a pure, host-tested path composer (`Elder Scrolls Online/<env segment>/AddOns`) and override-precedence branch; add host tests for override precedence and the composed subpath (FR-003 to FR-005).
- [x] T015 Implement the Windows backend in `src/beacon/windows.rs` (`cfg(windows)`): resolve the AddOns dir from `dirs::document_dir()` (no literal path) and implement `probe_game_running` via a `windows-sys` process snapshot returning `RunningState`. Clippy-clean on the host.
- [x] T016 Implement the Linux backend in `src/beacon/linux.rs` (`cfg(target_os = "linux")`): resolve the Steam root, read `steamapps/libraryfolders.vdf`, use `steam::library_paths_for_app` to find the library, compose the compatdata AddOns path, and implement `probe_game_running` via `/proc`. Type-check with the linux target.

## Phase 8: Settings and observability

- [x] T017 Add the additive opaque `beacon` section to `Settings` in `src/config/mod.rs` (field plus `RawSettings` plus default, no schema bump) and implement `BeaconPrefs` (de)serialization to/from it in `src/beacon/mod.rs`, with a host test round-tripping prefs and defaulting on absent or null (FR-004, constitution additive-settings).
- [x] T018 Add `tracing` records to `resolve_addons_dir`, `install`, and `uninstall` (success at info; `NotFound`/`AddonsDirMissing`/`Unmanaged` at warn; unexpected I/O at error), logging no file contents (FR-021).

## Phase 9: Polish and cross-cutting

- [x] T019 [P] Add module and item documentation across `src/beacon/*.rs`.
- [x] T020 Update `CHANGELOG.md` `[Unreleased]` with an Added line for the Beacon Manager (no new crates; additive `beacon` settings section) and a dated Decisions entry for the two architecture-affecting choices: single-sourcing the embedded addon version from the embedded manifest, and reusing the additive opaque config-section pattern for beacon settings.
- [x] T021 Run CI parity: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, plus `cargo check --target x86_64-unknown-linux-gnu`.

## Dependencies and order

- Setup (T001) then Foundational (T002 to T004). Verify (US2, T005 to T006)
  precedes Install and Uninstall because their tests assert post-operation status.
  Install (US1, T007 to T008) and Uninstall (US3, T009 to T010) build on the pure
  helpers and status. The reminder acceptance (US4, T011) follows install and
  uninstall. Discovery and the platform backends (T012 to T016) depend on the core
  types but not on the lifecycle logic. Settings and observability (T017 to T018)
  follow the operations they wrap. Polish (T019 to T021) last; T021 is the gate.

## Parallel opportunities

- The test tasks (T003, T005, T007, T009, T012) all share `tests/beacon.rs` and
  land sequentially despite the `[P]` marker on their authoring.
- T015 (Windows backend) and T016 (Linux backend) touch different files and can be
  written in parallel.
- Documentation (T019) is independent of the platform-specific code once the core
  is in place.

## MVP scope

Verify (US2) plus Install (US1) plus the safety-critical marker-gated Uninstall
(US3) over an injected AddOns root are the minimum viable increment: the full
lifecycle with its safety guarantees, fully tested without a real ESO install. The
reminder acceptance, the discovery seam, the platform backends, and settings wiring
complete the feature.
