# Research: Documentation Diagram Legibility

## Current evidence

All four SVGs render and pass the S086 paint and aspect-ratio matrix. The defect is composition, not delivery. Current vertical connector runs are commonly 12 to 44 SVG units, edge labels sit within a few units of bends or node borders, and the Pixel Bus flow compresses multiple branches and merges into an 820-unit canvas. The existing browser gate cannot see those relationships because it samples each SVG as one rasterized image.

## Decision 1: Measure elements in the browser

**Selected**: Load each generated SVG directly through the established loopback server and measure its SVG DOM with the existing Chrome DevTools connection.

**Why**: `getBBox`, path length, and sampled points use the actual generated asset and browser geometry. This proves relationships between labeled elements without a new parser or raster baseline.

**Rejected**:

- Pixel-golden snapshots vary with browser rasterization and do not explain which relationship failed.
- Source-string checks can prove metadata exists but cannot measure text bounds or path geometry faithfully.
- A third-party SVG geometry package would add a dependency for four bounded assets.

## Decision 2: Use explicit semantic topology metadata

**Selected**: Mark node rectangles with `data-node` and `data-stage`, connector paths with `data-edge`, `data-from`, and `data-to`, and branch text with `data-edge-label`.

**Why**: The metadata documents topology independently of color and allows actionable failures such as an edge entering an unrelated node. It is inert, local, and safe in both Pages and bundled delivery.

**Rejected**:

- Inferring identity from DOM order is brittle and gives poor failures.
- Inferring meaning from stroke color violates the non-color accessibility boundary.

## Decision 3: Grow vertically, retain 400-unit width

**Selected**: Keep the existing 400-unit width and expand height per flow.

**Why**: The issue identifies top-to-bottom compression. Retaining width preserves current responsive text scale at 320 CSS pixels, while taller canvases create stage separation and routing lanes without shrinking labels.

**Rejected**:

- Uniform scaling leaves all relative crowding unchanged.
- A wider canvas would shrink labels further on narrow viewports.

## Decision 4: Orthogonal independent routes

**Selected**: Use horizontal and vertical path segments with distinct lanes and terminal points. No S103 edge shares a segment or requires a junction.

**Why**: Orthogonal routes are easy to trace and allow deterministic segment intersection and unrelated-node clearance checks.

## Decision 5: Scroll tall expanded figures at intrinsic width

**Selected**: Remove the dialog image's viewport-height cap, retain the bounded panel's vertical scrolling, and reject any expanded diagram that renders narrower than its paired normal observation.

**Why**: At 1280 by 920 CSS pixels, fitting the 400 by 1080 Pixel Bus diagram into the available 744-pixel image row reduced it to roughly 276 pixels wide. Expansion therefore made its 14-unit labels smaller. The existing panel already owns overflow, so intrinsic-width scrolling is the smallest correction and does not introduce another viewer.

**Rejected**:

- Reducing the reconstructed canvas height would reintroduce the crowded routing issue.
- Widening the SVG would shrink its labels on narrow normal views.
- A new zoom library or second viewer would duplicate the established S088 interaction.

## Thresholds

- Successive stage separation: at least 36 SVG units.
- Edge clearance from unrelated nodes: at least 10 SVG units.
- Edge label association: 4 to 24 SVG units from its named edge.
- Label clearance from bends and terminal arrowhead: at least 12 SVG units.
- Browser numeric tolerance: 1 SVG unit for boundary contact and floating-point measurements.

These thresholds are larger than stroke widths and arrow markers while remaining achievable on the 400-unit canvas.
