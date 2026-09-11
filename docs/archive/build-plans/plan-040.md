# Plan 040: Evidence-Scoped Encounter Recommendations

Status: Complete, Archived

Sequence:

1. S090 implements issue #136 as a pure `s090-v1` recommendation consumer over
   one immutable S077 projection. It adds conservative sample and loss gates,
   rule-local unknown-ID handling, at most two provisional review prompts,
   complete per-item provenance, and a separate display-only encounter-history
   section.

The plan adds no new capture, import, storage, catalog mutation, network,
telemetry, model, or gameplay-action surface. Observed metrics remain the fact
authority. Recommendations are local, in-memory, reproducible, and unable to
authorize input.

Issue #131 may supply later live calibration evidence but never blocks this plan.
Installed v0.15.1 verification in issue #110 and catalog-field verification in
issue #129 are also independent Release verification work.

Plan 040 completed when PR #162 merged and closed issue #136.
