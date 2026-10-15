# Native v15 Grammar and Compatibility Contract

Date: 2026-10-15. Applies to S131, issues #296 and #297.

## Evidence threshold

The inspected installed producer is the 72,562,448-byte x86_64 Mach-O ESO
executable, SHA-256
`208f83b53576bf1a04ecf35e6e4a99779cf157e9e89fbf4457361b0923953d84`.
Its BEGIN_LOG writer `0x100648b66` writes literal logVersion 15 and gameVersion
`eso.live.10.3`. Addresses are unslid virtual addresses. This is static writer
and call-site evidence, not a game execution trace. No binary is redistributed.
Detailed independent receipts are in [header/profile evidence](../research-producer-header.md),
[primitive evidence](../research-producer.md) and
[ability/effect evidence](../research-producer-abilities.md).

The [official uploader inspection](../research-official-uploader.md) pins current
and historical publisher assets. Its current parser endpoint requires login;
the [public CDN parser](../research-parser-resource.md) is historical. Neither
establishes current inline/verbose syntax. Synthetic tests below derive from
the static producer and cannot substitute for that evidence.

## Common byte rules

- Normal records separate fields with ASCII comma. Verbose records separate
  fields with literal TAB and prefix each nonempty field name with exact
  `name: ` (colon, one ASCII space). Unnamed array members have no label.
- The common fields are `offsetMS` (unsigned decimal) and `lineType` (enum token).
  Names and order are exact; repeated names in flat states are intentional.
- Bracket groups `[...]` have their own first-field sentinel. Their members use
  the same selected comma/TAB delimiter, independently at every nesting depth.
- Strings are double-quote, raw string bytes, double-quote. The writer does not
  double quotes or apply backslash escaping. Literal backslashes and delimiters
  inside a quoted string are data. Ambiguous embedded quotes/control newlines
  are not assigned an invented escape interpretation.
- Booleans are `T`/`F`; ordinary IDs are unsigned decimal. Producer `%d` fields
  include cast duration and combat hit/overflow; preserve signs. powerType also
  uses `%d` but this producer zero-extends its byte argument (0..255).
  Unit health/magicka/stamina/ultimate/werewolf are unsigned `current/maximum`;
  shield is unsigned and mapNX/mapNY/heading are `%.4f`.
- Verbose and inline flags are independent. Setters `0x100648fe8` and
  `0x100648ff6` only store a byte and emit no marker/boundary. Decode locally
  for every record, including profile changes inside a fight. BEGIN_LOG carries
  no profile declaration. Enabling/disabling logging emits BEGIN_LOG/END_LOG.

## Ordered field layouts

Every row follows the two common fields. `U` expands to the ten flat fields
`unitId,health,magicka,stamina,ultimate,werewolf,shield,mapNX,mapNY,heading`.
`TGT` is another full U or a single `target: *` in verbose (`*` in normal).
There are no invented source/target state objects.

`A` is the four-field inline ability tail
`name,iconPath,interruptible,blockable`, after the numeric ability ID.
Null ability still emits `"","",F,F`. `E` is
`effectType,statusEffectType,effectBarDisplayBehaviour`, optionally followed by
the exact producer spelling `grantsSyngeryAbilityId` (unsigned, including zero).
Concrete E values are enum words; null E is exactly three quoted empty strings.
Synergy IDs remain scalar. Inline expansion never adds a second ability ID.

| Kind | Ordered payload |
| --- | --- |
| BEGIN_LOG | timeSinceEpochMS,logVersion,realmName,language,gameVersion |
| END_LOG / BEGIN_COMBAT / END_COMBAT | none |
| COMBAT_EVENT | actionResult,damageType,powerType,hitValue,overflow,castTrackId,abilityId,[A],U,TGT |
| BEGIN_CAST | durationMS,channeled,castTrackId,abilityId,[A],U,TGT |
| END_CAST | endReason,castTrackId,abilityId,[A]; if INTERRUPTED append interruptingAbilityId,[A],interruptingUnitId |
| EFFECT_CHANGED | changeType,stackCount,castTrackId,abilityId,[A,E],U,TGT; optional playerInitiatedRemoveCastTrackId after state, for remove events |
| HEALTH_REGEN | effectiveRegen,U |
| UNIT_ADDED | unitId,unitType,isLocalPlayer,playerPerSessionId,monsterId,isBoss,classId,raceId,name,displayName,characterId,level,championPoints,ownerUnitId,reaction,isGroupedWithLocalPlayer |
| UNIT_CHANGED | unitId,classId,raceId,name,displayName,characterId,level,championPoints,ownerUnitId,reaction,isGroupedWithLocalPlayer |
| UNIT_REMOVED | unitId |
| ABILITY_INFO | abilityId,name,iconPath,interruptible,blockable; scribed form appends quoted primaryScript,secondaryScript,tertiaryScript |
| EFFECT_INFO | abilityId,E (no A tail even in inline mode) |
| PLAYER_INFO | unitId,longTermEffectAbilityIds,longTermEffectStackCounts,equipment,primaryAbilities,backupAbilities |
| MAP_CHANGED | id,name,texturePath |
| ZONE_CHANGED | id,name,dungeonDifficulty |
| TRIAL_INIT | id,inProgress,completed,startTimeMS,durationMS,success,finalScore |
| BEGIN_TRIAL | id,startTimeMS |
| END_TRIAL | id,durationMS,success,finalScore,finalVitalityBonus |
| ENDLESS_DUNGEON_INIT / ENDLESS_DUNGEON_BEGIN | id,startTimeMS,solo |
| ENDLESS_DUNGEON_END | id,durationMS,finalScore,solo |
| ENDLESS_DUNGEON_STAGE_END | id,durationMS |
| ENDLESS_DUNGEON_BUFF_ADDED / ENDLESS_DUNGEON_BUFF_REMOVED | id,abilityId (no expansion) |

The A tail is either present for every eligible ability reference on that
record or absent. EFFECT_CHANGED inline adds both A and E. Optional E synergy
and final remove-track ID are independent and resolved at typed positions.
The producer condition for synergy is ability byte +0x320 = 0x4c and flag
+0x1ea & 0x20, not nonzero synergy ID. Event A tails never include scripts.
PLAYER_INFO may emit standalone ability/effect metadata even with inline on.

PLAYER_INFO ID/count arrays use unnamed unsigned elements in every profile;
effects and stack-count arrays have matching lengths. Equipment is an unnamed
array of eleven-field bracket entries:
`slot,id,isCP,level,trait,quality,setId,enchantType,isEnchantCP,enchantLevel,enchantQuality`.
For the no-enchantment branch the ninth label is instead exactly `enchantCP`,
with suffix values `INVALID,F,0,NORMAL`. Other aliases are not invented.

## Semantic and compatibility contract

The schema 2 decoder validates names and added tails before exposing the shared
semantic fields. It retains the original physical record separately and never
serializes a normalized line as evidence. Header decoding, context selection,
canonical validation, projection and checkpoint restoration use the same view.
No run-global format state or new checkpoint field is required.

Schema 1 remains the frozen normal/separate batch parser. Existing schema 2
normal/separate accepted escaping and shapes remain compatible; they are not
claimed as producer escaping. Previously accepted normal strings keep that
interpretation. Previously rejected literal strings may use the producer
decoder, including standalone metadata coexisting with inline. Added inline
tails and verbose profiles use the literal producer grammar.
Normal headers have no inline bit and keep the established string interpretation
for identity compatibility. Verbose headers use literal strings. Raw headers
are retained in either case. Old classification of trial records as unknown
remains unchanged, as do unknown counts in canonical fingerprints. MAP_INFO
and ZONE_INFO remain legacy compatibility names, not newly observed emitters.

Malformed known labels/layouts cannot fall back to opaque unknown records.
Unknown kinds retain original evidence after valid common fields and bounded
lexical framing. Unsupported log versions retain a distinct diagnostic.
Malformed segments recover only at a fresh valid BEGIN_LOG. All existing
line/token/depth/context/fight/canonical/checkpoint bounds remain enforced.

No new grammar changes raw storage shape, logical run/fight IDs, physical
offsets, schema 7 transactions, copied-run deduplication or deletion suppression.
Different raw formats under the same logical key remain divergent evidence.
