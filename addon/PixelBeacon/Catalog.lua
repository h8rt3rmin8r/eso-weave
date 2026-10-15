local ADDON_NAME = "PixelBeacon"
local MODULE_NAMESPACE = ADDON_NAME .. "Catalog"
local COLLECTOR_VERSION = 1
local SCHEMA_VERSION = 1
local COLLECTOR_CHECKSUM = "4c4655d942c94689e66d181bfd59d1f8409ea624eea34eeae31c9f1515237647"

local MAX_STEPS_PER_TICK = 64
local MAX_MILLISECONDS_PER_TICK = 4
local MAX_SERIALIZE_BYTES_PER_TICK = 4096
local MAX_SNAPSHOT_BYTES = 67108864
local MAX_RECORDS = 500000
local MAX_STRING_BYTES = 65536
local MAX_CHUNK_BYTES = 65536
local MAX_CHUNKS = 1024
local MAX_PARENTS = 4096
local RETRY_MS = 30000
local REFRESH_COALESCE_MS = 1000
local SAVE_REQUEST_MS = 30000
local UPDATE_NAME = MODULE_NAMESPACE .. "Update"

local CATEGORY_ORDER = {
    "player-skills", "crafted-abilities", "ability-metadata", "effects-and-status",
    "items-and-gear", "item-sets", "champion-skills", "consumables", "mundus-effects",
    "companions-races-classes", "combat-statistics", "constants",
}
local LIMITATIONS = {
    ["player-skills"] = "Character-visible skill lines and public progression, base/morph and known-rank API views; not every character or locked variant.",
    ["crafted-abilities"] = "Public crafted-ability/script iterator visibility; repeated scripts retain all supported parents.",
    ["ability-metadata"] = "Ability IDs supplied by visible skills, variants and action bars; character-scaled public metadata, not a global ID sweep.",
    ["effects-and-status"] = "Currently visible player buff ability IDs and public effect/status classifications; not all possible effects.",
    ["items-and-gear"] = "Current worn/backpack item IDs and public type metadata; not all items, variants, enchantments or binary icons.",
    ["item-sets"] = "Public account collection set and piece iterators; not all set bonuses or item variants.",
    ["champion-skills"] = "Public visible discipline and champion-skill iterators.",
    ["consumables"] = "Current worn/backpack items classified by public food, drink, potion or poison item type; not all recipes or effects.",
    ["mundus-effects"] = "Current player Mundus buff indices and their public ability IDs; not unobserved stones.",
    ["companions-races-classes"] = "Public class iterator, current race and active companion; no invented unobserved identities.",
    ["combat-statistics"] = "Reviewed public derived-stat enum identities and available localized names; not transient player stat values.",
    ["constants"] = "API-version-scoped bundle of actual reviewed public enum values; not a universal constant registry.",
}
local records, runtime, known, completedInSession = {}, nil, {}, nil
local active, loaded, disabled, channel = false, false, true, nil
local dirty, nextAttempt, lastSaveRequest, savePending = true, 0, -SAVE_REQUEST_MS, false
local identity, diagnostics = nil, { state = "waiting", reason = nil }
PixelBeaconCatalogDiagnostics = diagnostics

local function now() return GetGameTimeMilliseconds() end
local function available(name) return type(_G[name]) == "function" end
local function optional(name, ...)
    if available(name) then return _G[name](...) end
end
local function integer(value) return type(value) == "number" and value == math.floor(value) and math.abs(value) <= 9007199254740991 end
local function validRoot()
    return type(EsoWeaveDataSaved) == "table" and EsoWeaveDataSaved.schema_version == 1
        and EsoWeaveDataSaved.addon_version == 1
end
local function setState(state, reason)
    diagnostics.state, diagnostics.reason = state, reason
end
local function warning(text)
    for _, existing in ipairs(runtime.envelope.warnings) do if existing == text then return end end
    if #runtime.envelope.warnings < 64 then table.insert(runtime.envelope.warnings, text) end
end
local function unavailable(category)
    runtime.completeness[category] = "unknown"
    warning(category .. " public API view is unavailable")
end
local function readIdentity()
    return table.concat({ channel or "", tostring(GetAPIVersion()), tostring(optional("GetESOVersionString") or "unknown"),
        tostring(GetCVar("language.2")), tostring(optional("GetCurrentCharacterId") or "unknown") }, "|")
end
local function fail(reason)
    runtime, records = nil, {}
    dirty, nextAttempt = true, now() + RETRY_MS
    setState("retry-wait", reason)
end

local function measure(value)
    if type(value) == "string" then
        if #value > MAX_STRING_BYTES then error("string-limit") end
        return #value
    elseif type(value) == "table" then
        local bytes = 0
        for key, entry in pairs(value) do bytes = bytes + measure(key) + measure(entry) end
        return bytes
    end
    return 16
end
local function addParent(record, parent)
    if not parent then return end
    local key = parent.relation .. "/" .. parent.kind .. "/" .. tostring(parent.stable_id)
    if record.parent_keys[key] then return end
    if #record.parents >= MAX_PARENTS then error("parent-limit") end
    record.parent_keys[key] = true
    table.insert(record.parents, parent)
    record.parent = record.parent or parent
    runtime.estimated_bytes = runtime.estimated_bytes + measure(parent)
end
local function addRecord(category, kind, stableId, fields)
    if not integer(stableId) or stableId <= 0 then return end
    fields = fields or {}
    local key = kind .. "/" .. tostring(stableId)
    local existing = runtime.seen[key]
    if existing then
        addParent(existing, fields.parent)
        for _, parent in ipairs(fields.parents or {}) do addParent(existing, parent) end
        for name, value in pairs(fields.attributes or {}) do
            if existing.attributes[name] == nil then
                existing.attributes[name] = value
                runtime.estimated_bytes = runtime.estimated_bytes + measure(name) + measure(value)
            end
        end
        for _, name in ipairs({ "name", "description", "icon_path" }) do
            if (existing[name] == nil or existing[name] == "") and fields[name] then
                existing[name] = fields[name]
                runtime.estimated_bytes = runtime.estimated_bytes + measure(fields[name])
            end
        end
    else
        if #records >= MAX_RECORDS then error("record-limit") end
        local parent, suppliedParents = fields.parent, fields.parents
        fields.category, fields.kind, fields.stable_id = category, kind, stableId
        fields.source_key = category .. "/" .. key
        fields.sort_key = fields.source_key
        fields.attributes = fields.attributes or {}
        fields.parents, fields.parent_keys, fields.parent = {}, {}, nil
        runtime.estimated_bytes = runtime.estimated_bytes + measure(fields)
        addParent(fields, parent)
        for _, extra in ipairs(suppliedParents or {}) do addParent(fields, extra) end
        runtime.seen[key] = fields
        runtime.known[key] = true
        table.insert(records, fields)
        runtime.category_counts[category] = (runtime.category_counts[category] or 0) + 1
    end
    if runtime.estimated_bytes > MAX_SNAPSHOT_BYTES then error("snapshot-limit") end
end
local function ability(stableId, parent, attributes)
    if not integer(stableId) or stableId <= 0 then return end
    attributes = attributes or {}
    for _, entry in ipairs({ {"duration_ms", "GetAbilityDuration"}, {"radius_cm", "GetAbilityRadius"} }) do
        local value = optional(entry[2], stableId, nil, "player")
        if integer(value) then attributes[entry[1]] = value end
    end
    local cooldown = optional("GetAbilityCooldown", stableId, "player")
    if integer(cooldown) then attributes.cooldown_ms = cooldown end
    local channeled, duration = optional("GetAbilityCastInfo", stableId, nil, "player")
    if type(channeled) == "boolean" then attributes.channeled = channeled end
    if integer(duration) then attributes.cast_ms = duration end
    local minimum, maximum = optional("GetAbilityRange", stableId, nil, "player")
    if integer(minimum) then attributes.min_range_cm = minimum end
    if integer(maximum) then attributes.max_range_cm = maximum end
    addRecord("ability-metadata", "ability", stableId, { parent = parent, attributes = attributes,
        name = optional("GetAbilityName", stableId, "player"),
        description = optional("GetAbilityDescription", stableId, nil, "player"),
        icon_path = optional("GetAbilityIcon", stableId) })
end

local function playerSkillsStep(cursor)
    if not available("GetNumSkillTypes") or not available("GetNumSkillLines")
        or not available("GetSkillLineId") or not available("GetNumSkillAbilities")
        or not available("GetSkillAbilityId") then unavailable("player-skills"); return true end
    cursor.skill_type, cursor.skill_line, cursor.skill = cursor.skill_type or 1, cursor.skill_line or 1, cursor.skill or 0
    if cursor.skill_type > GetNumSkillTypes() then return true end
    if cursor.skill_line > GetNumSkillLines(cursor.skill_type) then
        cursor.skill_type, cursor.skill_line, cursor.skill = cursor.skill_type + 1, 1, 0
        return false
    end
    local lineId = GetSkillLineId(cursor.skill_type, cursor.skill_line)
    if cursor.skill == 0 then
        cursor.skill, cursor.phase = 1, "current"
        addRecord("player-skills", "skill-line", lineId, { attributes = {
            skill_type_source_index = cursor.skill_type, version_scoped_order = cursor.skill_line },
            name = optional("GetSkillLineNameById", lineId) })
        return false
    end
    if cursor.skill > GetNumSkillAbilities(cursor.skill_type, cursor.skill_line) then
        cursor.skill_line, cursor.skill = cursor.skill_line + 1, 0
        return false
    end
    local lineParent = { relation = "skill-line-has-ability", kind = "skill-line", stable_id = lineId }
    if cursor.phase == "current" then
        ability(GetSkillAbilityId(cursor.skill_type, cursor.skill_line, cursor.skill, false), lineParent,
            { version_scoped_order = cursor.skill })
        if optional("IsCraftedAbilitySkill", cursor.skill_type, cursor.skill_line, cursor.skill) == true then
            cursor.skill, cursor.phase = cursor.skill + 1, "current"
            return false
        end
        cursor.progression = optional("GetProgressionSkillProgressionId", cursor.skill_type, cursor.skill_line, cursor.skill)
        if integer(cursor.progression) and cursor.progression > 0 then
            addRecord("player-skills", "ability-progression", cursor.progression, { parent = {
                relation = "skill-line-has-progression", kind = "skill-line", stable_id = lineId } })
        end
        cursor.passive = optional("IsSkillAbilityPassive", cursor.skill_type, cursor.skill_line, cursor.skill) == true
        cursor.max_morph = cursor.passive and 0 or 2
        cursor.morph, cursor.rank, cursor.phase = 0, 1, "variant"
        return false
    end
    if not available("GetSpecificSkillAbilityInfo") then
        warning("player-skills variant API is unavailable; only currently visible ability IDs are retained")
        cursor.skill, cursor.phase = cursor.skill + 1, "current"
        return false
    end
    local maxRank
    if cursor.passive then maxRank = optional("GetNumPassiveSkillRanks", cursor.skill_type, cursor.skill_line, cursor.skill)
    else maxRank = optional("GetSkillLineProgressionAbilityRank", cursor.skill_type, cursor.skill_line, cursor.skill, cursor.morph) end
    if not integer(maxRank) or maxRank < 1 then maxRank = 1 end
    if maxRank > 64 then error("rank-limit") end
    if cursor.rank > maxRank then
        cursor.morph, cursor.rank, cursor.morph_id = cursor.morph + 1, 1, nil
        if cursor.morph > cursor.max_morph then cursor.skill, cursor.phase = cursor.skill + 1, "current" end
        return false
    end
    local variant = GetSpecificSkillAbilityInfo(cursor.skill_type, cursor.skill_line, cursor.skill, cursor.morph, cursor.rank)
    if integer(variant) and variant > 0 then
        local progressionParent = lineParent
        if integer(cursor.progression) and cursor.progression > 0 then
            progressionParent = { relation = "progression-has-morph", kind = "ability-progression", stable_id = cursor.progression }
        end
        if cursor.rank == 1 and not cursor.passive then
            cursor.morph_id = variant
            addRecord("player-skills", "ability-morph", variant, { parent = progressionParent,
                attributes = { morph_choice = cursor.morph }, name = optional("GetAbilityName", variant, "player") })
        end
        local rankParent = cursor.morph_id and { relation = "morph-has-rank", kind = "ability-morph", stable_id = cursor.morph_id } or lineParent
        addRecord("player-skills", "ability-rank", variant, { parent = rankParent,
            attributes = { rank = cursor.rank, morph_choice = cursor.morph } })
        ability(variant, { relation = "rank-has-ability", kind = "ability-rank", stable_id = variant })
    end
    cursor.rank = cursor.rank + 1
    return false
end

local function craftedAbilitiesStep(cursor)
    if not available("GetNumCraftedAbilities") or not available("GetCraftedAbilityIdAtIndex") then unavailable("crafted-abilities"); return true end
    cursor.index, cursor.slot, cursor.script = cursor.index or 1, cursor.slot or 0, cursor.script or 1
    if cursor.index > GetNumCraftedAbilities() then return true end
    local id = GetCraftedAbilityIdAtIndex(cursor.index)
    if cursor.slot == 0 then
        cursor.slot = 1
        addRecord("crafted-abilities", "crafted-ability", id, { name = optional("GetCraftedAbilityDisplayName", id),
            description = optional("GetCraftedAbilityDescription", id), icon_path = optional("GetCraftedAbilityIcon", id) })
        return false
    end
    local slots = { SCRIBING_SLOT_PRIMARY, SCRIBING_SLOT_SECONDARY, SCRIBING_SLOT_TERTIARY }
    if cursor.slot <= #slots and available("GetNumScriptsInSlotForCraftedAbility") and available("GetScriptIdAtSlotIndexForCraftedAbility") then
        local slot = slots[cursor.slot]
        if cursor.script <= GetNumScriptsInSlotForCraftedAbility(id, slot) then
            local script = GetScriptIdAtSlotIndexForCraftedAbility(id, slot, cursor.script)
            cursor.script = cursor.script + 1
            addRecord("crafted-abilities", "script", script, { parent = {
                relation = "crafted-ability-has-script", kind = "crafted-ability", stable_id = id },
                attributes = { scribing_slot = slot }, name = optional("GetCraftedAbilityScriptDisplayName", script),
                description = optional("GetCraftedAbilityScriptGeneralDescription", script)
                    or optional("GetCraftedAbilityScriptDescription", id, script), icon_path = optional("GetCraftedAbilityScriptIcon", script) })
            return false
        end
        cursor.slot, cursor.script = cursor.slot + 1, 1
        return false
    end
    if not available("GetNumScriptsInSlotForCraftedAbility") then warning("crafted-ability script API is unavailable") end
    cursor.index, cursor.slot, cursor.script = cursor.index + 1, 0, 1
    return false
end

local function itemSetsStep(cursor)
    if not available("GetNextItemSetCollectionId") or not available("GetNumItemSetCollectionPieces") or not available("GetItemSetCollectionPieceInfo") then unavailable("item-sets"); return true end
    if cursor.set_id and cursor.piece <= cursor.piece_count then
        local piece = select(1, GetItemSetCollectionPieceInfo(cursor.set_id, cursor.piece))
        cursor.piece = cursor.piece + 1
        addRecord("item-sets", "item-set-piece", piece, { parent = {
            relation = "item-set-has-piece", kind = "item-set", stable_id = cursor.set_id } })
        return false
    end
    local id = GetNextItemSetCollectionId(cursor.last_id)
    if id == nil or id == 0 then return true end
    if not integer(id) or id <= 0 or id == cursor.last_id then error("set-iterator") end
    cursor.last_id, cursor.set_id, cursor.piece, cursor.piece_count = id, id, 1, GetNumItemSetCollectionPieces(id)
    addRecord("item-sets", "item-set", id, { attributes = { visible_piece_count = cursor.piece_count } })
    return false
end
local function championSkillsStep(cursor)
    if not available("GetNumChampionDisciplines") or not available("GetChampionDisciplineId")
        or not available("GetNumChampionDisciplineSkills") or not available("GetChampionSkillId") then unavailable("champion-skills"); return true end
    cursor.discipline, cursor.skill = cursor.discipline or 1, cursor.skill or 0
    if cursor.discipline > GetNumChampionDisciplines() then return true end
    local id = GetChampionDisciplineId(cursor.discipline)
    if cursor.skill == 0 then
        cursor.skill = 1
        addRecord("champion-skills", "collection-category", id, { name = optional("GetChampionDisciplineName", id) })
        return false
    end
    if cursor.skill > GetNumChampionDisciplineSkills(cursor.discipline) then
        cursor.discipline, cursor.skill = cursor.discipline + 1, 0
        return false
    end
    local skill = GetChampionSkillId(cursor.discipline, cursor.skill)
    cursor.skill = cursor.skill + 1
    addRecord("champion-skills", "champion-skill", skill, { parent = { relation = "discipline-has-champion-skill", kind = "collection-category", stable_id = id },
        name = optional("GetChampionSkillName", skill), description = optional("GetChampionSkillDescription", skill, 0), icon_path = optional("GetChampionSkillIcon", skill) })
    return false
end
local function identitiesStep(cursor)
    cursor.index = cursor.index or 1
    if available("GetNumClasses") and available("GetClassInfo") and cursor.index <= GetNumClasses() then
        local id, lore, icon = GetClassInfo(cursor.index)
        cursor.index = cursor.index + 1
        addRecord("companions-races-classes", "class", id, { name = optional("GetClassName", GENDER_MALE or 0, id), description = lore, icon_path = icon })
        return false
    end
    if not cursor.race_done then
        cursor.race_done = true
        local id = optional("GetUnitRaceId", "player")
        addRecord("companions-races-classes", "race", id, { name = optional("GetRaceName", GENDER_MALE or 0, id), attributes = { visibility = "current-character" } })
        return false
    end
    if not cursor.companion_done then
        cursor.companion_done = true
        local id = optional("GetActiveCompanionDefId")
        addRecord("companions-races-classes", "companion", id, { attributes = { visibility = "active-companion" }, name = optional("GetCompanionName", id) })
        return false
    end
    if not available("GetNumClasses") then unavailable("companions-races-classes") end
    return true
end
local function abilityMetadataStep(cursor)
    cursor.bar, cursor.slot = cursor.bar or 1, cursor.slot or 3
    local bars = { HOTBAR_CATEGORY_PRIMARY, HOTBAR_CATEGORY_BACKUP }
    if not available("GetSlotBoundId") or not bars[cursor.bar] then return true end
    ability(GetSlotBoundId(cursor.slot, bars[cursor.bar]), nil, { visibility = "current-action-bars" })
    cursor.slot = cursor.slot + 1
    if cursor.slot > 8 then cursor.bar, cursor.slot = cursor.bar + 1, 3 end
    return false
end
local function nextItem(cursor, category)
    if not available("GetBagSize") or not available("GetItemLink") or not available("GetItemLinkItemId") then unavailable(category); return nil, true end
    cursor.bag, cursor.slot = cursor.bag or 1, cursor.slot or 0
    local bags = { BAG_WORN, BAG_BACKPACK }
    local bag = bags[cursor.bag]
    if not bag then return nil, true end
    if cursor.slot >= GetBagSize(bag) then cursor.bag, cursor.slot = cursor.bag + 1, 0; return nil, false end
    local link = GetItemLink(bag, cursor.slot, LINK_STYLE_DEFAULT or 0)
    cursor.slot = cursor.slot + 1
    if type(link) ~= "string" or link == "" then return nil, false end
    return link, false
end
local function itemFields(link)
    local attributes = { visibility = "current-worn-and-backpack" }
    for _, entry in ipairs({ {"item_type", "GetItemLinkItemType"}, {"armor_type", "GetItemLinkArmorType"},
        {"weapon_type", "GetItemLinkWeaponType"}, {"equipment_type", "GetItemLinkEquipType"},
        {"quality", "GetItemLinkDisplayQuality"}, {"trait_type", "GetItemLinkTraitInfo"} }) do
        local value = optional(entry[2], link)
        if integer(value) then attributes[entry[1]] = value end
    end
    return { attributes = attributes, name = optional("GetItemLinkName", link), icon_path = optional("GetItemLinkIcon", link) }
end
local ITEM_TYPES = {
    { "armor_type", "armor-type", "SI_ARMORTYPE" }, { "weapon_type", "weapon-type", "SI_WEAPONTYPE" },
    { "equipment_type", "equipment-type", "SI_EQUIPTYPE" }, { "quality", "quality", "SI_ITEMQUALITY" },
    { "trait_type", "trait", "SI_ITEMTRAITTYPE" },
}
local function itemsStep(cursor)
    local link, done = nextItem(cursor, "items-and-gear")
    if link then
        local fields = itemFields(link)
        fields.parents = {}
        for _, entry in ipairs(ITEM_TYPES) do
            local value = fields.attributes[entry[1]]
            if integer(value) and value > 0 then
                addRecord("items-and-gear", entry[2], value, { attributes = { enum_family = entry[3] },
                    name = _G[entry[3]] and optional("GetString", _G[entry[3]], value) or nil })
                table.insert(fields.parents, { relation = entry[2] .. "-describes-item", kind = entry[2], stable_id = value })
            end
        end
        addRecord("items-and-gear", "item", GetItemLinkItemId(link), fields)
    end
    return done
end
local function consumablesStep(cursor)
    local link, done = nextItem(cursor, "consumables")
    if link and available("GetItemLinkItemType") then
        local itemType = GetItemLinkItemType(link)
        local kind
        if itemType == ITEMTYPE_FOOD or itemType == ITEMTYPE_DRINK then kind = "food-drink"
        elseif itemType == ITEMTYPE_POTION then kind = "potion"
        elseif itemType == ITEMTYPE_POISON then kind = "poison" end
        if kind then
            local id = GetItemLinkItemId(link)
            local fields = itemFields(link)
            fields.parent = { relation = "item-has-consumable-definition", kind = "item", stable_id = id }
            addRecord("consumables", kind, id, fields)
        end
    end
    return done
end
local function effectRecord(index, category, kind)
    local name, _, _, _, _, icon, _, effectType, abilityType, statusType, id = GetUnitBuffInfo("player", index)
    local attributes = { visibility = "current-player-buffs" }
    if integer(effectType) then attributes.effect_type = effectType end
    if integer(abilityType) then attributes.ability_type = abilityType end
    if integer(statusType) then attributes.status_effect_type = statusType end
    addRecord(category, kind, id, { attributes = attributes, name = name, icon_path = icon,
        description = integer(id) and optional("GetAbilityDescription", id, nil, "player") or nil })
end
local function effectsStep(cursor)
    if not available("GetNumBuffs") or not available("GetUnitBuffInfo") then unavailable("effects-and-status"); return true end
    cursor.index = cursor.index or 1
    if cursor.index > GetNumBuffs("player") then return true end
    effectRecord(cursor.index, "effects-and-status", "effect")
    cursor.index = cursor.index + 1
    return false
end
local function mundusStep(cursor)
    if not available("GetUnitActiveMundusStoneBuffIndices") or not available("GetUnitBuffInfo") then unavailable("mundus-effects"); return true end
    cursor.indices, cursor.index = cursor.indices or { GetUnitActiveMundusStoneBuffIndices("player") }, cursor.index or 1
    if cursor.index > #cursor.indices then return true end
    effectRecord(cursor.indices[cursor.index], "mundus-effects", "mundus")
    cursor.index = cursor.index + 1
    return false
end
local STAT_NAMES = { "STAT_ARMOR_RATING", "STAT_ATTACK_POWER", "STAT_BLOCK", "STAT_CRITICAL_CHANCE", "STAT_CRITICAL_RESISTANCE", "STAT_CRITICAL_STRIKE",
    "STAT_DODGE", "STAT_HEALING_DONE", "STAT_HEALING_TAKEN", "STAT_HEALTH_MAX", "STAT_HEALTH_REGEN_COMBAT", "STAT_HEALTH_REGEN_IDLE",
    "STAT_MAGICKA_MAX", "STAT_MAGICKA_REGEN_COMBAT", "STAT_MAGICKA_REGEN_IDLE", "STAT_MITIGATION", "STAT_OFFENSIVE_PENETRATION",
    "STAT_PHYSICAL_PENETRATION", "STAT_PHYSICAL_RESIST", "STAT_POWER", "STAT_SPELL_CRITICAL", "STAT_SPELL_MITIGATION", "STAT_SPELL_PENETRATION",
    "STAT_SPELL_POWER", "STAT_SPELL_RESIST", "STAT_STAMINA_MAX", "STAT_STAMINA_REGEN_COMBAT", "STAT_STAMINA_REGEN_IDLE", "STAT_WEAPON_AND_SPELL_DAMAGE" }
local function statisticsStep(cursor)
    cursor.index = cursor.index or 1
    local key = STAT_NAMES[cursor.index]
    if not key then
        if not runtime.category_counts["combat-statistics"] then unavailable("combat-statistics") end
        return true
    end
    cursor.index = cursor.index + 1
    local id = _G[key]
    addRecord("combat-statistics", "combat-stat", id, { attributes = { enum_name = key }, name = integer(id) and SI_DERIVEDSTATS and optional("GetString", SI_DERIVEDSTATS, id) or nil })
    return false
end
local CONSTANT_NAMES = { "POWERTYPE_HEALTH", "POWERTYPE_MAGICKA", "POWERTYPE_STAMINA", "POWERTYPE_ULTIMATE", "HOTBAR_CATEGORY_PRIMARY", "HOTBAR_CATEGORY_BACKUP",
    "SCRIBING_SLOT_PRIMARY", "SCRIBING_SLOT_SECONDARY", "SCRIBING_SLOT_TERTIARY", "ITEMTYPE_FOOD", "ITEMTYPE_DRINK", "ITEMTYPE_POTION", "ITEMTYPE_POISON" }
local function constantsStep(cursor)
    if cursor.done then return true end
    cursor.done = true
    local attributes = { api_version = GetAPIVersion(), identity_scope = "api-version-public-enum-bundle" }
    for _, key in ipairs(CONSTANT_NAMES) do if integer(_G[key]) then attributes[key] = _G[key] end end
    addRecord("constants", "constant", GetAPIVersion(), { attributes = attributes, name = "Public API constants" })
    return false
end
local ADAPTERS = { ["player-skills"] = playerSkillsStep, ["crafted-abilities"] = craftedAbilitiesStep,
    ["ability-metadata"] = abilityMetadataStep, ["effects-and-status"] = effectsStep, ["items-and-gear"] = itemsStep,
    ["item-sets"] = itemSetsStep, ["champion-skills"] = championSkillsStep, ["consumables"] = consumablesStep,
    ["mundus-effects"] = mundusStep, ["companions-races-classes"] = identitiesStep, ["combat-statistics"] = statisticsStep, ["constants"] = constantsStep }

-- Strings are escaped in small slices. A coroutine yields every JSON token or
-- <=512 input bytes; serialization and checksum never walk a whole generation.
local function escapeSlice(value)
    return (string.gsub(value, '[%z\1-\31\\"]', function(character)
        if character == '"' then return '\\"' elseif character == '\\' then return '\\\\' end
        return string.format('\\u%04x', string.byte(character))
    end))
end
local function jsonString(value)
    if #value > MAX_STRING_BYTES then error("string-limit") end
    if #value <= 512 then
        coroutine.yield('"' .. escapeSlice(value) .. '"')
        return
    end
    coroutine.yield('"')
    for index = 1, #value, 512 do coroutine.yield(escapeSlice(string.sub(value, index, index + 511))) end
    coroutine.yield('"')
end
local function sortedKeys(value)
    local keys = {}; for key in pairs(value) do table.insert(keys, key) end
    table.sort(keys)
    return keys
end
local function jsonObject(value)
    coroutine.yield("{")
    for index, key in ipairs(sortedKeys(value)) do
        if index > 1 then coroutine.yield(",") end
        jsonString(key); coroutine.yield(":")
        local entry = value[key]
        if type(entry) == "string" then jsonString(entry)
        elseif type(entry) == "number" and integer(entry) then coroutine.yield(tostring(entry))
        elseif type(entry) == "boolean" then coroutine.yield(entry and "true" or "false")
        else error("non-scalar-json-fact") end
    end
    coroutine.yield("}")
end
local function recordEncoder(record)
    return coroutine.create(function()
        coroutine.yield("{")
        local first = true
        for _, key in ipairs({ "category", "kind", "stable_id", "source_key", "parent", "parents", "attributes", "name", "description", "icon_path" }) do
            local value = record[key]
            if value ~= nil and value ~= "" and (type(value) ~= "table" or next(value)) then
                if not first then coroutine.yield(",") end
                first = false; jsonString(key); coroutine.yield(":")
                if key == "parents" then
                    coroutine.yield("[")
                    for index, parent in ipairs(value) do
                        if index > 1 then coroutine.yield(",") end
                        jsonObject(parent)
                    end
                    coroutine.yield("]")
                elseif type(value) == "table" then jsonObject(value)
                elseif type(value) == "string" then jsonString(value)
                else coroutine.yield(tostring(value)) end
            end
        end
        coroutine.yield("}")
    end)
end
local function sortStep()
    local sort = runtime.sort
    local count = #records
    if sort.width >= count then
        runtime.phase, runtime.encode_index = "serializing", 1
        runtime.chunk = { parts = {}, bytes = 0, count = 0, a = 1, b = 0 }
        return
    end
    if not sort.i then
        if sort.left > count then
            records, sort.target = sort.target, records
            sort.width, sort.left = sort.width * 2, 1
            return
        end
        sort.i, sort.j = sort.left, math.min(sort.left + sort.width, count + 1)
        sort.middle, sort.finish, sort.write = sort.j, math.min(sort.left + 2 * sort.width, count + 1), sort.left
    end
    if sort.i < sort.middle and (sort.j >= sort.finish or records[sort.i].sort_key <= records[sort.j].sort_key) then
        sort.target[sort.write], sort.i = records[sort.i], sort.i + 1
    elseif sort.j < sort.finish then
        sort.target[sort.write], sort.j = records[sort.j], sort.j + 1
    else
        sort.left, sort.i = sort.left + 2 * sort.width, nil
        return
    end
    sort.write = sort.write + 1
end
local function commitChunk()
    local chunk = runtime.chunk
    if chunk.count == 0 then return end
    if #runtime.envelope.chunks >= MAX_CHUNKS then error("chunk-limit") end
    local payload = table.concat(chunk.parts, "\n")
    table.insert(runtime.envelope.chunks, { sequence = #runtime.envelope.chunks + 1, record_count = chunk.count,
        byte_count = chunk.bytes, checksum = string.format("%08x", chunk.b * 65536 + chunk.a), payload = payload })
    runtime.chunk = { parts = {}, bytes = 0, count = 0, a = 1, b = 0 }
end
local function serializeStep(byteBudget)
    if not runtime.encoding then
        local record = records[runtime.encode_index]
        if not record then
            commitChunk()
            for _, category in ipairs(CATEGORY_ORDER) do
                table.insert(runtime.envelope.coverage, { category = category, completeness = runtime.completeness[category] or "bounded",
                    scope = "client account character unlock locale API channel visibility", record_count = runtime.category_counts[category] or 0,
                    limitations = LIMITATIONS[category] })
            end
            runtime.phase, runtime.compare_index, runtime.same = "comparing", 1, true
            return 0
        end
        runtime.encoding = { thread = recordEncoder(record), parts = {}, bytes = 0, a = 1, b = 0 }
    end
    local encoding = runtime.encoding
    if not encoding.pending then
        local ok, part = coroutine.resume(encoding.thread)
        if not ok then error("encoding-failed") end
        encoding.pending = part
    end
    local part = encoding.pending
    if part then
        if #part > byteBudget then return byteBudget end
        encoding.pending = nil
        encoding.bytes = encoding.bytes + #part
        if encoding.bytes > MAX_CHUNK_BYTES then error("record-size-limit") end
        for index = 1, #part do
            encoding.a = (encoding.a + string.byte(part, index)) % 65521
            encoding.b = (encoding.b + encoding.a) % 65521
        end
        table.insert(encoding.parts, part)
        return #part
    end
    local chunk = runtime.chunk
    local separator = chunk.count > 0 and 1 or 0
    if chunk.bytes + separator + encoding.bytes > MAX_CHUNK_BYTES then commitChunk(); chunk = runtime.chunk; separator = 0 end
    if separator == 1 then
        chunk.a = (chunk.a + 10) % 65521
        chunk.b = (chunk.b + chunk.a) % 65521
    end
    chunk.b = (chunk.b + encoding.b + encoding.bytes * (chunk.a - 1)) % 65521
    chunk.a = (chunk.a + encoding.a - 1) % 65521
    chunk.bytes, chunk.count = chunk.bytes + separator + encoding.bytes, chunk.count + 1
    table.insert(chunk.parts, table.concat(encoding.parts))
    runtime.payload_bytes = runtime.payload_bytes + separator + encoding.bytes
    if runtime.payload_bytes > MAX_SNAPSHOT_BYTES then error("snapshot-size-limit") end
    runtime.encoding, runtime.encode_index = nil, runtime.encode_index + 1
    return 0
end
local function requestSave()
    if not savePending or now() - lastSaveRequest < SAVE_REQUEST_MS then return end
    local manager = optional("GetAddOnManager")
    if manager and type(manager.RequestAddOnSavedVariablesPrioritySave) == "function" then
        lastSaveRequest = now()
        local ok = pcall(manager.RequestAddOnSavedVariablesPrioritySave, manager, ADDON_NAME)
        if ok then
            savePending, lastSaveRequest = false, now()
            diagnostics.save_requested_at = tostring(GetTimeStamp())
            -- A request changes queue priority only. No disk-save acknowledgement exists.
        end
    else savePending = false end
end
local function compareStep()
    local previous, envelope = EsoWeaveDataSaved.catalog, runtime.envelope
    if runtime.compare_index == 1 then
        if previous ~= completedInSession or type(previous) ~= "table" or previous.schema_version ~= SCHEMA_VERSION
            or previous.collector_version ~= COLLECTOR_VERSION or previous.status ~= "complete"
            or previous.collector_checksum ~= COLLECTOR_CHECKSUM
            or type(previous.started_at) ~= "string" or previous.started_at == ""
            or type(previous.finished_at) ~= "string" or previous.finished_at == ""
            or previous.cancellation_reason ~= nil then runtime.same = false
        else
            for _, key in ipairs({ "channel", "game_version", "api_version", "locale", "platform", "megaserver", "scope_key" }) do
                if previous[key] ~= envelope[key] then runtime.same = false end
            end
            if type(previous.selected_categories) ~= "table" or #previous.selected_categories ~= #CATEGORY_ORDER then runtime.same = false
            else
                for index, category in ipairs(CATEGORY_ORDER) do
                    if previous.selected_categories[index] ~= category then runtime.same = false end
                end
            end
            if type(previous.checkpoint) ~= "table" or previous.checkpoint.adapter ~= #CATEGORY_ORDER
                or previous.checkpoint.cursor ~= #records then runtime.same = false end
            if type(previous.chunks) ~= "table" or #previous.chunks ~= #envelope.chunks then runtime.same = false end
            if type(previous.coverage) ~= "table" or #previous.coverage ~= #envelope.coverage then runtime.same = false
            else
                for index, coverage in ipairs(envelope.coverage) do
                    local older = previous.coverage[index]
                    if type(older) ~= "table" or older.category ~= coverage.category or older.completeness ~= coverage.completeness
                        or older.scope ~= coverage.scope or older.record_count ~= coverage.record_count
                        or older.limitations ~= coverage.limitations then runtime.same = false end
                end
            end
            -- Prior saved data is not trusted to contain bounded string arrays.
            -- Compare only the <=64 known current warnings, never concat old data.
            if type(previous.warnings) ~= "table" or #previous.warnings ~= #envelope.warnings then runtime.same = false
            else
                for index, value in ipairs(envelope.warnings) do
                    if previous.warnings[index] ~= value then runtime.same = false end
                end
            end
        end
    end
    local chunk = envelope.chunks[runtime.compare_index]
    if runtime.same and chunk then
        local older = previous.chunks[runtime.compare_index]
        if type(older) ~= "table" or older.sequence ~= chunk.sequence or older.record_count ~= chunk.record_count
            or older.byte_count ~= chunk.byte_count or older.checksum ~= chunk.checksum
            or older.payload ~= chunk.payload then runtime.same = false end
        runtime.compare_index = runtime.compare_index + 1
        return
    end
    -- Revalidate source identity at the publication boundary as well as during
    -- scanning; a locale/character transition cannot label mixed data as complete.
    local currentIdentity = readIdentity()
    if currentIdentity ~= runtime.identity then
        identity, dirty, nextAttempt, runtime, records = currentIdentity, true, 0, nil, {}
        setState("deferred", "identity-changed"); return
    end
    if not runtime.same then
        envelope.status, envelope.finished_at = "complete", tostring(GetTimeStamp())
        envelope.checkpoint = { adapter = #CATEGORY_ORDER, cursor = #records }
        -- The completed table is published in one assignment; partial work is runtime-only.
        EsoWeaveDataSaved.catalog = envelope
        completedInSession = envelope
        diagnostics.generation = diagnostics.generation + 1
        savePending = true
    end
    known = runtime.known
    runtime, records = nil, {}
    nextAttempt = now() + REFRESH_COALESCE_MS
    setState("idle")
    requestSave()
end
local function startCapture()
    if not validRoot() then setState("unavailable", "unsupported-saved-root"); nextAttempt = now() + RETRY_MS; return end
    local previous = EsoWeaveDataSaved.catalog
    if previous ~= nil and (type(previous) ~= "table" or previous.schema_version ~= SCHEMA_VERSION) then
        setState("unavailable", "unsupported-saved-catalog"); nextAttempt = now() + RETRY_MS; return
    end
    local sourceApi, sourceVersion, sourceLocale, sourceCharacter = GetAPIVersion(), optional("GetESOVersionString"), GetCVar("language.2"), optional("GetCurrentCharacterId")
    if not integer(sourceApi) or sourceApi <= 0 or type(sourceVersion) ~= "string" or sourceVersion == ""
        or type(sourceLocale) ~= "string" or #sourceLocale < 2 or sourceCharacter == nil or tostring(sourceCharacter) == "" then
        setState("deferred", "provenance-initialization"); nextAttempt = now() + RETRY_MS; return
    end
    records = {}
    local selected = {}; for _, category in ipairs(CATEGORY_ORDER) do table.insert(selected, category) end
    local envelope = { schema_version = SCHEMA_VERSION, collector_version = COLLECTOR_VERSION, collector_checksum = COLLECTOR_CHECKSUM,
        status = "running", channel = channel, game_version = sourceVersion, api_version = sourceApi,
        locale = sourceLocale, platform = tostring(optional("GetPlatformServiceType") or "unknown"), megaserver = optional("GetWorldName") or "",
        scope_key = "character-" .. tostring(sourceCharacter), started_at = tostring(GetTimeStamp()), finished_at = "",
        selected_categories = selected, coverage = {}, warnings = {}, checkpoint = { adapter = 1, cursor = 0 }, chunks = {} }
    runtime = { envelope = envelope, adapter = 1, cursor = {}, phase = "scanning", estimated_bytes = 0, payload_bytes = 0,
        seen = {}, known = {}, category_counts = {}, completeness = {},
        identity = table.concat({ channel, tostring(sourceApi), sourceVersion, sourceLocale, tostring(sourceCharacter) }, "|") }
    for _, category in ipairs({ "effects-and-status", "items-and-gear", "consumables", "mundus-effects", "ability-metadata" }) do runtime.completeness[category] = "opportunistic" end
    diagnostics.attempt = diagnostics.attempt + 1
    dirty = false
    setState("scanning")
end
local function update()
    diagnostics.last_tick_steps, diagnostics.last_tick_bytes = 0, 0
    if validRoot() and EsoWeaveDataSaved.catalog_maintenance ~= diagnostics then EsoWeaveDataSaved.catalog_maintenance = diagnostics end
    if not loaded or disabled then return end
    if not active then setState("deferred", "player-inactive"); return end
    if IsUnitInCombat("player") then setState("deferred", "combat"); return end
    if available("AreSkillsInitialized") and not AreSkillsInitialized() then setState("deferred", "api-initialization"); return end
    if now() >= (diagnostics.next_identity_check or 0) then
        diagnostics.next_identity_check = now() + 5000
        local current = readIdentity()
        if current ~= identity then
            identity, dirty, nextAttempt = current, true, 0
            runtime, records = nil, {}
        end
    end
    requestSave()
    if not runtime then
        if not dirty or now() < nextAttempt then return end
        startCapture()
        if not runtime then return end
    end
    local started, steps, bytes = now(), 0, 0
    while runtime and steps < MAX_STEPS_PER_TICK and bytes < MAX_SERIALIZE_BYTES_PER_TICK and now() - started < MAX_MILLISECONDS_PER_TICK do
        steps = steps + 1
        setState(runtime.phase)
        if runtime.phase == "scanning" then
            local category = CATEGORY_ORDER[runtime.adapter]
            if not category then
                if #records == 0 or #records == (runtime.category_counts["constants"] or 0) then
                    fail("no-useful-definitions"); return
                end
                runtime.phase, runtime.sort = "sorting", { width = 1, left = 1, target = {} }
            elseif ADAPTERS[category](runtime.cursor) then runtime.adapter, runtime.cursor = runtime.adapter + 1, {} end
            diagnostics.checkpoint = { adapter = runtime.adapter, cursor = #records }
        elseif runtime.phase == "sorting" then sortStep()
        elseif runtime.phase == "serializing" then bytes = bytes + serializeStep(MAX_SERIALIZE_BYTES_PER_TICK - bytes)
        elseif runtime.phase == "comparing" then compareStep() end
    end
    diagnostics.last_tick_steps, diagnostics.last_tick_bytes = steps, bytes
end
local function guardedUpdate()
    local ok = pcall(update)
    if not ok then fail("api-or-resource-failure") end
end
local function changed()
    dirty = true
    if not runtime then nextAttempt = math.max(nextAttempt, now() + REFRESH_COALESCE_MS) end
end
local function onActivated()
    active = true
    local ok, current = pcall(readIdentity)
    if ok and current ~= identity then identity, dirty, nextAttempt, runtime, records = current, true, 0, nil, {} end
end
local function registerEvent(suffix, event, callback)
    if type(event) == "number" then EVENT_MANAGER:RegisterForEvent(MODULE_NAMESPACE .. suffix, event, callback) end
end
local function onLoaded(_, addonName)
    if addonName ~= ADDON_NAME or loaded then return end
    loaded = true
    EVENT_MANAGER:UnregisterForEvent(MODULE_NAMESPACE, EVENT_ADD_ON_LOADED)
    local config = ESO_WEAVE_STARTUP_CONFIG
    if type(config) ~= "table" or type(config.catalog_disabled) ~= "boolean" or (config.channel ~= "live" and config.channel ~= "pts") then
        setState("unavailable", "invalid-startup-config"); return
    end
    disabled, channel = config.catalog_disabled, config.channel
    if validRoot() then
        local older = EsoWeaveDataSaved.catalog_maintenance
        diagnostics.generation = type(older) == "table" and integer(older.generation) and older.generation >= 0 and older.generation or 0
        diagnostics.attempt = type(older) == "table" and integer(older.attempt) and older.attempt >= 0 and older.attempt or 0
        EsoWeaveDataSaved.catalog_maintenance = diagnostics
    else diagnostics.generation, diagnostics.attempt = 0, 0 end
    diagnostics.startup_disabled = disabled
    if disabled then setState("disabled"); return end
    registerEvent("Activate", EVENT_PLAYER_ACTIVATED, onActivated)
    registerEvent("Deactivate", EVENT_PLAYER_DEACTIVATED, function() active = false end)
    registerEvent("Combat", EVENT_PLAYER_COMBAT_STATE, function(_, inCombat)
        if not inCombat then nextAttempt = math.min(nextAttempt, now()) end
    end)
    for _, key in ipairs({ "EVENT_SKILLS_FULL_UPDATE", "EVENT_SKILL_LINE_ADDED", "EVENT_SKILL_RANK_UPDATE", "EVENT_SKILL_RESPEC_RESULT",
        "EVENT_CRAFTED_ABILITY_LOCK_STATE_CHANGED", "EVENT_CRAFTED_ABILITY_SCRIPT_LOCK_STATE_CHANGED", "EVENT_CRAFTED_ABILITY_RESET",
        "EVENT_ITEM_SET_COLLECTION_UPDATED", "EVENT_ITEM_SET_COLLECTIONS_UPDATED", "EVENT_COLLECTION_UPDATED",
        "EVENT_COMPANION_ACTIVATED", "EVENT_CHAMPION_PURCHASE_RESULT", "EVENT_CHAMPION_SYSTEM_UNLOCKED",
        "EVENT_HOTBAR_SLOT_UPDATED", "EVENT_INVENTORY_FULL_UPDATE", "EVENT_EFFECTS_FULL_UPDATE" }) do
        registerEvent(key, _G[key], changed)
    end
    -- Ignore stack/duration/durability changes for definitions already seen.
    -- New source identities coalesce into one scan without retaining raw events.
    registerEvent("Inventory", EVENT_INVENTORY_SINGLE_SLOT_UPDATE, function(_, bag, slot, isNewItem)
        if isNewItem == true and (bag == BAG_WORN or bag == BAG_BACKPACK) then changed(); return end
        if (bag == BAG_WORN or bag == BAG_BACKPACK) and available("GetItemLink") and available("GetItemLinkItemId") then
            local ok, id = pcall(function() return GetItemLinkItemId(GetItemLink(bag, slot, LINK_STYLE_DEFAULT or 0)) end)
            if ok and integer(id) and id > 0 and not known["item/" .. tostring(id)] then changed() end
        end
    end)
    registerEvent("Effects", EVENT_EFFECT_CHANGED, function(_, _, _, _, unitTag, _, _, _, _, _, _, _, _, _, _, id)
        if unitTag == "player" and integer(id) and id > 0 and not known["effect/" .. tostring(id)] then changed() end
    end)
    EVENT_MANAGER:RegisterForUpdate(UPDATE_NAME, 10, guardedUpdate)
end
EVENT_MANAGER:RegisterForEvent(MODULE_NAMESPACE, EVENT_ADD_ON_LOADED, onLoaded)
