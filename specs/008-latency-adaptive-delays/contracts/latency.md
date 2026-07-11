# Contract: Latency-Adaptive Delays (weave module additions)

Signatures are the intended Rust shape; names may be refined during
implementation provided the behavior and the no-regression guarantee hold.

## Types

```rust
pub struct LatencyConfig {
    pub enabled: bool,   // default false
    pub k: f64,          // default 0.25, valid finite in [0.0, 4.0]
}

pub const MAX_LATENCY_BONUS_MS: u32 = 300;
```

- `LatencyConfig::default()` = `{ enabled: false, k: 0.25 }`.

## Pure functions (unit tested)

```rust
pub fn effective_delay(base: u32, latency_ms: Option<u16>, cfg: &LatencyConfig) -> u32;

pub fn sequence_for_adapted(
    slot: &SkillSlot,
    timing: &TimingConfig,
    latency_ms: Option<u16>,
    cfg: &LatencyConfig,
) -> Vec<WeaveStep>;

pub fn sequence_for(slot: &SkillSlot, timing: &TimingConfig) -> Vec<WeaveStep>;
```

### `effective_delay`

- Returns `base` if `!cfg.enabled` or `latency_ms` is `None`.
- Otherwise returns `base + clamp(round(cfg.k * latency), 0, MAX_LATENCY_BONUS_MS)`,
  using round-half-away-from-zero and saturating addition; the result is always in
  `[base, base + 300]`.

### `sequence_for_adapted`

- Builds the same step sequence as the current `sequence_for` for the slot's weave
  type, except the `d_weave` and `d_bash` waits use `effective_delay(...)`. The
  `d_heavy` wait and all emitted operations are unchanged. `global_cooldown` is not
  part of the sequence and is unaffected.

### `sequence_for`

- Delegates to `sequence_for_adapted(slot, timing, None, &LatencyConfig::default())`.
  This MUST be byte-for-byte identical to the pre-feature sequences (FR-008), so
  the existing S003 sequence tests keep passing unchanged.

## Engine intake and integration

```rust
impl WeaveEngine {
    pub fn set_latency(&mut self, latency: Option<u16>);       // Some(ms) or None (signal lost)
    pub fn latency_config(&self) -> &LatencyConfig;
    pub fn set_latency_config(&mut self, cfg: LatencyConfig);
    // handle(...) now builds sequences via sequence_for_adapted with the engine's
    // current_latency and latency config.
}
```

- `current_latency` starts `None`, is set by `set_latency`, and is never written to
  the config file (transient runtime state).
- With the default config (feature off) or `current_latency == None`, `handle`
  produces the same steps as before this feature.

## Configuration

- `WeaveEngine::load(settings)` reads `settings.latency` into `LatencyConfig`
  (absent/null yields defaults; a non-finite or out-of-range `k` falls back to 0.25
  with an `InvalidValue` notice; `enabled` absent yields false).
- `WeaveEngine::store(settings)` writes `settings.latency` as `{ "enabled": bool,
  "k": number }`. No `schema_version` bump.

## Required no-regression tests (must not be weakened)

- With the default (feature off), `sequence_for_adapted(slot, timing, Some(latency),
  &default)` equals `sequence_for(slot, timing)` for every weave type.
- With the feature enabled but `latency_ms == None`, the sequence equals the base
  sequence for every weave type.
- `d_heavy` and `global_cooldown` are identical across all latencies and configs.
