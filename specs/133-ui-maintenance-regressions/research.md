# Research: S133

## 2026-10-15 - Baseline and decisions

Baseline main cf0d66e87277a31eca9f2f843b1f695be1fa5928 (0.19.0). Five issues are the entire scope.

Decision: reuse existing measured grow-only sizing instead of adding log height on every toggle. Alternative unconditional grow/shrink is the defect and conflicts with the owner's requirement. Usable-screen constraints must include origin/decorations, not just full monitor dimensions.

Decision: visibility is an explicit presentation preference alongside saved pane height. A separate state schema is unnecessary. Flush pending layout preferences on graceful close and preserve them across stale Settings drafts.

Decision: use egui Modal, matching Settings, rather than a top-level wrapped row or native popup. Existing lifecycle transaction remains authoritative.

Decision: installed manifest declarations and current selected-channel API evidence determine incompatibility presentation. Bundled support alone does not establish installed support. Unknown evidence is not a proven incompatibility.

Decision: automatic missing-root migration is a no-op. Windows discovery emits Live and PTS candidate directories regardless of existence, while mutation root validation rejects absent directories. This reproducibly creates a warning even without installed packages or a running game. Explicit install/uninstall root validation stays unchanged.

Decision: keep periodic bounded retries; remember only the last typed diagnostic for each root and emit on change/recovery. Do not suppress all errors or gate maintenance on game process presence. Outcomes distinguish committed cleanup, recovery pending, and other failed reconciliation without unsupported preservation claims.
