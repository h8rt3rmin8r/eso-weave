# Data Model: Documentation Diagram Legibility

## Diagram node

- `id`: unique stable identifier from `data-node`
- `stage`: positive integer from `data-stage`
- `bounds`: browser-measured `x`, `y`, `width`, and `height`

Rules: node bounds are positive, remain inside the root viewBox, and stages progress top to bottom with the required gap.

## Diagram edge

- `id`: unique stable identifier from `data-edge`
- `from`: source node identifier from `data-from`
- `to`: destination node identifier from `data-to`
- `samples`: ordered path points in root SVG coordinates
- `bends`: interior direction-change points
- `terminal`: final sampled point under the arrowhead

Rules: source and destination exist and differ, endpoints touch their declared node boundaries, paths are orthogonal, unrelated nodes retain clearance, and peer edges do not cross or share segments.

## Edge label

- `edge_id`: one edge identifier from `data-edge-label`
- `bounds`: browser-measured text bounds
- `text`: visible non-empty outcome text

Rules: the named edge exists, each branch edge has one label, label bounds remain inside the canvas and clear nodes and peer labels, and the label center remains near its own edge but clear of bends and the terminal arrowhead.

## Layout observation

- `diagram_id`: one of `S082-D01` through `S082-D04`
- `surface`: `generated-loopback`
- `node_count`, `edge_count`, `label_count`
- `visible_element_count`: every rendered text or graphical element outside non-rendered SVG definition containers
- `visible_elements_inside_canvas`: whether every inventoried visible bound remains inside the root viewBox
- `minimum_stage_gap`
- `topology_complete`
- `orthogonal_routes`
- `minimum_unrelated_node_clearance`
- `edge_crossings`
- `shared_segments`
- `labels_associated`
- `minimum_label_node_clearance`
- `minimum_label_peer_clearance`
- `minimum_label_bend_clearance`
- `minimum_label_terminal_clearance`
- `failures`: actionable element-level messages

## Layout receipt

- `layout_schema_version`: `1`
- `layout_sentinel`: fixed S103 pass sentinel
- `layout_observations`: exactly four unique diagram observations
- `layout_failures`: aggregate browser collection failures

The receipt passes only when all observations are complete and every threshold succeeds.
