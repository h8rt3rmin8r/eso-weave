# Feature Specification: Documentation Completeness

**Slice**: S059
**Issue**: #81
**Status**: Implemented
**Created**: 2026-09-07

## User Scenarios

### User Story 1: Complete a task without reading source code (Priority: P1)

As a player, I can install, start, configure, operate, diagnose, update, and remove ESO Weave from task-oriented documentation that explains every visible control and status in plain language.

**Independent test**: Prove every shipped user surface in the coverage manifest has a canonical destination, source evidence, and recovery guidance.

### User Story 2: Understand automation and safety logic (Priority: P1)

As a developer or reviewer, I can trace each state machine, authorization gate, protocol boundary, persistence rule, test layer, and delivery mechanism from a stable conceptual page to named source symbols and tests.

**Independent test**: Validate every developer topic against the logic-to-page matrix and confirm safety-sensitive entries distinguish physical input, synthesized input, unavailable evidence, and non-authorization.

### User Story 3: Find concepts using player language (Priority: P2)

As a reader, I can search with common ESO and interface terms and reach the canonical page without knowing internal module names.

**Independent test**: Run terminology fixtures and generated search checks for every required alias, then verify aliases are visible prose rather than hidden metadata.

### User Story 4: Keep completeness enforceable (Priority: P2)

As a maintainer, I can add or change a product surface without silently leaving documentation incomplete because a machine-readable manifest and policy suite check ownership, evidence, page dimensions, aliases, and accessibility.

**Independent test**: Mutate one contract dimension at a time and prove policy rejects omissions, duplicates, false completion, missing platform or safety detail, and inaccessible visuals.

## Edge Cases

- A setting exists in persisted configuration but is not exposed in the UI.
- Current source behavior contradicts existing prose or an intended safety rule.
- Windows and Linux differ in discovery, focus, paths, packaging, or input support.
- A feature requires fresh PixelBus data while another remains useful offline.
- A diagram conveys a transition that is not expressed in adjacent text.
- A stable user promise depends on ESO behavior that still needs field verification.
- A broad rewrite accidentally removes an S058 preservation excerpt.

## Functional Requirements

- **FR-001**: The corpus MUST provide distinct, connected user and developer tracks.
- **FR-002**: The user track MUST cover installation, checksum verification, first launch, configuration, operation, statuses, troubleshooting, updates, removal, privacy, network behavior, and platform differences.
- **FR-003**: The settings reference MUST account for every visible setting, keybinding, skill control, persisted layout preference, default, range or choice, effect, interaction, and application timing.
- **FR-004**: The status reference MUST account for every user-visible state with meaning, automation impact, and recovery.
- **FR-005**: Feature pages MUST provide setup, operation, decisions or states, safety limits, recovery, and related settings.
- **FR-006**: Developer pages MUST cover ownership, input, scheduling, state machines, PixelBus, persistence, logging, tests, packaging, and release flow.
- **FR-007**: Safety documentation MUST identify exact authorization boundaries and treat unavailable evidence as non-authorization wherever verified behavior does so.
- **FR-008**: A machine-readable manifest MUST map each topic to one page and anchor, audience, source symbols, tests, dimensions, platforms, safety class, aliases, stability, and evidence.
- **FR-009**: A readable matrix MUST expose ownership and evidence without becoming a competing manual.
- **FR-010**: Policy MUST reject omitted or duplicate topics, dangling destinations, missing evidence or dimensions, absent platform or safety detail, hidden aliases, and inaccessible informative visuals.
- **FR-011**: Player-language aliases MUST appear in visible prose or glossary entries and generated search.
- **FR-012**: Informative visuals MUST have useful text alternatives, and color MUST remain supplementary.
- **FR-013**: Documentation MUST describe verified current behavior even when it reveals a defect. Defects MUST be filed separately and MUST NOT be fixed in this docs-only slice.
- **FR-014**: Claims MUST cite stable source symbols and tests, and version-sensitive ESO facts MUST be labeled.
- **FR-015**: The S058 migration ledger and preservation manifest MUST stay green.
- **FR-016**: Published Markdown MUST remain uniquely navigable, searchable, link-valid, offline-capable, UTF-8 without BOM, LF-only, and free of forbidden dash characters and mojibake.
- **FR-017**: Spelling MUST be checked by a pinned tool with a narrow project dictionary.
- **FR-018**: Documentation CI MUST run the completeness and prose gates with least privilege and exact pins.
- **FR-019**: Plan 028 MUST be archived with PR #88 and issue #80 evidence, and plan 029 MUST become the sole Active plan.
- **FR-020**: Workflow and pinned-tool changes MUST receive a dated changelog decision.
- **FR-021**: S059 MUST NOT change runtime behavior, embed documentation, release a package, or claim live-game verification.

## Key Entities

- **Coverage topic**: One bounded user surface or developer logic contract with a stable identifier and one canonical destination.
- **Canonical destination**: Published page and fragment that owns a topic.
- **Evidence reference**: Named source path and symbol or test name supporting a claim.
- **Required dimension**: Needed content such as setup, default, decision flow, recovery, platform behavior, or safety consequence.
- **Search alias**: Visible player or contributor terminology leading to a topic.
- **Safety classification**: Whether missing or stale evidence must prevent input.

## Success Criteria

- **SC-001**: Every audited user surface and developer logic area has exactly one Complete manifest row with an existing destination and evidence.
- **SC-002**: All 25 modal settings, 10 keybindings, seven skill rows, and persisted non-modal preferences are covered with defaults and effect timing.
- **SC-003**: Every status has meaning, automation impact, and recovery or an explicit no-action explanation.
- **SC-004**: Negative fixtures fail for every FR-010 contract class and pass after the matching policy exists.
- **SC-005**: Navigation, aliases, visual alternatives, links, search, offline resources, and spelling pass automated validation.
- **SC-006**: Existing preservation, generated-site, workflow, and full Cargo CI parity remain green with zero runtime source changes.

## Assumptions and Autopilot Decisions

- Use one checked JSON manifest plus one published readable matrix.
- Add focused pages and expand existing pages rather than duplicate the manual.
- Pin `typos-cli` 1.50.1 and record the workflow decision.
- Use Markdown tables and text sequences. No Mermaid runtime is introduced.
- File implementation contradictions separately and document verified behavior.
- Do not create a blog post because no runtime capability is added.

## Dependencies and Exclusions

- Depends on merged S058 and the v0.14.0 baseline.
- Closes issue #81 only.
- Leaves #82, #84, #77, and newly filed runtime defects open.
