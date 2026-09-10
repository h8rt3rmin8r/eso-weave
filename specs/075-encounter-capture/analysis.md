# S075 Spec-kit Analysis

## Pre-implementation Gate

Result: PASS after the corrections recorded below.

The specification, clarification decisions, requirements checklist, capture
safety checklist, research, data model, schema, fixture, plan, quickstart, and
tasks form one bounded implementation slice for issue #132.

## Coverage Matrix

| Concern | Requirement authority | Planned evidence |
| --- | --- | --- |
| Explicit one-shot consent | FR-002, FR-003, FR-018, FR-019 | Executed Lua command and state tests |
| Event family coverage | FR-005 through FR-011 | Representative callback harness |
| Anonymous actors | FR-004, FR-012 | Sensitive-string injection and actor-map assertions |
| Bounds and truthful loss | FR-013 through FR-017 | Event, byte, actor, clock, and lifecycle boundary tests |
| Local ownership | FR-018 through FR-020 | Clear and cross-addon immutability tests |
| Confinement | FR-021 through FR-024 | Executed teardown tests and forbidden-source scan |
| Contract and docs | FR-025, FR-026, FR-028 | Fixture invariants, canonical docs, and project gates |
| Governance | FR-027 | Constitution 2.2.0 and aligned agent guidance |

## Findings and Resolutions

1. The 2.1.0 constitution authorizes exactly two addon bridges. Issue #132
   requires a genuinely distinct combat-capture owner. S075 explicitly amends
   the boundary to three narrow addons and adds encounter-specific confinement
   requirements before implementation.
2. Source-string-only tests cannot prove the privacy and loss state machine.
   The plan adds a test-only vendored Lua 5.1 runtime that executes the exact
   production source against deterministic ESO callbacks.
3. Exact SavedVariables serialized size is not observable inside the addon.
   Requirements now call the byte counter a conservative estimate and reserve
   both event and estimated-byte terminal capacity. Exact storage remains issue
   #131 verification.
4. The S069 envelope includes a SHA-256 content hash, but implementing
   cryptography in addon Lua would duplicate importer authority. S075 emits a
   bounded versioned envelope; issue #133 canonicalizes and hashes it.
5. A fixture cannot be both complete and contain an omitted sequence. The
   representative contract fixture is intentionally partial, includes every
   event family, declares one omitted sequence before its discontinuity, and
   uses counts that distinguish stored events from authoritative sequence span.
6. Capturing immediately when armed during combat would mislabel a partial
   encounter. The state machine waits for combat to end, then begins only at the
   next clean combat start.
7. Desktop lifecycle controls would expand S075 into application interface and
   filesystem work not required by issue #132. S075 ships a distinct managed
   identity and capture artifact; later lifecycle work must retain the separate
   marker gate.

## Constitution Gate

- Full spec-kit artifact sequence exists before implementation.
- Constitution amendment is a required first implementation task, not a hidden
  exception.
- PixelBeacon, discovery collector, input, automation, and desktop data remain
  outside the capture owner.
- Test-first tasks execute the actual Lua state machine.
- Configuration and raw desktop storage remain unchanged.
- A full Cargo merge gate is mandatory because Cargo metadata changes.
- No pinned workflow, packaging, release, or script artifact is planned.

No unresolved CRITICAL conflict, ambiguous requirement, duplicate authority,
unsupported live claim, impossible durability promise, or template token
remains. Implementation may begin after applying the planned 2.2.0 amendment.

## Post-implementation Gate

Result: PASS.

The production addon source runs under the vendored Lua 5.1 test harness. The
seven S075 tests establish dormant load, explicit one-shot consent, clean
encounter boundaries, all required event families, anonymous actor identity,
event and estimated-byte overflow accounting, actor overflow, clock-reset
discontinuity, lifecycle finalization, callback containment, teardown, saved
capture recovery, source confinement, and normalized fixture invariants.

The representative fixture is reconciled with the implemented contract as a
truthful partial capture: sequence 13 is omitted, sequence 14 declares the
exact loss, sequence 15 terminates the encounter, 14 events are stored, and
one sequence is omitted. The full Rust suite, formatting, lint, optimized
binary build, mdBook test and build, link checking, documentation policy,
spelling, JSON parsing, whitespace, UTF-8 without BOM, forbidden-dash, and
mojibake gates pass.

No unresolved critical conflict, privacy leak, unsafe authority expansion,
unbounded capture path, or unsupported completion claim remains.
