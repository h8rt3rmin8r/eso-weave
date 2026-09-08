# Feature Specification: Settings Runtime Parity

**Feature Branch**: `codex/s062-settings-runtime-parity`

**Created**: 2026-09-07

**Status**: Implemented

**Input**: Issue #95: make Fishing and Pixel Bus settings application timing and Fishing binding claims accurate.

## User Scenarios & Testing

### User Story 1 - Fishing Changes Apply Safely (Priority: P1)

As an operator, I need Fishing timing and interact-key edits to take effect without restarting the application, while any active automation stops safely before it can mix old and new settings.

**Independent Test**: Change each Fishing setting while disabled and while requested or active, then prove the effective controller configuration changes immediately, active work emits no input and becomes disabled, and the next explicit enable uses only the new configuration.

**Acceptance Scenarios**:

1. **Given** Fishing is disabled, **When** a timing or interact-key value changes, **Then** the live controller adopts the complete sanitized configuration without emitting input.
2. **Given** Fishing is requested, armed, waiting, reeling, or recasting, **When** its configuration changes, **Then** pending work is cancelled without synthesized input, the request turns off, and the UI reports that settings changed.
3. **Given** the submitted Fishing configuration is unchanged, **When** unrelated settings are applied, **Then** Fishing state and deadlines remain untouched.
4. **Given** Fishing was stopped by a settings change, **When** the operator explicitly enables it again, **Then** new deadlines and interact actions use only the new configuration.

---

### User Story 2 - Pixel Bus Reader Controls Apply Live (Priority: P1)

As an operator, I need color tolerance and sampling cadence edits to reach the running reader promptly, so the settings interface does not claim behavior that remains stale until restart.

**Independent Test**: Send reader updates while the worker is sleeping, prove the wait wakes and coalesces to the latest live values, and verify tolerance changes close stale safety evidence before fresh samples are accepted.

**Acceptance Scenarios**:

1. **Given** the reader worker is waiting on an old interval, **When** tolerance or either sample interval changes, **Then** the worker wakes, adopts the newest complete live configuration, and uses it on the next iteration.
2. **Given** several edits arrive rapidly, **When** the worker handles updates, **Then** it drains them and applies the newest values without blocking the UI.
3. **Given** color tolerance changes, **When** the reader adopts it, **Then** safety observations decoded under the old tolerance are invalidated before a new sample can reopen authorization.
4. **Given** only live reader fields change, **When** the update is applied, **Then** the running block size and internal heartbeat timeout remain unchanged.

---

### User Story 3 - The Interface States the Actual Contract (Priority: P2)

As an operator, I need every editable setting and its application timing to be visible and accurate, so I know what applies now and what still requires reload or restart.

**Independent Test**: Render the Settings modal and inspect canonical documentation, proving Fishing exposes Interact Key, live controls say they apply live, Block Size alone retains coordinated reload/restart guidance, and obsolete #95 gap claims are absent.

**Acceptance Scenarios**:

1. **Given** the Fishing settings group, **When** it renders, **Then** Interact Key is available from the canonical application key list and defaults to E for a default configuration.
2. **Given** an existing stored Fishing key, **When** Settings opens and applies unchanged, **Then** the key round-trips without schema migration or loss.
3. **Given** the Pixel Bus controls, **When** help text is read, **Then** tolerance and sampling intervals are described as live while Block Size explicitly requires managed redeploy, ESO reload or relog, and ESO Weave restart.
4. **Given** a successful coalesced write, **When** confirmation appears, **Then** it claims only that settings were saved and does not falsely imply that staged geometry already changed.

### Edge Cases

- A Fishing timing or reader interval is submitted outside its supported range.
- A Fishing edit arrives while menu, focus, travel, life, or suspend safety gates are closed.
- Multiple reader edits arrive before the worker handles its first update.
- The reader update receiver disconnects after settings have already changed in memory.
- Color tolerance changes while prior menu or life evidence is authorizing generated input.
- Block Size changes together with one or more live reader fields.
- Persistence fails after a live setting has already been applied.

## Requirements

### Functional Requirements

- **FR-001**: Fishing Arm Timeout, Reel Delay, Recast Delay, and Interact Key MUST apply to the live controller without application restart.
- **FR-002**: The Settings modal MUST expose Fishing Interact Key from the canonical `Key::ALL` universe with a stable accessible label and control identity.
- **FR-003**: Fishing configuration MUST be sanitized once and the same effective values MUST be applied live and persisted.
- **FR-004**: Applying a changed Fishing configuration while Fishing is requested or active MUST cancel its pending work without input, clear its request, and expose a settings-change stop reason.
- **FR-005**: Applying an unchanged Fishing configuration MUST NOT alter Fishing request, state, deadline, recovery, or emitted input.
- **FR-006**: A later explicit Fishing enable MUST use the newly applied timings and interact key for all newly scheduled work.
- **FR-007**: Pixel Bus Color Tolerance, Fishing Sample Interval, and Idle Sample Interval MUST apply to the running reader without application restart.
- **FR-008**: Reader updates MUST wake a sleeping worker and MUST be coalesced latest-wins without blocking the GUI thread.
- **FR-009**: The same live reader configuration MUST drive both pixel decoding and poll-interval selection at an iteration boundary.
- **FR-010**: A tolerance change MUST invalidate reader safety observations before a fresh sample may authorize generated input.
- **FR-011**: Block Size MUST remain fixed for the running reader and MUST NOT be sent through the live-update path.
- **FR-012**: Block Size help and diagnostics MUST retain the managed redeploy, ESO reload or relog, and ESO Weave restart contract.
- **FR-013**: The internal heartbeat timeout MUST remain non-user-configurable and unchanged by live reader updates.
- **FR-014**: Fishing timings MUST remain within 0 through 60,000 ms and reader intervals within 1 through 60,000 ms, matching domain loaders.
- **FR-015**: A failed reader update delivery MUST be observable in diagnostics while persisted settings remain available for restart recovery.
- **FR-016**: Settings confirmation MUST distinguish successful persistence from runtime application and MUST NOT claim staged Block Size is already active.
- **FR-017**: Canonical settings, Fishing, PixelBeacon, architecture, safety, and testing documentation MUST describe the implemented per-setting application contract.
- **FR-018**: Semantic tests and documentation policy MUST reject obsolete claims that all settings apply immediately, that live reader fields require restart, or that Fishing Interact Key lacks an editor.
- **FR-019**: S062 MUST close issue #95, archive completed plan 031 with PR #99 evidence, and establish plan 032 as the sole active build plan.

### Key Entities

- **Effective Fishing configuration**: The sanitized four-field value used by the live controller and persistence.
- **Fishing configuration generation boundary**: A safe transition that cancels requested or active work before the next explicit enable uses a new configuration.
- **Live reader configuration**: Tolerance plus fast and idle sampling intervals, excluding block geometry and heartbeat timeout.
- **Reader update channel**: A non-blocking sender and wakeable receiver that coalesce rapid complete-value updates.
- **Staged geometry**: Persisted block size and managed add-on deployment state that become coherent after ESO reload or relog and application restart.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Automated tests prove all four Fishing controls round-trip and the controller sees changed values before restart.
- **SC-002**: Every tested requested or active Fishing phase produces zero sink operations during reconfiguration and ends disabled with no deadline.
- **SC-003**: Reader tests prove a live update wakes an old wait, rapid edits resolve to the newest value, and block size plus heartbeat timeout stay unchanged.
- **SC-004**: Tolerance-change tests prove old safety observations cannot remain authoritative across the configuration boundary.
- **SC-005**: Zero shipped strings or canonical pages retain the obsolete application-timing or missing-editor claims tracked by #95.
- **SC-006**: Full format, strict Clippy, locked tests, documentation policy, spelling, text hygiene, and mdBook validation pass.

## Assumptions

- Fishing configuration edits are rare and user initiated, so stopping requested automation is safer and clearer than mixing configuration generations.
- The existing coalesced persistence confirmation accurately means saved to disk and may remain unchanged.
- The standard library channel is sufficient; this slice does not need a new async runtime or dependency.
- Pixel Bus block geometry remains a coordinated two-process contract between PixelBeacon and ESO Weave.

## Explicit Deviation

The original issue permits either live application or restart-required labeling. S062 chooses a hybrid contract because treating Block Size like scalar reader settings would desynchronize add-on output and capture geometry, while forcing Fishing and scalar reader edits to restart would preserve unnecessary stale runtime state. Changed Fishing configuration deliberately turns an active or requested session off rather than retaining old deadlines, because an old deadline combined with a new key or timing generation could synthesize an unintended action.

## Out of Scope

- Live Block Size switching or automatic geometry renegotiation.
- A user-facing heartbeat-timeout control.
- PixelBeacon protocol, add-on payload, settings schema, or dependency changes.
- Hot reloading the ESO AddOns path fallback.
- Embedded documentation expansion, release work, and live-game verification.
