# Contract: Page Completeness

## General Contract

Every published page must:

- have exactly one non-empty level-one heading;
- state its audience or make that audience clear from its opening paragraph;
- use canonical interface and protocol names with exact capitalization;
- link directly to prerequisite or deeper material rather than duplicating it;
- distinguish Unknown or unavailable evidence from an observed safe value;
- keep user guarantees separate from implementation detail and diagnostic advice;
- identify platform-specific and version-sensitive behavior where applicable;
- remain understandable without screenshots, color, or diagrams; and
- be listed exactly once in `docs/src/SUMMARY.md`, except `404.md`.

## Landing Pages

The book landing page must expose two visible paths:

1. A task-oriented path for installation, first launch, feature use, and
   troubleshooting.
2. A curious/developer path for state machines, safety, protocol, architecture,
   configuration, tests, packaging, and release guarantees.

Section landing pages summarize their children and do not become competing
feature explanations.

## Task Pages

Installation, responsible-use, and troubleshooting pages must answer:

- What is the reader trying to accomplish?
- Which platform, package, permission, or account boundary applies?
- What observable result confirms success?
- What are the most likely failures in diagnostic order?
- What safe recovery action follows each failure?
- Which feature or reference page owns the deeper explanation?

Installation must cover Windows MSI and Linux `.deb`, AppImage, and tarball use,
first launch, update, uninstall, input permission, package guarantees, and
checksum verification without implying that PixelBeacon is a separate release
asset.

Troubleshooting must begin with shared prerequisites before branching by feature:

1. ESO process and focus
2. Platform input access
3. PixelBeacon installation and current version
4. Overlay geometry and visibility
5. Header and heartbeat availability
6. Required per-feature observations
7. Configuration and logging evidence

## Feature Pages

Each feature page must include substantive coverage of:

1. Outcome and prerequisites
2. User configuration and defaults
3. Normal behavior or sequence
4. User-visible states and unavailable states
5. Safety and authorization boundaries
6. Failure symptoms and likely causes
7. Recovery, including whether requests are retained or cleared
8. Platform or ESO-version qualifications
9. Links to the relevant concept and reference pages

The headings may vary naturally. Mechanical empty sections are prohibited.

### Weaving

Must distinguish physical interception, queue handoff, worker authorization,
global cooldown, mid-sequence cancellation, held-output release, active weapon
bar timing, unknown bar fallback, and observable-only telemetry.

### Fishing

Must explain all controller states and every normal transition, menu deadline
deferral, arm timeout, defensive bite handling, FishingStopped recovery, life,
world, travel, focus, runtime, and SignalLost exits, and the different request
retention and restart policies.

### Auto Potion

Must list the actual first-blocker evaluation order, OR resource rule, inclusive
threshold, explicit potion and cooldown requirements, retry floor, effective
states, request persistence, and why Unknown cannot authorize required evidence.
It must state that Roll Dodge is not an Auto Potion prerequisite.

### PixelBeacon

Must cover discovery, managed installation, status, update, block-size redeploy,
uninstall, reload requirements, API upkeep, overlay constraints, compatibility,
diagnostics, and bounded failure outcomes.

### Interface and Ultimate

Must cover truthful unavailable presentation, non-color cues, stable geometry,
active-bar cost selection, readiness, threshold markers, and display-only scope.

## Concept Pages

Concept pages explain relationships and invariants. They must not repeat complete
setup instructions.

The action-authorization concept must compare all input-producing paths in one
truth table. Each cell must name one of:

- Required positive evidence
- Blocks
- Passes physical input
- Retains request without output
- Clears request
- Not applicable

The game-observation concept must explain state entry, invalidation, watchdog,
recovery, and Unknown semantics for runtime, focus, Game Context, life, world,
travel, roll dodge, and sprint.

## Reference Pages

Reference pages carry exact values, bounds, formats, schemas, encodings, and
failure semantics. They must prefer tables over scattered prose when fields
repeat.

Configuration must distinguish store ownership, schema behavior, module
validation, write scheduling, close flush, geometry restoration, and the
different corruption behavior of `config.json` and `state.json`.

Logging must distinguish global capture level from the Live Log display filter,
state ring capacity and eviction direction, identify platform paths and monthly
files, and explain the observable result of a file-sink failure.

The Pixel Bus Protocol remains the byte-level authority. Feature pages link to
it rather than copying encodings.

## Development Pages

Architecture must identify thread and ownership boundaries and link to detailed
state-machine pages.

Test Strategy must map each correctness boundary to its test seam and suite,
including pure logic, deterministic clocks, platform traits, embedded addon
contract parsing, headless interface geometry, shell contracts, and Windows and
Linux CI.

Release and Packaging must describe the stable state sequence and guarantees,
not duplicate the human command ritual. It must link maintainers to
`docs/project/releasing.md` as the sole procedural authority.

## Labels

A page must use a clear label when a reader could mistake the status of a claim:

| Label | Use |
| --- | --- |
| User guarantee | Stable externally observable promise |
| Implementation detail | Current mechanism that may change without changing the promise |
| Diagnostic advice | Ordered observation or recovery step |
| Version-sensitive | ESO API or client behavior that requires future verification |

Labels can be Markdown headings, bold lead-ins, or consistent admonitions. The
validator checks stable phrases, not presentation markup.

## Visual Contract

Every Mermaid diagram or screenshot must have:

- a meaningful title or nearby introduction;
- a prose or table equivalent that communicates the same safety result;
- labels that remain distinct without color;
- no external runtime dependency; and
- local, descriptive alternative text for images.

## Review Checklist

A page is complete only when a reviewer can answer all of the following without
opening source code:

- What permits the feature or process to act?
- What does Unknown mean here?
- What happens when evidence disappears?
- Is queued or requested work retained, discarded, or replayed?
- What can the user observe and do next?
- Does behavior differ on Windows, X11, XWayland, or by package?
- Which statement is a guarantee and which is implementation detail?

