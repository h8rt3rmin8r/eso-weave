# Research: Ultimate Auto Potion Resource Watch

**Date**: 2026-09-17

## Decision 1: Consume `UltimateTelemetry` directly in `PotionReadings`

**Decision**: Add the existing atomic Ultimate observation beside `ResourceSet` and `QuickslotState` in the per-tick Auto Potion readings.

**Rationale**: `ResourceSet` carries already normalized Health, Magicka, and Stamina percentages, while Ultimate intentionally preserves exact current and maximum values. Keeping the pair atomic avoids lossy shared-state conversion and uses the same current-evidence cache already updated and cleared by Pixel Bus routing.

**Alternatives rejected**:

- Add Ultimate to `ResourceSet`. Rejected because that changes a protocol-facing three-pool model and discards the exact denominator needed for correct comparison.
- Cache Ultimate in `AutoPotionController`. Rejected because it creates a second observation authority and makes stale clearing easier to miss.
- Re-read UI presentation state. Rejected because retained presentation is deliberately unavailable to action-producing paths.

## Decision 2: Compare exact ratios by cross multiplication

**Decision**: Treat a known current and positive maximum as low when `u32::from(current) * 100 <= u32::from(threshold) * u32::from(maximum)`.

**Rationale**: Ultimate fields are bounded 9-bit values, so widened multiplication cannot overflow. The comparison is exact, inclusive, allocation-free, and avoids both floating-point ambiguity and floor-truncation false positives.

**Alternatives rejected**:

- Divide to an integer percentage first. Rejected because 71 of 200 would truncate to 35 and incorrectly qualify at a 35-percent threshold.
- Use floating point. Rejected because integer cross multiplication is exact and simpler to audit.
- Clamp current to maximum. Rejected because malformed above-maximum evidence would become actionable at 100 percent.

## Decision 3: Use ceiling percentage for a qualifying diagnostic

**Decision**: For a qualifying Ultimate cause, report `(current * 100).div_ceil(maximum)` as `observed_percent`.

**Rationale**: Ceiling is the integer representation consistent with exact threshold qualification. A 34.5-percent observation reports 35 and qualifies at 35; a 35.5-percent observation reports 36 and does not qualify. Triggering values are never above 100, so they fit the existing `u8` diagnostic model.

**Alternatives rejected**:

- Floor the display. Rejected because it can visually understate a ratio and suggests the unsafe comparison the implementation explicitly avoids.
- Expand the public cause to numerator and denominator. Rejected because it would complicate established diagnostics beyond issue scope.
- Round to nearest. Rejected because it is not monotonic with the inclusive integer threshold boundary.

## Decision 4: Append Ultimate to deterministic OR order

**Decision**: Retain Health, Magicka, Stamina order and evaluate Ultimate fourth.

**Rationale**: Existing first-cause diagnostics are observable. Appending preserves every legacy mixed-watch result while adding deterministic behavior for the new watch.

**Alternatives rejected**:

- Put Ultimate first. Rejected because it would silently change named causes for existing configurations once Ultimate is enabled.
- Report every qualifying cause. Rejected because action behavior needs one attempt and the existing model intentionally names one deterministic cause.
- Make order configurable. Rejected because the fixed OR rule and cause order are diagnostic details, not policy controls.

## Decision 5: Migrate additively inside the opaque potion section

**Decision**: Add optional raw `ultimate`, load it through the existing `ResourceWatch` default, and emit it on every current save without changing `CURRENT_SCHEMA_VERSION`.

**Rationale**: The top-level settings contract explicitly treats potion JSON as additive and backward compatible. An omitted nested watch has an unambiguous safe default, so a global schema migration would add noise without stronger recovery.

**Alternatives rejected**:

- Bump the top-level schema. Rejected because no top-level shape or incompatible meaning changes.
- Infer Ultimate enablement from another watch. Rejected because issue #173 requires independent configuration and safe default-off migration.
- Omit disabled Ultimate on save. Rejected because explicit current output improves inspectability and round-trip evidence.

## Decision 6: Reuse the settings row and existing safety gates

**Decision**: Add Ultimate to the current vertically stacked row loop and only extend cause projection and documentation. Do not change controller gate order, worker timing, or synthesis.

**Rationale**: The current control already supplies bounded numeric input, checkbox keyboard interaction, help text, and modal sizing. Existing gates already reject stale and non-active evidence before the resource predicate.

**Alternatives rejected**:

- Add a separate Ultimate settings panel. Rejected as inconsistent and unnecessarily expansive.
- Add an Ultimate freshness timestamp. Rejected because heartbeat and event invalidation already define the action-evidence lifetime.
- Special-case Ultimate cooldown or potion type. Rejected because ESO Weave does not know which resources a selected potion restores.

## Dependency and Security Review

No dependency, protocol, addon, filesystem, network, or input-backend change is required. The new path can only authorize the existing Quickslot native action after all existing fail-closed gates pass.
