# Feature Specification: Safety Boundaries

**Feature Branch**: `codex/s060-safety-boundaries`

**Created**: 2026-09-07

**Status**: Implemented

**Input**: Issues #92 and #94: stop queued automation when authorization gates close, and prevent PixelBeacon lifecycle actions from overwriting unmanaged add-on files.

## User Scenarios & Testing

### User Story 1 - Closed Gates Stop Generated Input (Priority: P1)

As an operator, I need every queued or active automation sequence to stop generating new input as soon as ESO loses focus, ESO Weave is suspended, or an in-game menu opens, so delayed work cannot land in another application or text field.

**Why this priority**: Generated input after authorization closes violates the product's central input-safety promise.

**Independent Test**: Exercise every gate before admission, close focus and suspension during running work, and deterministically close the menu gate across light, heavy, bash, and block-casting sequences; verify that no later press begins while any already-held generated input is safely released.

**Acceptance Scenarios**:

1. **Given** a queued weave, **When** focus is lost, suspension begins, or menu evidence becomes gated before the worker admits it, **Then** the queued action emits nothing even if the gate reopens before dequeue.
2. **Given** a running light, heavy, or bash sequence, **When** any applicable authorization gate closes, **Then** no new down event begins and any generated input already held down is released.
3. **Given** ESO does not hold focus, **When** the operator uses a bound physical key, **Then** the key passes through and no generated weave is queued.

---

### User Story 2 - Suspended Fishing Cannot Act (Priority: P1)

As an operator, I need suspension to stop Fishing casts, reels, and recasts without silently changing my saved request, so F2 and pending timers cannot generate an interact while the application is suspended.

**Why this priority**: Fishing synthesizes autonomously and bypasses the interception decision, so it must share the same authoritative gate closure as the input engine.

**Independent Test**: Suspend before enable, during Armed, Waiting, Reeling, and Recast, then advance every deadline and verify no interact is emitted until the documented recovery condition occurs.

**Acceptance Scenarios**:

1. **Given** ESO Weave is suspended, **When** Fishing is enabled through F2 or the UI, **Then** the request is preserved but no initial cast is sent.
2. **Given** Fishing has a pending reel, recast, or recast-arm timeout, **When** suspension begins, **Then** the active session is disabled, its deadline is cancelled, and no pending interact replays on resume.
3. **Given** suspension has ended with Fishing still requested, **When** no fresh manual cast is observed, **Then** Fishing remains idle and emits nothing.
4. **Given** suspension has ended with Fishing still requested, **When** a fresh FishingStarted observation arrives under every open safety gate, **Then** the controller may adopt the manual cast in Waiting without synthesizing it.
5. **Given** a menu surface temporarily gates an otherwise active Fishing session, **When** a reel or recast becomes due, **Then** the existing bounded deferral policy remains in force and sends nothing until the menu closes.

---

### User Story 3 - Unmanaged PixelBeacon Files Remain Untouched (Priority: P1)

As an operator with a hand-installed or foreign PixelBeacon directory, I need ESO Weave to identify it as unmanaged and refuse install, update, or uninstall actions, so my files cannot be overwritten or deleted.

**Why this priority**: Existing unmarked content belongs to the user, and overwriting it crosses the same ownership boundary that already protects uninstall.

**Independent Test**: Seed unmanaged directories with sentinel files and cover direct lifecycle calls plus UI intents, proving byte-for-byte preservation and an actionable unmanaged state.

**Acceptance Scenarios**:

1. **Given** a PixelBeacon directory whose manifest lacks the managed marker, **When** status is derived, **Then** the UI reports a distinct unmanaged state with no Install, Update, or Uninstall action.
2. **Given** a PixelBeacon directory with no readable manifest, **When** status is derived, **Then** the existing directory is treated as unmanaged rather than absent.
3. **Given** unmanaged content, **When** install or update is invoked through a lower-level or stale UI path, **Then** the operation fails safely and every existing file remains unchanged.
4. **Given** an outdated managed installation, **When** Update is invoked, **Then** only the managed files are refreshed without first deleting the directory.
5. **Given** no PixelBeacon directory, **When** Install is invoked, **Then** the managed installation is created as before.

### Edge Cases

- A gate closes between queueing, worker dequeue, controller locking, and the first generated event.
- A gate closes and reopens between worker observations.
- A gate closes while a generated key or mouse button is down.
- Multiple authorization gates close or reopen in different orders.
- Suspension is toggled while Fishing is disabled, requested-but-paused, and while a timer is due.
- A stale caller attempts PixelBeacon Install or Update after the UI state was derived.
- The PixelBeacon target exists without a manifest, has invalid UTF-8, is unreadable, is a file, or is a link.
- Sibling add-ons and extra files within an unmanaged PixelBeacon directory remain untouched.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST maintain independently shareable authorization gates for game activity, focus, suspension, menu state, life state, roll dodge, world state, and travel state without blocking the input callback.
- **FR-002**: The shared authorization state MUST advance a monotonic epoch on every safe-to-unsafe transition so queued or running work is invalidated even if a gate reopens before its next check.
- **FR-003**: Queued weave work MUST carry its admission epoch, and the worker MUST reject non-toggle work whose epoch no longer matches current authorization.
- **FR-004**: Weave admission and every timed weave step MUST recheck all applicable shared gates and the admitted epoch, including focus, suspension, and menu state.
- **FR-005**: After a weave gate closes, the system MUST reject new generated down events while allowing only the matching release of generated keys or mouse buttons already held by that sink.
- **FR-006**: Physical input MUST continue to pass through outside ESO focus and while interception authorization is closed.
- **FR-007**: Fishing synthesis MUST recheck shared suspension authorization at the final emission boundary for initial cast, reel, recast, and timeout-driven re-cast operations.
- **FR-008**: Entering suspension MUST cancel the active Fishing state and deadline, preserve the operator's requested-enabled choice, and expose a specific suspended stop reason.
- **FR-009**: Leaving suspension MUST NOT replay or synthesize cancelled Fishing work; recovery requires a fresh manual FishingStarted observation under open gates or an explicit off-then-on user request.
- **FR-010**: Existing menu-gate behavior MUST continue to defer Fishing reel and recast operations without advancing the state machine while gated.
- **FR-011**: Focus, suspension, and menu authorities MUST be published to synthesis paths before code waits on controller or weave mutexes.
- **FR-012**: PixelBeacon status MUST distinguish an absent directory, a managed current directory, a managed outdated directory, and any existing target that cannot prove the managed marker.
- **FR-013**: An existing PixelBeacon target without a readable marked manifest, including a file or link, MUST be classified as unmanaged.
- **FR-014**: Install, update, block-size redeploy, API-version update, and uninstall MUST refuse to write to, follow, or delete an unmanaged PixelBeacon target even if invoked without a fresh UI status check.
- **FR-015**: The unmanaged UI state MUST explain that files were not modified and that manual removal or relocation is required before ESO Weave can install its managed copy.
- **FR-016**: The unmanaged UI state MUST expose no lifecycle action that could imply ESO Weave owns the target.
- **FR-017**: Managed Update MUST refresh the managed installation in place rather than deleting the directory before a replacement is known to be writable.
- **FR-018**: New-install, managed-current, managed-outdated, unmanaged-marker-missing, manifest-missing, manifest-unreadable, target-file, target-link, and AddOns-root-missing behavior MUST be covered by automated tests where the host platform supports the shape.
- **FR-019**: A combined deterministic matrix MUST cover every gate before admission, focus and suspension during running work, and menu closure across light, heavy, bash, and block-casting sequences, including safe release of already-held generated input.
- **FR-020**: Tests MUST cover suspension before Fishing enable and during initial cast, reel, recast, and recast-arm timeout paths.
- **FR-021**: Canonical documentation MUST state the corrected cross-feature authorization and managed-ownership guarantees without claiming unverified live-game evidence.
- **FR-022**: S060 MUST close issues #92 and #94, archive completed plan 029 with PR #97 evidence, and establish plan 030 as the sole active build plan.

### Key Entities

- **Synthesis authorization**: Cloneable atomic gate state plus a monotonic invalidation epoch read by interception, weave, and autonomous controllers without acquiring controller locks.
- **Queued action envelope**: A handed-off action paired with the authorization epoch observed when physical input was admitted.
- **Fishing recovery state**: Requested choice, effective state, cancelled deadline, and stop reason that determine whether recovery may observe a fresh manual cast.
- **PixelBeacon ownership state**: Absent, managed current, managed outdated, or unmanaged, derived from target shape and a readable managed-marker manifest.
- **Lifecycle action matrix**: Allowed Install, Update, Redeploy, API Update, and Uninstall operations for each ownership state.

## Success Criteria

### Measurable Outcomes

- **SC-001**: The automated matrix produces zero new generated down events after any applicable authorization gate closes across every weave type and Fishing action phase.
- **SC-002**: Every generated input held when a weave is cancelled receives exactly one matching release and no unrelated release is emitted.
- **SC-003**: All suspended Fishing scenarios emit zero interact transitions until a qualifying recovery action occurs.
- **SC-004**: Every unmanaged PixelBeacon scenario preserves all seeded file bytes and directory entries across Install, Update, Redeploy, API Update, and Uninstall attempts.
- **SC-005**: Every PixelBeacon ownership state maps to one unambiguous status and only its permitted UI actions.
- **SC-006**: Full format, strict Clippy, locked test, documentation-policy, spelling, and mdBook validation pass on Windows-compatible source and Linux CI paths.

## Assumptions

- Suspension recovery follows the existing life/world/travel fail-safe pattern: preserve the request, cancel autonomous work, and require fresh manual game evidence before resuming.
- Menu gating remains a temporary deferral rather than cancellation because the existing controller state remains aligned with a live fishing session.
- Focus-loss auto-rearm remains unchanged, but the final emission gate still prevents a cast while focus is closed.
- The managed-marker line is the sole proof that ESO Weave owns a PixelBeacon directory.
- A target that exists but cannot provide a readable marked manifest is user-owned for safety purposes.
- Managed Update preserves unrelated managed-directory contents and refreshes only shipped files.

## Out of Scope

- Linux virtual-input capability issue #93.
- Settings application timing and fishing binding issue #95.
- UI/help copy drift issue #96 except text directly changed by the new safety behavior.
- Live-game or released-package verification.
- Atomic multi-file replacement for already-managed PixelBeacon content; unexpected partial-write recovery remains separate from unmanaged ownership protection.
