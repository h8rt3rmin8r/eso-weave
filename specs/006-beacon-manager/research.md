# Phase 0 Research: Beacon Manager

All decisions below were made under the Build-Phase Autopilot Protocol against the
constitution, the master specification (sections 9.4, 9.5), and the existing code
patterns. None were escalated.

## Decision: Embed the addon files with `include_str!` and single-source the version

- **Decision**: Embed `addon/PixelBeacon/PixelBeacon.txt` and
  `addon/PixelBeacon/PixelBeacon.lua` into the binary with `include_str!`. Derive
  the embedded version by parsing the `## Version:` line of the embedded manifest
  at runtime rather than declaring a separate constant.
- **Rationale**: The spec requires install to write the canonical files verbatim
  and verify to compare an embedded version. Parsing the version from the embedded
  manifest guarantees there is exactly one source of truth, so the written file and
  the compared version can never drift. `include_str!` keeps the files in the repo
  as the authored artifact from feature 004.
- **Alternatives considered**: A hand-maintained `EMBEDDED_VERSION` constant
  (rejected: duplicates the manifest and can drift). Reading the files from disk at
  runtime (rejected: the app must be self-contained; the master spec says the
  application ships the canonical copy).

## Decision: Represent the beacon settings as an additive opaque config section

- **Decision**: Add a `#[serde(default)] beacon: serde_json::Value` section to
  `Settings`, owned and (de)serialized by the beacon module into a `BeaconPrefs`
  (a `path_override: Option<PathBuf>` and an `environment: Environment`). No
  `schema_version` bump.
- **Rationale**: This mirrors the existing `timing` and `skills` sections exactly,
  keeps the config module decoupled from the beacon module's shape, and is backward
  and forward compatible per the constitution's additive-settings rule.
- **Alternatives considered**: A typed field on `Settings` (rejected: couples
  config to the beacon module and diverges from the established opaque-section
  pattern). A new schema version (rejected: unnecessary for a purely additive
  field).

## Decision: Resolve the Windows AddOns directory via the Documents known folder

- **Decision**: Use `dirs::document_dir()` (which calls
  `SHGetKnownFolderPath(FOLDERID_Documents)` on Windows) and join
  `Elder Scrolls Online/<env>/AddOns`. Never assume a literal `Documents` path.
- **Rationale**: The master spec explicitly requires resolving a possibly relocated
  Documents folder through the shell API. `dirs` is already a dependency and wraps
  the correct known-folder call, so no new crate and no literal-path assumption.
- **Alternatives considered**: Calling `SHGetKnownFolderPath` directly through
  `windows-sys` (rejected: `dirs` already provides it, avoiding unsafe FFI here).
  Hardcoding `%USERPROFILE%/Documents` (rejected: violates FR-002 and the master
  spec).

## Decision: Parse `libraryfolders.vdf` with a small purpose-built extractor

- **Decision**: On Linux, locate the Steam root
  (`~/.steam/steam` or `~/.local/share/Steam`, and `~/.var/app/...` Flatpak
  fallback), read `steamapps/libraryfolders.vdf`, and extract, for each library
  block, its `path` and whether its `apps` map contains app id `306130`. Resolve
  the matching library's
  `steamapps/compatdata/306130/pfx/drive_c/users/steamuser/Documents/Elder Scrolls Online/<env>/AddOns`.
  The VDF text extraction is a pure function in `steam.rs`, compiled on all targets
  and unit-tested on the host.
- **Rationale**: The needed slice of VDF is tiny (quoted key/value tokens in nested
  blocks). A focused extractor keeps the single-crate, minimal-dependency posture
  the constitution favors and is fully testable with crafted VDF text on the
  Windows host, so the Linux discovery logic has host test coverage.
- **Alternatives considered**: Adding a VDF/`steamlocate` crate (rejected:
  unnecessary dependency for a small, stable format). Assuming a single default
  library path (rejected: multi-library installs are common and FR-001 requires
  locating the library that actually contains the app id).

## Decision: Inject the AddOns root and the running state; keep lifecycle logic pure

- **Decision**: `status`, `install`, and `uninstall` operate on a caller-provided
  `addons_root: &Path`. Discovery (`resolve_addons_dir`) is a separate seam that
  produces that path or a typed `DiscoveryError`. The running-game state is an
  injected `RunningState` enum (`Running`, `NotRunning`, `Unknown`); the reminder
  decision is a pure function of it. A thin platform `probe_game_running()` returns
  `RunningState` in production.
- **Rationale**: This is the seam that makes the safety-critical logic testable
  against a `tempfile` directory without the real known-folder API or ESO process,
  satisfying Principle III and FR-020. Keeping the reminder rule pure lets FR-018
  and its fail-safe (Unknown surfaces the reminder) be tested directly.
- **Alternatives considered**: Having lifecycle functions perform discovery
  internally (rejected: not injectable, forces real-environment tests). A boolean
  running flag (rejected: cannot express the indeterminate case the spec's
  fail-safe requires).

## Decision: Install creates only the `PixelBeacon` subfolder; the AddOns dir must pre-exist

- **Decision**: Install confirms `addons_root` is an existing directory, then
  creates and populates only `addons_root/PixelBeacon`. It does not create the
  `Elder Scrolls Online/<env>/AddOns` chain itself. All writes target paths under
  `addons_root/PixelBeacon`.
- **Rationale**: FR-010 requires confirming an existing AddOns directory before any
  write, and FR-009 confines writes to the `PixelBeacon` subtree. A missing AddOns
  directory means the game is not installed for that environment, which is a
  not-found condition, not something to fabricate.
- **Alternatives considered**: Creating the whole path chain (rejected: would write
  outside a verified AddOns directory and could mask a wrong path). Writing a temp
  dir then renaming (considered for atomicity; rejected as unnecessary because the
  master spec states install-over-existing is always safe and the two-file write is
  simple, though a failed write is still reported as a failure per FR-010).

## Decision: Uninstall re-reads the on-disk manifest and gates deletion on the marker

- **Decision**: Uninstall reads `addons_root/PixelBeacon/PixelBeacon.txt` from disk
  at the moment of the call, checks for the exact managed-marker line, and only then
  removes exactly `addons_root/PixelBeacon`. A missing manifest or a manifest
  without the marker yields an `Unmanaged` refusal with nothing deleted.
- **Rationale**: This is the constitution's named safety surface. Re-reading the
  actual on-disk manifest (not a cached or assumed value) guarantees the gate
  reflects reality, and deleting exactly the `PixelBeacon` folder keeps the delete
  confined per FR-017.
- **Alternatives considered**: Trusting a prior `status` result (rejected: the
  on-disk state may have changed; the gate must read the manifest it is about to
  delete next to). Deleting the AddOns root or globbing (rejected: violates
  confinement).

## Decision: Windows/Linux running-game probe is best-effort and advisory

- **Decision**: On Windows, snapshot processes and look for the ESO executable
  name; on Linux, scan `/proc/*/comm` (or cmdline) for it. Any error or ambiguity
  yields `RunningState::Unknown`. The probe never blocks or fails the lifecycle
  operation.
- **Rationale**: FR-019 forbids the probe from affecting whether an operation
  proceeds; FR-018 only uses it to decide the reminder, with Unknown failing safe
  toward reminding. A read-only process-existence check stays within Principle V
  (no memory or gameplay access).
- **Alternatives considered**: Window-title or focus detection (rejected: heavier
  and unnecessary; existence is enough for the reminder). Treating an error as
  NotRunning (rejected: the spec's fail-safe direction is to remind).
