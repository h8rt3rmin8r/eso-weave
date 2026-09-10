# Quickstart: S077 Verification

1. Build a compatible test catalog and import the canonical capture fixture.
2. Run `encounter-project` with explicit store, catalog, output, session, and
   encounter arguments.
3. Verify DPS 300, HPS 80, shares 0.5/0.5, uptime 0.6, and cast order
   100/999999/100.
4. Verify declared loss degrades every metric and exposes its exact range.
5. Repeat the projection and compare canonical output bytes.
6. Rebuild against a later catalog and verify 999999 becomes known while raw hash
   and metric values remain unchanged.
7. Exercise ordering, mismatch, zero-duration, alias, link, and no-clobber failures.
8. Run full Cargo, documentation, encoding, whitespace, JSON, spelling,
   forbidden-dash, and mojibake gates.

Synthetic results establish deterministic behavior only. Issue #131 retains live
Combat Metrics parity and tolerance evidence.
