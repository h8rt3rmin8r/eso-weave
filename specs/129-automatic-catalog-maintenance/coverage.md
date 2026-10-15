# S129 Catalog Source Coverage Audit

Date: 2026-10-15. This audit describes implemented collection paths and explicit
remaining source gaps. Synthetic API doubles prove contract handling; they are
not production definitions or installed-game evidence.

The primary source is the stock [ESO UI API documentation](https://raw.githubusercontent.com/esoui/esoui/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt)
at revision f76cf16c4e5be7b234d15dc7f676febffa64c5bb. Actual IDs, names,
metadata, enum values and relationships enter production only from the running
client's public API results. The addon does not sweep invented ID ranges.

| Catalog group | Implemented source and facts | Scope and remaining gaps |
| --- | --- | --- |
| player-skills | GetNumSkillTypes/GetNumSkillLines/GetSkillLineId/GetNumSkillAbilities; GetSkillAbilityId; GetProgressionSkillProgressionId, GetSpecificSkillAbilityInfo, GetSkillLineProgressionAbilityRank and GetNumPassiveSkillRanks. Typed skill-line/progression/rank/morph identities and all supported edges. | Bounded character-visible lines and known ranks. Passive skills use passive rank counts and no morph calls; crafted skills use their separate adapter. Base and two morph choices are API parameters, never invented ability IDs. No claim that all locked or other-class variants were observed. |
| crafted-abilities | GetNumCraftedAbilities/GetCraftedAbilityIdAtIndex; GetNumScriptsInSlotForCraftedAbility/GetScriptIdAtSlotIndexForCraftedAbility; public names/descriptions/icons. | Bounded public iterator/account view. One shared script entity preserves parents for every observed crafted ability. Universal script description is preferred over a context-dependent description when available. |
| ability-metadata | Real IDs from visible skills and variants plus front/back action bars via GetSlotBoundId; public name/description/icon, duration/cooldown/radius/cast/range. | Opportunistic supported source IDs only. Descriptions and numerical API metadata can depend on the current character. No unbacked ID receives invented metadata, and no global ability sweep is claimed. |
| effects-and-status | GetNumBuffs/GetUnitBuffInfo on player, public ability ID/name/icon and effect/ability/status types. | Opportunistic current player buffs, not every possible effect, proc, debuff or NPC ability. Raw encounter observations remain independent. |
| items-and-gear | Worn/backpack GetBagSize/GetItemLink/GetItemLinkItemId and public name/icon/type/armor/weapon/equip/trait/quality APIs. Actual enum identities become typed classification records, with all observed classification-to-item relationships retained. | Opportunistic current worn/backpack contents. Base item IDs are not falsely called full variant identities. No global item, enchantment or variant enumeration. Zero-valued enum facts remain attributes because normalized entity IDs must be positive. |
| item-sets | GetNextItemSetCollectionId/GetNumItemSetCollectionPieces/GetItemSetCollectionPieceInfo. | Bounded collection set/piece visibility. No claim of complete bonus/enchantment/item-variant metadata. |
| champion-skills | GetNumChampionDisciplines/GetChampionDisciplineId/GetNumChampionDisciplineSkills/GetChampionSkillId and public localized name/description/icon. | Bounded client/account discipline visibility. |
| consumables | Current worn/backpack item links classified by public ITEMTYPE_FOOD/DRINK/POTION/POISON, related back to actual Item IDs. | Opportunistic observed consumable definitions. No complete recipe, reagent, potency or effect registry is inferred. |
| mundus-effects | GetUnitActiveMundusStoneBuffIndices and GetUnitBuffInfo yield actual current Mundus ability IDs, names/icons/descriptions. | Opportunistic active player stones only, not a fabricated list of every stone or every scaled value. |
| companions-races-classes | GetNumClasses/GetClassInfo/GetClassName; current GetUnitRaceId/GetRaceName; GetActiveCompanionDefId/GetCompanionName. | Bounded public class list/current race/active companion. No global race/companion coverage inferred from a character view. |
| combat-statistics | Reviewed public STAT_* enum values and available GetString(SI_DERIVEDSTATS, id) names. | Bounded named stat definitions. Character's changing stat values are not converted into global Catalog facts. Unsupported enum names are omitted. |
| constants | One API-version-scoped Constant record keyed by actual GetAPIVersion, containing actual reviewed public resource/hotbar/scribing/item enum values. | Explicit version-scoped public-constant bundle. Enum families do not receive fabricated globally unique numeric IDs. Not an exhaustive registry. |
| localized-text | Importer derives locale-specific name/description records from accepted source facts. | Local generated evidence, with retained locales/scopes and deterministic conflict handling. No locale is authority to delete another locale. |
| icon-references | Importer derives virtual path references supplied by public source APIs. | Opportunistic references, with no claim that a DDS file was extracted or that its bytes are available. |
| icon-bytes | No binary icon acquisition path added. | Unknown. The automatic collector cannot obtain game-owned image bytes through the public definition API. Existing reference consumers keep their fallback behavior. |

Every source envelope declares actual game/API/channel/locale/scope identity,
per-group record counts, and bounded/opportunistic/unknown completeness. Missing
API views produce unknown declarations and bounded diagnostics. Category
presence never asserts exhaustive game visibility. No native encounter logger is
needed to enumerate these independent sources.

## Generation and relationship contract

The completed version-1 envelope remains under EsoWeaveDataSaved.catalog.
Its global name does not choose the disk filename: PixelBeacon.txt reads/saves
SavedVariables/PixelBeacon.lua, while legacy SavedVariables/EsoWeaveData.lua stays
unchanged for separate desktop Catalog and encounter readers. The new Lua addon
does not load the legacy file. Compatible legacy/current facts reconcile in
application-owned accumulation, preserving old usable definitions while a fresh
current-file generation runs. No game save is copied, moved, or rewritten. New
captures retain the first parent field for compatibility and provide the full
deduplicated parents array. Import validates each referenced typed entity and
preserves every edge. Source positions are version-scoped ordering facts, not
replacements for stable API IDs. Existing supported legacy checksum captures
remain accepted; arbitrary collector implementations do not become recognized
simply by declaring a version.

Runtime work uses bounded adapters, incremental merge sorting and coroutine JSON
encoding/checksum slices. Only a completed generation replaces the prior table.
Reloads restart the bounded in-memory generation automatically. Unknown schema
roots remain untouched, and transient failures retry without deleting usable
snapshots. RequestAddOnSavedVariablesPrioritySave requests game save priority;
it never proves bytes reached disk. Desktop stable-file evidence and immutable
activation are verified separately from Lua execution.

## Verification evidence

The automatic Lua harness feeds actual emitted SavedVariables into the strict
Rust parser. Its manifest-named file model derives the output filename from the
actual deployed manifest stem, checks both desktop input paths, preserves the
legacy file bytes, and roundtrips the current envelope/reload. It is a contract
model, not evidence that a game save ran. Other cases check populated records in all twelve source groups, validates a
shared script's two parents, and exercises base/morph/passive ranks, crafted
skill separation, readiness, combat/loading, fresh load order, restart/retry,
quiet operation, natural-load optout, Live/PTS identity, repeated source noise,
large payload budgets and failure during serialization. Existing legacy recorder
fixtures preserve historical read-format regressions without loading that
recorder in the production package.

Compiler/service regressions separately start from the real packaged baseline,
inspect useful entities/relations/localization, accumulate independent scopes and
locales, preserve API history, reject false/genuine coverage removals correctly,
and distinguish semantic no-op from acquisition lineage. These tests do not
establish installed-game behavior, exhaustive source coverage or save latency.
