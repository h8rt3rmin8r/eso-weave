# Analysis: Collision-Safe Status Rows

## Pre-implementation gate

Status: PASS

Issue #231 maps to FR-001 through FR-016, and both affected surfaces share
`dashboard_metric_row`. The design covers measured alignment, constrained widths,
actions, clipping, accessibility, themes, states, and logical text enlargement.
It uses the existing Rust CI suite, leaves issue #231 as the sole tracker, changes
no pinned artifact or safety-critical behavior, and retains the pre-push halt.

Row and container scoping avoids modal-layer false positives, text styles replace
pixels-per-point for enlargement, and explicit clips preserve the invariant if
truncation changes. S116 also avoids the reserved S115 identifier. No critical,
high, or medium conflict remains.

## Post-implementation gate

Status: PASS

### Test-first evidence

Before the production fix, `dashboard_status_row_rejects_painted_text_overlap`
failed because `Data Addon Ownership` painted through the fixed 118-point title
cell and intersected the status dot by 10 by 9 points. The same test passes after
the shared measured-column and clipping contract is applied.

- Live HUD, System and State, and Data Details each calculate one title-column
  width from the exact active SemiBold body font and their complete title inventory.
- The width yields to reserved actions, cell gaps, and a 72-point minimum value
  region. Title, value, and interaction painters are clipped independently.
- Constrained titles truncate visibly while their exact AccessKit labels and full
  hover text remain available without changing the keyboard focus sequence.
- Row geometry now exposes the outer, label, value, and optional interaction
  rectangles. Component tests verify containment, ordering, paint bounds, and a
  0.5-point collision tolerance.
- Full System and State and Data Details paint oracles cover dark and light themes,
  normal and 125 percent logical text, narrow through 1200-point widths, and the
  required addon state and action matrix.

The complete `app_ui_sizing` integration target passes all 49 tests, including
the three new collision gates and the exact accessible-label assertions.

## Hosted verification

Pull request #232 passed issue linkage, trust policy, dependency review,
documentation, Ubuntu, Windows, and CodeQL checks. The automatic Codex review
and the one authorized final `@Codex` round both returned an account usage-limit
notice with no code finding or review thread. No further review round was
requested.
