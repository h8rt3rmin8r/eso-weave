# S125 Research and Decisions

- The 14 raw `ui.strong` surface calls bypass the existing `widgets::label_strong` helper and resolve through active-control foreground. Reuse the helper and verify actual painted color/background, including alpha; keep active-control roles.
- The compact dashboard row explicitly truncates into a fixed-height cell. Essential remediation requires content-derived wrapping, not merely removing truncation.
- Both production Lua modules currently call `d`; current tests make that debug sink look like user chat. ESO's [official chat handlers](https://github.com/esoui/esoui/blob/live/esoui/ingame/chatsystem/chathandlers.lua) expose separate system and debug routes. Use system replies with actual chat-readiness handling and independent sinks in regression tests.
- The existing history worker serializes background operations. Source summaries currently arrive only after import. Add a read-only command instead of a new worker/service architecture.
- EncounterError currently preserves only I/O, database and string validation; the service erases them all into OperationFailed. Add controlled version/environment types and separate source/store stages; never expose validation payloads or classify by matching error strings.
- Current startSession refuses retained records; #273 owns session rotation. Explicit start preserves this contract and gives applicable guidance.
- Catalog acknowledgment already binds to SHA-256 identity. This slice corrects eligibility/placement while retaining service checks and explaining origin confidence separately.

All design questions are resolved by owner scope, current code and constitution. No new package dependency is required.
