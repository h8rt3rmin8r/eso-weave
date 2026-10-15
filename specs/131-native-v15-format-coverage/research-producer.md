# S131 producer documentation and static evidence

Date: 2026-10-15. Follow-up research for #296 and #297. This report records newly
retrieved producer documentation, specific attachment-access results and passive
inspection of the installed Windows producer. The client was not executed, no
process memory was read and no logging, anonymity or other game setting changed.

## Original ZOS documentation attachment

The [Update 22 API staff post](https://forums.elderscrollsonline.com/en-gb/discussion/comment/6170265)
links the original
[ESOUIDocumentationP22.txt attachment](https://us.v-cdn.net/5020507/uploads/editor/hi/h6h1nh927aqh.txt).
The attachment itself was retrieved successfully, rather than relying on its
filename or search excerpt:

- HTTP 200, 715,525 bytes.
- SHA-256: `fc8af782ad63bed82bebd220b6007fbb265ed312e759bf2e5b1624fb4ce45874`.
- The encounter enum begins at line 2892 and the encounter unit-type enum at
  line 2915 (LF-delimited source lines, trailing CR bytes ignored for display).
- Lines 13907-13923 declare the enabled, verbose and inline setter/getter pairs
  and the native version getter. These are API signatures with boolean/integer
  types, not serialized record examples.
- The whole decoded document contains no `timeSinceEpochMS`, `sourceUnitState`
  or comma-delimited `COMBAT_EVENT,` description. Its occurrence of `BEGIN_LOG`
  belongs to an enum constant. There is no named-field punctuation, grouping,
  string-escape or inline-expansion declaration.

This is evidence that the original attachment does not fill the grammar gap.
It is not evidence about the body of every later attachment, or about the
behavior of an installed client.

## Specific current and historical attachments

The [Update 5.0 thread](https://www.esoui.com/forums/showthread.php?t=8465) exposes
ESOUIDocumentationP22.txt and the later P22-3 revision. The
[Update 51 thread](https://www.esoui.com/forums/showthread.php?t=11716) exposes
ZOS_DanBatson's P51 and P51_2 documentation posts. The latter is dated
2026-09-28. These specific links were followed:

| Attachment | Direct link | Observed anonymous response |
| --- | --- | --- |
| P22 | [attachment 1105](https://www.esoui.com/forums/attachment.php?attachmentid=1105&d=1555350500) | HTTP 200 HTML requiring login or access permission, not the text document. |
| P22-3 | [attachment 1128](https://www.esoui.com/forums/attachment.php?attachmentid=1128&d=1558451546) | The same access-message category, not the text document. |
| P51 | [attachment 1969](https://www.esoui.com/forums/attachment.php?attachmentid=1969&d=1785772064) | HTTP 200, `text/html`, login/access message. Its listed document body was not observed. |
| P51_2 | [attachment 1986](https://www.esoui.com/forums/attachment.php?attachmentid=1986&d=1790624314) | HTTP 200, `text/html`, login/access message. Its listed document body was not observed. |

An HTTP success status for these links must not be described as a successful
documentation download. No login or permission workaround was attempted.

Public source mirrors were inspected independently, without claiming a
byte-for-byte comparison against those inaccessible attachment bodies:

| Pinned document | Bytes and SHA-256 | Actual encounter content |
| --- | --- | --- |
| [PTS5.0 API 100027](https://github.com/esoui/esoui/blob/bd47b8ed98006b6be205d3c7126b52c60f5b36ca/ESOUIDocumentation.txt) | 697,007; `934141c2d4a28236532bfb70e873d392bf082021fc7f21af47a090f56e79ff09` | Encounter enums and seven logging/version API signatures. No native byte grammar. |
| [PTS9.3 API 101041](https://github.com/esoui/esoui/blob/ff8b61d21e8bfcca0a8ae797529450a2acb020a8/ESOUIDocumentation.txt) | 949,821; `91d0fba7464b259e4e8fa4803c3b28c3f22b7a9a3ca56c703791d906ecb91024` | The same relevant signature categories. No named-field or inline byte grammar. |
| [Reviewed live API 101051](https://github.com/esoui/esoui/blob/6639eb2adecc0480557d9068579319919a0c3fe6/ESOUIDocumentation.txt) | 1,098,881; `8e17623327db133716337daf83c5c4ab5d76cc889165e1e40532803dad00fd62` | Encounter enums begin at line 3234; logging/version signatures at lines 19544-19560. No serialized record examples or punctuation rules. |

API 101051 is the document's addon API version. It does not establish that any
native source record has logVersion 15, or supply an addon API value for native
encounter evidence.

## Historical source and technical discussion

The PTS5.0 branch contains the original helper in
[pc/slashcommands_pc.lua](https://github.com/esoui/esoui/blob/bd47b8ed98006b6be205d3c7126b52c60f5b36ca/esoui/ingame/slashcommands/pc/slashcommands_pc.lua),
rather than the later shared-file location. It constructs chat-help lines from
literal field lists and delegates option changes to native functions. Its
BEGIN_LOG help omits the later gameVersion field and its END_TRIAL help omits
the later finalVitalityBonus field. These differences are further reason to
avoid treating an older field list as an exact v15 serialization declaration.

The [PTS9.3 shared helper](https://github.com/esoui/esoui/blob/ff8b61d21e8bfcca0a8ae797529450a2acb020a8/esoui/ingame/slashcommands/slashcommands_shared.lua)
was also read outside default-branch search. Its `format` branch still prints
manually authored field descriptions; it does not call the disk serializer or
print an actual source line. The verbose/inline command branches call their
native setters without consulting the enabled flag. That proves the Lua
command's control flow, not when the native producer changes record format or
whether it writes a transition marker.

The stable [Encounter Log technical thread](https://www.esoui.com/forums/showthread.php?t=8904)
and its [complete archive](https://www.esoui.com/forums/archive/index.php/t-8904.html)
were retrieved directly. The previously indexed `goto=nextnewest&t=9015` URL
resolves to this different thread, so t=8904 is the appropriate citation. Its
four replies discuss where to find help and whether the filename can change;
none gives a verbose/inline line example.

An additional primary observer example occurs in the
[Hyper Tools comments](https://www.esoui.com/downloads/fileinfo.php?id=3057&page=1&so=oldest),
dated 2021-05-06/07. A player quotes ABILITY_INFO and EFFECT_INFO from their own
combat log as separate positional records. No BEGIN_LOG or mode declaration
accompanies them. This corroborates only those observed positional lines, not
v15 or either requested serialization profile. No account names or source log
contents from that discussion were copied into fixtures.

## Passive producer inspection

The owner-directed follow-up also permitted static reads of the installed
`eso64.exe`, without launching it. The exact inspected file was:

- 68,729,125 bytes.
- SHA-256: `256febdff13bc84775728729c6e4513993125a715186254eee6dbea513b479c3`.
- PE image base `0x140000000`.
- `.text`: RVA `0x1000`, raw offset `0x400`; `.rdata`: RVA `0x30ef000`, raw
  offset `0x30ed000`.

The existing ignored research helper uses Capstone 5.0.7 and pefile. Its output
is static file evidence, not a runtime trace or installed-game verification.
The executable is not a repository artifact and was not modified or uploaded.

### Direct label and call-site evidence

The producer's NUL-terminated label pool begins at RVA `0x33b73f8`. It contains
`lineType`, `offsetMS`, `primaryAbilities`, `backupAbilities`, `equipment`,
`longTermEffectAbilityIds`, `effectBarDisplayBehaviour` and the exact misspelling
`grantsSyngeryAbilityId`, among other labels. These bytes differ from some help
descriptions. Mere presence in a pool is not proof that a label appears in every
profile, or proof of its surrounding punctuation.

The readable function at RVA `0x7ef870` supplies stronger local evidence: it
passes equipment field-name pointers such as `slot`, `isCP`, `level`, `trait`,
`quality`, `setId`, `enchantType`, `isEnchantCP`, `enchantLevel` and
`enchantQuality` into the shared routine at `0x7ec1a0`. Some branches use
`enchantCP` instead of `isEnchantCP`. This identifies actual label arguments and
branch differences; it does not justify flattening the branches into one
invented fixed verbose layout.

At `0x7ef8bc` the readable function loads `9`; at `0x7ef8c1` it loads `0x2c`.
It tests a byte at `[owner + 0x178]` at `0x7ef8c6`, selects `0x2c` when that byte
is zero and passes the selected byte to routine `0x4422d0` at `0x7ef8d3`. The
same caller passes `0x5b` immediately afterwards. This is concrete call-site
evidence for a conditional tab/comma value and a following opening bracket.
Calling it a verbose delimiter rule additionally requires proving the flag's
meaning and the callee's write operation. Those links are not established by
this agent's inspection alone.

### Generic writer limitation

The PE exception table identifies the shared routine as the region
`0x7ec1a0-0x7ec28d`. Its RVA maps to raw file offset `0x7eb5a0`. Independent
direct slicing and pefile both return the same first 32 bytes:

`8cbd42357bbd6f35685f87e271603ae46fb216c54c162ee0213168be21ba717538d`

Capstone cannot decode the first instruction there. Several immediate helper
regions also fail to yield coherent instruction sequences. The file mapping
was checked against the section table, the original hash was rechecked and the
same helper correctly decodes the equipment caller. A particular protection or
transformation mechanism was not established; unreadable bytes must not be
interpreted as ordinary instructions or as evidence that the routine is empty.

The static direct-call scan finds fourteen calls to `0x7ec1a0`, all in the one
readable equipment function. It did not find another readable copy that reveals
name/value punctuation. The formatting wrapper at `0x43ec10` is readable and
forwards arguments to `0x431f90`, but that alone supplies no key delimiter or
inline-ability grammar.

## Readable Mac producer writer

A later passive read found a second installed producer whose relevant machine
code is readable. This supersedes the generic-punctuation uncertainty above for
this inspected producer, without changing what could be established from the
Windows file alone:

- File: installed Mac client `eso`, 72,562,448 bytes.
- SHA-256: `208f83b53576bf1a04ecf35e6e4a99779cf157e9e89fbf4457361b0923953d84`.
- The header emitter at VA `0x100648b66` writes native logVersion 15 and the
  game-version components `eso.live.10.3` (independently inspected in the header
  research). Thus this is actual v15 producer code, not an inferred v15 label
  applied to addon API documentation.
- Addresses below are virtual addresses in that exact Mach-O image. The ignored
  static helper reads file bytes and Mach-O tables; no target function was run.

### Separators and named fields

The shared prefix routine at `0x100647eb4` maintains a per-group first-field
sentinel. A first item in a group has no leading separator. A subsequent item
uses one literal byte:

| Condition in the inspected writer | Separator byte | Relevant instructions |
| --- | --- | --- |
| Owner byte at offset `0x180` is zero | comma, `0x2c` | Store at `0x100647fc0`. |
| Owner byte at offset `0x180` is nonzero | tab, `0x09` | Flag test at `0x100647efe`; store at `0x100647f36`. |

The same flag controls names. When it is nonzero and the supplied name pointer
is nonnull/nonempty, the writer appends the literal name followed by two bytes,
colon and ASCII space (`0x3a 0x20`). The literal `: ` is referenced at
`0x100648082` and copied at `0x1006480ec-0x1006480f6`. With a zero flag, no name
or colon is emitted. An empty name remains unnamed even when the flag is set.

This establishes the generic byte rules behind positional comma-separated and
named tab-separated output. Binding the flag to the public verbose getter and
setter belongs to the accompanying profile-control inspection. Neither spaces
around commas nor equals signs occur in this prefix routine.

### Primitive values and literal string bytes

| Writer | Proven output operation |
| --- | --- |
| `0x1006482da` | Prefix, then a float formatted with `%.4f`. |
| `0x100648310` | Prefix, then `T` for nonzero boolean input or `F` for zero. |
| `0x100648428` | Prefix, then one supplied literal byte. |
| `0x1006484ae` | Prefix, then two unsigned quantities formatted with `%u/%u`. |
| `0x100648180` | Prefix, literal double quote, supplied C-string bytes, literal double quote. |
| `0x100647d0c` | Prefix, literal double quote, supplied length-bounded path/string bytes, literal double quote. |
| `0x100648350` | Format a localized string, then delegate to `0x100648180`. |

The quote stores in `0x100648180` are at `0x1006481d8` and `0x100648244`.
Between them, the call at `0x100648201` delegates to `0x100738620`. That helper
uses `_strlen` through import stub `0x10360a082`, followed by a bounded byte
copy through `0x1035dda49`. The copy wrapper calls `_strncpy` through import
stub `0x10360a09a`. These imported names were verified through the Mach-O
indirect-symbol table. There is no quote-doubling, backslash-escape or other
content-rewriting loop in that path. The path writer uses the same copy wrapper
with an explicit length. The formatting wrappers ultimately use `_vsnprintf`
at import stub `0x10360a1ae`.

Accordingly, the source operation is `quote + literal supplied bytes + quote`.
It is not evidence for a CSV escape convention. Any constraints imposed on the
input strings earlier in the game are separate from this writer observation;
the research did not fabricate strings containing quotes or control bytes and
claim that the game produced them.

### Unit state and self-target abbreviation

The wrapper at `0x10064a834` writes `unitId` and calls `0x10064c94e` for the
remaining nine values. That latter routine emits these names in order:

`health`, `magicka`, `stamina`, `ultimate`, `werewolf`, `shield`, `mapNX`,
`mapNY`, `heading`.

The first five are unsigned current/max pairs, shield is unsigned, and the
three coordinate/heading values use `%.4f`. A null unit pointer produces unitId
zero and zero-valued remaining components through the same routines. There is
no unit-state group opener or `sourceUnitState` label in these functions.

The source/target dispatcher at `0x100649b12` first calls the unit writer for
the source. It compares source and target pointers at `0x100649b2a`. Distinct
pointers cause a second complete flat unit state to be written. Equal pointers
instead call the single-byte writer with name `target` and byte `0x2a`, yielding
the self-target abbreviation. In named output that final field is exactly
`target: *`. A distinct target repeats the ten unit-state field names flat; it
does not introduce a named nested target group.

### Bracketed arrays and groups

The separate header/PLAYER_INFO inspection identifies group begin
`0x10064855c` and group end `0x100648664`: begin prefixes its supplied key,
appends `[` and pushes an empty first-field sentinel; end appends `]` and pops
that depth. The unsigned-array helper `0x1006484ec` emits unnamed `%u` values
inside such a group. Consequently, group members use the same conditional tab
or comma separator, and empty-name array values do not acquire invented labels.
Record-specific calls, optional tails and inline ability/effect expansion are
owned by the accompanying layout research.

## Result and evidence boundary

The original attachment and later API mirrors do not supply serialized grammar.
The readable Mac v15 producer now supplies direct evidence for named-field
punctuation, conditional separators, primitive values, literal string wrapping,
flat unit state and self-target abbreviation. These findings replace guesses
with reproducible static call sites and exact byte constants.

This is source-derived grammar research. It is not a recorded run of the game,
an assertion that every producer version has identical optional fields, or an
implemented/tested profile-support claim. The unified contract must combine
these generic rules with the independently inspected event layouts, inline
metadata and option-transition control flow. Preserve original evidence and
state the provenance of derived synthetic fixtures explicitly.
