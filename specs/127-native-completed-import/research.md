# S127 Research

Date: 2026-10-13. Read-only research agents covered native contracts and source-aware
history/worker/storage integration. No raw payload values were published.

## Native profile

Decision: normal/separate native log version15 is the first supported profile.
Rationale: all three owner-file headers declare15, and observed shapes match the
documented normal positional vocabulary. Verbose/inline serialization is not fully
declared by the reviewed controls and is not represented in the sample.
Alternatives: guess verbose/inline layouts (rejected); postpone all completed
import for the historical live matrix (rejected, conflicts with#272's completed-file scope).

Primary sources:
- [Pinned stock logging help](https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/esoui/ingame/slashcommands/slashcommands_shared.lua#L43-L94)
- [Record vocabulary](https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt#L3073-L3099)
- [Logging controls](https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt#L18147-L18163)
- [ZOS effective/overflow declaration](https://forums.elderscrollsonline.com/en-gb/discussion/comment/6170265)

Header fields: elapsed milliseconds, BEGIN_LOG, epoch milliseconds, log version,
realm, language, game version. No API or addon version is provided. Public help
uses MAP_INFO/ZONE_INFO, while the API vocabulary and sample use MAP_CHANGED/ZONE_CHANGED.
Both declared names can be tested without inventing different semantics.

Observed structural counts, not platform verification: 43518907 bytes, 298596
terminated records, three closed runs, 205 paired combats (0/187/18), UTF-8 without
BOM and CRLF. Maximum record1920 bytes; largest combat11651 records/1869151 bytes.
113720 records and20948 combat events occur outside combat boundaries. Historical
issue counts201082/187 describe an earlier sample, not the current file.

Observed total field counts including time/type: BEGIN_LOG7; boundaries2;
COMBAT_EVENT20/29; BEGIN_CAST17/26; EFFECT_CHANGED17/26/27; HEALTH_REGEN13;
UNIT_ADDED18; UNIT_CHANGED13; UNIT_REMOVED3; ABILITY_INFO7; EFFECT_INFO6/7;
PLAYER_INFO8 with nested groups; MAP/ZONE5; END_CAST5/7.
END_CAST's extra integer differs from stock help and remains opaque. An optional
effect removal reference with an abbreviated target is tested synthetically.
Quoted commas and Unicode occur; simple comma splitting is invalid.
No producer quote-escape convention was exercised, so supported tokenizer escape
rules are explicit and tested rather than claimed observed.

## Source model and identity

Decision: separate native models and native canonical schema1, with raw ordered lines
and relevant pre-combat metadata context. Header-derived run IDs and run/combat
ordinals plus selected environment distinguish copied files without path identity.
Original source hashes remain distinct from canonical encounter hashes.
Rationale: EncounterCapture is an addon callback/replay contract with mandatory
addon/API values. Reusing it would fabricate facts and weaken strict validation.

## Storage

Decision: store schema6 adds immutable native runs/encounters, using existing
encounter_session_order. Supported schemas1-5 migrate inside the import transaction.
Rationale: S126 fixed migration surviving rejected imports. Native import must retain
that boundary and byte-preservation behavior.
Alternatives: second independent database (rejected, mixed history/deletion/backup
would lose a coherent transaction); broaden addon raw schema (rejected).

## Metrics

Decision: a native projection with its own algorithm version exposes observed facts
without a catalog gate. Paired duration, record/cast/effect counts and supported
hit/overflow quantities remain distinct from addon interval/replay/recommendation data.
Local-player attribution requires an actual unit declaration. Unknown/opaque fields
and missing semantic evidence yield explicit capability limits.
Native hit and overflow values stay separate; do not blindly reuse existing addon
healing subtraction. The potential legacy addon interpretation is a separate follow-up,
not proof of native/addon parity and not a scope expansion here.

## UI and readers

Decision: reuse no-follow stable snapshots with native-specific bounds, serialized
history worker and transient path selection. Native and addon inspections remain
separate; source kind/path/channel/generation prevent stale cross-source results.
No file chooser dependency exists; the established editable path field is sufficient
for deliberate local file selection. A future OS picker can be separately scoped.
Default summaries and errors use counts and controlled labels, never raw fields.

## Historical qualification

Decision: document completed-file support independently of the S092 live-tail/platform
qualification matrix. Preserve historical artifacts and explicit unsupported live
tailing. Repository fixtures and CI do not prove game flush cadence or platform runtime.
No owner field-verification request is created.
