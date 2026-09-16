# Contract: Documentation Diagram Layout

## Source markup

Each logical node is a rectangle with unique identity and stage:

```xml
<rect data-node="capture" data-stage="1" .../>
```

Each connector is one orthogonal path with declared topology:

```xml
<path data-edge="capture-to-validate" data-from="capture" data-to="validate" .../>
```

Each visible branch outcome names its edge:

```xml
<text data-edge-label="header-invalid" ...>Invalid</text>
```

Metadata is inert. It neither receives focus nor changes SVG accessibility. Root `title`, `desc`, `role`, `aria-labelledby`, and adjacent prose remain the accessible content contract.

## Geometry contract

1. Node stages increase downward and successive stage bounds have at least 36 units between them.
2. An edge begins on its source boundary and ends on its destination boundary within a 1-unit tolerance.
3. Edge segments are horizontal or vertical.
4. An edge stays at least 10 units from unrelated node bounds.
5. Peer edges neither cross nor share a segment. No junction is used in S103.
6. A branch label names exactly one edge and has non-empty visible text.
7. A label does not intersect a node or peer label.
8. A label stays 4 to 24 units from its edge and at least 12 units from bends and the terminal arrowhead.
9. All element bounds remain within the root viewBox.

## Browser receipt

The existing documentation smoke loads each generated SVG directly, collects one observation, and validates exactly four unique diagram IDs. The smoke fails closed when metadata is missing, measurement throws, or any threshold fails. Rendering and figure-interaction receipts remain separate and mandatory.

## Compatibility boundary

S103 owns comprehension geometry. S086 continues to own decode, paint, aspect ratio, byte identity, and media type. S088 continues to own expansion semantics, modal focus, captions, and figure containment.
