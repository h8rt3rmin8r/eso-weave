# Feature Specification: Evidence-Scoped Encounter Recommendations

**Feature Branch**: `codex/s090-evidence-scoped-recommendations`

**Created**: 2026-09-11

**Status**: Implemented, pending external review

**Input**: Issue #136 and the completed S077 metric and S078 encounter-history
handoffs

## User Scenarios and Testing

### User Story 1 - Review cautious encounter guidance (Priority: P1)

A user selects an encounter and sees a small set of deterministic review prompts
derived from that encounter's versioned observed metrics. The interface keeps the
observed facts visibly separate from the prompts and never presents a correlation
as a proven cause or an optimal rotation.

**Why this priority**: This is the feature's direct user value. It turns the
existing descriptive encounter record into bounded next questions without
overstating what one local observation proves.

**Independent Test**: Project a complete synthetic encounter with sufficient
duration, casts, catalog coverage, one dominant damage share, and one low observed
effect uptime. Verify the same ordered review prompts and evidence citations are
produced repeatedly and appear beneath a distinct Recommendations heading.

**Acceptance Scenarios**:

1. **Given** a complete encounter of at least 10 seconds with at least three
   observed casts and adequate catalog coverage, **when** its detail opens, **then**
   deterministic review prompts may identify a dominant damage contribution or a
   low observed effect uptime using provisional, non-causal wording.
2. **Given** one generated prompt, **when** the user reads it, **then** the prompt
   cites the encounter identity, projection schema, calculation algorithm, catalog
   version, and catalog semantic identity that produced it.
3. **Given** the same projection bytes, **when** recommendations are rebuilt 100
   times, **then** prompt identities, order, wording, status, and citations are
   identical.
4. **Given** an encounter with no threshold-crossing observation, **when** detail
   opens, **then** the interface truthfully reports that no review prompt was
   produced rather than inventing generic advice.

---

### User Story 2 - Understand qualified or suppressed guidance (Priority: P1)

A user can tell when capture loss, unresolved IDs, or a small sample makes advice
unreliable. The interface explains the exact evidence gate and either qualifies
the remaining prompts or suppresses them completely.

**Why this priority**: Advice without visible evidence limits would undermine the
quality model delivered by S077 and S078.

**Independent Test**: Evaluate fixtures at every duration, cast-count, loss-ratio,
and unknown-ID-ratio boundary. Verify exact ready, qualified, and suppressed
states, stable reason codes, and no advice in a suppressed report.

**Acceptance Scenarios**:

1. **Given** an encounter shorter than 10 seconds or with fewer than three
   observed casts, **when** recommendations are evaluated, **then** all advice is
   suppressed with an explicit insufficient-sample reason.
2. **Given** declared loss below 10 percent of the encounter sequence span,
   **when** recommendations are evaluated, **then** eligible prompts remain visible
   but are labeled qualified and cite the exact missing ranges.
3. **Given** declared loss covering at least 10 percent of the encounter sequence
   span, **when** recommendations are evaluated, **then** all advice is suppressed
   as materially incomplete.
4. **Given** any unknown positive numeric references, **when** recommendations are
   evaluated, **then** otherwise eligible known-ID prompts are qualified and
   unknown-ID targets are omitted.
5. **Given** unknown abilities account for at least 25 percent of observed damage,
   **when** recommendations are evaluated, **then** damage-concentration advice is
   suppressed while unrelated known-effect advice may remain qualified.
6. **Given** any suppressed state, **when** detail renders, **then** observed
   metrics remain visible and unchanged above the recommendation explanation.

---

### User Story 3 - Keep recommendations local and display-only (Priority: P2)

A user can inspect recommendations knowing they are an in-memory interpretation
of one selected encounter, not persisted truth and never an input to automation.

**Why this priority**: Recommendation generation must preserve the project's local
privacy boundary and cannot acquire gameplay authority.

**Independent Test**: Generate and render recommendations through the encounter
history detail path. Verify no recommendation file or database row is created, no
network or telemetry dependency exists, and input, weaving, fishing, potion,
Pixel Bus, focus, and addon services are not referenced by the recommendation
module.

**Acceptance Scenarios**:

1. **Given** a selected encounter, **when** recommendations are generated, **then**
   they exist only with the disposable in-memory detail and do not modify raw
   encounter storage, catalogs, settings, or derived files.
2. **Given** a recommendation report, **when** application action authorization is
   evaluated, **then** the report cannot enqueue, synthesize, or authorize input.
3. **Given** a catalog replacement, **when** the selected encounter is rebuilt,
   **then** recommendations are recalculated from the new projection and cite the
   new compatible catalog without changing raw observations.

### Edge Cases

- Encounter duration is zero, exactly 9,999 ms, or exactly 10,000 ms.
- Cast count is zero, two, or exactly three.
- First and last sequence are equal, or declared ranges touch the sequence edges.
- Loss ranges are large enough to overflow naive inclusive-count arithmetic.
- Known plus unknown reference count is zero.
- Unknown ability damage share is immediately below or exactly at 25 percent.
- A dominant share or effect uptime value is unavailable, non-finite, or outside
  the valid zero-through-one ratio after projection validation.
- Several abilities have equal shares or several effects have equal uptimes.
- The only threshold-crossing target is unknown to the catalog.
- A degraded report has no otherwise eligible prompt.
- A projection schema, metric algorithm, source range, or quality value is
  inconsistent with its declared loss metadata.
- The active catalog changes while an earlier detail result is pending.
- The encounter window is narrow or contains a long list of evidence reasons.

## Requirements

### Functional Requirements

- **FR-001**: Recommendations MUST consume one immutable S077
  `EncounterProjection`, MUST accept only projection schema 1 with the `s069-v1`
  metric algorithm, and MUST NOT read raw capture text independently.
- **FR-002**: Recommendation generation MUST be deterministic, local, synchronous,
  side-effect free, and independent of network, telemetry, model, or service calls.
- **FR-003**: The recommendation ruleset MUST have the stable version
  `s090-v1`, distinct from the projection schema and calculation algorithm.
- **FR-004**: A report MUST preserve the encounter session ID, encounter ID,
  channel, API version, raw-content SHA-256, projection schema version,
  calculation algorithm version, catalog schema version, catalog version, and
  catalog semantic SHA-256.
- **FR-005**: Every advice item MUST carry the complete report citation and a
  stable rule identifier.
- **FR-006**: Reports MUST distinguish `ready`, `qualified`, and `suppressed`
  availability with stable machine-testable reason codes and plain-language
  explanations.
- **FR-007**: Reports MUST keep deterministic evidence facts structurally and
  visually separate from advice items.
- **FR-008**: Advice wording MUST use review-prompt language and MUST NOT claim
  causation, optimality, guaranteed performance improvement, live Combat Metrics
  parity, or universal build correctness.
- **FR-009**: Duration below 10,000 ms MUST suppress all advice as an insufficient
  duration sample; exactly 10,000 ms satisfies the duration gate.
- **FR-010**: Fewer than three observed casts MUST suppress all advice as an
  insufficient cast sample; exactly three casts satisfies the cast gate.
- **FR-011**: Declared missing sequence count MUST be evaluated against the
  inclusive encounter sequence span using overflow-safe arithmetic, and any
  reversed declared loss range MUST fail closed to suppression.
- **FR-012**: Any declared loss below 10 percent of the sequence span MUST qualify
  eligible advice and preserve every exact sorted loss range and reason.
- **FR-013**: Declared loss at or above 10 percent of the sequence span MUST
  suppress all advice as material capture loss.
- **FR-014**: Known and unknown positive IDs MUST remain sorted and deduplicated in
  the evidence facts, and unknown ability damage share MUST be calculated by
  summing valid damage-share values whose ability IDs are unknown.
- **FR-015**: Any unknown positive ID MUST qualify otherwise eligible known-ID
  advice, and a prompt whose target is unknown MUST be omitted.
- **FR-016**: Unknown ability damage share at or above 25 percent MUST suppress
  damage-concentration advice as materially uncertain, but MUST NOT suppress an
  unrelated known-effect prompt.
- **FR-017**: No joined IDs or no valid unknown ability damage shares MUST produce
  zero unknown damage share without division by zero or invented uncertainty.
- **FR-018**: A dominant-damage review prompt MUST be eligible only for a known
  ability whose finite observed damage share is at least 40 percent.
- **FR-019**: A low-uptime review prompt MUST be eligible only for a known effect
  whose finite observed uptime is at most 50 percent.
- **FR-020**: Equal eligible candidates MUST use ascending positive numeric ID as
  the deterministic tie-breaker; report items MUST use stable rule order.
- **FR-021**: Unavailable, non-finite, negative, or greater-than-one candidate
  ratios MUST NOT produce advice. Recommendation-bearing metric values MUST carry
  the expected algorithm, source range, and internally consistent quality plus
  loss metadata; any mismatch MUST suppress advice.
- **FR-022**: A suppressed report MUST contain no advice items but MUST retain its
  evidence facts, provenance, and all suppression reasons.
- **FR-023**: A qualified report MUST mark every retained advice item qualified
  and make the applicable qualification reasons visible adjacent to the advice.
- **FR-024**: Recommendations MUST be calculated only as part of the selected
  encounter's disposable detail projection and MUST NOT add persistence, caching,
  automatic refresh, upload, or retention behavior.
- **FR-025**: Recommendation code MUST have no dependency on input, weave,
  fishing, potion, game-focus, Pixel Bus, addon lifecycle, or action-authorization
  modules.
- **FR-026**: The encounter-history UI MUST render Observed Metrics before a
  distinct Recommendations section and MUST leave existing metrics visible for
  ready, qualified, suppressed, and empty recommendation outcomes.
- **FR-027**: Pure recommendation and presentation helpers MUST be testable without
  a window manager; the critical ready, qualified, and suppressed journeys MUST
  have headless rendered coverage at wide and narrow widths.
- **FR-028**: Canonical encounter documentation MUST define the provisional
  thresholds, provenance, wording limits, non-persistence, and display-only
  isolation.
- **FR-029**: S090 MUST archive completed Plan 039 with S089 and epic #119
  completion evidence and establish the game-data recommendation plan as the sole
  active build plan.
- **FR-030**: Verification issues #110, #129, and #131 MUST remain independent
  Release verification work and MUST NOT block S090 implementation or closure.

### Key Entities

- **RecommendationReport**: One deterministic in-memory result containing the
  ruleset version, availability, evidence facts, reasons, advice items, and complete
  projection/catalog provenance.
- **RecommendationEvidence**: The immutable sample, loss, coverage, metric, and
  provenance facts used by the rules without advice wording.
- **RecommendationReason**: A stable ordered qualification or suppression code,
  exact supporting values, and a safe user-facing explanation.
- **AdviceItem**: One versioned review prompt with a stable rule ID, target kind and
  numeric ID, observed value, status, bounded wording, and complete evidence
  citation.

## Success Criteria

### Measurable Outcomes

- **SC-001**: One complete qualifying fixture produces byte-for-byte equivalent
  logical reports across 100 evaluations, with stable ordering and citations.
- **SC-002**: Automated boundary tests cover 9,999 and 10,000 ms, two and three
  casts, loss immediately below and exactly at 10 percent, and unknown ability
  damage share immediately below and exactly at 25 percent.
- **SC-003**: Every suppressed fixture produces zero advice items while retaining
  all applicable reasons and unchanged observed metrics.
- **SC-004**: Every rendered advice item exposes ruleset, encounter, calculation,
  and catalog identity, and every qualified item exposes its reasons.
- **SC-005**: Static dependency inspection and integration tests find no path from
  recommendation output to synthesized input, automation controllers, network,
  telemetry, raw-store mutation, or catalog mutation.
- **SC-006**: Full Rust CI parity and repository documentation, encoding,
  whitespace, forbidden-dash, JSON, spelling, and link gates pass.

## Clarifications

### Session 2026-09-11

- Q: Should recommendations use a hosted AI or external model? A: No. Use a pure,
  versioned local ruleset so outputs remain private, reproducible, reviewable, and
  independent of network availability.
- Q: Are the provisional thresholds claims about optimal ESO play? A: No. They are
  conservative evidence gates and review-prompt triggers for `s090-v1`, not
  gameplay targets. Issue #131 may later justify a separately versioned revision.
- Q: Does any declared loss suppress every prompt? A: No. Sub-material loss
  qualifies advice and remains fully visible; material loss at 10 percent or more
  suppresses advice.
- Q: Can an unknown target produce an ID-only recommendation? A: No. Known-ID
  advice may survive catalog uncertainty, but an unknown target is omitted because
  its semantics cannot be safely interpreted. Material unknown damage share blocks
  only the damage-concentration rule, not unrelated known-effect advice.
- Q: Should recommendations be stored with encounter history? A: No. They are
  rebuilt in memory with the selected S077 projection so provenance never becomes
  stale or confused with raw evidence.

## Assumptions and Dependencies

- S076 raw import and persistence, S077 projection, and S078 quality-aware history
  UI are complete.
- Issue #136 is the actionable implementation authority.
- Issue #131 may refine future thresholds or wording but is not a prerequisite.
- The current projection supplies numeric IDs and complete catalog identity but not
  localized catalog names. S090 therefore uses explicit Ability and Effect numeric
  identifiers and does not invent names.
- No installation, release, live ESO session, or user-provided capture is required
  to complete this slice.

## Out of Scope

- Build, skill, gear, champion-point, or rotation prescriptions that require facts
  absent from the S077 projection.
- Claims of causation, Combat Metrics parity, or performance improvement.
- Machine learning, large-language-model, hosted inference, uploads, telemetry, or
  web lookup.
- Recommendation persistence, history search, comparison across encounters, trend
  analysis, or automatic pruning.
- Any input synthesis, gameplay automation, addon behavior, catalog mutation, or
  raw encounter mutation.
