# Specification Analysis Report: Data Addon Lifecycle UI

**Analyzed**: 2026-09-13
**Artifacts**: `spec.md`, `plan.md`, `research.md`, `data-model.md`,
`tasks.md`, lifecycle contract, and constitution 3.0.0

## Findings

No CRITICAL or HIGH specification finding remains before implementation.

| ID | Category | Severity | Finding | Resolution |
| --- | --- | --- | --- | --- |
| A1 | Evidence | HIGH | Issue #185 requests enabled, loaded, and collecting states but S092 approved no live bridge | Each fact is explicit and unconfirmed unless supported evidence exists; saved values are historical only |
| A2 | Safety | HIGH | A naive Repair control could delete a valid package before replacement | Update and Repair both delegate to the existing atomic installer |
| A3 | Consistency | MEDIUM | Catalog Update currently owns a second lifecycle surface | Remove its mutation controls and share the main observation authority |
| A4 | Performance | MEDIUM | SavedVariables can be 128 MiB and cannot be parsed during paint | Use cached bounded observation refreshed outside rendering |
| A5 | Reload | MEDIUM | A transient notice could lose the required reload state | Retain reload outcome until stopped-state evidence makes clearing defensible |

## Implementation Decision

The implemented design deliberately narrows preliminary research option R2:
S093 does not parse `EsoWeaveData.lua` for historical presentation. The file can
reach 128 MiB, is hostile input, is stale until an ESO lifecycle flush, and
cannot establish any current-session fact requested by issue #185. Loading it
at application startup would add disproportionate latency and attack surface.
The UI therefore presents enabled, loaded, catalog, and encounter as separate
unconfirmed facts. This is an explicit deviation from the preliminary option to
surface historical values, made to keep the implementation truthful and
scope-proportional.

## Traceability

- FR-001 through FR-005 map to view-model and row-order tests.
- FR-006 through FR-013 map to lifecycle, intent, confirmation, and modal tests.
- FR-014 through FR-017 map to bounded-read, sizing, and accessibility tests.
- FR-018 through FR-020 map to documentation, screenshot, static policy, and
  delivery tasks.

## Gate Result

PASS for implementation entry and local publication readiness.

## Local Review Resolution

Three independent read-only passes covered code architecture, lifecycle
security, and UI-domain documentation. Their actionable findings are resolved:

- A stopped ESO boundary now permanently clears the reload reminder, while an
  uncertain mutation failure retains it conservatively.
- Inspection availability is separate from retained and failure-time lifecycle
  observations, so a failed path resolution preserves evidence but disables
  every mutation.
- Lifecycle logs suppress path-bearing lower-level errors and the UI provides
  safe remediation.
- Automatic status and lifecycle snapshot reads are size-bounded. Oversized
  managed drift is rejected without moving or changing the package.
- Stale uninstall, unmanaged no-op, narrow and wide placement, and restart
  transitions have regression coverage.
- The preliminary optional historical SavedVariables projection was removed
  from S093 commitments and the truthfulness checklist now matches the no-read
  implementation.

All three reviewers reported no remaining findings after the fixes.

## Local Validation

- `cargo fmt --all -- --check`: PASS
- strict locked all-target Clippy: PASS
- `cargo test --all --locked`: PASS
- deterministic documentation capture: PASS, 28 images; affected canonical
  images are byte-identical to the final verified capture
- spelling and text hygiene: PASS
- documentation policy unit and rendered-site suites: PASS, 140 tests
- `mdbook test docs`, `mdbook build docs`, and linkcheck2: PASS
