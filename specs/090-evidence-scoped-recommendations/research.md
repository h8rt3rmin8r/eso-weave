# Research: Evidence-Scoped Encounter Recommendations

## Decision 1: Use a local deterministic ruleset

**Decision**: Generate advice through a pure repository-owned ruleset identified as
`s090-v1`.

**Rationale**: The project already has versioned deterministic facts, has no
telemetry, and must remain usable offline. A hosted AI or model would introduce
privacy, availability, reproducibility, and provenance problems without supplying
missing encounter context.

**Alternatives rejected**:

- Hosted or local language model: nondeterministic, adds dependencies, and cannot
  invent trustworthy build context.
- Static generic tips: not scoped to the selected evidence and add little value.
- No recommendations until live verification: conflicts with the user's standing
  nonblocking verification policy and issue #136 readiness.

## Decision 2: Limit advice to defensible review prompts

**Decision**: S090 may identify one dominant known-ability damage share and one low
known-effect uptime, using neutral prompts that ask the user to compare the
observation with their intended encounter context.

**Rationale**: The projection contains shares, uptime, casts, quality, and
provenance. It does not contain a role, target encounter, build goal, normative
baseline, or causal experiment. Review prompts use the available facts without
claiming a correct build or rotation.

**Alternatives rejected**:

- DPS/HPS targets: no role or encounter baseline exists.
- Ability replacement advice: catalog relations and build snapshots are absent.
- Rotation scoring: the projection has ordered casts but no validated optimal
  reference or complete timing model.

## Decision 3: Apply global and rule-local evidence gates

**Decision**: Duration below 10 seconds, fewer than three casts, and material loss
at or above 10 percent suppress the whole report. Any loss below that threshold or
any unknown ID qualifies advice. Unknown targets are omitted, and unknown abilities
owning at least 25 percent of observed damage suppress only damage-concentration
advice.

**Rationale**: Sample and material loss affect the complete observation. Unknown
identity is more local: an unrelated unknown effect should not erase a valid known
ability fact. The rule-local policy is safer and less restrictive than global ID
coverage suppression.

**Alternatives rejected**:

- Any loss suppresses all advice: discards useful bounded observations even when
  exact limited loss is visible.
- Global unknown-count suppression: unrelated IDs can block valid advice.
- Ignore unknown targets: could assign semantic meaning the catalog does not know.

## Decision 4: Keep projection and advice contracts separate

**Decision**: Add a top-level recommendation domain and leave
`EncounterProjection` unchanged.

**Rationale**: S077 facts are already a canonical rebuildable artifact. Embedding
advice would change its meaning, blur the facts boundary, and unnecessarily affect
the maintainer projection command.

**Alternatives rejected**:

- Add recommendations to `EncounterProjection`: couples version lifecycles and
  changes existing canonical JSON.
- Put policy in egui: makes correctness frame-driven and hard to test.
- Put policy in encounter storage service: makes storage own a downstream product.

## Decision 5: Carry citations per advice item

**Decision**: Copy encounter, raw, projection, calculation, catalog, channel, API,
and recommendation-policy identities into every item.

**Rationale**: The acceptance criterion says every recommendation cites its
versions. Per-item citations remain correct when items are logged in tests,
reordered in future UI work, or viewed independently.

**Alternatives rejected**:

- One implicit page-level citation: easier to separate accidentally from an item.
- Reopen catalog for names: can mix versions after atomic catalog replacement.
- Wall-clock generation timestamp: harms determinism and provides no evidence.

## Decision 6: Keep output bounded and non-actionable

**Decision**: Produce at most two advice items in fixed rule order and expose no UI
button, action, sink, callback, or automation mapping.

**Rationale**: A bounded report is readable, predictable, and cannot amplify a
large capture into unbounded UI work. Lack of an action surface makes the
display-only boundary concrete.

**Alternatives rejected**:

- One item per ability/effect: noisy and potentially large.
- Apply-to-build or execute controls: the project lacks the facts and authority.
- Persisted advice history: adds invalidation and privacy surface without a need.
