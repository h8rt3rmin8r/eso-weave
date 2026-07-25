# Data Model: Pixel-Bus Block Size Single Source of Truth

This feature adds one authoritative scalar and derives existing geometry from it.
No new persistent entities beyond an additive settings field.

## Entities and fields

### ReaderConfig (in-memory, `src/pixelbus/mod.rs`)

| Field | Type | Change | Notes |
| --- | --- | --- | --- |
| `block_px` | `u32` | NEW (single source of truth) | Even, 2..=32; default 16. |
| `status_point` / `fishing_point` / `latency_point` / `weapon_point` | `(u32, u32)` | CHANGED from stored fields to derived methods | `block_center(block_px, 0..=3)`. |
| `tolerance` | `u8` | unchanged | default 2. |
| `heartbeat_timeout_ms` | `u64` | unchanged | not a user setting. |
| `interval_fishing_ms` / `interval_idle_ms` | `u64` | unchanged | user settings. |

Derivation helpers (pure, `src/pixelbus/mod.rs`):

- `const NUM_BLOCKS: u32 = 4`
- `const DEFAULT_BLOCK_PX: u32 = 16`, `const MIN_BLOCK_PX: u32 = 2`,
  `const MAX_BLOCK_PX: u32 = 32`
- `fn block_center(block_px: u32, index: u32) -> (u32, u32)`
  = `(block_px * index + block_px / 2, block_px / 2)`
- `fn capture_dims(block_px: u32) -> (u32, u32)` = `(block_px * NUM_BLOCKS, block_px)`
- `fn sanitize_block_px(value: u32, notices: &mut Vec<Notice>) -> u32`
  (even-and-range correction with a non-fatal notice on change)

### RawPixelBus (deserialization view, `src/pixelbus/mod.rs`)

| Field | Type | Change |
| --- | --- | --- |
| `block_px` | `Option<u32>` | NEW; `#[serde(default)]` |
| `tolerance` | `Option<u8>` | unchanged |
| `interval_fishing_ms` / `interval_idle_ms` | `Option<u64>` | unchanged |

`load_reader_config`: `block_px = raw.block_px.map(|v| sanitize_block_px(v, notices)).unwrap_or(DEFAULT_BLOCK_PX)`.
`store_reader_config`: adds `"block_px": config.block_px`.

### Persisted config: `pixelbus` section (`config.json`)

Additive JSON field `block_px` inside the existing opaque `pixelbus` section.
No `schema_version` bump (the section is `#[serde(default)]`); older files load
with `block_px` defaulting to 16.

Example:

```json
"pixelbus": {
  "tolerance": 2,
  "interval_fishing_ms": 100,
  "interval_idle_ms": 1000,
  "block_px": 16
}
```

### GdiSampler (Windows, `src/pixelbus/windows.rs`)

| Field | Type | Change |
| --- | --- | --- |
| `capture_w` / `capture_h` | `i32` | NEW, from `capture_dims(block_px)`; replaces the `CAPTURE_W` / `CAPTURE_H` module consts |
| `hwnd`, `frame` | unchanged | |

Constructor gains the size: `GdiSampler::for_window(title: &str, block_px: u32)`.

## Validation rules

- `block_px` MUST be even and within `[MIN_BLOCK_PX, MAX_BLOCK_PX]`.
- Correction (never a panic; a `Notice` of kind `InvalidValue` is recorded when
  the value is changed):
  - odd -> next even below (`value & !1`)
  - below range -> `MIN_BLOCK_PX`
  - above range -> `MAX_BLOCK_PX`
  - absent / wrong type -> `DEFAULT_BLOCK_PX`

## State transitions (addon deploy on size change)

```text
apply_settings(new block_px):
  if new == deployed size            -> no-op
  else if status is managed          -> install(new block_px); notice "re-deployed; /reloadui + restart"
  else if status is Unmanaged        -> no write; notice "unmanaged; not modified"
  else (NotInstalled)                -> no write; reader uses new size next start
```

## Contract cross-reference

The exact derivation formulas and the byte-for-byte agreement requirement are
specified in [contracts/geometry.md](contracts/geometry.md).
