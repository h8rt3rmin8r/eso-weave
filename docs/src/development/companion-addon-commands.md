# Companion-to-Addon Commands

This page explains the maintainer control boundary. Player instructions for
installing ESO Weave Data, recording, saving and importing are on
[Encounter Capture](../features/encounter-capture.md). The desktop can show
instructions and import ESO's saved addon data, while `/ewcollect` and
`/ewencounter` commands are entered by the user inside ESO.

S092 makes a no-go decision for real-time desktop-to-addon command ingress. The
approved desktop command vocabulary is empty. Catalog and encounter capture
remain explicit user actions through `/ewcollect` and `/ewencounter`.

The documented ESO addon API has no inbound socket, file-watch, clipboard, or
URL primitive. Generated custom bindings have unknown account-associated
visibility, contextual focus and action-layer failures, and no supported
acknowledgement. Piggybacking a native action adds gameplay side effects, while
generated slash-command typing can leak text into chat. Writing SavedVariables
while ESO owns its in-memory state is neither real-time nor safe.

This decision is separate from native binding discovery for user-configured
input features. Reconsider real-time ingress only if ESO publishes a documented
inbound API with adequate delivery and visibility guarantees, or if the operator
separately authorizes a live-account binding experiment after accepting its
unknown server-visibility risk.

## Deliberate recording commands

Use `/ewencounter start live single`, `start pts single`, or the
`continuous` variants inside ESO. Start validates the complete arguments,
keeps an already enabled recorder unchanged, and never replaces a retained
session. `/ewencounter stop` retains data and is idempotent.
`/ewencounter status` gives concise activity and next-step guidance;
`status details` preserves technical diagnostics. Existing aliases remain
compatible. Desktop Copy actions copy command text but do not execute it.

Both data modules use the shared system-chat message route, independently of
debug logging. Delayed initialization queues bounded replies and retries; it
never silently falls back to the intercepted debug sink. The exact-data warning
must be delivered before capture authority is enabled.
