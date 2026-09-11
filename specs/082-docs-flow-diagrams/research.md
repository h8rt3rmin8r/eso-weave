# Research: Documentation Flow Diagrams

## Visual-content audit

| Candidate | Decision | Rationale |
| --- | --- | --- |
| Architecture input and observation paths | Diagram | Two independently owned paths converge on input-producing consumers. A visual clarifies separation and ownership better than two inline arrow strings. |
| Action authorization | Diagram | Multiple positive gates converge on one authorization decision while every negative branch fails closed. The branch relationship is difficult to scan in prose. |
| Safety recovery | Diagram | Closure must precede synchronization, and recovery must republish a coherent baseline before reopening. Ordering is correctness-bearing. |
| Pixel Bus frame validation | Diagram | One-frame capture, the header and layout gate, heartbeat loss, independent per-block decoding, and recovery form a branching sequence with distinct failure scopes. |
| Individual Fishing cycle | Keep table | The transition table already names every state, event, side effect, deadline, and recovery more precisely than a compact image. |
| Auto Potion eligibility | Keep table | The ordered failure table is already the exact decision algorithm and supports search and copy better than a second rendering. |
| Configuration lifecycle | Keep table and prose | Settings and session ownership have several orthogonal attributes, so a matrix is clearer than a flow. |
| Release pipeline | Keep table and prose | The existing ordered gate table is operationally precise and does not need another representation. |
| Troubleshooting | Keep checklist | Readers need copyable recovery actions and branching questions, which the existing text provides directly. |

## Rendering decision

Three options were evaluated:

1. A page-time Mermaid runtime would keep terse source but add JavaScript, renderer, theme, security, and offline-delivery obligations.
2. Build-time Mermaid would require a new pinned tool and generated-asset drift checks for four small diagrams.
3. Checked-in static SVG keeps the asset and editable source identical, needs no new dependency, and is copied by existing mdBook behavior.

Option 3 is selected. SVG is not treated as generated Mermaid output, so no `.mmd` source is necessary. The policy validates the actual shipped structure and safety properties.

## Accessibility decision

Each SVG uses `role="img"`, `aria-labelledby`, one unique `<title>`, and one unique `<desc>`. Markdown supplies a purpose-specific alternative. A visible `Text equivalent` section immediately after the image names the complete order, branches, and outcomes. Color reinforces meaning but labels such as `Authorized`, `Blocked`, `Invalidate`, and `Republish` carry it independently.

## Visual system decision

Every diagram uses a 400 CSS unit wide top-down view box with an opaque ink background, light text, gold primary paths, teal positive outcomes, red fail-closed outcomes, and visible written status labels. System fonts avoid external resources. A page-scoped image class bounds width, adds a border, and prevents page overflow.

## Policy seams

The documentation policy will own a four-record manifest and pure validators for:

- exact page, source, alternative, and adjacent-text contracts;
- SVG root accessibility, view box, top-down marker, minimum text size, local-only safety, and labeled non-color outcomes;
- generated HTML references and all four copied asset outputs;
- responsive CSS containment.

Mutation tests will fail before implementation for missing assets, weak alternatives, missing text equivalents, horizontal orientation, active or remote SVG content, accessibility loss, missing generated output, and CSS containment loss.
