local ADDON_NAME = "EsoWeaveCollector"
local COLLECTOR_VERSION = 1
local SCHEMA_VERSION = 1
local COLLECTOR_CHECKSUM = "7571d13a1040ccea25a4c8ea714061e5dbce650373684dabba8f7bc7ba7969ae"

local MAX_RECORDS_PER_TICK = 64
local MAX_MILLISECONDS_PER_TICK = 4
local MAX_SNAPSHOT_BYTES = 67108864
local MAX_RECORDS = 500000
local MAX_STRING_BYTES = 65536
local MAX_CHUNK_BYTES = 65536
local MAX_CHUNKS = 1024

local UPDATE_NAME = ADDON_NAME .. "Update"
local records = {}
local runtime = nil

local CATEGORY_ORDER = {
    "player-skills",
    "crafted-abilities",
    "item-sets",
    "champion-skills",
    "companions-races-classes",
}

local function message(text)
    d("[ESO Weave Collector] " .. text)
end

local function addWarning(text)
    for _, existing in ipairs(EsoWeaveCollectorSaved.warnings) do
        if existing == text then return end
    end
    table.insert(EsoWeaveCollectorSaved.warnings, text)
end

local function jsonEscape(value)
    if #value > MAX_STRING_BYTES then
        error("string exceeds MAX_STRING_BYTES")
    end
    value = string.gsub(value, "\\", "\\\\")
    value = string.gsub(value, "\"", "\\\"")
    value = string.gsub(value, "\b", "\\b")
    value = string.gsub(value, "\f", "\\f")
    value = string.gsub(value, "\n", "\\n")
    value = string.gsub(value, "\r", "\\r")
    value = string.gsub(value, "\t", "\\t")
    return "\"" .. value .. "\""
end

local function sortedKeys(value)
    local keys = {}
    for key in pairs(value) do
        table.insert(keys, key)
    end
    table.sort(keys, function(left, right) return tostring(left) < tostring(right) end)
    return keys
end

local function jsonValue(value)
    local valueType = type(value)
    if valueType == "string" then
        return jsonEscape(value)
    elseif valueType == "number" then
        if value ~= math.floor(value) then
            error("only integer numeric facts are supported")
        end
        return tostring(value)
    elseif valueType == "boolean" then
        return value and "true" or "false"
    elseif valueType == "table" then
        local parts = {}
        for _, key in ipairs(sortedKeys(value)) do
            table.insert(parts, jsonEscape(tostring(key)) .. ":" .. jsonValue(value[key]))
        end
        return "{" .. table.concat(parts, ",") .. "}"
    end
    error("unsupported JSON value")
end

local function encodeRecord(record)
    local parts = {
        "\"category\":" .. jsonEscape(record.category),
        "\"kind\":" .. jsonEscape(record.kind),
        "\"stable_id\":" .. tostring(record.stable_id),
        "\"source_key\":" .. jsonEscape(record.source_key),
    }
    if record.parent then
        table.insert(parts, "\"parent\":" .. jsonValue(record.parent))
    end
    if record.attributes and next(record.attributes) then
        table.insert(parts, "\"attributes\":" .. jsonValue(record.attributes))
    end
    if record.name and record.name ~= "" then
        table.insert(parts, "\"name\":" .. jsonEscape(record.name))
    end
    if record.description and record.description ~= "" then
        table.insert(parts, "\"description\":" .. jsonEscape(record.description))
    end
    if record.icon_path and record.icon_path ~= "" then
        table.insert(parts, "\"icon_path\":" .. jsonEscape(record.icon_path))
    end
    return "{" .. table.concat(parts, ",") .. "}"
end

local function adler32(value)
    local a = 1
    local b = 0
    for index = 1, #value do
        a = (a + string.byte(value, index)) % 65521
        b = (b + a) % 65521
    end
    return string.format("%08x", b * 65536 + a)
end

local function setFailure(reason)
    EVENT_MANAGER:UnregisterForUpdate(UPDATE_NAME)
    EsoWeaveCollectorSaved.status = "failed"
    EsoWeaveCollectorSaved.cancellation_reason = reason
    EsoWeaveCollectorSaved.finished_at = tostring(GetTimeStamp())
    runtime = nil
    message("Capture failed: " .. reason)
end

local function addRecord(category, kind, stableId, fields)
    if type(stableId) ~= "number" or stableId <= 0 then
        return
    end
    if #records >= MAX_RECORDS then
        error("record limit reached")
    end
    local identity = kind .. "/" .. tostring(stableId)
    if runtime.seen[identity] then
        addWarning("duplicate stable entity normalized once: " .. identity)
        return
    end
    local record = fields or {}
    record.category = category
    record.kind = kind
    record.stable_id = stableId
    record.source_key = category .. "/" .. kind .. "/" .. tostring(stableId)
    record.encoded = encodeRecord(record)
    if #record.encoded > MAX_CHUNK_BYTES then
        error("record exceeds MAX_CHUNK_BYTES")
    end
    record.sort_key = record.source_key
    runtime.estimated_bytes = runtime.estimated_bytes + #record.encoded
    if runtime.estimated_bytes > MAX_SNAPSHOT_BYTES then
        error("snapshot byte limit reached")
    end
    table.insert(records, record)
    runtime.seen[identity] = true
    runtime.category_counts[category] = (runtime.category_counts[category] or 0) + 1
end

local function playerSkillsStep(cursor)
    cursor.skill_type = cursor.skill_type or 1
    cursor.skill_line = cursor.skill_line or 1
    cursor.ability = cursor.ability or 0
    while cursor.skill_type <= GetNumSkillTypes() do
        local lineCount = GetNumSkillLines(cursor.skill_type)
        if cursor.skill_line > lineCount then
            cursor.skill_type = cursor.skill_type + 1
            cursor.skill_line = 1
            cursor.ability = 0
        else
            local lineId = GetSkillLineId(cursor.skill_type, cursor.skill_line)
            if cursor.ability == 0 then
                cursor.ability = 1
                addRecord("player-skills", "skill-line", lineId, {
                    attributes = {
                        skill_type_source_index = cursor.skill_type,
                        version_scoped_order = cursor.skill_line,
                    },
                })
                return false
            end
            local abilityCount = GetNumSkillAbilities(cursor.skill_type, cursor.skill_line)
            if cursor.ability <= abilityCount then
                local abilityIndex = cursor.ability
                cursor.ability = cursor.ability + 1
                local abilityId = GetSkillAbilityId(cursor.skill_type, cursor.skill_line, abilityIndex, false)
                addRecord("player-skills", "ability", abilityId, {
                    parent = { relation = "skill-line-has-ability", kind = "skill-line", stable_id = lineId },
                    attributes = { version_scoped_order = abilityIndex },
                    name = GetAbilityName(abilityId),
                    description = GetAbilityDescription(abilityId),
                    icon_path = GetAbilityIcon(abilityId),
                })
                return false
            end
            cursor.skill_line = cursor.skill_line + 1
            cursor.ability = 0
        end
    end
    return true
end

local function craftedAbilitiesStep(cursor)
    cursor.index = cursor.index or 1
    cursor.slot = cursor.slot or 0
    cursor.script = cursor.script or 1
    if type(GetNumCraftedAbilities) ~= "function" or type(GetCraftedAbilityIdAtIndex) ~= "function" then
        addWarning("crafted-abilities API is unavailable for this client")
        return true
    end
    if cursor.index > GetNumCraftedAbilities() then return true end
    local craftedId = GetCraftedAbilityIdAtIndex(cursor.index)
    if cursor.slot == 0 then
        cursor.slot = 1
        addRecord("crafted-abilities", "crafted-ability", craftedId, {
            attributes = { version_scoped_order = cursor.index },
            name = type(GetCraftedAbilityDisplayName) == "function" and GetCraftedAbilityDisplayName(craftedId) or nil,
            icon_path = type(GetCraftedAbilityIcon) == "function" and GetCraftedAbilityIcon(craftedId) or nil,
        })
        return false
    end
    local slots = { SCRIBING_SLOT_PRIMARY, SCRIBING_SLOT_SECONDARY, SCRIBING_SLOT_TERTIARY }
    if type(GetNumScriptsInSlotForCraftedAbility) == "function"
        and type(GetScriptIdAtSlotIndexForCraftedAbility) == "function"
        and cursor.slot <= #slots then
        local slot = slots[cursor.slot]
        local count = GetNumScriptsInSlotForCraftedAbility(craftedId, slot)
        if cursor.script <= count then
            local sourceIndex = cursor.script
            cursor.script = cursor.script + 1
            local scriptId = GetScriptIdAtSlotIndexForCraftedAbility(craftedId, slot, sourceIndex)
            addRecord("crafted-abilities", "script", scriptId, {
                parent = { relation = "crafted-ability-has-script", kind = "crafted-ability", stable_id = craftedId },
                attributes = { scribing_slot = slot, version_scoped_order = sourceIndex },
                name = type(GetCraftedAbilityScriptDisplayName) == "function" and GetCraftedAbilityScriptDisplayName(scriptId) or nil,
                description = type(GetCraftedAbilityScriptDescription) == "function" and GetCraftedAbilityScriptDescription(craftedId, scriptId) or nil,
            })
            return false
        end
        cursor.slot = cursor.slot + 1
        cursor.script = 1
        return false
    end
    if cursor.slot == 1 then
        addWarning("crafted-ability script API is unavailable for this client")
    end
    cursor.index = cursor.index + 1
    cursor.slot = 0
    cursor.script = 1
    return false
end

local function itemSetsStep(cursor)
    if type(GetNextItemSetCollectionId) ~= "function"
        or type(GetNumItemSetCollectionPieces) ~= "function"
        or type(GetItemSetCollectionPieceInfo) ~= "function" then
        addWarning("item-set collection API is unavailable for this client")
        return true
    end
    if cursor.set_id and cursor.piece <= cursor.piece_count then
        local sourceIndex = cursor.piece
        cursor.piece = cursor.piece + 1
        local pieceId = select(1, GetItemSetCollectionPieceInfo(cursor.set_id, sourceIndex))
        addRecord("item-sets", "item-set-piece", pieceId, {
            parent = { relation = "item-set-has-piece", kind = "item-set", stable_id = cursor.set_id },
            attributes = { version_scoped_order = sourceIndex },
        })
        return false
    end
    local setId = GetNextItemSetCollectionId(cursor.last_id)
    if not setId then
        return true
    end
    cursor.last_id = setId
    local pieceCount = GetNumItemSetCollectionPieces(setId)
    cursor.set_id = setId
    cursor.piece = 1
    cursor.piece_count = pieceCount
    addRecord("item-sets", "item-set", setId, {
        attributes = { visible_piece_count = pieceCount },
    })
    return false
end

local function championSkillsStep(cursor)
    if type(GetNumChampionDisciplines) ~= "function" or type(GetChampionDisciplineId) ~= "function" then
        addWarning("champion skill API is unavailable for this client")
        return true
    end
    cursor.discipline = cursor.discipline or 1
    cursor.skill = cursor.skill or 0
    while cursor.discipline <= GetNumChampionDisciplines() do
        local disciplineId = GetChampionDisciplineId(cursor.discipline)
        if cursor.skill == 0 then
            cursor.skill = 1
            addRecord("champion-skills", "collection-category", disciplineId, {
                attributes = { version_scoped_order = cursor.discipline },
                name = type(GetChampionDisciplineName) == "function" and GetChampionDisciplineName(disciplineId) or nil,
            })
            return false
        end
        local count = GetNumChampionDisciplineSkills(cursor.discipline)
        if cursor.skill <= count then
            local sourceIndex = cursor.skill
            cursor.skill = cursor.skill + 1
            local skillId = GetChampionSkillId(cursor.discipline, sourceIndex)
            addRecord("champion-skills", "champion-skill", skillId, {
                parent = { relation = "discipline-has-champion-skill", kind = "collection-category", stable_id = disciplineId },
                attributes = { version_scoped_order = sourceIndex },
                name = GetChampionSkillName(skillId),
                description = GetChampionSkillDescription(skillId, 0),
                icon_path = type(GetChampionSkillIcon) == "function" and GetChampionSkillIcon(skillId) or nil,
            })
            return false
        end
        cursor.discipline = cursor.discipline + 1
        cursor.skill = 0
    end
    return true
end

local function identitiesStep(cursor)
    if type(GetNumClasses) ~= "function" or type(GetClassInfo) ~= "function" then
        addWarning("class identity API is unavailable for this client")
        return true
    end
    cursor.index = cursor.index or 1
    if cursor.index <= GetNumClasses() then
        local sourceIndex = cursor.index
        cursor.index = cursor.index + 1
        local classId = select(1, GetClassInfo(sourceIndex))
        addRecord("companions-races-classes", "class", classId, {
            attributes = { version_scoped_order = sourceIndex },
        })
        return false
    end
    if not cursor.race_done then
        cursor.race_done = true
        local raceId = GetUnitRaceId("player")
        addRecord("companions-races-classes", "race", raceId, {
            attributes = { visibility = "current-character" },
        })
        return false
    end
    if not cursor.companion_done then
        cursor.companion_done = true
        if type(GetActiveCompanionDefId) == "function" then
            addRecord("companions-races-classes", "companion", GetActiveCompanionDefId(), {
                attributes = { visibility = "active-companion" },
            })
        end
        return false
    end
    return true
end

local ADAPTERS = {
    ["player-skills"] = playerSkillsStep,
    ["crafted-abilities"] = craftedAbilitiesStep,
    ["item-sets"] = itemSetsStep,
    ["champion-skills"] = championSkillsStep,
    ["companions-races-classes"] = identitiesStep,
}

local LIMITATIONS = {
    ["player-skills"] = "Bounded by character, class, unlocks, locale, API version, and channel.",
    ["crafted-abilities"] = "Bounded by account unlocks, locale, API version, and channel.",
    ["item-sets"] = "Bounded by account collection visibility, locale, API version, and channel.",
    ["champion-skills"] = "Bounded by account and character visibility, locale, API version, and channel.",
    ["companions-races-classes"] = "Bounded by current character, account unlocks, API version, and channel.",
}

local function finalizeCapture()
    table.sort(records, function(left, right) return left.sort_key < right.sort_key end)
    local chunks = {}
    local payloadParts = {}
    local payloadBytes = 0
    local function commitChunk()
        if #payloadParts == 0 then return end
        if #chunks >= MAX_CHUNKS then error("chunk limit reached") end
        local payload = table.concat(payloadParts, "\n")
        table.insert(chunks, {
            sequence = #chunks + 1,
            record_count = #payloadParts,
            byte_count = #payload,
            checksum = adler32(payload),
            payload = payload,
        })
        payloadParts = {}
        payloadBytes = 0
    end
    for _, record in ipairs(records) do
        local separator = #payloadParts == 0 and 0 or 1
        if payloadBytes + separator + #record.encoded > MAX_CHUNK_BYTES then
            commitChunk()
        end
        table.insert(payloadParts, record.encoded)
        payloadBytes = payloadBytes + separator + #record.encoded
    end
    commitChunk()
    local coverage = {}
    for _, category in ipairs(runtime.selected_categories) do
        table.insert(coverage, {
            category = category,
            completeness = "bounded",
            scope = "current client, account, character, unlock, locale, API version, and channel visibility",
            record_count = runtime.category_counts[category] or 0,
            limitations = LIMITATIONS[category],
        })
    end
    EsoWeaveCollectorSaved.status = "complete"
    EsoWeaveCollectorSaved.finished_at = tostring(GetTimeStamp())
    EsoWeaveCollectorSaved.coverage = coverage
    EsoWeaveCollectorSaved.chunks = chunks
    EsoWeaveCollectorSaved.checkpoint = { adapter = #runtime.selected_categories, cursor = 0 }
    EVENT_MANAGER:UnregisterForUpdate(UPDATE_NAME)
    runtime = nil
    message("Capture complete. Use /reloadui, logout, or exit before desktop import.")
end

local function updateCapture()
    if not runtime or EsoWeaveCollectorSaved.status ~= "running" then return end
    if IsUnitInCombat("player") then
        EsoWeaveCollectorSaved.status = "paused"
        EVENT_MANAGER:UnregisterForUpdate(UPDATE_NAME)
        message("Capture paused for combat. Use /ewcollect resume after combat.")
        return
    end
    local started = GetGameTimeMilliseconds()
    local processed = 0
    while processed < MAX_RECORDS_PER_TICK and GetGameTimeMilliseconds() - started < MAX_MILLISECONDS_PER_TICK do
        local category = runtime.selected_categories[runtime.adapter]
        if not category then
            local ok, reason = pcall(finalizeCapture)
            if not ok then setFailure(tostring(reason)) end
            return
        end
        local ok, done = pcall(ADAPTERS[category], runtime.cursor)
        if not ok then
            setFailure(tostring(done))
            return
        end
        if done then
            runtime.adapter = runtime.adapter + 1
            runtime.cursor = {}
        else
            processed = processed + 1
        end
        EsoWeaveCollectorSaved.checkpoint = { adapter = runtime.adapter, cursor = #records }
    end
end

local function parseSelection(arguments)
    local selected = {}
    local seen = {}
    for token in string.gmatch(arguments, "%S+") do
        if not ADAPTERS[token] then
            return nil, token
        elseif not seen[token] then
            seen[token] = true
            table.insert(selected, token)
        end
    end
    if #selected == 0 then
        for _, category in ipairs(CATEGORY_ORDER) do table.insert(selected, category) end
    end
    table.sort(selected)
    return selected, nil
end

local function startCapture(arguments)
    if runtime then
        message("A capture is already active.")
        return
    end
    if IsUnitInCombat("player") then
        message("Capture cannot start in combat.")
        return
    end
    local channel, selection = string.match(arguments or "", "^(%S+)%s*(.-)%s*$")
    if channel ~= "live" and channel ~= "pts" then
        message("Choose the environment explicitly: /ewcollect start live|pts [categories].")
        return
    end
    local selected, invalid = parseSelection(selection)
    if invalid then
        message("Unknown category: " .. invalid)
        return
    end
    records = {}
    runtime = {
        selected_categories = selected,
        adapter = 1,
        cursor = {},
        estimated_bytes = 0,
        category_counts = {},
        seen = {},
    }
    local characterId = type(GetCurrentCharacterId) == "function" and GetCurrentCharacterId() or 0
    EsoWeaveCollectorSaved = {
        schema_version = SCHEMA_VERSION,
        collector_version = COLLECTOR_VERSION,
        collector_checksum = COLLECTOR_CHECKSUM,
        status = "running",
        channel = channel,
        game_version = type(GetESOVersionString) == "function" and GetESOVersionString() or "unknown",
        api_version = GetAPIVersion(),
        locale = GetCVar("language.2"),
        platform = GetPlatformServiceType and tostring(GetPlatformServiceType()) or "pc",
        megaserver = GetWorldName and GetWorldName() or "",
        scope_key = "character-" .. tostring(characterId),
        started_at = tostring(GetTimeStamp()),
        finished_at = "",
        selected_categories = runtime.selected_categories,
        coverage = {},
        warnings = {},
        cancellation_reason = nil,
        checkpoint = { adapter = 1, cursor = 0 },
        chunks = {},
    }
    EVENT_MANAGER:RegisterForUpdate(UPDATE_NAME, 10, updateCapture)
    message("Capture started outside combat.")
end

local function resumeCapture()
    if not runtime or EsoWeaveCollectorSaved.status ~= "paused" then
        message("No paused in-session capture can be resumed.")
        return
    end
    if IsUnitInCombat("player") then
        message("Capture cannot resume in combat.")
        return
    end
    EsoWeaveCollectorSaved.status = "running"
    EVENT_MANAGER:RegisterForUpdate(UPDATE_NAME, 10, updateCapture)
    message("Capture resumed.")
end

local function cancelCapture()
    if not runtime then
        message("No active capture to cancel.")
        return
    end
    EVENT_MANAGER:UnregisterForUpdate(UPDATE_NAME)
    EsoWeaveCollectorSaved.status = "cancelled"
    EsoWeaveCollectorSaved.cancellation_reason = "user-cancelled"
    EsoWeaveCollectorSaved.finished_at = tostring(GetTimeStamp())
    runtime = nil
    message("Capture cancelled. The incomplete envelope cannot be imported.")
end

local function showStatus()
    local state = EsoWeaveCollectorSaved and EsoWeaveCollectorSaved.status or "idle"
    if runtime then
        message("Status: " .. tostring(state) .. ", adapter " .. tostring(runtime.adapter)
            .. "/" .. tostring(#runtime.selected_categories) .. ", records " .. tostring(#records))
    else
        message("Status: " .. tostring(state))
    end
end

local function command(arguments)
    local verb, rest = string.match(arguments or "", "^(%S*)%s*(.-)%s*$")
    if verb == "start" then
        startCapture(rest)
    elseif verb == "resume" then
        resumeCapture()
    elseif verb == "cancel" then
        cancelCapture()
    elseif verb == "status" then
        showStatus()
    else
        message("Use /ewcollect start live|pts [categories], /ewcollect status, /ewcollect resume, /ewcollect cancel, or /ewcollect help.")
    end
end

local function onCombatState(_, inCombat)
    if inCombat and runtime and EsoWeaveCollectorSaved.status == "running" then
        EsoWeaveCollectorSaved.status = "paused"
        EVENT_MANAGER:UnregisterForUpdate(UPDATE_NAME)
        message("Capture paused for combat.")
    end
end

local function onPlayerDeactivated()
    if runtime and EsoWeaveCollectorSaved.status == "running" then
        EsoWeaveCollectorSaved.status = "paused"
        EVENT_MANAGER:UnregisterForUpdate(UPDATE_NAME)
    end
end

local function onLoaded(_, addonName)
    if addonName ~= ADDON_NAME then return end
    EVENT_MANAGER:UnregisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED)
    SLASH_COMMANDS["/ewcollect"] = command
    EVENT_MANAGER:RegisterForEvent(ADDON_NAME .. "Combat", EVENT_PLAYER_COMBAT_STATE, onCombatState)
    EVENT_MANAGER:RegisterForEvent(ADDON_NAME .. "Deactivate", EVENT_PLAYER_DEACTIVATED, onPlayerDeactivated)
    message("Loaded. Collection is manual and local. Use /ewcollect help.")
end

EVENT_MANAGER:RegisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED, onLoaded)
