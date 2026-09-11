# Contract: Documentation Diagram Delivery

## Records

| ID | Page | Asset | Relationship |
| --- | --- | --- | --- |
| S082-D01 | `docs/src/development/architecture.md` | `docs/src/assets/diagrams/architecture-ownership.svg` | Separate input and observation ownership paths |
| S082-D02 | `docs/src/concepts/action-authorization.md` | `docs/src/assets/diagrams/action-authorization.svg` | Positive authorization gates and fail-closed branches |
| S082-D03 | `docs/src/development/state-machines.md` | `docs/src/assets/diagrams/safety-recovery.svg` | Close, synchronize, republish, and reopen order |
| S082-D04 | `docs/src/reference/pixel-bus-protocol.md` | `docs/src/assets/diagrams/pixel-bus-validation.svg` | Same-frame validation and invalidation branches |

## Source contract

Each page contains one exact local Markdown image reference for its record. The image alternative names the relationship and terminal safety outcome. A `Text equivalent` subsection immediately follows and contains every contracted anchor phrase.

Each asset is a standalone SVG source with top-down metadata, accessible title and description, system-font text, labeled outcomes, and no active or external content.

## Generated contract

The built page contains one local `img` reference with the meaningful alternative, and `assets/diagrams/` contains the corresponding SVG. No generated page references Mermaid, a remote URL, or a diagram runtime.

## Responsive contract

The `.docs-flow-diagram` wrapper is centered, bounded to the content width, and cannot create page-level overflow. Its image preserves aspect ratio at `max-width: 100%`. The adjacent text equivalent remains ordinary reflowing prose.

## Failure contract

Documentation policy returns actionable S082 errors for record drift, absent references, weak alternatives, incomplete equivalents, missing top-down metadata, inaccessible or unsafe SVGs, missing labeled outcomes, missing generated assets, remote runtime references, or lost responsive containment.
