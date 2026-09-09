# Quickstart: Documentation Release Verification

1. Confirm v0.15.0 and all five expected assets exist on the GitHub Releases page.
2. Download the artifacts into one isolated temporary directory and verify every digest against `SHA256SUMS`.
3. Install the MSI on Windows 11 and record its installed path and version.
4. Activate Help > Documentation by pointer and keyboard, then record the loopback URL.
5. Exercise search, navigation, themes, assets, code, deep links, 404 behavior, safe HTTP failures, listener reuse, and cleanup.
6. Install the deb on Ubuntu 24.04 WSLg and repeat the matrix.
7. Run the AppImage without a documentation sidecar and repeat the portable-package subset.
8. Record direct observations, automated inspections, limitations, and cleanup in the durable receipt.
9. Run spec-kit analysis, documentation policy, text hygiene, and diff checks.
10. Close #84 and #83 only when every criterion is evidenced; otherwise file a separate defect and retain Release verification state.
