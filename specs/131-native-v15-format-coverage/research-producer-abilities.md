# S131 producer ability and effect serialization

Date: 2026-10-15. Evidence is passive static inspection of the installed
first-party Mac ESO executable. No game execution, process-memory inspection,
settings change or product implementation was performed.

## Source identity and method

| Property | Value |
| --- | --- |
| Producer | `eso.app/Contents/MacOS/eso` |
| Size | 72,562,448 bytes |
| SHA-256 | `208f83b53576bf1a04ecf35e6e4a99779cf157e9e89fbf4457361b0923953d84` |
| Analysis helper | `target/s131-research/producer-macho.py` |
| First ability disassembly | `target/s129-s131-macho-abilities.log` |
| Inline caller disassembly | `target/s129-s131-macho-inline-callers.log` |
| Scribed caller disassembly | `target/s129-s131-macho-scribed-event.log` |

The helper reads Mach-O sections and function starts, uses Capstone to decode
instructions, and annotates references to printable static strings. Commands
ran through the existing hidden `CreateNoWindow` runner. The addresses below
are virtual addresses in this exact producer, not addresses in a running game.
The parent research owns header version and complete record-kind mapping.

The generic writer research establishes these shared byte rules:

- Logger byte `+0x180` selects verbose output. Normal fields are comma
  separated and omit field names. Verbose fields are separated by a literal
  tab; a nonempty field name is followed by literal `: ` (colon and space).
- Logger byte `+0x181` selects inline ability metadata at the call sites below.
- String helper `0x100648180` writes a literal double quote, the supplied C
  string bytes, and a literal double quote. It does not insert backslash or
  doubled-quote escapes. Localized helper `0x100648350` ultimately calls that
  string helper. Path helper `0x100647d0c` also quotes its raw bounded copy.
- Enum strings passed directly to `0x100738620` are unquoted. Unsigned numeric
  fields here use `%u`; booleans use the single uppercase bytes `T` and `F`.

These generic facts were independently traced by the runtime research agent.
This report specifies the ability/effect ordering and branches that use them.

## Ability metadata tail

`0x100649908` emits exactly four fields in this order:

| Order | Exact verbose key | Serialized value |
| --- | --- | --- |
| 1 | `name` | Quoted localized ability name |
| 2 | `iconPath` | Quoted icon path |
| 3 | `interruptible` | `T` iff ability flags at `+0x1e8` have bit `0x02`; otherwise `F` |
| 4 | `blockable` | `T` iff the same flags have bit `0x04`; otherwise `F` |

The ordinary standalone ABILITY_INFO wrapper `0x10064981c` emits line kind 8,
then unsigned `abilityId`, then this tail. Its cache check suppresses repeated
standalone records for the same ability. This wrapper dereferences a concrete
ability object and is not the null-metadata path.

If the tail receives a null ability pointer, it still emits all four fields:
quoted empty `name`, quoted empty `iconPath`, `F`, `F`. The shared empty C
string at `0x103e55300` was independently read as zero bytes up to its first
NUL. For example, the normal inline suffix after an already written ability
ID is exactly `,"","",F,F`. The verbose suffix is a tab before each of
`name: ""`, `iconPath: ""`, `interruptible: F`, `blockable: F`.

No ID-zero test suppresses this tail at the event call sites. An unresolved or
zero reference can therefore retain its numeric ID and carry these null
metadata values.

## Inline placement and absence of an ability wrapper

The numeric reference remains in its original position. The inline branch calls
the metadata tail directly after writing that ID. It does not emit an ability
group name or any `[`/`]` wrapper.

| Record | Producer function and inline call | Position |
| --- | --- | --- |
| BEGIN_CAST | `0x100649582`, tail call `0x10064972b` | After `abilityId`, before source/target state |
| END_CAST | `0x100649b66`, tail call `0x100649cf2` | After `abilityId` |
| END_CAST with interrupted reason | Same function, second tail call `0x100649d38` | After `interruptingAbilityId`, before `interruptingUnitId` |
| COMBAT_EVENT | `0x10064a544`, tail call `0x10064a743` | After `abilityId`, before source/target state |
| EFFECT_CHANGED | `0x10064aa56`, tail calls `0x10064ac06` and `0x10064ac11` | After `abilityId`: ability tail first, then effect tail, then source/target state |

For BEGIN_CAST, COMBAT_EVENT and EFFECT_CHANGED, the false `+0x181` branch
emits separate metadata before constructing the event. The true branch instead
adds the corresponding tail inside the event. END_CAST has equivalent separate
metadata preparation and true-branch additions for both referenced abilities.

All these additions are flat. A verbose interrupted END_CAST can therefore
contain the same keys `name`, `iconPath`, `interruptible` and `blockable` twice,
once for each ability. A decoder must preserve ordered fields rather than
collapse the record into a map keyed only by name.

## Effect metadata tail and optional synergy reference

`0x10064ad2c` emits these fields after any preceding ability metadata:

| Order | Exact verbose key | Serialized value |
| --- | --- | --- |
| 1 | `effectType` | Unquoted enum string for a concrete ability |
| 2 | `statusEffectType` | Unquoted enum string for a concrete ability |
| 3 | `effectBarDisplayBehaviour` | Unquoted enum string for a concrete ability |
| 4, conditional | `grantsSyngeryAbilityId` | Unsigned decimal ability ID |

The spelling `grantsSyngeryAbilityId` is the producer's literal spelling,
including its transposition. It must not be silently corrected when accepting
verbose producer records.

The fourth field is present only when both static producer conditions hold:
the byte at ability `+0x320` equals `0x4c`, and bit `0x20` at ability `+0x1ea`
is set. Its value is read from ability `+0x1b8`. There is no zero-value test:
when the branch is selected, a zero value is still serialized as `0`.

The synergy reference remains a scalar unsigned integer in both profiles. This
function has no inline flag check and does not call the ability metadata tail;
there is no nested or expanded synergy ability attached to that reference.

If the effect tail receives a null ability pointer, all three base fields are
written through the quoted string helper, each as literal `""`. There is no
synergy field on that null branch. These are quoted empty values, not empty
unquoted tokens and not enum words. The normal null effect suffix is exactly
`,"","",""`.

The standalone EFFECT_INFO wrapper `0x10064ac86` emits line kind `0x0a`,
unsigned `abilityId`, then this same effect tail. It does not add the four
ability metadata fields, even when the logger is in inline mode. Its reference
to synergy remains scalar for the same reason.

EFFECT_CHANGED always writes the unsigned main `abilityId`, and only when
`+0x181` is true calls the ability tail followed by this effect tail. Both tails
are called even when the underlying ability lookup returned null. A separate
optional `playerInitiatedRemoveCastTrackId` follows unit state only when the
change type is numeric 2 and the supplied removal ID is not `0xffffffff`.

## Scribed standalone ability metadata

The standalone scribed wrapper `0x10064c4d0` emits line kind 8 and unsigned
`abilityId`, then calls `0x10064c614`. Its tail emits the same four base ability
fields followed by three additional quoted strings, in this exact order:

1. `primaryScript`
2. `secondaryScript`
3. `tertiaryScript`

Each absent script object produces quoted empty `""`, rather than an omitted
field. When the base ability data is unavailable, the function still emits all
seven tail fields: two quoted empty strings, two `F` booleans, and three quoted
empty strings. With the timestamp, record kind and ID this creates the known
ten-field standalone ABILITY_INFO form.

Static direct-call lookup found only the wrapper as a caller of
`0x10064c614`. The wrapper itself has two direct call sites in PLAYER_INFO
construction `0x10064b8a8` at `0x10064be2d` and `0x10064bea9`. Those loops choose
the scribed wrapper when an action-bar entry resolves to a scribed ability;
otherwise they use the ordinary standalone wrapper `0x10064981c`.

The entire inspected PLAYER_INFO constructor contains no `+0x181` inline flag
check. Inline mode therefore does not imply that separate ABILITY_INFO records
are absent: player/loadout metadata can still emit them. The event inline
branches listed above call the ordinary four-field tail `0x100649908`, not the
seven-field scribed tail. Scripts must not be guessed as extra event fields.

## Additional inspected record emitters

`0x10064b4ea` and `0x10064b59c` each write a line kind (`0x18` and `0x19`
respectively), unsigned `id`, then unsigned `abilityId`. Neither checks the
inline flag or expands the ability reference. Their record-kind names belong
to the parent research's complete mapping; this report does not infer names
from the two numeric kinds.

## Remaining integration boundaries

The exact syntax above comes from first-party serializer control flow, rather
than invented normalization rules or third-party fixture assumptions. Header
version confirmation, mode setters, complete record vocabulary and source/
target state remain covered by the parallel producer reports. This report
establishes no game-run verification and contains no unsanitized account or
character values.
