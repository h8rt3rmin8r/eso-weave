# Research: Documentation Completeness

## Scope

S059 closes issue #81 by completing the published manual under `docs/src`.
This slice changes documentation and documentation validation only. It does not
change application behavior, PixelBeacon, packaging, release automation, or
repository governance.

## Decision 1: Treat current code and tests as behavioral evidence

**Decision**: Verify every action-authorizing statement against the current
implementation and its tests. Use shipped behavior as evidence, but do not turn
an implementation defect into a documented guarantee.

**Rationale**: The S058 corpus preserved the prior explanation at current factual
fidelity. Issue #81 requires a second pass for completeness and current logic.
The source and tests expose ordering, recovery, and failure behavior that the
prior monolith did not explain.

**Alternatives rejected**:

- Expand the preserved prose without rechecking code. This would retain known
  omissions and stale ordering claims.
- Treat tests as the only authority. Some platform and startup paths have no
  complete integration test, so source evidence remains necessary.

## Decision 2: Keep S059 documentation-only

**Decision**: Record code defects and test gaps as deferred discrepancies. Do not
repair or normalize them in the S059 manual.

**Rationale**: Issue #81 owns documentation completeness. Mixing input or
platform fixes into the slice would weaken review boundaries and violate the
requested docs-only scope.

The initial audit identified three deferred implementation concerns:

1. The Linux uinput capability list omits the default fishing interact key `E`
   and the Auto Potion toggle key `F3`, although later mappings use them.
2. A running weave sequence rechecks life, roll dodge, world, and travel, but its
   shared sink gate does not include focus, suspension, or menu state.
3. Source and test comments still describe missing menu evidence as relaxing the
   gate, while current routing treats unavailable evidence as gated.

S059 may state established user intent and current observable behavior, label a
platform limitation, or link a follow-up issue. It must not claim that a deferred
defect was fixed.

## Decision 3: Add one cross-feature authorization model

**Decision**: Publish one action-authorization truth table that compares physical
skill interception, application toggles, queued weaving, autonomous fishing, and
Auto Potion across the relevant safety axes.

**Rationale**: These paths intentionally differ. For example, Roll Dodge gates
weaving but not Auto Potion, explicit Sprinting gates Auto Potion but not
weaving, and menu state defers fishing timers while preserving their deadlines.
Repeating each fact only on feature pages makes the differences hard to audit.

The table is explanatory. Feature pages remain the authority for their detailed
state and recovery rules.

## Decision 4: Complete pages by audience contract

**Decision**: Apply different completeness contracts to task pages, feature
pages, concept pages, reference pages, and development pages.

**Rationale**: A setup page should lead to an action. A protocol page should
preserve exact encodings. Requiring identical headings everywhere would create
empty boilerplate and hide the information readers need.

Feature pages must cover prerequisites, configuration, normal behavior, state
meanings, failure, recovery, and related concepts. Concepts explain invariants
and comparisons. Reference pages carry exact values and schemas. Development
pages explain ownership, tests, packaging, and stable release guarantees.

## Decision 5: Separate guarantees, implementation, diagnosis, and game-sensitive facts

**Decision**: Use consistent prose labels or callouts for:

- User guarantee
- Implementation detail
- Diagnostic advice
- Version-sensitive ESO behavior

**Rationale**: Readers must know whether a statement is a product promise, a
current mechanism, a troubleshooting step, or an ESO API observation that may
change independently. The labels also reduce the chance that later refactoring
turns incidental implementation into a public contract.

## Decision 6: Add diagrams only for relationship-heavy logic

**Decision**: Use compact Markdown tables and text sequences for the following:

1. The cross-feature action-authorization truth table.
2. Fishing states and recovery transitions.
3. PixelBus negotiation, validation, invalidation, and routing sequence.
4. Configuration load, validation, dirty-settle, and close-flush lifecycle.
5. Release preparation, verification, platform builds, checksums, and publish.
6. A troubleshooting decision tree beginning with runtime and focus, then
   layout and heartbeat, then feature-specific evidence.

**Rationale**: Each candidate has three or more dependent branches or consumers.
Simple settings and single-step procedures remain prose or tables.

No Mermaid runtime or syntax is introduced. Every visual structure requires
meaningful surrounding text and must remain understandable without color.

## Decision 7: Preserve one authority for maintainer release procedure

**Decision**: Publish stable packaging and release guarantees in the developer
track, but keep `docs/project/releasing.md` as the sole command-by-command
maintainer procedure.

**Rationale**: Issue #81 requires packaging and release logic in the published
developer path. Copying the exact release ritual would create a second authority.
The public page can explain the state sequence, verification gates, assets, and
failure boundaries while linking maintainers to the project record.

## Decision 8: Make configuration and logging failure behavior explicit

**Decision**: Expand the reference track with exact store ownership, migration
behavior, corruption handling, coalesced writes, geometry restoration, logging
filters, ring retention, platform paths, and file-sink failure behavior.

**Rationale**: These are shipped logical processes and common diagnosis paths.
The current pages name the files and basic sinks but omit distinctions such as
`config.json` corruption preservation versus `state.json` fallback, best-effort
loading of newer settings schemas, and the in-memory log surviving a file-write
failure.

## Decision 9: Treat search vocabulary as a contract

**Decision**: Maintain a checked map from canonical product terms to likely
player and developer search terms. Every alias must appear on, or point directly
to, its canonical page.

**Rationale**: Local search is present, but it cannot find concepts whose common
names never occur in the corpus. ESO terminology and product terminology differ
often enough to require deliberate synonyms.

## Decision 10: Validate structure and substance independently

**Decision**: Extend documentation policy tests with fixture-backed checks for
coverage rows, page completeness metadata, terminology targets, diagram text
alternatives, exact navigation, UTF-8, LF, prohibited punctuation, and mojibake.
Run mdBook, link validation, and repository policy checks as separate gates.

**Rationale**: A successful book build proves syntax and links, not behavioral
coverage. A coverage manifest proves accounting, but not that prose is readable.
Both forms of validation are required by issue #81.

## Audited Coverage Summary

| Logic area | Current state before S059 | Primary destination |
| --- | --- | --- |
| Physical interception and handoff | Partial | Input Safety |
| Weave authorization and scheduler | Partial | Weaving and Action Authorization |
| Game context and lifecycle | Mostly covered | Game Observation and Safety State |
| Fishing state and recovery | Partial | Fishing |
| Auto Potion decision flow | Mostly covered, ordering correction needed | Auto Potion |
| PixelBus protocol and freshness | Covered, diagram needed | Pixel Bus Protocol |
| PixelBeacon lifecycle | Mostly covered, failure table needed | PixelBeacon |
| Configuration and session state | Partial | Configuration and Session State |
| Logging and privacy | Partial | Logging |
| Platform differences | Partial | Scope and Platform Model |
| Packaging and release | User guarantees only | Release and Packaging |
| Startup failure surfacing | Missing | Troubleshooting or Startup Failures |
| Test strategy | Missing | Test Strategy |
| Common diagnosis | Missing as a task path | Troubleshooting |

## Evidence Baseline

The initial audit used these implementation and test areas:

- `src/input`, `src/weave`, `src/fishing`, `src/potion`, and `src/app/routing.rs`
- `src/game`, `src/pixelbus`, `src/beacon`, and the embedded addon
- `src/config`, `src/logging`, `src/startup`, and `src/platform`
- `packaging`, `.github/workflows/release.yml`, `release.toml`, and release scripts
- Input, weave, fishing, potion, game-state, pixel-bus, beacon, configuration,
  logging, application-model, UI-sizing, and release-script tests

The exact row-level evidence contract is in
`contracts/coverage-manifest.md`.
