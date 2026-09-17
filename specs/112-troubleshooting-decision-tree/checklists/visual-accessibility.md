# Visual Accessibility Checklist: Troubleshooting Decision Tree

**Purpose**: Verify the S112 figure before publication

**Created**: 2026-09-17

**Feature**: [spec.md](../spec.md)

## Meaning and Topology

- [x] The root says to start with the first failing observation
- [x] Exactly five symptom families appear in the required order
- [x] Every matching branch terminates at the correct evidence boundary
- [x] The unmatched continuation ends at feature status and Live Log
- [x] No label claims a diagnosis, guaranteed cause, or guaranteed fix
- [x] Every visible connector has complete topology metadata
- [x] Branch labels associate with exactly one edge
- [x] No edge crosses, shares an undeclared segment, or enters an unrelated node

## Accessibility

- [x] SVG title and description are meaningful and referenced by `aria-labelledby`
- [x] Every visible label is at least 14 SVG units
- [x] Words, shapes, position, and direction carry meaning without color
- [x] The meaningful Markdown alternative matches the routing task
- [x] The unchanged prose is a complete adjacent text equivalent
- [x] Normal and expanded views remain readable at 200 percent zoom
- [x] Narrow and wide layouts remain contained without page overflow
- [x] No-script and print surfaces retain the static figure and prose

## Delivery and Governance

- [x] The asset is repository-owned, byte-identical after build, and served as SVG
- [x] The asset has no remote or active content
- [x] Source and generated policy cover placement, labels, prose, and bytes
- [x] The 40-cell rendering matrix includes S112-D01
- [x] The five-diagram layout receipt includes S112-D01
- [x] The finite figure inventory records five diagrams and 21 meaningful placements
- [x] Authorities and update triggers are recorded
