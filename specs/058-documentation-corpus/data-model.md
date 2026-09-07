# Data Model: Documentation Corpus Reorganization

## Source Artifact

| Field | Meaning |
| --- | --- |
| `source` | Exact baseline path relative to repository root |
| `kind` | Published, project, archive, infrastructure, or website |
| `disposition` | Retain, Move, Split, Archive, or Delete |
| `destinations` | One or more exact post-migration paths |
| `authority` | Canonical, project, historical, infrastructure, or superseded |
| `evidence` | Human-readable preservation or obsolescence proof |

Rules:

- The 50 baseline paths are unique and exhaustive.
- Move and Archive have one destination; Split has one or more.
- Delete requires a retained replacement and evidence.
- Every destination exists with exact case after migration.

## Specification Unit

| Field | Meaning |
| --- | --- |
| `heading` | Baseline H2 heading exactly as authored |
| `order` | One-based source order |
| `destinations` | One or more canonical page paths with normalized required excerpts |
| `evidence` | Preservation note or source marker |

All 20 H2 units are represented once in source order. The required-excerpt
manifest is independently frozen so removing an authoritative statement and
weakening its ledger row in the same change is rejected.

## Plan Disposition

| Field | Meaning |
| --- | --- |
| `plan` | Three-digit legacy plan ID |
| `completion` | Complete |
| `lifecycle` | Archived |
| `destination` | Exact archive path |
| `specs` | Implemented numbered spec packages |
| `evidence` | Commit, issue, PR, or release evidence |
| `notes` | Corrections or separate verification boundaries |

Plans 001 through 027 are contiguous and unique. Each evidence field names a
concrete pull request, commit, closed issue, or release. `postBaselinePlans` is an
append-only lifecycle list beginning at 028. An entry is either In Progress and
Active under `docs/project`, or Complete and Archived under `docs/archive` with
delivery evidence. At most one entry is Active, and a plan cannot exist in both
locations.

## Historical Reference Exception

| Field | Meaning |
| --- | --- |
| `file` | Exact file retaining an obsolete literal path |
| `literal` | Exact legacy path text |
| `reason` | Why rewriting would falsify historical provenance |

Exceptions are exact, reviewable rows. Directory-wide and file-wide wildcard
exceptions are invalid.

## Safety Crosswalk

Each constitutional safety invariant names at least one canonical published
destination: injected-input recursion breaking, focused-window-only suppression,
non-blocking hook callbacks, managed-marker-gated uninstall, AddOns containment,
and fishing SignalLost fail-closed behavior.
