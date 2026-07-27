# Data Model: Out-Of-Band Display Detection

**Feature**: `specs/034-display-detection/` | **Date**: 2026-07-27

All types live in `src/pixelbus/display.rs` unless noted. Every type here is
pure data with pure functions over it; nothing in this file performs input or
output.

## Geometry primitives

```rust
pub struct Size { pub width: u32, pub height: u32 }
pub struct Point { pub x: i32, pub y: i32 }
```

`Size` is unsigned because a negative extent is not a thing that exists;
platform code converting a signed rectangle rejects a non-positive value at the
boundary rather than carrying it inward. `Point` is signed because a monitor to
the left of the primary one has a negative origin, and so does a window on it.

Both are `Copy + Eq`. Every value in this model is integral, which is what makes
change detection an exact comparison (plan Decision 2).

## `MeasuredDisplay`

What a platform probe returns. Produced only by `SurfaceSampler::display()`.

| Field | Type | Meaning |
| --- | --- | --- |
| `surface` | `Size` | Client area in physical pixels. Always non-zero; the probe returns `None` rather than a zero. |
| `surface_origin` | `Point` | Client top-left in screen coordinates. |
| `display_origin` | `Option<Point>` | Top-left of the display the surface is on. |
| `display_size` | `Option<Size>` | Extent of that display. |
| `dpi` | `Option<u32>` | Effective dots per inch of that display. |

Per-platform availability, which differs deliberately (plan Decision 7):

| Field | Windows | Linux (X11) |
| --- | --- | --- |
| `surface`, `surface_origin` | yes | yes |
| `display_origin`, `display_size` | per monitor | the X screen, which on a multi-head session is the union of all heads |
| `dpi` | yes | never; X core has no scale concept |

## `DisplayDescriptor`

The feature's output. Constructed from a `MeasuredDisplay` or, narrowly, from
stored settings.

| Field | Type | Meaning |
| --- | --- | --- |
| `surface` | `Size` | The render surface. The only always-present field. |
| `surface_origin` | `Option<Point>` | Absent for a configured descriptor. |
| `display_origin` | `Option<Point>` | Absent for a configured descriptor. |
| `display_size` | `Option<Size>` | Absent for a configured descriptor. |
| `dpi` | `Option<u32>` | Absent for a configured descriptor and on X11. |
| `source` | `DisplaySource` | `Measured` or `Configured`. |

```rust
pub enum DisplaySource { Measured, Configured }
```

Accessor: `scale(&self) -> Option<f32>` returning `dpi / 96.0`. Computed, never
stored, so the struct stays `Eq` (plan Decision 2).

**Construction rules** (FR-004, FR-022):

- `from_measured(m: MeasuredDisplay) -> Option<Self>`: `None` when
  `m.surface` has a zero dimension. Otherwise `Measured`, carrying every field
  the probe supplied.
- `from_stored(s: &StoredVideoSettings) -> Option<Self>`: `Some` only when both
  stored pairs are present, equal, and non-zero. Otherwise `None`. The result is
  `Configured` and carries `surface` alone; every other field is `None`, because
  the settings file records a display *index*, and an index is not geometry.

The second rule is narrow on purpose. Because the window-mode value is not
mapped (FR-015), identical pairs are the only configuration in which the file
determines the live surface without a guess.

## `StoredVideoSettings`

What the settings file says, as found. Every field independently optional; a
missing, malformed, or unparsable entry yields `None` for that field and affects
no other (FR-014).

| Field | Type | Source key |
| --- | --- | --- |
| `fullscreen` | `Option<Size>` | `FullscreenWidth`, `FullscreenHeight` |
| `windowed` | `Option<Size>` | `WindowedWidth`, `WindowedHeight` |
| `window_mode_raw` | `Option<i64>` | `FULLSCREEN` |
| `prefer_exclusive_fullscreen` | `Option<i64>` | `PreferExclusiveFullscreen` |
| `prefer_maximized_window` | `Option<i64>` | `PreferMaximizedWindow` |
| `active_display` | `Option<i64>` | `ACTIVE_DISPLAY` |
| `overscan` | `Option<Point>` | `OverscanWidthAdjustment`, `OverscanHeightAdjustment` |
| `custom_ui_scale` | `Option<f64>` | `CustomUIScale` |
| `use_custom_ui_scale` | `Option<i64>` | `UseCustomUIScale` |
| `gamepad_custom_ui_scale` | `Option<f64>` | `GamepadCustomUIScale` |
| `use_gamepad_custom_ui_scale` | `Option<i64>` | `UseGamepadCustomUIScale` |

A resolution pair is `Some` only when both its width and its height parsed; a
half-present pair is `None`, because half a resolution is not a resolution.

`window_mode_raw` is `i64` and stays raw. There is deliberately no
`WindowMode` enum in this model (FR-015).

Overscan is recorded and never applied. Applying it would be a layout decision,
and layout is the next feature's job.

## Parsing

```rust
pub fn parse_user_settings(text: &str) -> StoredVideoSettings
```

Total: every input produces a value, and no input panics.

Line grammar, permissively read:

1. Trim the line. Skip it if empty.
2. Split off the first whitespace-delimited token. Skip the line unless it is
   `SET` (case-insensitive).
3. Split off the next token as the key. Skip the line if there is none.
4. Take the remainder, trim it, and strip one leading and one trailing double
   quote if both are present. Whatever remains is the raw value.
5. Normalize the key: strip a trailing `.` followed by one or more digits, then
   lowercase.
6. Match the normalized key against the table above. An unrecognized key is
   ignored. A recognized key whose value does not parse as its type leaves the
   field as it was.

Duplicates: the last assignment in the file wins.

The suffix rule is the point of step 5. The game bumps a key's `.N` suffix when
the setting's meaning changes, so `UseCustomUIScale.2` and a future
`UseCustomUIScale.3` must both match `usecustomuiscale`. Base keys are compared
whole rather than by prefix, so `FULLSCREEN` never matches `FullscreenWidth`.

## `Reconciliation`

The observational record of how the two sources related. Nothing branches on it
(FR-020).

```rust
pub enum Reconciliation {
    NoStored,
    NoPairs,
    Agreed { pair: StoredPair, mode_raw: Option<i64> },
    Ambiguous,
    Disagreed { measured: Size },
}

pub enum StoredPair { Fullscreen, Windowed }
```

| Variant | Condition |
| --- | --- |
| `NoStored` | The settings file produced nothing usable. |
| `NoPairs` | Settings were read but neither resolution pair is present. |
| `Agreed` | The measured surface matches exactly one stored pair. `mode_raw` is carried because this is the evidence about the unmapped enum (plan Decision 6). |
| `Ambiguous` | The measured surface matches both pairs, which happens when the pairs are identical. Says nothing about the mode value. |
| `Disagreed` | The measured surface matches neither pair. |

```rust
pub fn reconcile(measured: Size, stored: Option<&StoredVideoSettings>) -> Reconciliation
```

Pure, and it never returns a descriptor: reconciliation observes, it does not
decide (FR-018).

## `DisplayDetector`

The stateful part, and the only one. Owns the current descriptor and nothing
else, and performs no input or output.

It deliberately does not store the last reconciliation outcome. Reconciliation
is recomputed only when the descriptor changes, so a stored outcome could never
change independently of the descriptor; keeping one would be state that exists
to be compared against itself.

```rust
pub struct DisplayDetector { /* current descriptor only */ }

pub struct DisplayUpdate {
    pub descriptor: Option<DisplayDescriptor>,
    pub reconciliation: Option<Reconciliation>,
}

impl DisplayDetector {
    pub fn new() -> Self;
    pub fn current(&self) -> Option<&DisplayDescriptor>;
    pub fn update<F>(&mut self, measured: Option<MeasuredDisplay>, stored: F)
        -> Option<DisplayUpdate>
    where
        F: FnOnce() -> Option<StoredVideoSettings>;
}
```

`update` returns `Some` only when the descriptor changed, so the caller's log
line is change-driven by construction (FR-009, FR-021). The `reconciliation`
field is populated whenever a stored reading was consulted during that change,
and is `None` when it was not.

**The read-gating rule** (FR-016), which is the behaviour most worth testing:

- Compute the candidate descriptor from `measured`.
- If it equals the current descriptor, return `None`. **The closure is not
  called.** A stationary window therefore reads no files at all.
- Otherwise the descriptor changed. Call the closure once, reconcile against it,
  store both, and return the update.
- When `measured` is `None` and no descriptor has ever been produced, call the
  closure once and attempt a configured descriptor. Once that attempt has been
  made it is not repeated while the measurement remains absent, so a session
  with the game closed does not re-read the file every cycle.
- When `measured` becomes `None` after a descriptor existed, the descriptor
  clears to absent and that clearing is itself a change. The closure is not
  called: losing the window is not a reason to consult a file.

## Trait extension

```rust
pub trait SurfaceSampler {
    fn prepare(&self) {}
    fn sample(&self, x: u32, y: u32) -> Option<Rgb>;
    /// The measured display for this sampler's window, or `None`.
    fn display(&self) -> Option<MeasuredDisplay> { None }
}
```

Defaulted, so every existing implementation compiles unchanged and `MockSampler`
opts in for tests (plan Decision 1).

## Path resolution (`src/beacon/mod.rs`)

```rust
pub fn user_settings_path(addons_root: &Path) -> Option<PathBuf>
```

Returns `addons_root.parent()?.join("UserSettings.txt")`. Pure path
composition: it does not touch the filesystem, does not check existence, and
above all does not create anything.
