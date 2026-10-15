# S131 Producer Header, Profile Transitions and PLAYER_INFO

Date: 2026-10-15. These are static facts from the already installed Mac producer,
read without executing the game, reading process memory or changing settings.
They supplement the [generic writer evidence](research-producer.md) and the
independent ability/effect analysis. No producer binary or implementation is
copied into repository source; constructed tests must be labelled synthetic.

## Inspected producer and method

- File: `game_mac/pubplayerclient/eso.app/Contents/MacOS/eso` under the existing
  ESO Steam installation.
- Size: 72,562,448 bytes.
- SHA-256: `208f83b53576bf1a04ecf35e6e4a99779cf157e9e89fbf4457361b0923953d84`.
- Mach-O image base: `0x100000000`. Addresses below are unslid virtual addresses.
- The existing ignored `producer-macho.py` helper parses Mach-O sections and
  function starts, then uses pinned Capstone/macholib for static disassembly.
  Read-only enum pointer tables were decoded directly from file bytes.

The header itself writes native logVersion 15. The same function formats this
installed producer's gameVersion as `eso.live.10.3`. That build string describes
the inspected file, not the current game installation's active platform/build,
an addon API version or a field-verification result. These static serialization
facts are stronger than string-pool presence; they are still not a captured
installed-game execution trace or proof of every later build's behavior.

## Common record header and BEGIN_LOG

`0x100647b3e-0x100647ca0` resets the record buffer and its group-first-field
state, then writes these first two fields in order:

| Field label in verbose mode | Value rendering | Evidence |
| --- | --- | --- |
| `offsetMS` | Unsigned decimal `%u`; current timer minus the run-start timer, using the low 32-bit difference. | Label at `0x100647c34`; value at `0x100647c43-0x100647c51`. |
| `lineType` | Unquoted record-type enum string. | Enum call `0x100647c5a`; label `0x100647c62`; append `0x100647c77`. |

The enum helper `0x1034571e7` reads the pointer table at `0x104322e10`.
Indices 0, 1 and 13 resolve to `BEGIN_LOG`, `END_LOG` and `PLAYER_INFO`
respectively. These are the indices supplied by the emitters below.

`0x100648b66-0x100648ed6` saves the run-start timer and emits BEGIN_LOG. After
the two common fields, its exact ordered payload is:

| Field | Rendering | Evidence |
| --- | --- | --- |
| `timeSinceEpochMS` | Unsigned decimal `%llu`. | `0x100648bc2-0x100648be4`. |
| `logVersion` | Unsigned decimal literal 15. | Label `0x100648be9`; `mov edx, 0xf` at `0x100648c02`; `%u` append `0x100648c09`. |
| `realmName` | Quoted string helper. | `0x100648c13-0x100648c20`. |
| `language` | Quoted string helper. | `0x100648c2a-0x100648c37`. |
| `gameVersion` | Quoted formatted string. | Format `%s.%s.%u.%u` at `0x100648c55`, literal components `eso`, `live`, 10, 3, then field writer `0x100648c81-0x100648c8f`. |

There are seven top-level fields including the common header. Neither profile
flag appears in this payload. The actual offset need not be exactly zero because
the common header reads the timer after the run-start timer was saved.
`0x100648ed6-0x100648fe0` emits END_LOG using the common header with index 1 and
no payload, then clears the run timer and cached run state.

## Profile and enabled-state transitions

The logger constructor `0x100648706` sets enabled byte `[logger]` to zero and
the word at `[logger+0x180]` to zero. The latter initializes two independent
bytes. Their meanings are established by their concrete writer call sites:

| Function | Exact operation | Resulting serialization role |
| --- | --- | --- |
| `0x100648fe8-0x100648ff6` | Store incoming boolean to `[logger+0x180]`; return. | Generic prefix writer tests this byte: false gives commas without labels; true gives tabs and nonempty field labels followed by `: `. |
| `0x100648ff6-0x100649004` | Store incoming boolean to `[logger+0x181]`; return. | Cast/combat/effect emitters test this byte to choose separate ability metadata versus appended inline information. |
| `0x100647b32` | Read `[logger+0x180]`. | Verbose-mode getter. |
| `0x100649004` | Read `[logger+0x181]`. | Inline-mode getter. |

These two setters contain no call, run boundary, marker, flush or cache reset.
This is direct instruction evidence, rather than an inference from the Lua help.
The native enabled-state setter at `0x100648b16-0x100648b36` instead compares
the incoming value with `[logger]`: an unchanged value returns; a change to true
stores it and jumps to BEGIN_LOG; a change to false stores it and jumps to END_LOG.

Consequently a profile change can affect later records within the same run, even
within combat, without a new BEGIN_LOG. The header alone cannot identify inline
versus separate mode. A parser must interpret affected record layouts without
assuming one fixed profile for the whole run. This is a consequence of the
observed setters, not a claim about any unexecuted menu interaction or cadence.

## Arrays and nested equipment groups

`0x10064855c-0x100648664` calls the common field-prefix writer with its supplied
key, writes byte `0x5b` (`[`), then pushes a zero first-field sentinel.
`0x100648664-0x1006486e6` writes `0x5d` (`]`) and pops that sentinel. Separators
therefore follow the selected normal/verbose rule independently at each nesting
depth; there is no extra separator immediately inside an empty group.

`0x1006484ec-0x10064855c` writes an array as an outer named bracket group, loops
through unsigned 32-bit values using `%u` and an empty field key, then closes it.
The referenced empty-key address `0x103e55300` contains a NUL byte. Thus normal
arrays use comma-separated unsigned integers, verbose arrays use tab-separated
unsigned integers, and array elements have no invented names. Empty arrays are
`[]`. This helper does not expand ability IDs in inline mode.

## PLAYER_INFO exact mapping

`0x10064b8a8-0x10064c478` calls the common header with type index 13 and then
emits this ordered payload (eight top-level fields including the common header):

1. `unitId`: unsigned decimal `%u`, `0x10064bf02-0x10064bf28`.
2. `longTermEffectAbilityIds`: unsigned integer array, `0x10064bf2d-0x10064bf3e`.
3. `longTermEffectStackCounts`: unsigned integer array, `0x10064bf43-0x10064bf54`.
4. `equipment`: bracket group of unnamed bracket groups,
   `0x10064bf59-0x10064c30a`.
5. `primaryAbilities`: unsigned integer array, `0x10064c30f-0x10064c31d`.
6. `backupAbilities`: unsigned integer array, `0x10064c322-0x10064c330`.

The spelling above is taken from the actual verbose field arguments, rather than
the manually authored help's singular placeholder names. Inline mode does not
change these numeric arrays into embedded ability objects.

Each equipment entry has exactly these eleven fields in order:

| Position | Label | Value form |
| --- | --- | --- |
| 1 | `slot` | Unquoted slot enum token, for example `HEAD` or `MAIN_HAND`. |
| 2 | `id` | Unsigned decimal item ID. |
| 3 | `isCP` | `T` or `F`. |
| 4 | `level` | Unsigned decimal level or CP amount selected by `isCP`. |
| 5 | `trait` | Unquoted trait enum token. |
| 6 | `quality` | Unquoted quality enum token. |
| 7 | `setId` | Unsigned decimal set ID. |
| 8 | `enchantType` | Unquoted enchantment enum token. |
| 9 | `isEnchantCP` or `enchantCP` | `T` or `F`, with the exact branch-specific label below. |
| 10 | `enchantLevel` | Unsigned decimal enchantment level/CP, or decimal zero in the absent-enchantment branch. |
| 11 | `enchantQuality` | Unquoted quality enum token. |

The entry starts at `0x10064bfd3`; fields are emitted through
`0x10064c230`; its group closes at `0x10064c2d8`. The no-enchantment branch
`0x10064c23a-0x10064c2d0` writes the last four fields as:
`enchantType` = `INVALID`, **`enchantCP`** = `F`, `enchantLevel` = `0`,
`enchantQuality` = `NORMAL`. The present-enchantment branch uses
**`isEnchantCP`** at `0x10064c1a2`. Accepting the second spelling is an actual
producer branch requirement; replacing it with an arbitrary alias rule would
conceal the precise evidence.

The fallback tokens were verified beyond their numeric constants: enchantment
enum helper `0x103489740` maps 1000 to `INVALID`; quality helper `0x103474118`
reads the table at `0x104323b30`, where index 1 is `NORMAL`. That table's six
tokens are `TRASH`, `NORMAL`, `MAGIC`, `ARCANE`, `ARTIFACT`, `LEGENDARY`.
Slot and trait helpers read `0x1043235f0` and `0x104323760` respectively.

PLAYER_INFO also registers ability/effect metadata before its own record.
There is no inline-flag test in this function: regular metadata callbacks occur
at `0x10064b994`, `0x10064b9a0`, `0x10064be57` and `0x10064bed3`; scribed
callbacks occur at `0x10064be2d` and `0x10064bea9`. Lookup/deduplication conditions
can suppress individual records. An inline log therefore must not be rejected
merely because it also contains standalone ABILITY_INFO/EFFECT_INFO records.

## Local evidence receipts

Ignored full static outputs are retained as:

- `target/s129-s131-macho-header-player.log`.
- `target/s129-s131-macho-profile-groups.log`.
- `target/s129-s131-macho-profile-setters.log`.
- `target/s129-s131-macho-equipment-enums.log`.
- `target/s129-s131-macho-equipment-tables.log`.
- `target/s129-s131-macho-record-types.log` and
  `target/s129-s131-macho-record-type-table.log`.

No implementation, installed-game test, product capability or acceptance result
is implied by this research report alone.
