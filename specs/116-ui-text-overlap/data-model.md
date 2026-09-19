# Data Model: Collision-Safe Status Rows

## `DashboardRowLayout`

Calculated once per surface group:

- `label_width`: measured and responsively bounded shared title width
- `minimum_value_width`: non-zero retained status region
- `interaction_width`: optional trailing action reserve
- `gap`: cell separation

Every group row receives the same non-negative label width without consuming the value region, interaction reserve, or gaps.

## `DashboardRowGeometry`

- `row`: complete allocation
- `label`: title cell
- `value`: marker and state cell
- `interaction`: optional action cell

Every child is contained by `row`; cells are ordered; and visible paint is intersected with its cell clip.

## `TextCollisionObservation`

Test evidence retains semantic text, rectangles, clips, and owning cells. Unrelated observations collide only beyond 0.5 points on both axes.
