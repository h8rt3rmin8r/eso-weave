# Plan 043: Operator Intent Reliability

Status: Active

Sequence:

1. S097 completed issue #172 by persisting the requested Auto Potion toggle in
   session-state schema 4, restoring it through the existing fail-closed
   controller, converging UI and F3 persistence, and replacing obsolete
   session-only documentation without changing action eligibility.
2. S098 completed issue #171 with one bounded, process-local stale HUD
   presentation snapshot. Rendered values may be retained, while current game
   evidence remains the only authority for automation and synthesized input.
3. S099 implements issue #181 by making protected `main` the project-policy
   trust anchor, keeping collaboration and proposed branch content untrusted,
   enforcing immutable workflow dependencies and verified release tooling, and
   applying hosted pull-request and Actions controls. Its mandatory operator
   halt protocol was exercised before each authorized remediation phase.
4. Issue #188 remains a later multi-slice binding-discovery program and must be
   decomposed before implementation.

Release-verification issues #110, #129, #131, and #190 remain independent of
this repository-verifiable sequence. They do not become complete without their
named installed or platform evidence.
