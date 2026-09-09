# Plan 035: v0.15.0 Documentation Release Verification

Status: Complete, Archived

Sequence:

1. Bind the verification receipt to the published v0.15.0 tag, release commit,
   workflow run, artifact inventory, sizes, and checksums.
2. Install and exercise the Windows MSI, Linux deb, and Linux AppImage using
   the shipped Help > Documentation interaction.
3. Verify offline content, browser behavior, accessibility, loopback security,
   endpoint reuse, sidecar independence, and listener cleanup.
4. Record each direct observation, automated inspection, supporting contract,
   limitation, and cleanup action in one durable receipt.
5. Close verification issue #84 and documentation epic #83 only if every gate
   passes; otherwise file separate implementation work and retain verification.

This evidence slice changes no application runtime. Governance clarification
issue #104 is separate and remains outside S065.

Completion evidence: issues #84 and #83 closed when S065 merged in PR #106.
