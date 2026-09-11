# Data Model: Documentation Flow Diagrams

## Diagram record

| Field | Meaning | Constraint |
| --- | --- | --- |
| `id` | Stable policy identity | `S082-D01` through `S082-D04` |
| `page` | Canonical Markdown destination | One of the four selected pages |
| `asset` | Repository-relative SVG source | Under `docs/src/assets/diagrams/` |
| `output` | Built-site asset path | Same basename under `assets/diagrams/` |
| `alt` | Markdown alternative | At least 40 meaningful characters |
| `heading` | Adjacent text-equivalent heading | Exact visible heading unique to the page |
| `anchors` | Required text-equivalent meaning | Ordered phrases covering paths and outcomes |
| `kind` | Relationship clarified | Ownership, decision, recovery, or validation |

## Static SVG diagram

Each asset has:

- a `viewBox` beginning at `0 0` and no fixed CSS width;
- `data-flow-direction="top-down"` on the root;
- `role="img"`, `focusable="false"`, and `aria-labelledby` naming unique title and description IDs;
- an opaque background, visible boundary, arrow marker, nodes, connectors, and text of at least 14 units;
- explicit positive and fail-closed text where branches use color;
- no script, event handler, animation, foreign object, remote URL, external font, or linked resource.

## Text equivalent

The equivalent is normal Markdown following the diagram. It names all nodes and transitions in reading order, identifies each negative outcome, and states the positive terminal result. It is canonical content rather than hidden accessibility-only text.

## Relationships

One diagram record maps to exactly one Markdown page, one SVG source, and one generated SVG output. Each page reference maps back to its record and is followed by one complete text equivalent.

## Validation lifecycle

1. Read all four Markdown pages and SVG sources.
2. Validate exact record coverage and local references.
3. Validate accessible and safe SVG structure.
4. Build mdBook.
5. Validate generated page semantics and copied assets.
6. Run link, spelling, text-hygiene, and repository gates.
