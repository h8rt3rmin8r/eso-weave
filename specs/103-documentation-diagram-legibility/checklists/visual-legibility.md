# Visual Legibility Checklist

## Topology

- [x] Every node, edge, and branch label has stable identity metadata
- [x] Every edge names one source and one destination
- [x] Every branch and merge is traceable without guessing
- [x] No undeclared crossing, shared segment, or ambiguous junction remains

## Geometry

- [x] Successive stages have at least 36 SVG units of vertical separation
- [x] Edges clear unrelated nodes by at least 10 SVG units
- [x] Labels clear nodes, peer labels, bends, and arrowheads
- [x] Node text retains comfortable internal padding
- [x] Every visible text and graphical element remains inside the canvas

## Accessibility and surfaces

- [x] Visible words and shapes communicate meaning without color alone
- [x] Root title, description, alternative, and adjacent prose remain accurate
- [x] All labels use at least 14 SVG units
- [x] Normal and expanded states pass at 320 and 1280 CSS pixels
- [x] Expanded diagrams never render narrower than their paired normal state
- [x] Tall expanded diagrams retain intrinsic width and remain reachable by bounded panel scrolling
- [x] Navy and light themes retain legibility and page containment

## Regression evidence

- [x] Focused mutations reject each geometry failure family
- [x] Four direct-SVG layout observations pass
- [x] Existing 32-cell rendering matrix passes
- [x] Source-to-generated identity and offline delivery pass
