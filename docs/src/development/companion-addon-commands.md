# Companion-to-Addon Commands

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
