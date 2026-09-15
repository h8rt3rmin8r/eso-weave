# Research: Persistent Auto Potion Request

**Feature**: [spec.md](spec.md) | **Date**: 2026-09-15

All clarifications were resolved under the autopilot decision policy. No NEEDS
CLARIFICATION markers remain.

## R1: Store intent in session state

**Decision**: Persist the request in `state.json`.

**Alternatives considered**:

- Put it in `config.json` with thresholds and bindings.
- Create a dedicated Auto Potion runtime file.
- Keep it memory-only.

**Rationale**: Suspension and Fishing establish `state.json` as the owner of
restored operator intent. `config.json` owns reusable configuration, while a new
file would add synchronization and recovery paths for one Boolean. Memory-only
is the behavior issue #172 explicitly replaces.

## R2: Advance schema version and use an additive default

**Decision**: Advance session schema 3 to 4 and add a serde-defaulted Boolean
whose default is `false`.

**Alternatives considered**:

- Add the field without advancing the version.
- Write a manual per-version migration function.
- Reject pre-v4 state.

**Rationale**: The persisted shape changes and should identify itself accurately.
The only migration rule is absence to false, which serde already expresses
without destructive rewriting. Rejecting old state would discard suspension,
Fishing, API cache, and geometry unnecessarily.

## R3: Restore through the controller, never around it

**Decision**: Call `AutoPotionController::set_enabled` during model restoration
and do nothing else.

**Alternatives considered**:

- Restore a separate model Boolean and reconcile later.
- Tick immediately after restore.
- Restore selected effective-state fields or telemetry.

**Rationale**: The controller already owns the request and derives a truthful
immediate blocker from fail-closed defaults. A parallel Boolean can diverge. An
immediate tick or persisted evidence would turn storage into action authority and
violate the issue's safety boundary.

## R4: Reuse the session scheduler for both toggle paths

**Decision**: Mark session state in the existing `SetAutoPotion` intent.

**Alternatives considered**:

- Save directly inside the UI or F3 handlers.
- Save synchronously inside the controller.
- Add a second scheduler.

**Rationale**: Both interaction paths already converge on the intent and model.
The current scheduler coalesces writes and the normal-close flush protects the
settle window. Lower layers do not know the config directory and should not gain
storage responsibility.

## R5: Preserve the whole-document invalid fallback

**Decision**: Let a non-Boolean request fail session deserialization and use the
existing complete safe default plus notice.

**Alternatives considered**:

- Coerce strings or numbers.
- Ignore only the malformed field.

**Rationale**: Coercion creates ambiguous user intent. Partial application would
quietly mix untrusted state with defaults and differ from every existing invalid
session behavior. Failing the document is deterministic and safely disables all
restored action requests.

## R6: Explicitly reverse the historical non-persistence decision

**Decision**: Treat issue #172 as the newer product authority and preserve the
old safety concern through fail-closed restoration rather than non-persistence.

**Rationale**: S039 R7 and S043 FR-002 intentionally selected memory-only
enablement. Repeating that decision would directly violate the current issue.
Persisting only request intent achieves consistent toggle behavior without
persisting eligibility or bypassing any safety evidence.

## R7: Use canonical documentation instead of a new blog surface

**Decision**: Update the bundled feature, reference, startup, troubleshooting,
and architecture documentation. Do not create a blog artifact.

**Rationale**: The repository has no active website or blog tree. Canonical
bundled documentation is the maintained user-facing publication surface and is
covered by policy, link, render, and spelling gates.
