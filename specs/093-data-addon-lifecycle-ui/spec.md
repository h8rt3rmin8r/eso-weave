# Feature Specification: Data Addon Lifecycle UI

**Feature Branch**: `codex/s093-data-addon-lifecycle-ui`

**Created**: 2026-09-13

**Status**: Implemented

**Input**: Issue #185 under coordinating epic #182. Present ESO Weave Data
directly beneath PixelBeacon with safe lifecycle controls and evidence-scoped
status facts.

## Clarifications

Routine choices are resolved under the build-phase autopilot policy.

- S093 reuses the existing marker-gated, exact-inventory, atomic data-addon
  lifecycle authority. Repair is an atomic reinstall, never delete-then-install.
- Installed, managed, compatible, configured enabled, loaded, reload required,
  runtime available, and collection activity are separate facts.
- Supported desktop evidence cannot prove current-session ESO addon enablement,
  loading, or collection. Those facts display as unconfirmed unless a future
  supported source proves them. A running ESO process is not load evidence.
- A compatible last-flushed SavedVariables envelope is historical load
  evidence only. Module status read from that envelope is labeled last saved,
  never live.
- Catalog and encounter activity remain separate. S093 does not add a command
  transport, custom keybinding, operating mode, or bulk ingestion path.
- The Catalog Update dialog keeps capture and build workflow controls but no
  longer owns duplicate install or uninstall controls.
- Lifecycle failures retain the last known status and provide a safe, specific
  remediation. Unmanaged targets are never modified.

## User Scenarios and Testing

### User Story 1 - See truthful data-addon readiness (Priority: P1)

As an ESO Weave user, I want the data addon shown directly beneath PixelBeacon
so I can distinguish installation readiness from in-game and collection state.

**Independent Test**: Render missing, current, outdated, unmanaged, running,
awaiting-reload, unconfirmed, and unavailable observations at supported widths
and verify that no displayed fact implies another.

**Acceptance Scenarios**:

1. **Given** the System and State region is open, **When** it renders, **Then**
   ESO Weave Data appears immediately after PixelBeacon at every supported
   width.
2. **Given** a managed current package, **When** ESO is closed, **Then** the UI
   reports installed, managed, and compatible while keeping runtime, enabled,
   loaded, and collection facts separate.
3. **Given** ESO is running without same-session addon evidence, **When** the
   status renders, **Then** runtime is available but loaded and collecting are
   unconfirmed.
4. **Given** no supported same-session evidence channel, **When** enabled,
   loaded, catalog, and encounter facts render, **Then** each remains
   unconfirmed and explains the evidence boundary.

---

### User Story 2 - Safely manage the data addon (Priority: P1)

As an ESO Weave user, I want install, update, repair, and uninstall controls in
the main interface so lifecycle work is easy to find and remains ownership safe.

**Independent Test**: Exercise every action against absent, current, outdated,
unmanaged, linked, drifted, and failing targets and compare neighboring bytes.

**Acceptance Scenarios**:

1. **Given** no package is installed, **When** Install succeeds, **Then** the
   exact managed package is present and reload is required if ESO may be active.
2. **Given** a managed outdated package, **When** Update or Repair succeeds,
   **Then** it becomes current through the existing atomic replacement path.
3. **Given** an unmanaged target, **When** the UI renders or an action is
   attempted, **Then** mutation controls are unavailable and manual remediation
   names the exact target without deleting it.
4. **Given** a managed package, **When** Uninstall is confirmed, **Then** only
   that package is removed and reload guidance reflects ESO runtime certainty.
5. **Given** an operation fails, **When** the result is surfaced, **Then** the
   prior installation remains recoverable and the message identifies the next
   safe action.

---

### User Story 3 - Understand how data status differs from PixelBeacon (Priority: P2)

As an operator, I want concise guidance in the interface and documentation so I
do not confuse PixelBeacon's live visual signal with flush-bound data evidence.

**Independent Test**: Trace every UI term, tooltip, status-reference entry,
troubleshooting step, and screenshot to the same evidence contract.

**Acceptance Scenarios**:

1. **Given** both addon rows are visible, **When** equivalent lifecycle states
   occur, **Then** their action and ownership language is equivalent.
2. **Given** PixelBeacon is signaling while data evidence is unconfirmed,
   **When** the user reads the rows, **Then** neither row implies the other's
   transport is healthy.
3. **Given** any missing, outdated, disabled-or-unconfirmed, unmanaged, failed,
   or awaiting-reload state, **When** help is requested, **Then** documentation
   provides a precise safe remediation.

### Edge Cases

- The configured AddOns root is missing, inaccessible, a link, or not a directory.
- The target changes between inspection and mutation.
- A managed update fails before commit, during commit, rollback, or cleanup.
- ESO runtime detection is unknown rather than running or stopped.
- SavedVariables may be absent, oversized, malformed, unsupported, or stale;
  S093 does not read it for lifecycle presentation.
- The package was installed while ESO was running and status is refreshed
  before `/reloadui`, relog, or exit.
- Last-flushed module state says running or capturing even though ESO has since
  stopped or changed state.
- The Catalog Update dialog opens while the main lifecycle row changes.
- The window is narrow, text is long, or a confirmation dialog is open.

## Requirements

### Functional Requirements

- **FR-001**: The main interface MUST place an ESO Weave Data lifecycle row
  immediately beneath the PixelBeacon lifecycle row at every supported width.
- **FR-002**: The surface MUST present installation, managed ownership,
  compatibility, configured enablement, load evidence, reload requirement,
  runtime availability, catalog activity, and encounter activity as separate
  facts.
- **FR-003**: Unknown, unobservable, and historical facts MUST be labeled as
  such and MUST NOT be promoted to current-session confirmation.
- **FR-004**: A running ESO process MUST NOT by itself imply that the data addon
  is enabled, loaded, or collecting.
- **FR-005**: S093 MUST leave load and module activity unconfirmed. A future
  compatible saved-envelope reader MAY establish historical evidence only.
- **FR-006**: Lifecycle presentation MUST provide Install for an absent target,
  Update and Repair for managed drift, Repair for a managed current target, and
  Uninstall for any managed target.
- **FR-007**: Every lifecycle mutation MUST call the existing data-addon
  lifecycle authority and preserve its root resolution, link rejection,
  marker, exact inventory, race detection, atomic replacement, rollback, and
  neighbor-preservation guarantees.
- **FR-008**: Repair and Update MUST NOT delete the installed package before a
  verified replacement is ready.
- **FR-009**: Unmanaged or unavailable targets MUST expose no mutating action.
- **FR-010**: Uninstall MUST require a data-addon-specific confirmation that
  cannot be confused with PixelBeacon removal.
- **FR-011**: Successful mutations MUST refresh status and provide precise
  reload or restart guidance derived from running, stopped, or unknown runtime.
- **FR-012**: Failed inspection or mutation MUST preserve the last known status,
  expose an error role, and provide specific safe remediation without logging
  private absolute paths.
- **FR-013**: Main-interface and Catalog Update views MUST consume one lifecycle
  authority; the dialog MUST not retain duplicate install or uninstall buttons.
- **FR-014**: Lifecycle status inspection MUST avoid parsing the full shared
  SavedVariables payload on every UI frame.
- **FR-015**: Any saved-evidence read MUST retain the shared 128 MiB outer bound,
  stable no-follow read, token and entry limits, and version checks.
- **FR-016**: Equivalent PixelBeacon and data-addon lifecycle actions MUST use
  shared presentation vocabulary where their semantics match.
- **FR-017**: Controls MUST have unique accessible names, remain operable at
  supported widths, and never rely on color alone.
- **FR-018**: User guidance, status reference, troubleshooting, test strategy,
  and affected deterministic screenshots MUST match the shipped interface.
- **FR-019**: S093 MUST update issue #185 and its Project item while leaving
  epic #182 as the multi-slice coordinator.
- **FR-020**: S093 MUST NOT add custom bindings, desktop-to-addon commands,
  encounter operating modes, lossless capture, or native-log ingestion.

### Key Entities

- **Data Addon Observation**: One immutable snapshot of filesystem, ownership,
  compatibility, runtime, reload, unconfirmed module activity, and error.
- **Evidence Fact**: A value paired with current, historical, unconfirmed, or
  unavailable provenance.
- **Lifecycle Action**: Install, Update, Repair, or Uninstall, enabled only when
  the current ownership state makes that mutation safe.
- **Reload Requirement**: A retained outcome created by a successful mutation
  while ESO is running or uncertain, cleared only by defensible stopped-state
  evidence or a subsequent observation that proves the transition.
- **Module Activity**: Separate catalog and encounter states that remain
  unconfirmed until a supported evidence source exists.

## Success Criteria

### Measurable Outcomes

- **SC-001**: UI structure tests prove the data-addon lifecycle row immediately
  follows PixelBeacon at all supported narrow and wide geometries.
- **SC-002**: The required observation matrix covers 100 percent of missing,
  current, outdated, unmanaged, running, stopped, unknown, awaiting-reload,
  unconfirmed, unavailable, and failed cases.
- **SC-003**: Zero observation fixtures collapse installed, enabled, loaded,
  compatible, runtime, catalog, or encounter facts into a single state.
- **SC-004**: One hundred percent of unmanaged, linked, raced, and failed
  lifecycle fixtures leave the target and neighboring addon bytes unchanged.
- **SC-005**: Every enabled lifecycle control maps to exactly one safe action,
  every destructive action has a unique confirmation, and every failure has a
  specific remediation.
- **SC-006**: Deterministic screenshot coverage includes missing and managed
  current data-addon states in both themes and supported viewport families.
- **SC-007**: Static inspection finds zero new binding, input synthesis,
  command-ingress, PixelBus bulk-data, or SavedVariables deletion paths.
- **SC-008**: Specification analysis, formatting, strict lint, full locked
  tests, documentation policy, link, UTF-8, spelling, and mojibake gates pass.

## Assumptions

- S092's `EsoWeaveData` identity and lifecycle API are stable authority.
- ESO exposes no supported same-session desktop channel for data-addon load or
  collection evidence in this slice.
- Last-flushed SavedVariables may be stale and is useful only when provenance is
  explicit.
- Issue #186 owns later lossless capture and issue #183 owns later modes.

## Out of Scope

- Changes to PixelBeacon transport or live signal semantics.
- Addon-side lifecycle, package identity, or SavedVariables schema changes.
- A command transport, binding discovery, custom action, or generated input.
- Lossless capture, continuous capture, and native-log qualification.
- Release publication, version tagging, or merging the pull request.
