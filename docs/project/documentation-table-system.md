# Documentation Table System

S089 keeps every published Markdown table semantic, readable, and locally contained across the public and bundled offline documentation.

## Maintained inventory

The canonical source contains 54 tables across 26 published pages. Documentation policy freezes each page count, preserves generated `table`, `thead`, `th`, `tbody`, and `td` structure, and rejects unplanned additions, removals, or wrapper changes.

Five high-density references receive explicit profiles because their column relationships are correctness-bearing:

| Page | Tables | Profile |
| --- | ---: | --- |
| Coverage Matrix | 2 | Evidence |
| Test Strategy | 2 | Evidence |
| State Machines | 4 | State |
| Status Reference | 7 | State |
| Pixel Bus Protocol | 2 | Protocol |

Other tables use their generated header count. Two columns remain compact, three or four become dense, and five or more become very dense. These classifications set useful minimum widths without changing the authored table model.

## Runtime and accessibility contract

mdBook 0.5.4 already generates one `.table-wrapper` around each table with local horizontal overflow. The theme preserves that fallback and progressively adds one shell plus one visible instruction only when live geometry proves that a table overflows.

An overflowing wrapper becomes a named `region`, receives `tabindex="0"`, and is described by the visible instruction `Scroll horizontally to see all columns.` Left and right arrow keys scroll that focused region without moving the page. When the table fits, the theme removes the role, accessible name, description, focus stop, instruction, and stale horizontal offset.

One shared `ResizeObserver`, font readiness, and window resize refresh geometry through a batched animation frame. Scroll position records start, middle, or end state. Initialization is idempotent.

Cell text keeps normal word breaking and wrapping. The system does not use `break-all`, character stacking, duplicate mobile cards, hidden columns, smaller body text, or semantic display overrides.

## Print and script-free behavior

Print hides the interactive instruction, removes screen minimum widths, and exposes the full semantic table without a clipped scrolling surface. When scripts are blocked, the original mdBook wrapper remains locally scrollable and the semantic table remains available. Public Pages and bundled offline builds use the same generated files.

## Automated evidence

The browser receipt adds a 20-cell matrix covering the five named pages, Navy and Light themes, and 320 plus 1280 CSS-pixel widths. Every observation checks semantic markup, readable columns, normal token wrapping, body-size text, local containment, page containment, and conditional focus semantics.

Dedicated journeys prove trusted ArrowRight scrolling, resize-driven focus-state removal, real 200 percent page scale, print behavior, and blocked-script fallback. The existing diagram, syntax, and figure sentinels continue to pass in the same hidden local browser process.

## Maintenance rules

1. Author a real Markdown table when row and column relationships carry meaning.
2. Keep compact prose outside a table when no column relationship exists.
3. Do not hand-author a second responsive representation of the same data.
4. Update the exact inventory when adding, removing, or moving a published table.
5. Add a named-page profile only when its domain requires a stable width contract.
6. Preserve conditional focus behavior. A fitting table must not become a redundant keyboard stop.
7. Update source policy, generated policy, browser evidence, print behavior, and this record together when changing the table system.
