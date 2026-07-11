# Phase 1 Data Model: Beacon Manager

All types live in the `beacon` module. The pure types below carry the tested
logic; the platform backends only produce a resolved `PathBuf` or a
`RunningState`.

## Environment

The ESO game environment selecting the AddOns subdirectory.

| Variant | Directory segment | Notes |
| --- | --- | --- |
| `Live` | `live` | Default. |
| `Pts`  | `pts`  | Selectable in settings. |

- Serialized in settings as the lowercase segment string.
- `Environment::segment(&self) -> &'static str` returns `"live"` or `"pts"`.

## BeaconPrefs

The beacon module's view of its opaque `beacon` settings section.

| Field | Type | Default | Meaning |
| --- | --- | --- | --- |
| `path_override` | `Option<PathBuf>` | `None` | A manual AddOns directory; when set, takes precedence over auto-discovery. |
| `environment` | `Environment` | `Live` | The selected game environment. |

- (De)serialized from the `Settings.beacon` opaque `serde_json::Value`; absent or
  null yields defaults. Additive, no schema bump.

## EmbeddedAddon

The canonical addon shipped in the binary. Not a stored struct so much as module
constants plus a derived version.

| Item | Type | Source |
| --- | --- | --- |
| `MANIFEST` | `&'static str` | `include_str!(".../PixelBeacon.txt")` |
| `LUA` | `&'static str` | `include_str!(".../PixelBeacon.lua")` |
| `embedded_version()` | `u32` | `parse_manifest_version(MANIFEST)`, single-sourced |

- File names written on install: `PixelBeacon.txt` and `PixelBeacon.lua`.
- Subfolder name: `PixelBeacon`.
- Marker line (exact): `## X-ESO-Weave-Managed: true`.

### Manifest parsing (pure)

- `has_managed_marker(manifest: &str) -> bool`: true when a line, trimmed, equals
  `## X-ESO-Weave-Managed: true`.
- `parse_manifest_version(manifest: &str) -> Option<u32>`: parses the value of the
  first `## Version:` line as an unsigned integer; `None` if absent or unparsable.

## BeaconStatus

The classified installed state. Exactly one of:

| Variant | Predicate |
| --- | --- |
| `NotInstalled` | No `PixelBeacon` folder, or the folder has no readable `PixelBeacon.txt`. |
| `ManagedUpToDate` | Folder exists, manifest has the marker line, and `parse_manifest_version` equals `embedded_version()`. |
| `ManagedVersionMismatch` | Folder exists, manifest has the marker line, and the parsed version is absent or differs from `embedded_version()`. |
| `Unmanaged` | Folder exists, manifest is readable, but the marker line is absent. |

- State transitions: a fresh `install` moves any state to `ManagedUpToDate`; an
  `uninstall` of a managed folder moves `ManagedUpToDate`/`ManagedVersionMismatch`
  to `NotInstalled`; `uninstall` of `Unmanaged`/`NotInstalled` changes nothing.

## RunningState

Best-effort game-running signal for the reload reminder.

| Variant | Meaning |
| --- | --- |
| `Running` | The ESO client was detected. |
| `NotRunning` | The ESO client was not detected. |
| `Unknown` | The probe could not determine the state. |

- `reload_reminder(state: RunningState) -> bool` (pure): true for `Running` and
  `Unknown` (fail safe toward reminding), false for `NotRunning`.

## DiscoveryError

Returned by `resolve_addons_dir` when no AddOns directory can be produced.

| Variant | Meaning |
| --- | --- |
| `NotFound` | Auto-discovery failed and no override is set (or the override does not exist). |
| `Unsupported` | The platform has no discovery backend (should not occur on supported targets). |

## LifecycleOutcome and LifecycleError

Results of `install` and `uninstall`.

`LifecycleOutcome` (success):

| Field | Type | Meaning |
| --- | --- | --- |
| `status` | `BeaconStatus` | The status after the operation. |
| `reload_required` | `bool` | Whether the reload reminder applies (from `reload_reminder`). |

`LifecycleError` (typed failure):

| Variant | Raised when |
| --- | --- |
| `AddonsDirMissing` | The resolved AddOns root is not an existing directory. |
| `Unmanaged` | Uninstall refused because the on-disk manifest lacks the marker line (or is absent). |
| `Io(std::io::Error)` | A filesystem read or write failed; nothing partial is reported as success. |

## Steam VDF extraction (pure, `steam.rs`)

- `library_paths_for_app(vdf: &str, app_id: &str) -> Vec<PathBuf>`: parses
  `libraryfolders.vdf` text and returns, in file order, the `path` of each library
  block whose `apps` map contains `app_id`.
- Compiled on all targets; unit-tested on the host with crafted single-library,
  multi-library, and app-absent VDF text.

## Relationships

- `resolve_addons_dir(prefs: &BeaconPrefs) -> Result<PathBuf, DiscoveryError>` uses
  `prefs.path_override` first, else the platform backend, joining
  `Elder Scrolls Online/<environment.segment()>/AddOns`.
- `status(addons_root: &Path) -> BeaconStatus`, `install(addons_root, running) ->
  Result<LifecycleOutcome, LifecycleError>`, and `uninstall(addons_root, running)
  -> Result<LifecycleOutcome, LifecycleError>` all take the resolved root, so tests
  inject a temporary directory.
