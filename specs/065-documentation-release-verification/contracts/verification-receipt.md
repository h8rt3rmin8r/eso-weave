# Contract: v0.15.0 Documentation Verification Receipt

1. Identify the release tag, commit, workflow run, publication time, and release URL.
2. List every expected artifact with exact published and downloaded sizes.
3. Record expected and independently observed SHA-256 digests for every package.
4. Name the Windows and Linux environments, package paths, install paths, browsers, and relevant versions.
5. Map every issue #84 completion criterion to a direct observation or an explicit blocker.
6. Distinguish UI observation, browser request evidence, socket inspection, package inspection, and repository-contract support.
7. Demonstrate MSI, deb, and portable Linux behavior without rebuilding or substituting local binaries.
8. Record one reused IPv4 loopback endpoint and prove it stops responding after application exit.
9. Record safe GET, HEAD, unknown-method, unknown-path, traversal, and non-loopback behavior.
10. Report assistive-technology limitations honestly; test output alone cannot be described as a manual screen-reader result.
11. Name every temporary installation or extraction location and its cleanup disposition.
12. A fail or blocker keeps #84 and #83 open; a product failure links a separate implementation issue.
13. If the release operator explicitly waives an incomplete criterion and
    directs closure, label it `Operator-waived`, record the exact gap and
    deviation, and never relabel it as a pass.
