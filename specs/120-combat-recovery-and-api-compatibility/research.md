# S120 Research and Decisions

Date: 2026-10-05. Research agents dispatched under speckit-plan; read-only source research, no game/application execution.

## Desktop binding discovery

Decision: Exclude positively identified gamepad assignments before desktop chord conflict counting, preserving unsupported controller-only state and genuine desktop conflicts.

Rationale: The current addon counts all assignments and returns Conflicting when a keyboard skill or mouse Attack coexists with a gamepad assignment. InputEngine then queues no weave, independently of timing. Upstream supports four slots and device-specific lookup. Existing fixtures model only two. Equivalent snapshot epoch churn was investigated and disproven; equality guards already exist.

Sources: [four-slot manager](https://github.com/esoui/esoui/blob/6639eb2adecc0480557d9068579319919a0c3fe6/esoui/ingame/keybindings/keybindings_manager.lua), [device-specific utilities](https://github.com/esoui/esoui/blob/6639eb2adecc0480557d9068579319919a0c3fe6/esoui/libraries/utility/keybindingutils.lua), `src/input/mod.rs::resolve_combat`, `tests/beacon.rs`.

Alternatives rejected: Choosing the first desktop chord hides genuine ambiguity; guessing slot indices assumes undocumented default layout; replacing native controls with hardcoded mouse values regresses the approved binding program.

Limit: The owner supplied no captured binding evidence. This demonstrates and repairs a matching disabling path, not an independently observed diagnosis of the installed session.

## Version evidence and API audit

Decision: Read bounded commit history for the selected channel, tolerate merge heads, and retrieve numeric API documentation pinned to the observed head revision. Keep client, numeric API, source freshness, and reviewed package declarations independent. A stale or failed check remains unknown.

Rationale: Live HEAD [6639eb2](https://github.com/esoui/esoui/commit/6639eb2adecc0480557d9068579319919a0c3fe6), dated 2026-09-28, says Merge branch pts into live. Parsing that message as a client version necessarily fails. Its recent history includes [12.1.5](https://github.com/esoui/esoui/commit/c6a91c390a9acee843560542923c69d06c438cd3). PTS HEAD is [62dba3a](https://github.com/esoui/esoui/commit/62dba3a3fd06ec09b7d370022de667fd2d34c6a4), also 12.1.5. Both documentation blobs are `93cc53c88260c41a313dc01dab31d3d662fc3a87`, 1,098,881 bytes, declaring API 101051. PixelBeacon omits 101051 and claims an unreviewed 101054. Adopt reviewed 101051 plus the retained 101050 declaration in embedded packages; do not claim arbitrary future API support.

Audit: All eleven action names still exist in the pinned bindings.xml (`943858c2bdd2669b1562a5a4640e5d2160fc45e3`). Binding, resource, slot, cooldown, life/movement/recall, and catalog APIs remain documented. Registered combat/effect/resource/lifecycle events remain present with compatible parameter positions. The encounter callback positions are unchanged. IsTextEntryOpen remains an official Lua helper. GetChampionSkillIcon remains an optional guarded source; it is not in the published API list. No required signature migration is demonstrated.

Source: [immutable Live API documentation](https://github.com/esoui/esoui/blob/6639eb2adecc0480557d9068579319919a0c3fe6/ESOUIDocumentation.txt). This is published game UI source evidence, not proof of installed game behavior or a complete inventory of all releases.

Alternatives rejected: Incrementing an API number from a patch number; treating a successful HTTP fetch as proof of source freshness; writing an unsupported observed API into addon declarations; suppressing warnings after saving last-seen client version.

## Stationary HUD diagnostics

Decision: Delete the conditional UI row; keep internal presentation freshness metadata for existing consumers and log only loss/cause change/recovery and retention expiry.

Rationale: `live_hud` inserts the row before resource gauges. A separate diagnostic transition cell avoids altering retention deadlines or letting repaint count produce log spam.

Alternatives rejected: Reserved space, moving the notice, changing retention/input policy, or logging each elapsed second.

## Execution and governance

Decision: Correct stale constitution V wording outside the analyze command to describe already shipped telemetry/native bindings. This adds no surface or authority. The owner approved preserving product behavior; the S100-S102 implementation and canonical corpus already document this use.

Use the installed PowerShell spec-kit helpers and templates. Run Rust checks synchronously through a redirected CREATE_NO_WINDOW launcher. Publication is authorized; at most two Codex rounds, every external finding answered, owner merge. No runtime/field checks or release cut.
