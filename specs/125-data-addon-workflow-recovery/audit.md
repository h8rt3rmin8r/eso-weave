# S125 Surface and Workflow Audit

Audited on 2026-10-11 against issues #265 through #271.

## Surface emphasis (#265)

All 14 original raw `ui.strong` call sites in `src/app/ui.rs` now use
`label_surface_strong`. The helper derives the active palette from the actual
UI theme and preserves semantic emphasis with the existing strong-label helper.

| Original surface | Disposition |
| --- | --- |
| Reviewed import candidates | Surface text role |
| Catalog operation progress stage | Surface text role |
| Local history unavailable | Surface text role |
| First history diagnostic heading | Surface text role |
| Second history diagnostic heading | Surface text role |
| Old import failure heading | Replaced by controlled diagnostic surface |
| Session heading | Surface text role |
| Observed data quality | Surface text role |
| Ability damage share | Surface text role |
| Effect uptime | Surface text role |
| Cast order | Surface text role |
| Recommendation status | Surface text role |
| Recommendation title | Surface text role |
| Recommendation view label | Surface text role |

There are no retained raw-emphasis exceptions in this file. Filled-action
foregrounds retain their own role. Tests inspect emitted glyph foreground,
opacity and painted background, require 4.5:1 contrast in both themes, and
demonstrate that the wrong filled-action role fails on a plain card. Empty,
selected and busy catalog fixtures exercise production discovery and workers.

## Catalog intent (#266)

Acknowledgment appears immediately before installation only when the selected
candidate is compatible Live data. Empty, PTS-only and incompatible candidates
have no placeholder checkbox. Copy distinguishes hash/integrity checks from
trust in the candidate's origin. Intent binds to the selected candidate hash;
selection changes, discovery refresh, changed bytes and unchecking reset it.
Worker-busy state disables the checkbox and reports progress. Tests cover
keyboard Space, returning to a previously acknowledged candidate and real
installation contention. The service still validates the selected bytes.

## Essential instructions (#267)

The exact Data Addon Next Step row uses measured wrapping rather than truncation.
Other telemetry keeps the compact layout. Status-dot, label, value and trailing
control origins are preserved. Parent clipping is honored. Expanded cards share
the resulting measured height, including a requested second egui pass after
wrap changes. Data Details fixes its action row and Close outside a scroll body.

Copy branches cover unavailable configuration, not installed, managed/current,
managed/mismatch, unmanaged and unavailable inspection. Current/mismatch copy
adds reload only when appropriate; the explicit reload-required prefix is
composed once. Paint fixtures cover lifecycle, reload, inspection, both themes,
360/580 widths and 1.0/1.5 scale. Reduced-height fixtures scroll the complete
instruction into view and assert a reachable Close button. Existing alignment,
origin, collision, tooltip and accessibility checks remain in the suite.

## System replies and commands (#268, #270)

The existing bootstrap owns a shared route to `CHAT_ROUTER:AddSystemMessage`.
When present, `ZO_GetChatSystem` must return an initialized system. Both modules
use this route independently of debug output. A maximum of 64 pending replies
is retried; overflow produces a controlled notice, and successful flush removes
the retry. Queue admission and delivery must both succeed for the enable warning. No additional managed addon file is introduced.

`start live|pts single|continuous` validates the complete argument list before
mutation. Repeated start reports the enabled state, preserving mode/channel and
session identity. Failed and retained states require their existing recovery.
Stop is idempotent and aliases remain supported. Enable cannot acquire capture
authority before its warning reaches chat. Registered-command tests cover
immediate and delayed readiness, a router without initialized chat, duplicate
flush, bounded overflow, waiting/capturing repeated start and invalid syntax.
Existing lifecycle, warning order and module-isolation tests remain required.

## Last-saved source and diagnostics (#271)

Inspection runs on the existing serialized worker before first import. It reads
the selected file through the bounded stable reader, using modification metadata
from the same handle, and returns path, environment, controlled readiness and
finished/partial counts. Current capture is excluded from finished counts.
Inspection neither creates history nor rewrites source bytes. It cannot prove
the game is running, the addon is enabled or an in-memory session is active.

Requests and replies carry generation, path and environment; only matching
results reach the UI. Workflow entry rereads the source; Successful and failed imports invalidate and reread the source. Empty stopped sessions direct clearing/restart; stopped sessions with records preserve import-first guidance. Refresh saved source and Refresh imported history have
separate effects. Missing, inaccessible, malformed, unsupported-version and
wrong-environment sources have controlled categories; store append failures
retain their separate category. Raw hostile text is never used to classify or
display diagnostic guidance.

Fixtures cover initialized, waiting, active/current, stopped, interrupted,
failed, legacy finished data, empty and duplicate import, denied/missing,
oversized/malformed, unsupported/wrong-channel and corrupt store. Source/history
byte preservation and worker identity are asserted. Permission denial has a
deterministic typed-I/O unit fixture because ancestor-file lookup reports
NotFound on Windows.

## Complete workflow (#269)

Data Details exposes Record a fight and Open Encounter History directly. The
recording walkthrough orders enable/reload, selected-environment single start,
fight/status, save/import and viewing. Each command has an exact copy action and
an expected chat result. Continuous and recovery commands are secondary;
maintainer definitions/catalog collection is folded separately. The desktop
does not execute commands in ESO. Native Encounter.log ingestion remains
unsupported (#272), and multi-session retention remains #273.

Canonical encounter, interface, command, catalog, status, troubleshooting,
architecture and responsible-use documentation matches these surfaces and is
included in the release binary's bundled documentation build. Automated tests
are repository evidence, not installed/game-session verification.

First-review regression coverage also checks stopped empty-session guidance in
source inspection, worker import recovery and registered Live/PTS status/start
commands. Failed imports clear stale source readiness before rereading it.
