# Data Model: ESO API Version Check Automation

## GameVersion (new, `src/beacon/api_check.rs`)

A parsed, comparable game client version. Kept `Copy` so it can live inside the
`Copy` `SessionState`.

- `parts: [u16; 4]` - dot-separated numeric components, left-aligned and
  zero-padded (for example `12.0.6` becomes `[12, 0, 6, 0]`).
- Derives: `Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd,
  Ord`. Lexicographic ordering over the fixed array gives correct version
  ordering.
- Parsing (`parse_commit_message_version`): take the first whitespace-delimited
  token of the GitHub commit message, split on `.`, parse up to four components as
  `u16`. Any missing component is `0`; any unparseable component or an empty token
  yields `None`.

## ApiVersionCache (new, `src/config/state.rs`)

The persisted derived state for this feature, an additive `Copy` section on
`SessionState`.

- `last_known_api_version: Option<u32>` - the highest numeric API version the app
  has resolved; `None` before the first resolution. `serde(default)`.
- `last_seen_game_version: Option<GameVersion>` - the newest game version observed
  from the network signal; `None` before the first successful fetch.
  `serde(default)`.
- Derives: `Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default`.

### SessionState change

`SessionState` gains one field, additive and backward compatible:

- `#[serde(default)] pub api_version: ApiVersionCache`

`CURRENT_STATE_VERSION` bumps from `1` to `2`. Old `state.json` files (version 1,
no `api_version` key) load cleanly because the field defaults; `load` continues to
degrade to defaults on any error. `current_session_state()` and `restore_session()`
carry the cache through the existing save path.

## Constants (new, `src/beacon/mod.rs`)

- `DEFAULT_API_VERSION: u32 = 101050` - the compiled default numeric API version,
  the current live value, refreshed as release upkeep.
- `DEFAULT_GAME_VERSION: GameVersion` - the game version `12.0.6` that
  `DEFAULT_API_VERSION` corresponds to; the baseline for bump detection.

## ApiCheckOutcome (new, `src/beacon/api_check.rs`)

The value handed from the background thread to the GUI for persistence.

- `last_known_api_version: u32` - the resolved effective value to persist.
- `last_seen_game_version: Option<GameVersion>` - the fetched game version, if any.
- The bump notice itself is emitted from the thread via `tracing::warn!` and shown
  by the live log viewer; it is not part of this struct.

## Resolution and transition rules

- **Effective numeric version**: `effective = stored_last_known.unwrap_or(0).max(DEFAULT_API_VERSION)`.
  Persisted back as the new `last_known_api_version`. Monotonic non-decreasing.
- **Manifest write condition**: the addon is installed, the on-disk manifest
  carries the managed marker (`has_managed_marker`), and the manifest primary
  APIVersion token is less than `effective`. Otherwise no write.
- **Multi-value APIVersion token rule** (`rewrite_api_version`): primary token is
  `effective`; existing tokens greater than `effective` are preserved (sorted
  descending after the primary); existing tokens less than `effective` are dropped.
- **Bump detection**: emit the update notice when the fetched game version is
  greater than both `DEFAULT_GAME_VERSION` and any stored `last_seen_game_version`.
  Always record the fetched game version as the new `last_seen_game_version` when
  it is newer.
