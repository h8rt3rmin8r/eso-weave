# Companion-to-Addon Command Transport Decision

## Decision

No-go for real-time desktop-to-addon command ingress. The minimum approved
desktop command vocabulary is zero.

## Candidate disposition

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Custom addon binding plus generated key | No-go | Unknown server-visible metadata, contextual delivery, and no acknowledgement |
| Piggyback native action | No-go | Gameplay side effects and incorrect ownership |
| Generated slash-command typing | No-go | State-dependent delivery and possible chat leakage |
| Desktop-written SavedVariables | Deferred | Reload or next-launch only and unsafe while ESO owns in-memory state |
| User slash command or addon UI | Selected | Explicit, immediate, supported local control |
| Socket, inbound file watch, clipboard, or URL | Unavailable | No documented inbound addon API |
| PixelBus or encounter log | Wrong direction | Addon or game to desktop only |

## Reconsideration gate

Do not create a real-time implementation issue unless one of these occurs:

1. ESO publishes a documented inbound addon API with sufficient delivery and
   visibility guarantees.
2. The operator separately approves a live-account binding experiment after
   accepting the unknown server-visibility risk.

If a later policy change permits one command, the maximum vocabulary is one
idempotent `set_capture_mode` request with `off`, `next`, and `continuous`
states plus explicit channel and request identity. Status is outbound data;
clear and reload remain explicit user actions.
