# Safety and Logic Checklist: Documentation Completeness

**Purpose**: Verify that S059 documents shipped decision logic and input-safety
boundaries without weakening fail-closed guarantees or inventing behavior.

**Created**: 2026-09-07

**Feature**: [spec.md](../spec.md)

## Authority and Scope

- [x] CHK001 Every safety or logic claim cites current source and relevant test
  evidence using an exact repository path and stable symbol or test name.
- [x] CHK002 User guarantees, implementation details, diagnostics, and
  version-sensitive facts are distinguished wherever confusing them could
  change user action or maintainer interpretation.
- [x] CHK003 No documentation claim is made true by changing runtime, addon,
  packaging, or release behavior in this docs-only slice.
- [x] CHK004 A disagreement between existing prose and confirmed source plus
  tests is corrected in prose and recorded in coverage evidence.
- [x] CHK005 A disagreement between source and tests blocks completion of the
  affected coverage item and is surfaced for follow-up instead of being guessed.

## Input and Automation Boundaries

- [x] CHK006 Physical input, intercepted input, suppressed input, synthesized
  input, observed game state, and requested automation state are described as
  distinct concepts.
- [x] CHK007 Focus loss, suspension, invalid menu evidence, stale or lost beacon
  signal, unavailable readings, and Unknown safety state never appear to
  authorize automated input.
- [x] CHK008 The menu gate is described only as able to relax interception, not
  as authority to synthesize input.
- [x] CHK009 Hook callbacks are documented as non-blocking classification and
  handoff boundaries, including recursion rejection for generated input.
- [x] CHK010 Windows and Linux input implementations preserve the same user
  safety guarantees while platform-specific mechanisms remain labeled as
  implementation details.
- [x] CHK011 Rebinding documentation covers capture behavior, conflict rejection,
  defaults, persistence, and recovery without implying unsupported keys.

## Decision Flows and State Machines

- [x] CHK012 Game discovery and lifecycle documentation covers no installation,
  installed, inactive, active, travel, world, life, movement, roll dodge, menu,
  focus, suspension, and signal-loss states that affect shipped behavior.
- [x] CHK013 Weave documentation covers skill-slot selection, weapon-bar state,
  timing and latency inputs, physical-event replacement, cooldown behavior, and
  every condition that blocks or cancels a sequence.
- [x] CHK014 Fishing documentation covers setup, bait and interaction evidence,
  state transitions, status meanings, recovery, signal loss, and the rule that
  loss of trustworthy evidence prevents further input.
- [x] CHK015 Auto Potion documentation covers requested versus effective state,
  resource watches, the fixed OR rule, quickslot classification and cooldown,
  retry behavior, direct menu and suspension gates, and every fail-closed block.
- [x] CHK016 PixelBeacon and pixel-bus documentation covers geometry, version
  negotiation, markers, checksums or complements, freshness, corruption,
  supported backward compatibility, unavailable values, and recovery sampling.
- [x] CHK017 Ultimate documentation covers current and maximum resource, active
  primary or backup bar cost, special-bar unavailability, readiness, and the
  distinction between numeric zero and dormant or unavailable state.
- [x] CHK018 Configuration documentation covers schema ownership, migration,
  unknown fields or newer schemas, persisted settings, session state, hotkeys,
  geometry recovery, and logging without promising data that is not persisted.

## User Recovery and Diagnosis

- [x] CHK019 Every safety-sensitive procedure states its prerequisites, expected
  signal, failure signal, safe stopped state, recovery action, and diagnostic
  evidence when those dimensions apply.
- [x] CHK020 Troubleshooting steps never instruct a user to disable focus,
  suspension, signal-validation, containment, managed-marker, or other safety
  protections to make automation run.
- [x] CHK021 Install, update, and uninstall guidance preserves AddOns-directory
  containment and the managed-marker uninstall guard.
- [x] CHK022 Privacy and network guidance states what is local, what may use the
  network, when it is used, and what evidence supports that statement.
- [x] CHK023 Version-sensitive game, addon, operating-system, and package claims
  carry an as-of boundary or point to maintained verification guidance.

## Accessibility and Search Safety

- [x] CHK024 Every safety-relevant image or diagram has a meaningful text
  alternative that communicates the same states, branches, and outcomes.
- [x] CHK025 No safe, blocked, ready, unavailable, warning, or failure state is
  communicated by color alone.
- [x] CHK026 Player-language aliases for safety and recovery concepts are visible
  in published prose and resolve through generated search.
- [x] CHK027 Tables and sequences preserve a readable order and identify row and
  column meaning without relying on visual position alone.

## Test-first and Preservation Evidence

- [x] CHK028 Negative fixtures fail when a safety topic is omitted, mapped twice,
  mapped outside published documentation, or marked complete without shaped
  source and test evidence.
- [x] CHK029 Negative fixtures fail when Unknown or unavailable evidence is
  described as permissive, when a required block or recovery path is absent, or
  when a search alias exists only in comments or project metadata.
- [x] CHK030 Negative fixtures fail for empty or decorative alternatives on
  substantive visuals and for instructions that rely on color alone.
- [x] CHK031 All S058 preservation excerpts remain present, or an intentional
  correction updates the ledger, frozen manifest, and loss-prevention fixture
  together with reviewed evidence.
- [x] CHK032 Full documentation policy, mdBook test and build, generated-site
  validation, Cargo parity, and diff checks pass.
- [x] CHK033 Every changed text file is UTF-8 without a byte-order mark, uses LF,
  contains no mojibake, and contains no en dash or em dash.
