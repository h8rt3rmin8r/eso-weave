# Contract: API Version Check Seam and Manifest Upkeep

This feature is internal to the desktop app. Its contracts are the trait seam that
isolates the network, the pure manifest functions, and the startup and persistence
wiring.

## Network seam

```rust
/// A source of the current live ESO game client version.
pub trait GameVersionSource {
    fn fetch(&self) -> Result<GameVersion, ApiCheckError>;
}
```

- `GithubLiveSource` is the production implementation. It issues a blocking HTTPS
  GET to `https://api.github.com/repos/esoui/esoui/commits/live` with a descriptive
  `User-Agent` header and a short timeout (a few seconds), reads the JSON body, and
  extracts `.commit.message`. It parses the first token as a `GameVersion`.
- Errors (`ApiCheckError`): network failure, non-success status, body read failure,
  or an unparseable version. Every variant is non-fatal to the caller.
- Tests use a `MockSource` returning a canned `GameVersion` or a canned error; no
  network access occurs in tests.

## Pure manifest functions (`src/beacon/mod.rs`)

```rust
pub fn parse_api_version_primary(manifest: &str) -> Option<u32>;
pub fn rewrite_api_version(existing: &str, effective: u32) -> String;
pub fn render_manifest(effective: u32) -> String; // = rewrite_api_version(MANIFEST, effective)
```

- `parse_api_version_primary` returns the first (primary) numeric token of the
  first `## APIVersion:` line, or `None`.
- `rewrite_api_version` replaces only the `## APIVersion:` line, applying the
  multi-value token rule (primary is `effective`; keep greater tokens; drop lesser
  tokens). Every other line, including the managed marker, is preserved byte for
  byte. Line endings and the trailing structure are preserved.
- `render_manifest` produces the full manifest for install with the given version.

## Orchestration (`src/beacon/api_check.rs`)

```rust
pub fn run_check(
    source: &dyn GameVersionSource,
    addons_dir: Result<PathBuf, DiscoveryError>,
    stored_last_known: Option<u32>,
    stored_last_seen_game: Option<GameVersion>,
) -> ApiCheckOutcome;
```

- Resolves `effective = stored_last_known.unwrap_or(0).max(DEFAULT_API_VERSION)`.
- If `addons_dir` resolves and the on-disk manifest exists, carries the managed
  marker, and its primary token is `< effective`: rewrites the `## APIVersion:`
  line in place. A manifest lacking the marker, or unreadable, is never written.
  A manifest whose primary token is `>= effective` is left unchanged (no downgrade,
  no churn).
- Calls `source.fetch()`. On success, if the fetched game version is newer than
  `DEFAULT_GAME_VERSION` and any `stored_last_seen_game`, emits a `tracing::warn!`
  update notice and records the fetched version. On failure, proceeds silently.
- Returns `ApiCheckOutcome { last_known_api_version: effective, last_seen_game_version }`.
- Never blocks beyond the fetch timeout; never panics; all IO and fetch errors are
  swallowed into the returned outcome.

## Startup and persistence wiring

- `src/main.rs` clones the beacon prefs value and the restored
  `last_known_api_version` / `last_seen_game_version` before `settings` and
  `session` are moved into the model, creates an `mpsc::channel::<ApiCheckOutcome>`,
  and spawns a `std::thread` that calls `run_check` and sends the outcome. The
  interface never waits on this thread.
- `EsoWeaveApp::ui` drains the outcome receiver each frame (mirroring `toggle_rx`)
  and calls `AppModel::apply_api_check`.
- `AppModel::apply_api_check` updates the in-memory `ApiVersionCache`, and if it
  changed, calls `scheduler.mark_session(now)` so the existing coalesced save path
  persists it to `state.json`. `current_session_state()` and `restore_session()`
  include the cache.

## Install path change

- `beacon::install` renders the manifest with the resolved effective version via
  `render_manifest(effective)` instead of writing `MANIFEST` verbatim. The app
  resolves `effective` the same way (stored then default) at install time.
