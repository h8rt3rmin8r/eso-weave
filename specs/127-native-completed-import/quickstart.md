# S127 Repository Validation

Use the sanitized native fixture under specs/127-native-completed-import/fixtures
and synthetic failures in tests/encounter_native.rs. Never publish the owner log.

1. Run the focused native parser/store/service suite, including unsupported version,
   quotes/groups/limits, unknown/tail/unfinished boundaries, reimport/copy/collision,
   old-store rollback, mixed source order/deletion/backup and native capability cases.
2. Run serialized worker and narrow/wide UI fixtures for explicit source/path/channel,
   inspection/import and stale results. Addon inspection remains independently tested.
3. Run the complete foreground cargo fmt, all-targets/features Clippy and locked tests,
   release build, policy/documentation/links/rendering and text gates from plan.md.
4. For private representative-size evidence, read the existing owner file into an
   isolated temporary history store, report only counts, and assert source bytes stay
   unchanged. This is repository parsing/import evidence, not installed/game verification.
5. Publish the official PR after gates, resolve findings within two rounds, verify
   final-head CI and request owner merge. No agent merge/tag/release.
