# Specification Quality Checklist: Evidence-Scoped Encounter Recommendations

**Purpose**: Validate that S090 is complete, testable, bounded, and ready for
planning before implementation begins.

**Created**: 2026-09-11

**Feature**: [spec.md](../spec.md)

## Content Quality

- [x] CHK001 The specification describes user outcomes rather than implementation
  mechanics.
- [x] CHK002 User stories are prioritized and independently testable.
- [x] CHK003 Every requirement uses mandatory, measurable language.
- [x] CHK004 Success criteria are measurable and technology-neutral where
  practical.
- [x] CHK005 Assumptions, dependencies, and exclusions are explicit.

## Evidence Boundaries

- [x] CHK006 The input authority is one immutable S077 projection.
- [x] CHK007 Facts and advice have separate structural and visual requirements.
- [x] CHK008 Every advice item has complete encounter, projection, calculation,
  and catalog provenance.
- [x] CHK009 Recommendation and metric algorithm versions cannot be confused.
- [x] CHK010 Non-causal and non-optimal wording limits are explicit.
- [x] CHK011 Missing Combat Metrics parity is identified as nonblocking future
  evidence.

## Quality Gates

- [x] CHK012 Duration and cast-count sample gates define both sides of each
  boundary.
- [x] CHK013 Capture-loss materiality defines qualification and suppression.
- [x] CHK014 Unknown-ID materiality defines qualification, target omission, and
  rule-local suppression without blocking unrelated advice.
- [x] CHK015 Zero denominators and overflow-safe sequence arithmetic are covered.
- [x] CHK016 Unavailable, non-finite, and invalid ratios cannot produce advice.
- [x] CHK017 Suppressed reports retain evidence and contain no advice.
- [x] CHK018 Qualified advice exposes every applicable reason.
- [x] CHK019 Equal candidate and rule ordering are deterministic.

## Safety, Privacy, and Lifecycle

- [x] CHK020 Generation is local, deterministic, side-effect free, and has no
  network, telemetry, or hosted model dependency.
- [x] CHK021 Persistence, raw mutation, catalog mutation, and retention are out of
  scope.
- [x] CHK022 Input and automation dependencies are explicitly prohibited.
- [x] CHK023 Catalog replacement rebuild behavior cannot mutate raw evidence.
- [x] CHK024 Installed and field verification issues cannot block implementation
  or closure.

## Delivery Completeness

- [x] CHK025 Pure and rendered test expectations cover ready, qualified,
  suppressed, and empty outcomes.
- [x] CHK026 Narrow-width and long-reason presentation are covered.
- [x] CHK027 Canonical documentation requirements name thresholds, provenance,
  wording, persistence, and isolation.
- [x] CHK028 Plan 039 archival and sole-active-plan transition are required.
- [x] CHK029 The feature is traceable to issue #136 and S090.
- [x] CHK030 No unresolved clarification marker remains.

## Notes

- Review completed before planning on 2026-09-11.
- The chosen thresholds are provisional versioned evidence gates. They are not
  ESO performance targets.
