# Research: Responsive Documentation Tables

## Decision 1: Preserve every table and enhance mdBook's generated wrapper

**Decision**: Keep the authored Markdown and generated semantic table intact. Enhance mdBook's existing `.table-wrapper` with a visible instruction and conditional region behavior.

**Rationale**: mdBook already provides script-free local horizontal containment. Enhancing that boundary minimizes source churn, retains header and cell relationships, and gives public and bundled output the same behavior.

**Alternatives considered**:

- Convert rows to visual cards at narrow widths. Rejected because CSS display changes and duplicated labels can weaken table semantics and make comparison harder.
- Rewrite 54 Markdown tables as raw HTML wrappers. Rejected because Markdown inside HTML blocks is fragile and the churn would make future table maintenance error-prone.
- Apply overflow directly to the table element. Rejected because the generated wrapper is already the natural scrolling boundary and can receive a clearer accessible region role without changing table display semantics.

## Decision 2: Add interaction only for measured overflow

**Decision**: A wrapper receives focusability, region naming, instruction association, and visible cue only when `scrollWidth` exceeds `clientWidth` by a small rounding tolerance.

**Rationale**: The same table can fit or overflow after viewport, sidebar, font, or zoom changes. Geometry is the authoritative signal, and conditional enhancement avoids redundant keyboard stops on compact tables.

**Alternatives considered**:

- Make every table wrapper focusable. Rejected because 54 unconditional focus stops burden keyboard and screen-reader navigation.
- Use viewport breakpoints alone. Rejected because content, column count, font metrics, sidebar width, and page zoom can produce different overflow at the same viewport.
- Depend on scrollbar visibility. Rejected because overlay scrollbars may be hidden until interaction and differ across platforms.

## Decision 3: Prevent column collapse with bounded width tiers

**Decision**: Add stable local classes for compact, dense, and very dense tables from column count, with explicit profiles for the five named evidence pages. Dense tables receive a minimum inline size and use their local wrapper when space is insufficient.

**Rationale**: The current wrapper contains overflow but does not stop the table layout algorithm from squeezing text-heavy columns into narrow stacks. Width tiers preserve a useful comparison surface without tiny fonts or `break-all`.

**Alternatives considered**:

- Use `width: max-content` for all tables. Rejected because prose cells would become excessively wide and require unnecessary scrolling even on desktop.
- Apply `overflow-wrap: anywhere` to every table cell. Rejected because it can split identifiers and produces the exact narrow character stacks issue #127 prohibits.
- Maintain 54 handwritten per-table width values. Rejected as needlessly brittle. Named-page profiles plus column tiers provide explicit coverage with a small stable policy.

## Decision 4: Use one visible hint with unique contextual naming

**Decision**: Insert one hint adjacent to each generated table wrapper, hide it while the table fits, and associate it with the wrapper only while overflow is active. Build the region name from the nearest visible heading plus the table's one-based page order.

**Rationale**: A visible instruction works even when platform scrollbars are hidden, and a unique contextual name lets assistive technology distinguish several tables under the same heading.

**Alternatives considered**:

- Use an icon or edge shadow alone. Rejected because color or decoration alone does not explain the keyboard interaction.
- Put the hint inside the scrolling content. Rejected because it could move out of view with the table.
- Label from header text alone. Rejected because repeated header signatures occur on Settings and Status Reference pages.

## Decision 5: Refresh state with one observer and scheduled measurements

**Decision**: Use one `ResizeObserver` over wrappers and tables, plus font readiness and window resize signals, with updates batched through one animation-frame queue.

**Rationale**: Wrapper and content size changes cover sidebar, font, responsive, and most zoom changes without a per-table observer or continuous polling. Batching prevents repeated synchronous layout work.

**Alternatives considered**:

- Observe every table with separate observers. Rejected as unnecessary overhead and cleanup complexity.
- Poll geometry on an interval. Rejected because it performs work when nothing changes and weakens deterministic tests.
- Measure only at initial load. Rejected because it leaves stale focusability and cues after resize or zoom.

## Decision 6: Extend the existing browser receipt

**Decision**: Add S089 table observations and journeys to the existing Chrome DevTools Protocol harness and retain the S086, S087, and S088 sentinels.

**Rationale**: One browser process already proves local assets, themes, viewports, keyboard input, true page zoom, print, and blocked scripts. Extending it prevents a competing harness and directly guards regression across earlier documentation systems.

**Alternatives considered**:

- Add screenshots or a visual comparison service. Rejected because it adds remote infrastructure and image-baseline noise for relationships that can be measured directly.
- Add a downloaded browser package. Rejected because the repository intentionally uses the available host browser and no package graph.
- Rely on source CSS tests only. Rejected because geometry, focus, native keyboard scrolling, and page overflow require a real layout engine.
