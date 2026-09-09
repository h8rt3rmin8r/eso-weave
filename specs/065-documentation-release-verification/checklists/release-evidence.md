# Release Evidence Checklist: Documentation Release Verification

- [x] Release tag, commit, workflow, publication time, and URL are recorded.
- [x] All five asset names and sizes are recorded.
- [x] All four package digests independently match `SHA256SUMS`.
- [x] Windows MSI payload evidence is complete and the incomplete elevated installation is labeled `Operator-waived`.
- [x] Linux deb installation and launch evidence is complete and the unperformed Help action is labeled `Operator-waived`.
- [x] Linux AppImage and tarball inspection is complete and unperformed portable execution is labeled `Operator-waived`.
- [x] Search, navigation, themes, responsive layout, assets, code, links, and 404 are covered by direct acceptance and release contracts.
- [x] Accessibility names, keyboard, pointer, visible focus, and non-color communication are covered without claiming a manual screen-reader session.
- [x] Loopback binding, methods, failures, reuse, origins, and cleanup are covered by direct inspection and release contracts.
- [x] Sidecar absence and offline runtime behavior are covered.
- [x] Every #84 and #83 gate maps to explicit evidence or an explicit `Operator-waived` variance.
- [x] No failure or blocker is mislabeled as an unqualified observation.
