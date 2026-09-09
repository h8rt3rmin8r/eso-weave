# Runtime Safety Checklist: Death Recovery Safety

- [x] Dead closes the shared life gate before controller locks.
- [x] Gate closure advances the death and authorization epochs exactly once.
- [x] Alive-event receipt alone never authorizes synthesis.
- [x] Ghost recovery waits for exit evidence and a later coherent baseline.
- [x] Load recovery waits for activation, rebaseline, and a later coherent baseline.
- [x] No-load recovery waits for Alive evidence and a later coherent baseline.
- [x] Recovery routing refreshes action-driving observations before Alive.
- [x] Queued and running weave output cannot cross the death epoch.
- [x] Every fishing deadline is cancelled without replay.
- [x] Auto-potion waits a full new retry interval and uses fresh observations.
- [x] Matching releases for held generated input remain possible after closure.
- [x] Signal loss and unrecognized wire values remain fail closed.
- [x] Diagnostics contain no account, character, or input content.
- [x] Constitution safety invariants and full CI parity remain green.
