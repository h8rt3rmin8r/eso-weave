# Contract: Dashboard Text Collision Safety

## Group measurement

1. Measure titles using the same font family and logical size used to paint them.
2. Select the maximum measured width for the current surface group.
3. Bound it so row gaps, the row's largest interaction reserve, and the minimum value width still fit.
4. Recompute when font style or available width changes.

## Row rendering

1. Allocate the row at the standard interaction height.
2. Allocate label, value, and optional interaction cells in order.
3. Set each child UI painter clip to its exact cell.
4. Paint the title with truncation, full accessible text, and its full hover tooltip without adding a keyboard focus stop.
5. Paint the marker and state in the value cell, preserving the full accessible state.
6. Paint controls only inside the interaction cell.

## Test oracle

For each row fixture:

1. Assert all geometry is finite, ordered, disjoint, and row-contained.
2. Find the exact semantic title and state nodes.
3. Find their exact painted text shapes.
4. Intersect painted bounds with the owning clip.
5. Reject any unrelated visible intersection larger than 0.5 by 0.5 points.
6. Assert full semantic strings remain unchanged when visible paint truncates.

The oracle must report the involved text and rectangles when it fails.
