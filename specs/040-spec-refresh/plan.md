# Implementation Plan: Master Specification Refresh

**Branch**: `main` (trunk-based) | **Date**: 2026-07-28 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/040-spec-refresh/spec.md`

## Summary

Rewrite the master specification so it describes the shipping system, rename it to
a filename that carries no version, cut the diagram count from ten to four, and
rewrite the prose declaratively. Documentation only: no behavior changes.

## Technical Context

**Language/Version**: Markdown, plus a mechanical path rename across the
repository.

**Primary Dependencies**: none.

**Testing**: the document is verified by reading it against the source, subsystem
by subsystem. That no behavior changed is proven by the diff, not by the suite: a
green suite shows that whatever changed still works, which is a weaker claim.
The suite is run anyway, because the constitution requires it before any commit.

**Target Platform**: not applicable.

**Constraints**: UTF-8 without BOM, LF, no em-dashes or en-dashes. Pinned
artifacts are untouched.

**Scale/Scope**: one rewritten document, one rename, 47 reference updates.

## Constitution Check

| Principle | Status | Evidence |
| --- | --- | --- |
| I. Spec-Driven Development | PASS | This feature's subject *is* the architecture of record. It traces to issue #15 and runs the full sequence. |
| II. Safety-Critical Surfaces | PASS | No source changes. The document's description of those surfaces is corrected and expanded, never weakened. |
| III. Test-First With Explicit Seams | N/A | Documentation only; no behavior to test. The suite is run to prove exactly that. |
| IV. CI Parity Before Every Commit | PASS | Full gate run in the foreground regardless. All 43 referencing files are Markdown; no Rust or addon source names the document. |
| V. Bounded Scope: Outside The Game | PASS | Unchanged. |

**Post-design re-check**: PASS.

## Design Decisions

### D1: The filename loses its version

`docs/ESO-Weave-Specification.md` becomes
`docs/ESO-Weave-Specification.md`. A version in a filename forces every revision
to choose between churning every reference and letting the name lie; this
document chose the latter for nine slices. One mechanical pass now costs nothing
again.

### D2: The document version is independent of the application version

The header states this explicitly. A 0.2.0 document beside a 0.8.1 application
read as seven versions of drift when the document had simply never been
renumbered. The document goes to 1.0.0: it now describes the whole shipping
system.

### D3: Four diagrams, none adjacent

The component architecture, the interception decision, the fishing state machine,
and the pixel-bus data flow. Each is a topology, a branch, a state machine, or a
flow. What was previously drawn and is not:

| Was drawn | Becomes | Why |
| --- | --- | --- |
| Thread and ownership graph | A table of threads and responsibilities | It is a list of five threads, not a graph. |
| Light-attack weave sequence | Nothing; the sequence table above it already states it | It restated the adjacent table as a picture. |
| Beacon install/verify/uninstall | Three short subsections | Three subgraphs in one figure is the density problem in miniature. |
| API version check flow | An ordered list | A linear procedure. |
| Main window region stack | A region table | A vertical stack of five boxes. |
| Config and state stores | Prose plus the existing bullets | Two files and an arrow. |

### D4: The open-items ledger and the owed-validation appendix are removed

Both enumerate what is absent or owed, which contradicts a declarative statement
of what the system is. Anything real in them belongs on the tracker.

### D5: Auto-potion becomes a top-level section

It was appended as a second `## 10.6`, colliding with `### 10.6 Manifest and
versions` and sitting at the wrong heading level. It is a top-level capability
alongside the weave engine and fishing, and is numbered accordingly.

## Implementation Outline

1. Rewrite the document in full at the new path.
2. Update every reference to the old path across the repository.
3. Verify: no false statements, no hedging vocabulary, no dangling references,
   every table-of-contents anchor resolving, four diagrams, none adjacent.
4. Changelog and merge gate.

## Risks

- **A silent inaccuracy survives.** Mitigated by verifying subsystem by subsystem
  against the source rather than reading the document for plausibility.
- **A reference is missed.** Mitigated by a repository-wide search for the old
  filename as an explicit acceptance check (SC-005).
