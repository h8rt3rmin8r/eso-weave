# Research: Collision-Safe Status Rows

## Observed failure

`dashboard_metric_row` allocates every label exactly 118 points. `widgets::label_strong` lays out the complete SemiBold body-font galley in a child UI whose `max_rect` does not narrow the inherited painter clip. Long labels therefore paint past the label rectangle while the status marker and state begin at the fixed boundary. The same primitive renders System and State and ESO Weave Data Details, so both screenshots show one shared failure.

## Options evaluated

### Increase the constant

Rejected. It would fix only the current font and strings, waste width in smaller dashboard groups, and regress on future labels or enlarged logical text.

### Measure each row independently

Rejected as the primary design. It avoids collision but gives every row a different status origin, making related states harder to scan.

### Use a measured shared column per surface group

Selected. Measure the longest title with the exact SemiBold body font used by `label_strong`, bound that width against the available row width and reserved trailing controls, and pass it to each row in the group. This keeps origins aligned and adapts to current metrics.

### Depend on truncation alone

Rejected as an invariant. Truncation is the intended responsive behavior, but every child painter also needs an explicit cell clip so future widget changes cannot escape the allocation.

## Test seam

`Harness::output().shapes` exposes text and visual bounds, while AccessKit exposes semantic labels and rectangles. A global flattened comparison is unsafe under a modal, so the oracle scopes itself to a row or named container. `DashboardRowGeometry` exposes row, label, value, and interaction rectangles for ordering, containment, clip, and full-surface assertions.

## Text scaling

Pixels-per-point does not enlarge logical text. Tests modify the body `TextStyle` before measurement and rendering.

## Decision

Use bounded measured columns, clipped and non-focusable truncation, complete accessible text and hover help, component geometry/paint checks, and full-surface coverage in the existing Rust suite.
