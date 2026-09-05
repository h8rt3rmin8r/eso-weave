# Analysis: Negotiated Width-Aware Pixel Geometry

## Coverage Matrix

| Requirement group | Design artifact | Planned evidence |
| --- | --- | --- |
| FR-001 to FR-007 | `contracts/layout-header.md`, research R1-R5 | addon source tests and geometry tests |
| FR-008 to FR-014 | data model and header contract | decoder corruption, fit, legacy, and transition tests |
| FR-015 to FR-017 | research R6 and capture model | mock prepare counts plus GDI extent plumbing |
| FR-018 | research R8 | model routing and settings caption tests |
| FR-019 to FR-020 | header contract | manifest version and shared-constant tests |
| FR-021 to FR-023 | specification and quickstart | focused suite, full gate, and issue boundary audit |

## Cross-Artifact Consistency

- Every artifact uses three header cells and payload offset three.
- H0, H1, H2 bytes and the 3 through 65535 bound match across research, model,
  contract, and tasks.
- Only PixelBeacon derives capacity; the companion only validates it.
- Legacy requires a decoded status heartbeat and remains exactly 16 columns.
- Current payload count is 21. The superseded issue wording that says twenty is
  corrected to repository reality throughout this slice.
- Real-client evidence is consistently deferred to #44.

## Risk Review

### Plausible shifted payload

Control: invariant header, single authority, marker and checksum validation,
surface bound, and no fallback after recognizable corruption.

### Capture cost regression

Control: cached layout prepares one steady frame; only acquisition or growth can
prepare twice. Tests count requested extents.

### Resize race

Control: the header and payload are sampled from one containing frame whenever
possible. Growth triggers a fresh complete frame before payload sampling.

### Upgrade gap

Control: new reader recognizes the old magenta heartbeat and uses the frozen
16-column legacy layout. Manifest version 14 drives managed addon upgrade.

### Unsupported live environments

Control: automated contract proof closes implementation only. Issue #44 remains
open for release-build Windows and X11/Proton evidence.

## Implementation Conclusion

The specification is internally consistent, testable through existing seams,
and resolves the original fixed-column risk rather than reintroducing it. The
focused suites and complete local merge gate pass, including corruption-driven
immediate signal invalidation. No critical ambiguity or constitution conflict
remains.
