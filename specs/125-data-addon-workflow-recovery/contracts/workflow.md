# S125 Workflow Contract

1. Enable ESO Weave Data in the selected ESO environment; reload when required.
2. `/ewencounter start live single` (or selected pts/continuous) validates all arguments and starts only with no retained blocker. Repeated start reports current activity. `stop` is idempotent and keeps results; aliases retain behavior.
3. Expected replies lead with OFF, WAITING FOR COMBAT, RECORDING, INTERRUPTED or FAILURE and applicable retained counts/action. Replies use the shared visible system-chat sink.
4. Finish combat, stop continuous recording when desired, and save via `/reloadui`, logout or exit. Completed continuous-session records already saved remain importable.
5. Refresh saved source reads the selected saved file without mutation. Refresh imported history reads stored copies. Import Saved Capture atomically appends accepted terminal records; reimport is deterministic. Every successful or failed import invalidates and rereads the source summary. A stopped empty session directs deliberate clearing/restart instead of importing nonexistent encounters.
6. Readiness and diagnostics expose only controlled metadata. Private payloads never enter messages. Unknown versions remain recoverable and unchanged.
7. Data Details directly opens Encounter History. Exact commands have copy actions and expected replies; all essential instructions wrap and overflow scrolls vertically.
8. Catalog collection is maintainer game-definition work, separately disclosed. Eligible Live candidate acknowledgment appears with install actions and remains bound to identity. Native Encounter.log is unsupported here pending #272; retained multi-session behavior is pending #273.
