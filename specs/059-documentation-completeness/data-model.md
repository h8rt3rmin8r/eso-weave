# Data Model: Documentation Completeness

## Documentation Obligation

One independently reviewable fact or reader outcome required by issue #81.

| Field | Meaning |
| --- | --- |
| `id` | Stable identifier scoped by `USR`, `LOG`, `CFG`, `PLT`, `REL`, or `DIA` |
| `area` | Feature, state machine, safety gate, protocol, configuration, logging, platform, packaging, release, or failure mode |
| `audience` | User, curious reader, contributor, maintainer, or more than one |
| `statement` | The behavior or knowledge that must be documented |
| `destination` | One canonical published Markdown page |
| `source_evidence` | Current implementation, addon, packaging, or workflow paths with anchors |
| `test_evidence` | Tests that demonstrate the contract, or an explicit test-gap note |
| `coverage` | Missing, Partial, Covered, or Deferred |
| `labels` | Guarantee, Implementation, Diagnostic, or VersionSensitive |

Rules:

- Every identifier is unique and stable after publication.
- Every row has exactly one canonical destination.
- Covered rows cite existing prose and at least one current evidence source.
- A safety-sensitive row requires test evidence or an explicit deferred test gap.
- Deferred means the documentation slice found an implementation discrepancy.
  It does not mean that required documentation may be silently omitted.

## Published Page Profile

| Field | Meaning |
| --- | --- |
| `path` | Exact repository-relative path below `docs/src` |
| `title` | One non-empty level-one heading |
| `page_type` | Landing, Task, Feature, Concept, Reference, or Development |
| `audiences` | Intended reader groups |
| `obligations` | Coverage identifiers assigned to the page |
| `related_pages` | Direct links that continue the reader path |
| `search_terms` | Canonical and alias terms discoverable from this page |
| `visuals` | Diagram or screenshot identifiers, if any |

Page profiles do not need to exist as front matter. The coverage and terminology
contracts may derive them from machine-readable manifests or checked Markdown.

## Evidence Reference

| Field | Meaning |
| --- | --- |
| `path` | Exact repository-relative source, test, addon, workflow, or packaging path |
| `anchor` | Stable symbol, test name, heading, or bounded line-independent phrase |
| `kind` | Source, Test, Protocol, Packaging, Workflow, or Documentation |
| `claim` | The narrow fact supported by the reference |

Line numbers may be included for review notes but are not stable manifest keys.
Symbols, test names, headings, and exact phrases are preferred for durable checks.

## Action Path

An action path is one route that can suppress physical input or synthesize input.

| Field | Meaning |
| --- | --- |
| `name` | Physical Skill, App Toggle, Queued Weave, Fishing, or Auto Potion |
| `trigger` | Physical event, queued action, detector event, clock deadline, or pixel-bus tick |
| `authorities` | Observations and operator state that can permit or prevent work |
| `unknown_policy` | Pass physical, block synthesis, ignore as non-authorizing, or not applicable |
| `recovery` | Fresh press, fresh observation, retained request, new toggle, or no replay |
| `output` | Pass, suppress and enqueue, generated sequence, interact press, or quickslot press |

The action-authorization table is a projection of these entities. A cell must not
collapse an unavailable observation into an observed safe value.

## Feature State Machine

| Field | Meaning |
| --- | --- |
| `state` | User-visible or correctness-bearing controller state |
| `entry` | Event and preconditions that enter the state |
| `exit` | Event, deadline, or gate that leaves the state |
| `side_effect` | Input emitted, deadline armed, request changed, or no output |
| `recovery` | Automatic, fresh evidence, manual action, or disabled |

Fishing and Auto Potion each require a state model. Weaving requires an action
decision flow rather than a persistent feature-state diagram.

## Failure Mode

| Field | Meaning |
| --- | --- |
| `symptom` | What the reader can observe |
| `boundary` | Process, focus, addon, capture, decoder, controller, input backend, file store, log sink, package, or release gate |
| `safe_effect` | Pass physical input, stop synthesis, clear telemetry, retain request, clear request, use defaults, or fail the operation |
| `diagnosis` | Ordered checks a reader can perform without source access |
| `recovery` | Exact action or fresh evidence needed |
| `platform` | All, Windows, Linux, X11, XWayland, package-specific, or workflow-specific |

Failure modes appear in both the responsible feature page and the task-oriented
troubleshooting path without duplicating the canonical explanation. The
troubleshooting page links to the canonical contract for detail.

## Configuration Store

| Store | Owns | Does not own | Invalid input behavior | Write lifecycle |
| --- | --- | --- | --- | --- |
| `config.json` | User settings and module-owned sections | Session, runtime, and derived state | Preserve corrupt file with an `.invalid` suffix, load defaults, surface notice | Dirty-settle write and forced close flush |
| `state.json` | Suspend and fishing intent, API-version cache, and window geometry | Auto Potion request and user settings | Load safe defaults and surface notice; no `.invalid` preservation guarantee | Dirty-settle write and forced close flush |

Module validation may replace one invalid field with its safe default while
retaining other valid fields. A newer settings schema loads best effort with a
notice; it is not a forward-migration guarantee.

## Logging Pipeline

`event -> global level and input-suppression filter -> ring -> optional file sink`

| Component | Contract |
| --- | --- |
| Global level | Applies immediately to subsequent events and filters both sinks |
| Input suppression | Drops input-target contents while suspended |
| Ring | Retains the newest bounded set and evicts the oldest |
| Live Log filter | Filters presentation without changing the persisted global level |
| File sink | Creates the directory lazily, appends by UTC month, and may fail while the ring remains usable |

## Release State

`prepared -> tagged -> verified -> built -> checksummed -> published`

- Human authorization governs preparation and tag creation.
- Verification requires version agreement, a non-empty changelog section, and a
  valid concise Highlights excerpt.
- Windows and Linux assets build independently after verification.
- Publication consumes the checked artifacts and combined checksums.
- Failure at any required transition prevents publication.

The published developer page describes this stable state model. The exact
maintainer commands remain authoritative only in `docs/project/releasing.md`.

## Search Term Mapping

| Field | Meaning |
| --- | --- |
| `canonical` | Product term used in headings and interface text |
| `aliases` | Likely player, platform, or developer query terms |
| `target` | Canonical destination page |
| `context` | Natural sentence or glossary entry that makes the alias meaningful |

Aliases are aids, not alternate terminology authorities. Interface strings and
protocol names retain their canonical capitalization.

## Diagram Record

| Field | Meaning |
| --- | --- |
| `id` | Stable `DIA` identifier |
| `kind` | TruthTable, StateDiagram, Sequence, Lifecycle, or DecisionTree |
| `destination` | Page containing the visual |
| `relationships` | Coverage identifiers clarified by the visual |
| `text_equivalent` | Nearby prose or table that communicates the same result |

Every visual remains understandable in monochrome, has meaningful text context,
and does not carry the only copy of a safety requirement.

