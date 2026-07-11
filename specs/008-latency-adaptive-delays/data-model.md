# Phase 1 Data Model: Latency-Adaptive Delays

All new types live in the `weave` module. The computation is pure; the engine
holds the transient current latency and the persisted config.

## LatencyConfig

The persisted latency-adaptation configuration.

| Field | Type | Default | Meaning |
| --- | --- | --- | --- |
| `enabled` | `bool` | `false` | Whether latency scaling is applied. |
| `k` | `f64` | `0.25` | Scale factor on latency; valid range `[0.0, 4.0]`, finite. |

- Loaded from the additive `latency` settings section: absent or null yields
  defaults; a `k` that is not finite or outside `[0.0, 4.0]` falls back to 0.25
  with an `InvalidValue` notice; `enabled` absent defaults to false.
- `LatencyConfig::default()` is `{ enabled: false, k: 0.25 }`, so the default makes
  `effective_delay` a no-op.

## Constant

- `MAX_LATENCY_BONUS_MS: u32 = 300`: the inclusive cap on the latency bonus, so the
  effective delay is at most `base + 300`.

## effective_delay (pure)

`effective_delay(base: u32, latency_ms: Option<u16>, cfg: &LatencyConfig) -> u32`:

| Condition | Result |
| --- | --- |
| `!cfg.enabled` | `base` |
| `latency_ms == None` | `base` |
| otherwise | `base.saturating_add(clamp(round(cfg.k * latency), 0, MAX_LATENCY_BONUS_MS))` |

- `round` is `f64::round` (round-half-away-from-zero).
- The result always lies in `[base, base + 300]`.

## Adapted sequence builder

- `sequence_for_adapted(slot: &SkillSlot, timing: &TimingConfig, latency_ms:
  Option<u16>, cfg: &LatencyConfig) -> Vec<WeaveStep>`: identical to the current
  builder except that the `d_weave` and `d_bash` waits use
  `effective_delay(slot.d_weave(timing), latency_ms, cfg)` and
  `effective_delay(slot.d_bash(timing), latency_ms, cfg)`; `d_heavy` uses
  `slot.d_heavy(timing)` unchanged.
- `sequence_for(slot, timing)` = `sequence_for_adapted(slot, timing, None,
  &LatencyConfig::default())`, so it is byte-for-byte the current behavior.

## WeaveEngine additions

| Item | Type / Signature | Notes |
| --- | --- | --- |
| `latency` | `LatencyConfig` | The persisted config (default off). |
| `current_latency` | `Option<u16>` | Transient; `None` by default and after signal loss. Never persisted. |
| `set_latency` | `fn set_latency(&mut self, latency: Option<u16>)` | Intake: `Some(ms)` on a Latency event, `None` on signal loss. |
| `latency_config` | `fn latency_config(&self) -> &LatencyConfig` | Accessor. |
| `set_latency_config` | `fn set_latency_config(&mut self, cfg: LatencyConfig)` | Setter for GUI/config wiring. |
| `handle` | (existing) | Now builds via `sequence_for_adapted(&slot, &timing, self.current_latency, &self.latency)`. |
| `load` / `store` | (existing, extended) | Also read/write `settings.latency`. |

## Settings

- `Settings.latency: serde_json::Value` (additive opaque section, default null).
  Serialized shape: `{ "enabled": bool, "k": number }`. No `schema_version` bump.

## Relationships

- The reader's `PixelBusEvent::Latency(u16)` value is what the worker loop (out of
  scope) passes to `set_latency(Some(..))`; its `SignalLost` maps to
  `set_latency(None)`. This slice provides the intake and the computation, not the
  loop.
