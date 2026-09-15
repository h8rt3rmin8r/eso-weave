# Research: Stale HUD Retention

## R1. Retention authority belongs at the presentation boundary

**Decision**: Cache cloned rendered HUD fields in `AppModel`, after authoritative state has been read and projected.

**Rationale**: `route_reader_safety_gate`, `route_reader_event`, process observation, and controllers already close action authority immediately. A display-only cache in `AppModel` cannot reopen those paths and survives the engine's deliberate clearing of live observations long enough to satisfy the user-facing requirement.

**Rejected alternatives**:

- Retain decoded values inside `WeaveEngine`: rejected because Auto Potion and future consumers could mistake them for current evidence.
- Retain values inside `GameState`: rejected because Game Context is safety evidence, not UI memory.
- Add a retention worker or widget timer: rejected because it duplicates the existing monotonic clock and creates multiple expiry authorities.

## R2. One coherent snapshot and one interval

**Decision**: Snapshot all displayed player-state fields together only when runtime is active, focus is positive, heartbeat freshness is fresh, and surface evidence is available.

**Rationale**: Retaining fields independently could combine observations from different moments and make the HUD internally inconsistent. One snapshot and one loss timestamp satisfy the issue dependency explicitly.

**Rejected alternative**: Per-widget freshness timestamps. They complicate expiry, accessibility, testing, and the meaning of one stale cause.

## R3. Expiry follows the original loss

**Decision**: Store `lost_at_ms`, update a changing loss cause without resetting it, and derive the deadline from `lost_at_ms + configured_seconds` on each projection.

**Rationale**: This lets a live settings edit shorten the remaining interval or clear immediately while preventing a cause transition or form auto-submit from extending stale data.

**Rejected alternative**: Store only a fixed deadline. It would ignore a settings change until the next loss. Resetting on a cause change could retain values indefinitely through alternating failures.

## R4. Numeric control matches repository conventions

**Decision**: Use `egui::DragValue` with an inclusive range, integer speed, and seconds suffix.

**Rationale**: The settings modal already uses `DragValue` for numeric values. It supports direct keyboard entry and built-in increment/decrement interaction with no custom focus or accessibility behavior.

**Rejected alternative**: A slider. It makes exact values harder to enter across a 1000-value range.

## R5. Additive UI configuration needs no schema bump

**Decision**: Store `stale_retention_seconds` inside the opaque `ui` JSON section, default missing data to 120, and validate invalid data with a notice.

**Rationale**: The top-level settings contract explicitly treats UI additions as backward compatible. Existing older files omit the field and safely inherit the default.

## R6. Accessible stale status is visible content

**Decision**: Add a `HUD Freshness` status row before the values. Its state includes stale cause and age; its tooltip explains the retention and safety behavior.

**Rationale**: Color alone is insufficient. A visible labeled row participates in keyboard and screen-reader reading order and does not replace the values the operator needs.

## R7. Publication surface

**Decision**: Update the maintained bundled mdBook pages. Do not create a separate blog or website tree.

**Rationale**: The repository's shipped documentation is the canonical offline publication surface and there is no independent blog system to maintain. Inventing one would expand scope without an operator benefit.
