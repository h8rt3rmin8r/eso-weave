local ADDON_NAME = "EsoWeaveEncounter"
local ADDON_VERSION = 1
local SCHEMA_VERSION = 1
local PRIVACY_PROFILE = "anonymous-local-v1"

local MAX_EVENTS = 100000
local MAX_ESTIMATED_BYTES = 33554432
local MAX_ACTORS = 4096
local TERMINAL_EVENT_RESERVE = 2
local TERMINAL_BYTE_RESERVE = 2048
local BASE_ENVELOPE_BYTES = 512
local BASE_EVENT_BYTES = 160
local SAMPLE_UPDATE_MS = 250
local BOSS_SAMPLE_MS = 500
local PERFORMANCE_SAMPLE_MS = 1000

if type(EsoWeaveEncounterTestLimits) == "table" then
    MAX_EVENTS = EsoWeaveEncounterTestLimits.max_events or MAX_EVENTS
    MAX_ESTIMATED_BYTES = EsoWeaveEncounterTestLimits.max_estimated_bytes or MAX_ESTIMATED_BYTES
    MAX_ACTORS = EsoWeaveEncounterTestLimits.max_actors or MAX_ACTORS
end

local UPDATE_NAMESPACE = ADDON_NAME .. "Samples"
local runtime = nil

local CAPTURE_EVENTS = {
    EVENT_COMBAT_EVENT,
    EVENT_EFFECT_CHANGED,
    EVENT_POWER_UPDATE,
    EVENT_ACTION_SLOT_ABILITY_USED,
    EVENT_ACTIVE_WEAPON_PAIR_CHANGED,
    EVENT_PLAYER_DEAD,
    EVENT_PLAYER_ALIVE,
    EVENT_BOSSES_CHANGED,
    EVENT_ACTIVE_QUICKSLOT_CHANGED,
}

local function resultSet(...)
    local result = {}
    for index = 1, select("#", ...) do
        local value = select(index, ...)
        if type(value) == "number" then result[value] = true end
    end
    return result
end

local DAMAGE_RESULTS = resultSet(
    ACTION_RESULT_DAMAGE,
    ACTION_RESULT_CRITICAL_DAMAGE,
    ACTION_RESULT_DOT_TICK,
    ACTION_RESULT_DOT_TICK_CRITICAL,
    ACTION_RESULT_BLOCKED_DAMAGE,
    ACTION_RESULT_DAMAGE_SHIELDED)

local HEAL_RESULTS = resultSet(
    ACTION_RESULT_HEAL,
    ACTION_RESULT_CRITICAL_HEAL,
    ACTION_RESULT_HOT_TICK,
    ACTION_RESULT_HOT_TICK_CRITICAL)

local DEATH_RESULTS = resultSet(ACTION_RESULT_DIED, ACTION_RESULT_DIED_XP)

local function message(text)
    d("[ESO Weave Encounter] " .. text)
end

local function emptySaved()
    return {
        schema_version = SCHEMA_VERSION,
        addon_version = ADDON_VERSION,
        status = "idle",
        channel = nil,
        privacy_profile = PRIVACY_PROFILE,
        events = {},
        warnings = {},
        omitted_event_count = 0,
        stored_event_count = 0,
        estimated_bytes = BASE_ENVELOPE_BYTES,
    }
end

local function addWarning(name)
    local warnings = EsoWeaveEncounterSaved.warnings
    warnings[name] = (warnings[name] or 0) + 1
end

local function estimateValue(value, depth)
    if depth > 2 then return TERMINAL_BYTE_RESERVE end
    local valueType = type(value)
    if valueType == "number" then return 24 end
    if valueType == "boolean" then return 8 end
    if valueType == "string" then return 16 + #value end
    if valueType ~= "table" then return 16 end
    local size = 32
    for key, child in pairs(value) do
        size = size + estimateValue(key, depth + 1) + estimateValue(child, depth + 1)
    end
    return size
end

local function estimateEvent(kind, payload)
    return BASE_EVENT_BYTES + #kind + estimateValue(payload, 0)
end

local function setPartialReason(reason)
    if not runtime.partial_reason then runtime.partial_reason = reason end
end

local function nextSequence()
    runtime.source_sequence = runtime.source_sequence + 1
    EsoWeaveEncounterSaved.last_sequence = runtime.source_sequence
    return runtime.source_sequence
end

local function makeEvent(sequence, kind, payload)
    return {
        session_id = runtime.session_id,
        encounter_id = runtime.encounter_id,
        sequence = sequence,
        monotonic_ms = runtime.elapsed_ms,
        kind = kind,
        payload = payload,
    }
end

local function appendTerminal(kind, payload)
    local sequence = nextSequence()
    local event = makeEvent(sequence, kind, payload)
    local bytes = estimateEvent(kind, payload)
    if #EsoWeaveEncounterSaved.events >= MAX_EVENTS
        or EsoWeaveEncounterSaved.estimated_bytes + bytes > MAX_ESTIMATED_BYTES then
        addWarning("terminal_reserve_exhausted")
        return false
    end
    table.insert(EsoWeaveEncounterSaved.events, event)
    EsoWeaveEncounterSaved.estimated_bytes = EsoWeaveEncounterSaved.estimated_bytes + bytes
    EsoWeaveEncounterSaved.stored_event_count = #EsoWeaveEncounterSaved.events
    if EsoWeaveEncounterSaved.first_sequence == 0 then
        EsoWeaveEncounterSaved.first_sequence = sequence
    end
    EsoWeaveEncounterSaved.ended_monotonic_ms = runtime.elapsed_ms
    return true
end

local function noteOmitted(sequence, reason)
    if not runtime.loss_from then
        runtime.loss_from = sequence
        runtime.loss_reason = reason
    end
    runtime.loss_to = sequence
    EsoWeaveEncounterSaved.omitted_event_count =
        EsoWeaveEncounterSaved.omitted_event_count + 1
    setPartialReason(reason)
end

local function appendRegular(sequence, kind, payload)
    if runtime.loss_from then
        noteOmitted(sequence, runtime.loss_reason)
        return false
    end
    local event = makeEvent(sequence, kind, payload)
    local bytes = estimateEvent(kind, payload)
    local regularLimit = MAX_EVENTS - TERMINAL_EVENT_RESERVE
    local byteLimit = MAX_ESTIMATED_BYTES - TERMINAL_BYTE_RESERVE
    if #EsoWeaveEncounterSaved.events >= regularLimit
        or EsoWeaveEncounterSaved.estimated_bytes + bytes > byteLimit then
        noteOmitted(sequence, "capture-overflow")
        return false
    end
    table.insert(EsoWeaveEncounterSaved.events, event)
    EsoWeaveEncounterSaved.estimated_bytes = EsoWeaveEncounterSaved.estimated_bytes + bytes
    EsoWeaveEncounterSaved.stored_event_count = #EsoWeaveEncounterSaved.events
    if EsoWeaveEncounterSaved.first_sequence == 0 then
        EsoWeaveEncounterSaved.first_sequence = sequence
    end
    EsoWeaveEncounterSaved.ended_monotonic_ms = runtime.elapsed_ms
    return true
end

local function declareClockReset()
    if runtime.loss_from then
        noteOmitted(nextSequence(), runtime.loss_reason)
        return
    end
    local markerPayload = {
        missing_sequence_from = runtime.source_sequence + 1,
        missing_sequence_to = runtime.source_sequence + 1,
        reason = "clock-reset",
    }
    local regularLimit = MAX_EVENTS - TERMINAL_EVENT_RESERVE
    local byteLimit = MAX_ESTIMATED_BYTES - TERMINAL_BYTE_RESERVE
    if #EsoWeaveEncounterSaved.events >= regularLimit
        or EsoWeaveEncounterSaved.estimated_bytes
            + estimateEvent("discontinuity", markerPayload) > byteLimit then
        setPartialReason("clock-reset")
        noteOmitted(nextSequence(), "capture-overflow")
        return
    end
    local missing = nextSequence()
    EsoWeaveEncounterSaved.omitted_event_count =
        EsoWeaveEncounterSaved.omitted_event_count + 1
    setPartialReason("clock-reset")
    local markerSequence = nextSequence()
    markerPayload.missing_sequence_from = missing
    markerPayload.missing_sequence_to = missing
    appendRegular(markerSequence, "discontinuity", markerPayload)
end

local function updateClock()
    local now = GetGameTimeMilliseconds()
    if now < runtime.last_raw_ms then
        runtime.last_raw_ms = now
        runtime.last_boss_sample_raw = now
        runtime.last_performance_sample_raw = now
        declareClockReset()
        return
    end
    runtime.elapsed_ms = runtime.elapsed_ms + (now - runtime.last_raw_ms)
    runtime.last_raw_ms = now
end

local function record(kind, payload)
    if not runtime then return end
    updateClock()
    local sequence = nextSequence()
    appendRegular(sequence, kind, payload)
end

local function actorForKey(key)
    if not key then return 0 end
    local existing = runtime.actor_ids[key]
    if existing then return existing end
    if runtime.actor_count >= MAX_ACTORS then
        addWarning("actor_limit")
        return 0
    end
    runtime.actor_count = runtime.actor_count + 1
    runtime.actor_ids[key] = runtime.actor_count
    return runtime.actor_count
end

local function actorForUnitId(unitId)
    if type(unitId) ~= "number" or unitId <= 0 then return 0 end
    return actorForKey("unit:" .. tostring(unitId))
end

local function actorForUnitTag(unitTag)
    if type(unitTag) ~= "string" or unitTag == "" then return 0 end
    if unitTag == "player" then return actorForKey("role:player") end
    return actorForKey("tag:" .. unitTag)
end

local function actorForCombatUnit(unitId, unitType)
    if type(COMBAT_UNIT_TYPE_PLAYER) == "number"
        and unitType == COMBAT_UNIT_TYPE_PLAYER then
        return actorForKey("role:player")
    end
    return actorForUnitId(unitId)
end

local function unregisterCaptureHandlers()
    EVENT_MANAGER:UnregisterForUpdate(UPDATE_NAMESPACE)
    for _, event in ipairs(CAPTURE_EVENTS) do
        EVENT_MANAGER:UnregisterForEvent(ADDON_NAME, event)
    end
end

local function finishCapture(reason, requestedComplete)
    if not runtime then return end
    unregisterCaptureHandlers()
    updateClock()

    if runtime.loss_from then
        appendTerminal("discontinuity", {
            missing_sequence_from = runtime.loss_from,
            missing_sequence_to = runtime.loss_to,
            reason = runtime.loss_reason,
        })
    end

    local complete = requestedComplete and not runtime.partial_reason
    local terminalReason = complete and reason or (runtime.partial_reason or reason)
    appendTerminal("encounter-end", {
        reason = terminalReason,
        complete = complete,
    })

    EsoWeaveEncounterSaved.status = complete and "complete" or "partial"
    EsoWeaveEncounterSaved.partial_reason = complete and nil or terminalReason
    EsoWeaveEncounterSaved.finished_at = tostring(GetTimeStamp())
    EsoWeaveEncounterSaved.ended_monotonic_ms = runtime.elapsed_ms
    EsoWeaveEncounterSaved.first_sequence =
        EsoWeaveEncounterSaved.events[1] and EsoWeaveEncounterSaved.events[1].sequence or 0
    EsoWeaveEncounterSaved.last_sequence = runtime.source_sequence
    EsoWeaveEncounterSaved.stored_event_count = #EsoWeaveEncounterSaved.events
    runtime = nil
    if complete then
        message("Capture complete. Use /reloadui, logout, or exit before desktop import.")
    else
        message("Capture partial (" .. terminalReason .. "). Use /reloadui, logout, or exit to flush it.")
    end
end

local function failCapture()
    if runtime then finishCapture("callback-failed", false) end
end

local function safely(callback, ...)
    if not runtime then return end
    local ok = pcall(callback, ...)
    if not ok then failCapture() end
end

local function guarded(callback, ...)
    local ok = pcall(callback, ...)
    if not ok and runtime then failCapture() end
end

local function handleCombatEvent(
    _, result, _, _, _, _, _, sourceType, _, targetType, hitValue, powerType,
    damageType, _, sourceUnitId, targetUnitId, abilityId, overflow)
    local common = {
        result = result or 0,
        ability_id = abilityId or 0,
        amount = hitValue or 0,
        overflow = overflow or 0,
        power_type = powerType or 0,
        damage_type = damageType or 0,
        source_actor = actorForCombatUnit(sourceUnitId, sourceType),
        source_type = sourceType or 0,
        target_actor = actorForCombatUnit(targetUnitId, targetType),
        target_type = targetType or 0,
    }
    if DAMAGE_RESULTS[result] then
        record("damage", common)
    elseif HEAL_RESULTS[result] then
        record("healing", common)
    elseif DEATH_RESULTS[result] then
        record("death", {
            actor = common.target_actor,
            source_actor = common.source_actor,
            source = "combat-result",
            result = result,
            ability_id = common.ability_id,
        })
    elseif result == ACTION_RESULT_RESURRECT then
        record("resurrection", {
            actor = common.target_actor,
            source_actor = common.source_actor,
            source = "combat-result",
            result = result,
            ability_id = common.ability_id,
        })
    end
end

local function handleEffectChanged(
    _, changeType, _, _, unitTag, beginTime, endTime, stackCount, _, _,
    effectType, abilityType, statusEffectType, _, unitId, abilityId, sourceType)
    local actor = actorForUnitId(unitId)
    if actor == 0 then actor = actorForUnitTag(unitTag) end
    record("effect", {
        change_type = changeType or 0,
        ability_id = abilityId or 0,
        stack_count = stackCount or 0,
        begin_ms = math.floor((beginTime or 0) * 1000 + 0.5),
        end_ms = math.floor((endTime or 0) * 1000 + 0.5),
        target_actor = actor,
        source_type = sourceType or 0,
        effect_type = effectType or 0,
        ability_type = abilityType or 0,
        status_effect_type = statusEffectType or 0,
    })
end

local function handlePowerUpdate(_, unitTag, _, powerType, value, maximum, effectiveMaximum)
    record("resource", {
        actor = actorForUnitTag(unitTag),
        power_type = powerType or 0,
        value = value or 0,
        maximum = maximum or 0,
        effective_maximum = effectiveMaximum or 0,
    })
end

local function handleActionSlotUsed(_, slot)
    local abilityId = GetSlotBoundId(slot) or 0
    record("cast", { slot = slot or 0, ability_id = abilityId })
    if slot == GetCurrentQuickslot() then
        local quickslotAbilityId =
            GetSlotBoundId(slot, HOTBAR_CATEGORY_QUICKSLOT_WHEEL) or 0
        record("quickslot", {
            action = "used",
            slot = slot or 0,
            ability_id = quickslotAbilityId,
        })
    end
end

local function handleWeaponPairChanged(_, activePair, locked)
    record("bar-change", { active_pair = activePair or 0, locked = locked == true })
end

local function handlePlayerDead()
    record("death", { actor = actorForUnitTag("player"), source = "player-event" })
end

local function handlePlayerAlive()
    record("resurrection", { actor = actorForUnitTag("player"), source = "player-event" })
end

local function handleQuickslotChanged(_, slot)
    record("quickslot", {
        action = "selected",
        slot = slot or 0,
        ability_id = GetSlotBoundId(slot, HOTBAR_CATEGORY_QUICKSLOT_WHEEL) or 0,
    })
end

local function sampleBosses()
    for index = 1, 6 do
        local tag = "boss" .. tostring(index)
        if DoesUnitExist(tag) then
            local value, maximum, effectiveMaximum = GetUnitPower(tag, POWERTYPE_HEALTH)
            local signature = tostring(value) .. ":" .. tostring(maximum) .. ":" .. tostring(effectiveMaximum)
            if runtime.boss_samples[index] ~= signature then
                runtime.boss_samples[index] = signature
                record("boss-health", {
                    boss_index = index,
                    actor = actorForUnitTag(tag),
                    value = value or 0,
                    maximum = maximum or 0,
                    effective_maximum = effectiveMaximum or 0,
                })
            end
        else
            runtime.boss_samples[index] = nil
        end
    end
end

local function handleBossesChanged()
    sampleBosses()
end

local function sampleUpdate()
    local now = GetGameTimeMilliseconds()
    if now - runtime.last_boss_sample_raw >= BOSS_SAMPLE_MS then
        runtime.last_boss_sample_raw = now
        sampleBosses()
    end
    if now - runtime.last_performance_sample_raw >= PERFORMANCE_SAMPLE_MS then
        runtime.last_performance_sample_raw = now
        record("performance", {
            frames_per_second = math.floor((GetFramerate() or 0) + 0.5),
            latency_ms = GetLatency() or 0,
        })
    end
end

local function registerCaptureHandlers()
    local handlers = {
        [EVENT_COMBAT_EVENT] = handleCombatEvent,
        [EVENT_EFFECT_CHANGED] = handleEffectChanged,
        [EVENT_POWER_UPDATE] = handlePowerUpdate,
        [EVENT_ACTION_SLOT_ABILITY_USED] = handleActionSlotUsed,
        [EVENT_ACTIVE_WEAPON_PAIR_CHANGED] = handleWeaponPairChanged,
        [EVENT_PLAYER_DEAD] = handlePlayerDead,
        [EVENT_PLAYER_ALIVE] = handlePlayerAlive,
        [EVENT_BOSSES_CHANGED] = handleBossesChanged,
        [EVENT_ACTIVE_QUICKSLOT_CHANGED] = handleQuickslotChanged,
    }
    for event, handler in pairs(handlers) do
        EVENT_MANAGER:RegisterForEvent(ADDON_NAME, event, function(...)
            safely(handler, ...)
        end)
    end
    EVENT_MANAGER:RegisterForUpdate(UPDATE_NAMESPACE, SAMPLE_UPDATE_MS, function()
        safely(sampleUpdate)
    end)
end

local function boundedText(value, maximum, fallback)
    if type(value) ~= "string" or value == "" then return fallback end
    if #value > maximum then return string.sub(value, 1, maximum) end
    return value
end

local function beginCapture()
    local raw = GetGameTimeMilliseconds()
    local stamp = tostring(GetTimeStamp())
    local sessionId = "session-" .. stamp .. "-" .. tostring(raw)
    local encounterId = "encounter-" .. stamp .. "-" .. tostring(raw)
    local channel = EsoWeaveEncounterSaved.channel
    runtime = {
        session_id = sessionId,
        encounter_id = encounterId,
        source_sequence = 0,
        elapsed_ms = 0,
        last_raw_ms = raw,
        last_boss_sample_raw = raw,
        last_performance_sample_raw = raw,
        actor_ids = {},
        actor_count = 0,
        boss_samples = {},
        partial_reason = nil,
        loss_from = nil,
        loss_to = nil,
        loss_reason = nil,
    }
    EsoWeaveEncounterSaved = {
        schema_version = SCHEMA_VERSION,
        addon_version = ADDON_VERSION,
        status = "capturing",
        channel = channel,
        privacy_profile = PRIVACY_PROFILE,
        source = {
            api_version = GetAPIVersion(),
            game_version = boundedText(
                type(GetESOVersionString) == "function" and GetESOVersionString() or nil,
                128,
                "unknown"),
            locale = boundedText(GetCVar("language.2"), 16, "unknown"),
            platform = boundedText(
                type(GetPlatformServiceType) == "function" and tostring(GetPlatformServiceType()) or nil,
                32,
                "pc"),
        },
        session_id = sessionId,
        encounter_id = encounterId,
        started_at = stamp,
        finished_at = "",
        started_monotonic_ms = raw,
        ended_monotonic_ms = 0,
        first_sequence = 0,
        last_sequence = 0,
        stored_event_count = 0,
        omitted_event_count = 0,
        estimated_bytes = BASE_ENVELOPE_BYTES,
        partial_reason = nil,
        warnings = {},
        events = {},
    }
    appendRegular(nextSequence(), "encounter-start", { reason = "combat-started" })
    registerCaptureHandlers()
    message("Capture started. Names and personal identifiers are omitted.")
end

local function handleCombatState(_, inCombat)
    if runtime and not inCombat then
        finishCapture("combat-ended", true)
        return
    end
    if EsoWeaveEncounterSaved.status ~= "armed" then return end
    if EsoWeaveEncounterSaved.wait_for_clean_boundary then
        if not inCombat then EsoWeaveEncounterSaved.wait_for_clean_boundary = false end
        return
    end
    if inCombat then beginCapture() end
end

local function handlePlayerDeactivated()
    if runtime then finishCapture("player-deactivated", false) end
end

local function arm(arguments)
    if EsoWeaveEncounterSaved.status ~= "idle" then
        message("Clear or disarm the current state before arming another capture.")
        return
    end
    local channel = string.match(arguments or "", "^(%S+)")
    if channel ~= "live" and channel ~= "pts" then
        message("Choose a channel: /ewencounter arm live|pts.")
        return
    end
    EsoWeaveEncounterSaved.status = "armed"
    EsoWeaveEncounterSaved.channel = channel
    EsoWeaveEncounterSaved.wait_for_clean_boundary = IsUnitInCombat("player") == true
    message("Armed for one " .. channel .. " encounter.")
end

local function disarm()
    if EsoWeaveEncounterSaved.status ~= "armed" then
        message("No armed capture is waiting.")
        return
    end
    EsoWeaveEncounterSaved = emptySaved()
    message("Encounter capture disarmed.")
end

local function stop()
    if not runtime then
        message("No encounter capture is active.")
        return
    end
    finishCapture("user-stopped", false)
end

local function clear(arguments)
    if runtime or EsoWeaveEncounterSaved.status == "armed" then
        message("Stop or disarm before clearing capture data.")
        return
    end
    if string.match(arguments or "", "^%s*confirm%s*$") == nil then
        message("Use /ewencounter clear confirm to delete the saved encounter envelope.")
        return
    end
    EsoWeaveEncounterSaved = emptySaved()
    message("Saved encounter envelope cleared locally.")
end

local function showStatus()
    local saved = EsoWeaveEncounterSaved
    message("Status: " .. tostring(saved.status)
        .. ", stored " .. tostring(saved.stored_event_count or 0)
        .. ", omitted " .. tostring(saved.omitted_event_count or 0) .. ".")
end

local function command(arguments)
    local verb, rest = string.match(arguments or "", "^(%S*)%s*(.-)%s*$")
    if verb == "arm" then
        arm(rest)
    elseif verb == "disarm" then
        disarm()
    elseif verb == "stop" then
        stop()
    elseif verb == "status" then
        showStatus()
    elseif verb == "clear" then
        clear(rest)
    else
        message("Use /ewencounter arm live|pts, disarm, stop, status, clear confirm, or help.")
    end
end

local function recoverInterruptedSavedCapture()
    local saved = EsoWeaveEncounterSaved
    if saved.status ~= "capturing" then return end
    saved.events = type(saved.events) == "table" and saved.events or {}
    saved.warnings = type(saved.warnings) == "table" and saved.warnings or {}
    saved.warnings.recovered_interruption =
        (saved.warnings.recovered_interruption or 0) + 1
    local lastEvent = saved.events[#saved.events]
    runtime = {
        session_id = saved.session_id,
        encounter_id = saved.encounter_id,
        source_sequence = saved.last_sequence or (lastEvent and lastEvent.sequence) or 0,
        elapsed_ms = saved.ended_monotonic_ms or (lastEvent and lastEvent.monotonic_ms) or 0,
        last_raw_ms = GetGameTimeMilliseconds(),
        last_boss_sample_raw = GetGameTimeMilliseconds(),
        last_performance_sample_raw = GetGameTimeMilliseconds(),
        actor_ids = {},
        actor_count = 0,
        boss_samples = {},
        partial_reason = "player-deactivated",
        loss_from = nil,
        loss_to = nil,
        loss_reason = nil,
    }
    finishCapture("player-deactivated", false)
end

local function onLoaded(_, addonName)
    if addonName ~= ADDON_NAME then return end
    EVENT_MANAGER:UnregisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED)
    if type(EsoWeaveEncounterSaved) ~= "table"
        or EsoWeaveEncounterSaved.schema_version ~= SCHEMA_VERSION
        or EsoWeaveEncounterSaved.addon_version ~= ADDON_VERSION then
        EsoWeaveEncounterSaved = emptySaved()
    else
        recoverInterruptedSavedCapture()
    end
    SLASH_COMMANDS["/ewencounter"] = command
    EVENT_MANAGER:RegisterForEvent(ADDON_NAME .. "CombatState", EVENT_PLAYER_COMBAT_STATE, function(...)
        guarded(handleCombatState, ...)
    end)
    EVENT_MANAGER:RegisterForEvent(ADDON_NAME .. "Deactivate", EVENT_PLAYER_DEACTIVATED, function(...)
        guarded(handlePlayerDeactivated, ...)
    end)
    message("Loaded dormant. Use /ewencounter arm live|pts for one local anonymous capture.")
end

EVENT_MANAGER:RegisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED, onLoaded)
