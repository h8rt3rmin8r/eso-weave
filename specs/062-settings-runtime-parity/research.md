# Research: Settings Runtime Parity

## Decision 1: Apply a hybrid per-setting contract

**Decision**: Apply all Fishing fields plus Pixel Bus tolerance and sampling intervals live. Keep block size staged until coordinated ESO reload or relog and ESO Weave restart.

**Rationale**: Fishing is already shared with the application model, and scalar reader fields do not alter capture geometry. Block size changes both PixelBeacon output and the desktop reader footprint, so partial live adoption creates a guaranteed mismatch.

**Alternatives considered**:

- Restart for every field: safe but unnecessarily stale and inconsistent with neighboring live settings.
- Live apply every field: rejected because block geometry cannot become coherent atomically across the game add-on and desktop process.

## Decision 2: Stop changed active Fishing sessions

**Decision**: Replace the complete sanitized Fishing configuration atomically. If it differs while Fishing is requested or active, clear pending state and the request without sending input, and record a settings-change stop reason. Identical configuration is a strict no-op.

**Rationale**: Pending work stores absolute deadlines. Retaining it while swapping the interact key or delay values combines two configuration generations and can synthesize an unintended action. The modal auto-applies during editing, so explicit safe cancellation is more deterministic than rescheduling.

**Alternatives considered**:

- Preserve old deadline but use new key: rejected because the scheduled intent belongs to the old key generation.
- Recompute deadlines: rejected because trigger timestamps are not retained and the edit could make work immediately due.
- Replace the controller: rejected because it could discard shared gates and recovery semantics.

## Decision 3: Use a wakeable complete-value reader channel

**Decision**: Give AppModel a standard-library sender and the existing Pixel Bus worker a receiver. Send complete sanitized live subsets without blocking, replace sleep with `recv_timeout`, and drain queued values so the newest update wins.

**Rationale**: The worker owns both `PixelBusReader` and poll cadence. A complete-value message avoids partial ordering hazards, wakes a long wait, and requires no new thread or dependency.

**Alternatives considered**:

- Shared mutex snapshot only: rejected because an old idle wait delays application and provides no delivery failure signal.
- Bounded synchronous channel: rejected because UI edits could block or the newest value could be dropped.
- New async runtime: rejected as disproportionate.

## Decision 4: Invalidate tolerance-dependent safety evidence

**Decision**: A tolerance change clears cached safety observations and routes their closing events before the next sample. Interval-only changes retain decoded observations.

**Rationale**: Menu and life evidence decoded under a different tolerance cannot safely remain authoritative. Closing first preserves the fail-closed contract without discarding unrelated reader state for cadence-only changes.

## Decision 5: Reuse the canonical key universe

**Decision**: Render Fishing Interact Key from `Key::ALL`, using a dedicated label and stable combo ID. It remains a generated-action setting rather than an input binding and does not enter binding-conflict checks.

**Rationale**: S061 established one exhaustive application key universe. A second key list would drift, while binding conflict rules do not apply to the key Fishing synthesizes.

## Decision 6: Keep persistence confirmation narrow

**Decision**: Preserve the existing “Settings saved” confirmation and add accurate inline/help copy for application timing. Delivery failure is logged and surfaced as a notice when possible.

**Rationale**: The toast claims only successful persistence. It need not become a noisy per-frame runtime status, but staged Block Size must remain explicit where it is edited.
