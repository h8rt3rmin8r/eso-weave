# Feature Specification: Game Runtime and Context Truth

**Feature Branch**: `codex/041-game-runtime-context`

**Created**: 2026-09-03

**Status**: Draft

**Input**: GitHub issues #22 and #23, grouped as work slice S041.

## Overview

ESO Weave currently describes its own process as running while also presenting
gameplay as the active game context before The Elder Scrolls Online has started.
The product needs a truthful observation model that keeps installation,
launcher, game, focus, PixelBeacon freshness, and in-game surface separate.

This slice provides the shared runtime and context foundation used by the
interface and by every feature that can emit input. It closes issues #22 and #23
without repairing quickslot detection or auto-potion triggering, which remain
separate follow-up work.

## Clarifications

### Session 2026-09-03

- Q: Should the field be named Game Focus or Game Context? -> A: Game Context,
  because the value combines operating-system focus with the observed in-game
  surface.
- Q: What happens to requested automation state when the game becomes inactive?
  -> A: Runtime inactivity blocks effective action without resetting a request;
  existing feature-specific signal-loss reset policies remain until their own
  tracked redesigns.
- Q: Does an ESO uninstall entry prove the standalone provider? -> A: No. A
  platform-owned manifest wins for the same validated root; a generic ESO entry
  is standalone evidence only when no stronger provider claims that root.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - See Whether ESO Is Available and Running (Priority: P1)

As an ESO Weave user, I can see whether ESO is installed, whether its launcher
is open, and whether the game itself is active, so the application never claims
that gameplay is occurring merely because ESO Weave is running.

**Why this priority**: Every live metric and input-driving feature depends on a
truthful answer to whether the game exists and is active.

**Independent Test**: Start ESO Weave through each supported lifecycle state and
confirm that installation and runtime are reported independently and converge
within two seconds of each transition.

**Acceptance Scenarios**:

1. **Given** ESO is not installed and no relevant process is present, **When**
   ESO Weave starts, **Then** installation is Not detected and runtime is
   Inactive.
2. **Given** a validated ESO installation and no relevant process, **When** ESO
   Weave starts, **Then** its provider is identified and runtime is Inactive.
3. **Given** a validated launcher process and no game process, **When** the
   launcher opens, **Then** runtime becomes Launcher open without claiming that
   the launcher is ready.
4. **Given** the launcher and game are both present, **When** the launcher
   closes, **Then** runtime remains Active until the game itself exits.
5. **Given** the game is Active and the launcher remains open, **When** the game
   exits, **Then** runtime becomes Launcher open.
6. **Given** a platform observation fails or conflicts with another observation,
   **When** runtime is reduced, **Then** the result is Unknown rather than a
   guessed positive or negative state.

---

### User Story 2 - See Truthful Game Context (Priority: P1)

As an ESO Weave user, I can distinguish active gameplay, an open in-game
surface, an unfocused game, and an unavailable PixelBeacon signal, so the Game
Context field always describes evidence the application actually has.

**Why this priority**: The existing Game Menu field turns missing evidence into
Gameplay, which is both misleading and unsafe for downstream automation.

**Independent Test**: Exercise active gameplay, every recognized in-game
surface, focus changes, invalid samples, loading, addon reload, and signal loss;
each condition produces one unambiguous Game Context value.

**Acceptance Scenarios**:

1. **Given** ESO is not Active, **When** the main view is shown, **Then** Game
   Context says Not detected and never says Gameplay.
2. **Given** ESO is Active and unfocused, **When** the main view is shown,
   **Then** Game Context says Unfocused without inventing a new in-game surface.
3. **Given** ESO is Active and focused with a fresh valid observation that no
   surface is open, **When** the main view is shown, **Then** Game Context says
   Gameplay.
4. **Given** ESO is Active and focused with a fresh valid named surface, **When**
   the main view is shown, **Then** Game Context names that surface and input is
   gated.
5. **Given** ESO is Active but the signal is stale, invalid, loading, or never
   observed, **When** the main view is shown, **Then** Game Context says Signal
   unavailable and never says Gameplay.
6. **Given** a newly introduced or unidentified surface, **When** it is observed,
   **Then** Game Context says Other menu and input remains gated.

---

### User Story 3 - Enter and Recover From Dormancy Safely (Priority: P1)

As an ESO Weave user, I see one consistent dormant presentation when ESO is not
active, while my saved preferences and requested toggle choices remain intact
for a later session.

**Why this priority**: Truthful top-level state is not sufficient if stale live
metrics or automation controls continue to imply that the game is observable or
safe to act on.

**Independent Test**: Exit and restart ESO without restarting ESO Weave; verify
that all live observations become dormant, no input can be emitted while
dormant, and observation recovers when the game and PixelBeacon return.

**Acceptance Scenarios**:

1. **Given** live metrics were observed, **When** ESO exits, **Then** weapon,
   combat, movement, resources, quickslot, cooldown, and context values become
   dormant within two seconds.
2. **Given** an input-driving feature was requested, **When** ESO becomes
   inactive or unfocused, **Then** the feature cannot act but the user's
   requested choice is not silently destroyed.
3. **Given** ESO remains Active while PixelBeacon is unavailable, **When** the
   main view is shown, **Then** runtime remains Active while only
   PixelBeacon-derived observations become unavailable.
4. **Given** ESO and PixelBeacon return after dormancy or signal loss, **When**
   fresh observations resume, **Then** the interface recovers without either
   application being restarted.

### Edge Cases

- More than one supported ESO installation is detected.
- Installation evidence points to a root that was moved or uninstalled.
- The launcher starts or exits during the same observation interval as the game.
- The game exits while a menu or text-entry surface is open.
- The game remains running while the launcher closes.
- Process, window, or installation evidence becomes inaccessible mid-session.
- A focus transition occurs while PixelBeacon is stale.
- A valid no-menu observation follows an invalid sample without a restart.
- An older PixelBeacon version publishes a valid surface code.
- A localized or non-default installation path contains non-ASCII characters.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The product MUST represent installation state independently from
  runtime state.
- **FR-002**: Installation state MUST distinguish Not detected, one validated
  provider and root, and Ambiguous or Unknown evidence.
- **FR-003**: Supported provider discovery MUST cover standalone ESO on Windows,
  Steam on Windows, Epic Games Store on Windows, and Steam Proton on Linux.
- **FR-004**: Provider identity MUST be based on provider evidence and validated
  installation artifacts; a Documents or AddOns directory alone MUST NOT prove
  that ESO is installed.
- **FR-005**: Runtime state MUST distinguish Inactive, Launcher open, Active,
  and Unknown.
- **FR-006**: A valid game process or window observation MUST take precedence
  over every launcher observation.
- **FR-007**: Launcher open MUST describe process presence only and MUST NOT be
  presented as launcher readiness.
- **FR-008**: Runtime and installation observations MUST refresh during the
  application session and invalidate stale paths or vanished processes.
- **FR-009**: Observation failures and contradictory evidence MUST fail to
  Unknown or Ambiguous without panicking or fabricating a state.
- **FR-010**: The product MUST represent game runtime, operating-system focus,
  PixelBeacon freshness, and in-game surface as independent observations.
- **FR-011**: A valid observation that no in-game surface is open MUST remain
  distinguishable from an invalid, stale, lost, or never-observed signal.
- **FR-012**: The main view MUST replace Game Menu with Game Context and provide
  the same explanatory text through pointer hover and keyboard focus.
- **FR-013**: Game Context MUST distinguish Not detected, Unfocused, Gameplay,
  System menu, Map, Inventory, Mail, Character, Guild store, Crown store,
  Journal, Chat entry, Other menu, Signal unavailable, and Unknown.
- **FR-014**: Gameplay MUST be presented only when ESO is Active and focused and
  a fresh valid surface observation says that no menu is open.
- **FR-015**: Every recognized menu or text-entry state, including Other menu,
  MUST gate input-driving behavior.
- **FR-016**: When ESO is not Active, every game-derived metric MUST use one
  shared dormant meaning instead of Gameplay, remembered data, dashes, or a
  mixture of unrelated unavailable labels.
- **FR-017**: When ESO is Active but PixelBeacon is unavailable, runtime MUST
  remain Active and only PixelBeacon-derived observations MUST become
  unavailable.
- **FR-018**: Runtime dormancy MUST prevent effective input-driving behavior
  without changing saved configuration or requested toggle choices. Existing
  feature-specific signal-loss reset policies remain unchanged until their
  separately tracked redesigns.
- **FR-019**: State changes MUST be emitted and logged only on transitions at
  normal logging levels.
- **FR-020**: Normal logs MUST identify the evidence category and resulting
  state without exposing personal installation paths.
- **FR-021**: Lifecycle and context transitions MUST converge within two seconds
  under normal local-system conditions.
- **FR-022**: The feature MUST recover from game restart, launcher exit, focus
  changes, and PixelBeacon signal restoration without restarting ESO Weave.
- **FR-023**: Existing input recursion prevention, focused-window scoping,
  non-blocking hook behavior, fishing signal-loss safety, and managed addon
  lifecycle protections MUST remain intact.
- **FR-024**: The feature MUST NOT read game memory, inject into game processes,
  inspect network traffic, launch ESO, or infer launcher readiness from launcher
  interface pixels.
- **FR-025**: User and maintainer documentation MUST describe installation,
  runtime, focus, freshness, context, dormancy, and recovery using the same
  vocabulary as the interface.

### Key Entities

- **Installation observation**: Evidence that a provider owns a validated ESO
  installation root, or that evidence is absent, ambiguous, or unavailable.
- **Runtime observation**: Independent evidence about the launcher and game,
  reduced with game activity taking precedence.
- **Focus observation**: Whether the active ESO window owns keyboard focus, is
  unfocused, or cannot be determined.
- **PixelBeacon freshness**: Whether a valid observation is fresh, stale or
  lost, or has never been observed.
- **In-game surface observation**: A valid named surface, a valid no-surface
  state, Other menu, or no authoritative observation.
- **Game Context**: The user-facing projection of runtime, focus, freshness, and
  in-game surface evidence.
- **Dormant observation**: The shared meaning applied to game-derived values
  while ESO is not Active.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In a lifecycle matrix covering no installation, installed and
  inactive, launcher open, game active, launcher exit during play, game exit
  with launcher open, and full exit, every state is correct within two seconds.
- **SC-002**: All supported distribution providers are proven by reviewable,
  sanitized evidence and a moved or invalid root is rejected on the next
  observation.
- **SC-003**: Across every valid surface, focus state, and freshness state, zero
  unavailable or stale cases are presented as Gameplay.
- **SC-004**: Within two seconds of ESO exiting, 100 percent of game-derived
  fields use the shared dormant meaning and zero input attempts can occur.
- **SC-005**: Closing the launcher during active play causes zero false runtime
  demotions.
- **SC-006**: Twenty consecutive unchanged observation intervals produce no
  repeated normal-level transition log entries.
- **SC-007**: Game restart, focus recovery, and PixelBeacon signal restoration
  each recover without restarting ESO Weave.
- **SC-008**: Pointer hover and keyboard focus expose identical Game Context
  help text, and color is never the only state cue.
- **SC-009**: The full existing safety and regression suite passes without
  weakening or skipping a protected test surface.
- **SC-010**: Windows and Linux validation cover successful, absent, ambiguous,
  and access-failure observations for the providers each platform supports.

## Assumptions

- S041 closes GitHub issues #22 and #23 and updates coordinator issue #21.
- Game Context is the canonical field label because the value includes both
  operating-system focus and in-game surface state.
- Runtime inactivity preserves requested feature choices while making their
  effective state dormant or blocked. S041 does not change existing
  feature-specific signal-loss reset policies or redesign the full auto-potion
  status interface tracked by #25.
- Quickslot reconstruction (#24), auto-potion repair (#25), PixelBeacon geometry
  negotiation (#26), and the dashboard redesign (#28 and #29) remain out of
  scope.
- Exact standalone, Epic, and launcher identifiers require reviewable evidence
  during planning and are never inferred from anecdotes.
