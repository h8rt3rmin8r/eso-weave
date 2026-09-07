# Checklist: Add-on Ownership

- [x] Only an absent target is classified NotInstalled.
- [x] Existing unproven directories, files, and links are Unmanaged.
- [x] Missing, invalid, unreadable, and marker-free manifests fail safe.
- [x] Every public writer checks ownership at mutation time.
- [x] Fresh creation cannot follow or adopt a target that appeared after inspection.
- [x] Managed Update does not delete before replacement.
- [x] Unmanaged mutation attempts preserve the complete target and siblings.
- [x] UI status distinguishes managed outdated from unmanaged.
- [x] Unmanaged state offers no lifecycle action and gives manual guidance.
- [x] Managed current, managed outdated, absent, unmanaged, and missing-root actions are tested.
- [x] Platform-specific link tests are gated without weakening cross-platform coverage.
