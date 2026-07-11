# Contract: Beacon Manager (`beacon` module)

This is the public surface the later GUI slice consumes. Signatures are the
intended Rust shape; exact names may be refined during implementation as long as
the behavior and the safety guarantees below hold.

## Types

```rust
pub enum Environment { Live, Pts }

pub struct BeaconPrefs {
    pub path_override: Option<std::path::PathBuf>,
    pub environment: Environment,
}

pub enum BeaconStatus {
    NotInstalled,
    ManagedUpToDate,
    ManagedVersionMismatch,
    Unmanaged,
}

pub enum RunningState { Running, NotRunning, Unknown }

pub enum DiscoveryError { NotFound, Unsupported }

pub struct LifecycleOutcome {
    pub status: BeaconStatus,
    pub reload_required: bool,
}

pub enum LifecycleError {
    AddonsDirMissing,
    Unmanaged,
    Io(std::io::Error),
}
```

## Pure functions (fully unit-tested, no I/O)

```rust
pub fn has_managed_marker(manifest: &str) -> bool;
pub fn parse_manifest_version(manifest: &str) -> Option<u32>;
pub fn embedded_version() -> u32;              // parse_manifest_version(MANIFEST)
pub fn reload_reminder(state: RunningState) -> bool;   // true for Running and Unknown
```

- `has_managed_marker`: true iff some line, trimmed, equals
  `## X-ESO-Weave-Managed: true`.
- `parse_manifest_version`: value of the first `## Version:` line parsed as `u32`;
  `None` if absent or unparsable.
- `reload_reminder`: `Running => true`, `Unknown => true`, `NotRunning => false`.

## Discovery seam

```rust
pub fn resolve_addons_dir(prefs: &BeaconPrefs) -> Result<std::path::PathBuf, DiscoveryError>;
```

- If `prefs.path_override` is `Some(p)` and `p` is an existing directory, return
  `p` joined nothing further is required (the override IS the AddOns root).
- Otherwise dispatch to the platform backend:
  - **Windows**: `dirs::document_dir()` then
    `Elder Scrolls Online/<env>/AddOns`. Never assume a literal Documents path.
  - **Linux**: read `steamapps/libraryfolders.vdf`, use
    `steam::library_paths_for_app(vdf, "306130")` to find the library, then
    `steamapps/compatdata/306130/pfx/drive_c/users/steamuser/Documents/Elder Scrolls Online/<env>/AddOns`.
- Return `DiscoveryError::NotFound` when neither an override nor auto-discovery
  yields a path. `resolve_addons_dir` never creates directories.

```rust
pub fn probe_game_running() -> RunningState;   // thin platform adapter; never panics
```

## Lifecycle operations (over an injected AddOns root)

```rust
pub fn status(addons_root: &std::path::Path) -> BeaconStatus;

pub fn install(addons_root: &std::path::Path, running: RunningState)
    -> Result<LifecycleOutcome, LifecycleError>;

pub fn uninstall(addons_root: &std::path::Path, running: RunningState)
    -> Result<LifecycleOutcome, LifecycleError>;
```

### `status` contract

- `addons_root/PixelBeacon` absent, or its `PixelBeacon.txt` unreadable ->
  `NotInstalled`.
- Manifest readable and marker present:
  - parsed `## Version:` equals `embedded_version()` -> `ManagedUpToDate`.
  - else -> `ManagedVersionMismatch`.
- Manifest readable and marker absent -> `Unmanaged`.
- `status` performs only reads; it never writes or deletes.

### `install` contract

1. If `addons_root` is not an existing directory -> `Err(AddonsDirMissing)`.
2. Ensure `addons_root/PixelBeacon` exists (create if absent).
3. Write `PixelBeacon/PixelBeacon.txt` = `MANIFEST` and
   `PixelBeacon/PixelBeacon.lua` = `LUA`, overwriting existing files (safe
   update). Any I/O failure -> `Err(Io(..))` with no success claimed.
4. On success return `LifecycleOutcome { status: ManagedUpToDate, reload_required:
   reload_reminder(running) }`.
5. **Guarantee**: every path written is under `addons_root/PixelBeacon`. Nothing
   outside that subtree is ever created or modified.

### `uninstall` contract (safety-critical)

1. Read `addons_root/PixelBeacon/PixelBeacon.txt` from disk now.
2. If the folder or manifest is absent, or the manifest lacks the managed-marker
   line -> `Err(Unmanaged)`, delete nothing.
3. Only when the marker line is verified present: remove exactly
   `addons_root/PixelBeacon` (recursively). Any I/O failure -> `Err(Io(..))`.
4. On success return `LifecycleOutcome { status: NotInstalled, reload_required:
   reload_reminder(running) }`.
5. **Guarantee**: nothing outside `addons_root/PixelBeacon` is ever deleted, and a
   folder lacking the verified marker line is never deleted.

## Settings integration

- `BeaconPrefs` (de)serializes to/from the additive opaque `Settings.beacon`
  section (`serde_json::Value`), owned by the beacon module. Absent/null -> default
  (`Live`, no override). No config `schema_version` bump.

## Observability

- `resolve_addons_dir`, `status` (when called as an action), `install`, and
  `uninstall` emit a structured `tracing` record: success at `info`; `NotFound`,
  `AddonsDirMissing`, and the `Unmanaged` uninstall refusal at `warn`; unexpected
  I/O at `error`. Addon file contents are never logged.

## Required safety tests (never weakened or skipped)

- Uninstall deletes a managed folder (marker present) and verify then reports
  `NotInstalled`.
- Uninstall of a folder whose manifest lacks the marker line returns `Unmanaged`
  and leaves the folder and its files intact.
- Uninstall of a folder with no manifest returns `Unmanaged` and deletes nothing.
- Install writes only under `addons_root/PixelBeacon`; a sentinel file/dir
  elsewhere in `addons_root` is untouched.
- Uninstall never removes anything outside `addons_root/PixelBeacon` (a sibling
  sentinel survives).
