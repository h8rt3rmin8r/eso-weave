# Plan 039: Documentation Presentation Refresh

Status: Active

Sequence:

1. S079 completed in PR #147 and closed issue #120. It replaces the split bullet glossary
   with one formal alphabetical reference, preserves the complete S059 terminology
   map and legacy glossary inventory, adds accessible alphabet navigation, and
   enforces source and generated-search structure through documentation policy.
2. S080 completed in PR #148 and closed issue #121. It replaces the landing-page icon with
   the official wordmark, publishes an authoritative build-time project metadata
   snapshot, and enforces source and generated-output alignment.
3. S081 is in progress under issue #122. It brings approved source brand assets,
   valid surface examples, reproduction guidance, and accessible color swatches
   into the Brand Standard.
4. A later slice adds purposeful top-down diagrams to high-value documentation
   flows under issue #123.
5. A later slice builds the isolated deterministic screenshot sandbox under issue
   #124 before screenshot content begins.
6. After #124, a later slice adds guided setup and feature screenshots under issue
   #125 without requiring a live game capture.
7. A later slice enforces compact `S###` work-slice references in published prose
   and moves exact implementation symbols to layout-safe evidence under issue #126.
8. A later slice audits and fixes responsive documentation tables under issue #127.

Issues #120 and #121 closed independently, and issue #122 also changes no runtime
behavior. The only implementation dependency in this plan is #125 on #124. Public and
bundled offline documentation must continue to share the same local sources,
assets, and checks.
