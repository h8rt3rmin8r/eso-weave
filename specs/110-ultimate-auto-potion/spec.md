# Feature Specification: Ultimate Auto Potion Resource Watch

**Feature Branch**: `codex/s110-ultimate-auto-potion`

**Created**: 2026-09-17

**Status**: Implemented

**Input**: Work slice S110 implements GitHub issue #173 after Plan 045 completed through S109.

## Clarifications

### Session 2026-09-17

- Q: Which Ultimate observation may authorize Auto Potion? -> A: Only the existing atomic `UltimateTelemetry` observation. S110 does not add a decoder, addon field, cache, or second telemetry pipeline.
- Q: How is an integer threshold compared with an exact Ultimate ratio? -> A: Compare `current * 100 <= threshold * maximum` with widened integers. This preserves inclusive mathematical percentage semantics without floating-point or truncation errors. The named trigger diagnostic uses the ceiling integer percentage so the displayed value cannot claim an above-threshold ratio was at the threshold.
- Q: What counts as unavailable Ultimate? -> A: Unknown current, unknown maximum, or a zero maximum is unavailable and cannot qualify. A current value above maximum remains fresh but above every accepted threshold, so it cannot trigger.
- Q: How are mixed watches ordered? -> A: Preserve the established deterministic Health, Magicka, Stamina order and append Ultimate last. The rule remains OR-based, and the first qualifying watch in that order supplies the named trigger cause.
- Q: Does stale or inactive evidence need a new timestamp? -> A: No. Existing heartbeat, game lifecycle, life, world, travel, focus, suspension, and context gates already reject stale or non-active observations before resource evaluation. Signal loss also clears cached Ultimate to unknown.
- Q: How does configuration migrate? -> A: The additive `ultimate` watch defaults through serde-compatible loading to `{ enabled: false, threshold: 35 }`. Current saves always emit it. The top-level settings schema remains unchanged because the opaque potion section is explicitly additive and backward compatible.
- Q: Does the fourth row require a new settings layout? -> A: No. It joins the existing vertically stacked resource-watch loop with the same checkbox, 0 through 100 numeric control, help treatment, and keyboard accessibility.

## User Scenarios and Testing

### User Story 1 - Trigger from low Ultimate (Priority: P1)

An operator can independently watch Ultimate and have Auto Potion act when fresh Ultimate is at or below the configured percentage while every existing safety and potion gate passes.

**Why this priority**: This is the feature value and extends the action-driving rule, so its exact boundary and fail-closed behavior are the primary acceptance surface.

**Independent Test**: Enable only the Ultimate watch in a pure controller fixture, provide exact current and maximum Ultimate plus otherwise eligible inputs, and verify one Ultimate-named trigger at equality and below, with no trigger above.

**Acceptance Scenarios**:

1. **Given** Ultimate is the only enabled watch at 35 percent, **when** fresh Ultimate is 70 of 200 and every existing gate passes, **then** exactly one attempt is authorized with an Ultimate trigger cause.
2. **Given** the same configuration, **when** Ultimate is 71 of 200, **then** Auto Potion is ready but does not trigger.
3. **Given** a non-divisible ratio just above the threshold, **when** the rule evaluates it, **then** integer truncation cannot make it qualify.

### User Story 2 - Fail closed on unusable Ultimate evidence (Priority: P1)

An operator can trust missing, invalid, stale, or non-active Ultimate observations never to authorize a keypress.

**Why this priority**: Ultimate is an additional action authorization input. Its failure direction must match every existing Auto Potion observation and lifecycle gate.

**Independent Test**: With Ultimate as the only watch and every unrelated condition eligible, evaluate unknown current, unknown maximum, zero maximum, signal loss, and inactive world state and verify no attempt is emitted and the typed blocker remains truthful.

**Acceptance Scenarios**:

1. **Given** Ultimate is enabled, **when** current is unknown or maximum is unknown or zero, **then** it contributes no fresh percentage and cannot trigger.
2. **Given** a previously fresh low Ultimate observation, **when** the beacon becomes unavailable or clears the observation, **then** it cannot authorize another attempt.
3. **Given** low Ultimate, **when** world state is not active, **then** the existing world blocker wins before resource evaluation.

### User Story 3 - Configure and understand the fourth watch (Priority: P2)

An operator can enable Ultimate, choose a 0 through 100 threshold, persist the choice, and understand both the OR rule and exact ratio calculation from settings and documentation.

**Why this priority**: The trigger is useful only when the independent setting is discoverable, durable, accessible, and diagnostically explicit.

**Independent Test**: Load a legacy potion object without Ultimate, verify disabled 35-percent defaults, save and reload an enabled custom value, inspect settings labels and help, and project an Ultimate trigger into status text.

**Acceptance Scenarios**:

1. **Given** an older valid potion configuration, **when** it loads, **then** Ultimate defaults disabled at the same 35-percent threshold as the other watches and existing values remain unchanged.
2. **Given** an operator enables Ultimate at a valid threshold, **when** settings save and reload, **then** both values are preserved.
3. **Given** Ultimate caused an attempt, **when** status is displayed or diagnostics are emitted, **then** the cause is named Ultimate with its observed and configured percentages.
4. **Given** any one of four enabled watches is low, **when** the rule evaluates them, **then** OR behavior remains deterministic and no existing watch changes meaning.

### Edge Cases

- Threshold zero permits only an exact zero numerator with a positive maximum.
- Threshold 100 permits any current at or below maximum, while current above maximum remains non-qualifying.
- A non-divisible ratio is compared exactly by cross multiplication rather than a rounded or truncated intermediate percentage.
- Ultimate can be unavailable while another enabled resource is fresh and low; the fresh low resource still qualifies under OR semantics.
- Ultimate can be fresh and low while all other enabled resources are unavailable; Ultimate still qualifies.
- All four watches disabled yields the existing `NoWatchedResource` blocker.
- At least one enabled watch fresh but none low yields Ready; all enabled watches unavailable yields `ResourcesUnavailable`.
- Existing quickslot, cooldown, retry, synthesis, and safety gates continue to take their established precedence.

## Requirements

### Functional Requirements

- **FR-001**: `AutoPotionConfig` MUST contain an independent Ultimate resource watch with the same Boolean enable and inclusive 0 through 100 threshold model as Health, Magicka, and Stamina.
- **FR-002**: The Ultimate watch MUST default disabled with a threshold of 35 percent, including when a valid legacy potion configuration omits the field.
- **FR-003**: Current configuration serialization MUST emit the Ultimate watch, and loading MUST preserve valid existing watches and notices while accepting the additive field without a top-level schema bump.
- **FR-004**: Auto Potion MUST consume the existing atomic `UltimateTelemetry` observation and MUST NOT add another decoder, addon field, cache authority, or observation pipeline.
- **FR-005**: A known current and known positive maximum MUST normalize through exact inclusive comparison equivalent to `current / maximum <= threshold / 100`.
- **FR-006**: Percentage comparison MUST avoid floating-point and truncation false positives by using widened cross multiplication.
- **FR-007**: Unknown current, unknown maximum, or zero maximum MUST NOT count as a fresh Ultimate percentage and MUST NOT authorize an attempt.
- **FR-008**: Current above maximum MUST remain non-qualifying for every accepted threshold rather than being clamped into the actionable range.
- **FR-009**: Ultimate MUST participate in the existing OR rule after Health, Magicka, and Stamina, preserving deterministic first-cause order.
- **FR-010**: When Ultimate is the qualifying watch, the typed trigger cause, structured diagnostic state, and user-visible status MUST name Ultimate and report a truthful integer observed percentage plus the configured threshold.
- **FR-011**: The observed integer percentage for a qualifying Ultimate cause MUST use ceiling division so it remains consistent with exact threshold qualification.
- **FR-012**: Existing heartbeat, focus, suspension, game-context, life, world, travel, sprint, quickslot, cooldown, retry, native binding, and synthesis gates MUST retain their order and behavior.
- **FR-013**: Signal loss, inactive game lifecycle, and non-active world state MUST block before stale or cached Ultimate can authorize input.
- **FR-014**: The settings interface MUST present Watch Ultimate with the same checkbox, bounded threshold control, help, spacing, and keyboard interaction model as the other three watches.
- **FR-015**: Settings labels, help text, Auto Potion guidance, reference settings, state-machine documentation, test strategy, and coverage matrix MUST describe four watches and the direct `current / maximum` Ultimate percentage calculation where relevant.
- **FR-016**: Automated tests MUST cover equality, below and above threshold, non-divisible ratios, 0 and 100 boundaries, unknown and zero inputs, above-maximum input, configuration migration and round trip, mixed watches, deterministic cause order, diagnostics, and displayed status text.
- **FR-017**: Existing Auto Potion, input safety, Pixel Bus, application routing, settings, UI sizing, documentation, and full CI tests MUST remain passing without weakened assertions.
- **FR-018**: S110 MUST archive completed Plan 045 with PR #218 as delivery evidence and establish Plan 046 as the active build-plan authority for issue #173.
- **FR-019**: S110 MUST update the `[Unreleased]` changelog and leave issue #173 ready to close through the pull request while preserving operator merge authority.
- **FR-020**: S110 MUST NOT change potion selection, quickslot selection, cooldown or retry behavior, the Pixel Bus protocol, addon files, sampling cadence, session enablement, or another synthesized-input path.

### Key Entities

- **Ultimate resource watch**: Operator-owned enabled flag and integer threshold for Ultimate participation in Auto Potion.
- **Ultimate ratio observation**: Existing atomic current and maximum Ultimate values interpreted only when both are known and maximum is positive.
- **Trigger cause**: Deterministic first low watch, now one of Health, Magicka, Stamina, or Ultimate, with a diagnostic percentage and threshold.
- **Legacy potion configuration**: Valid persisted potion object that predates the additive Ultimate watch and therefore loads its safe default.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Ultimate-only fixtures trigger at and below every tested threshold boundary and never trigger above it, including non-divisible ratios that would be false positives under floor truncation.
- **SC-002**: Unknown current, unknown maximum, zero maximum, above-maximum current, stale signal, and non-active world fixtures produce zero Ultimate-authorized attempts.
- **SC-003**: Legacy potion JSON loads Ultimate disabled at 35 percent, and current JSON round-trips every one of the eight watch values plus retry interval without loss.
- **SC-004**: All single-low and mixed four-watch fixtures retain OR behavior and deterministic Health, Magicka, Stamina, Ultimate first-cause ordering.
- **SC-005**: Settings and status tests contain the fourth accessible watch and exact Ultimate trigger text without exceeding existing modal sizing guarantees.
- **SC-006**: Formatting, strict Clippy, the full locked test suite, documentation checks, UTF-8, LF, mojibake, and forbidden-dash checks pass.

## Assumptions

- `UltimateTelemetry` remains the canonical current and maximum observation and is cleared by existing routing when evidence is lost.
- The established 35-percent `ResourceWatch` default is the intended default for Ultimate.
- No top-level settings schema change is needed because `Settings::potion` is an additive opaque section and omitted nested watches already migrate through defaults.
- A manual settings pass can use the existing UI sizing and accessibility harness without requiring a downloadable release.

## Out of Scope

- Selecting a potion, selecting a quickslot, or inferring whether the selected potion restores Ultimate.
- New addon observations, protocol geometry, freshness timestamps, or Ultimate telemetry fields.
- Changes to requested Auto Potion persistence, action-safety gates, retry policy, or synthesis.
- Installed-game verification, release publication, or live operator evidence for issues #110, #129, #131, or #190.
