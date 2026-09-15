local ADDON_NAME = "EsoWeaveData"
local MODULE_NAMESPACE = ADDON_NAME .. "Encounter"
local ADDON_VERSION = 2
local SCHEMA_VERSION = 2

local MAX_EVENTS = 100000
local MAX_RAW_OBSERVATIONS = 100000
local MAX_ESTIMATED_BYTES = 33554432
local MAX_ACTORS = 4096
local TERMINAL_EVENT_RESERVE = 2
local RAW_TERMINAL_RESERVE = 2
local TERMINAL_BYTE_RESERVE = 4096
local MAX_STRING_BYTES = 65536
local MAX_RAW_VALUES = 256
local BASE_ENVELOPE_BYTES = 512
local BASE_EVENT_BYTES = 160
local SAMPLE_UPDATE_MS = 250
local BOSS_SAMPLE_MS = 500
local PERFORMANCE_SAMPLE_MS = 1000

if type(EsoWeaveEncounterTestLimits) == "table" then
    MAX_EVENTS = EsoWeaveEncounterTestLimits.max_events or MAX_EVENTS
    MAX_RAW_OBSERVATIONS = EsoWeaveEncounterTestLimits.max_raw_observations
        or MAX_RAW_OBSERVATIONS
    MAX_ESTIMATED_BYTES = EsoWeaveEncounterTestLimits.max_estimated_bytes or MAX_ESTIMATED_BYTES
    MAX_ACTORS = EsoWeaveEncounterTestLimits.max_actors or MAX_ACTORS
    MAX_STRING_BYTES = EsoWeaveEncounterTestLimits.max_string_bytes or MAX_STRING_BYTES
end
MAX_RAW_OBSERVATIONS = math.max(3, MAX_RAW_OBSERVATIONS)
MAX_ESTIMATED_BYTES = math.max(8192, MAX_ESTIMATED_BYTES)

local UPDATE_NAMESPACE = MODULE_NAMESPACE .. "Samples"
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
        events = {},
        raw_observations = {},
        warnings = {},
        omitted_event_count = 0,
        raw_observation_count = 0,
        raw_omitted_observation_count = 0,
        raw_first_sequence = 0,
        raw_last_sequence = 0,
        raw_loss = nil,
        stored_event_count = 0,
        estimated_bytes = BASE_ENVELOPE_BYTES,
    }
end

local function addWarning(name)
    local warnings = EsoWeaveDataSaved.encounter.warnings
    warnings[name] = (warnings[name] or 0) + 1
end

local function estimateValue(value, depth)
    if depth > 4 then return TERMINAL_BYTE_RESERVE end
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
    if not runtime.partial_reason then
        runtime.partial_reason = reason
        EsoWeaveDataSaved.encounter.pending_partial_reason = reason
    end
end

local function nextSequence()
    runtime.source_sequence = runtime.source_sequence + 1
    EsoWeaveDataSaved.encounter.last_sequence = runtime.source_sequence
    return runtime.source_sequence
end

local function nextRawSequence()
    runtime.raw_source_sequence = runtime.raw_source_sequence + 1
    EsoWeaveDataSaved.encounter.raw_last_sequence = runtime.raw_source_sequence
    if EsoWeaveDataSaved.encounter.raw_first_sequence == 0 then
        EsoWeaveDataSaved.encounter.raw_first_sequence = runtime.raw_source_sequence
    end
    return runtime.raw_source_sequence
end

local function encodeNumber(value, position)
    if value ~= value or value == math.huge or value == -math.huge then return nil end
    local sign = 1
    if value < 0 or (value == 0 and 1 / value == -math.huge) then sign = -1 end
    local magnitude = math.abs(value)
    if magnitude == 0 then
        return {
            position = position,
            value_type = "number",
            sign = sign,
            significand = "0",
            exponent = 0,
        }
    end
    local fraction, exponent = math.frexp(magnitude)
    local significand = math.floor(fraction * 9007199254740992)
    exponent = exponent - 53
    while significand % 2 == 0 do
        significand = significand / 2
        exponent = exponent + 1
    end
    return {
        position = position,
        value_type = "number",
        sign = sign,
        significand = string.format("%.0f", significand),
        exponent = exponent,
    }
end

local function encodeRawValues(stringLimit, ...)
    local count = select("#", ...)
    if count > MAX_RAW_VALUES then return nil, "record-limit" end
    local values = {}
    for position = 1, count do
        local value = select(position, ...)
        local valueType = type(value)
        local encoded
        if valueType == "nil" then
            encoded = { position = position, value_type = "nil" }
        elseif valueType == "boolean" then
            encoded = { position = position, value_type = "boolean", boolean = value }
        elseif valueType == "string" then
            if #value > stringLimit then return nil, "string-limit" end
            encoded = { position = position, value_type = "string", string = value }
        elseif valueType == "number" then
            encoded = encodeNumber(value, position)
            if not encoded then return nil, "unsupported-value" end
        else
            return nil, "unsupported-value"
        end
        values[position] = encoded
    end
    return values, nil
end

local function noteRawOmitted(sequence, reason)
    if not runtime.raw_loss_from then
        runtime.raw_loss_from = sequence
        runtime.raw_loss_reason = reason
    end
    runtime.raw_loss_to = sequence
    local saved = EsoWeaveDataSaved.encounter
    saved.raw_omitted_observation_count = saved.raw_omitted_observation_count + 1
    saved.raw_loss = {
        missing_sequence_from = runtime.raw_loss_from,
        missing_sequence_to = runtime.raw_loss_to,
        reason = runtime.raw_loss_reason,
    }
    setPartialReason(reason)
end

local function makeRawObservation(sequence, sourceKind, sourceId, sourceCode,
        argumentCount, returnCount, values)
    return {
        session_id = runtime.session_id,
        encounter_id = runtime.encounter_id,
        sequence = sequence,
        monotonic_ms = runtime.elapsed_ms,
        api_version = runtime.api_version,
        source_kind = sourceKind,
        source_id = sourceId,
        source_code = sourceCode,
        source_version = 1,
        argument_count = argumentCount,
        return_count = returnCount,
        values = values,
    }
end

local function appendRaw(sourceKind, sourceId, sourceCode, argumentCount,
        returnCount, terminal, ...)
    if not runtime.raw_enabled then return nil end
    local sequence = nextRawSequence()
    local stringLimit = terminal and 65536 or MAX_STRING_BYTES
    local values, encodingLoss = encodeRawValues(stringLimit, ...)
    if not values then
        noteRawOmitted(sequence, encodingLoss)
        return nil, sequence
    end
    if runtime.raw_loss_from and not terminal then
        noteRawOmitted(sequence, runtime.raw_loss_reason)
        return nil, sequence
    end
    local observation = makeRawObservation(sequence, sourceKind, sourceId,
        sourceCode, argumentCount, returnCount, values)
    local bytes = BASE_EVENT_BYTES + estimateValue(observation, 0)
    local saved = EsoWeaveDataSaved.encounter
    local recordLimit = terminal and MAX_RAW_OBSERVATIONS
        or (MAX_RAW_OBSERVATIONS - RAW_TERMINAL_RESERVE)
    local byteLimit = terminal and MAX_ESTIMATED_BYTES
        or (MAX_ESTIMATED_BYTES - TERMINAL_BYTE_RESERVE)
    if #saved.raw_observations >= recordLimit then
        noteRawOmitted(sequence, "record-limit")
        return nil, sequence
    end
    if saved.estimated_bytes + bytes > byteLimit then
        noteRawOmitted(sequence, "byte-limit")
        return nil, sequence
    end
    table.insert(saved.raw_observations, observation)
    saved.estimated_bytes = saved.estimated_bytes + bytes
    saved.raw_observation_count = #saved.raw_observations
    if saved.raw_first_sequence == 0 then saved.raw_first_sequence = sequence end
    return sequence, nil
end

local function rawApiSample(sourceId, argumentCount, returnCount, ...)
    return appendRaw("api-sample", sourceId, nil, argumentCount, returnCount,
        false, ...)
end

local function rawLifecycle(sourceId, terminal, ...)
    return appendRaw("lifecycle", sourceId, nil, select("#", ...), 0,
        terminal, ...)
end

local function makeEvent(sequence, kind, payload, projection)
    local event = {
        session_id = runtime.session_id,
        encounter_id = runtime.encounter_id,
        sequence = sequence,
        monotonic_ms = runtime.elapsed_ms,
        kind = kind,
        payload = payload,
    }
    if runtime.raw_enabled and projection then
        event.source_sequence = projection.source_sequence
        event.projection_ordinal = projection.projection_ordinal
        projection.projection_ordinal = projection.projection_ordinal + 1
    end
    return event
end

local function appendTerminal(kind, payload, projection)
    local sequence = nextSequence()
    local event = makeEvent(sequence, kind, payload, projection)
    local bytes = estimateEvent(kind, payload)
    if #EsoWeaveDataSaved.encounter.events >= MAX_EVENTS
        or EsoWeaveDataSaved.encounter.estimated_bytes + bytes > MAX_ESTIMATED_BYTES then
        addWarning("terminal_reserve_exhausted")
        return false
    end
    table.insert(EsoWeaveDataSaved.encounter.events, event)
    EsoWeaveDataSaved.encounter.estimated_bytes = EsoWeaveDataSaved.encounter.estimated_bytes + bytes
    EsoWeaveDataSaved.encounter.stored_event_count = #EsoWeaveDataSaved.encounter.events
    if EsoWeaveDataSaved.encounter.first_sequence == 0 then
        EsoWeaveDataSaved.encounter.first_sequence = sequence
    end
    EsoWeaveDataSaved.encounter.ended_monotonic_ms = runtime.elapsed_ms
    return true
end

local function noteOmitted(sequence, reason)
    if not runtime.loss_from then
        runtime.loss_from = sequence
        runtime.loss_reason = reason
    end
    runtime.loss_to = sequence
    EsoWeaveDataSaved.encounter.pending_loss_from = runtime.loss_from
    EsoWeaveDataSaved.encounter.pending_loss_to = runtime.loss_to
    EsoWeaveDataSaved.encounter.pending_loss_reason = runtime.loss_reason
    EsoWeaveDataSaved.encounter.omitted_event_count =
        EsoWeaveDataSaved.encounter.omitted_event_count + 1
    setPartialReason(reason)
end

local function appendRegular(sequence, kind, payload, projection)
    if runtime.loss_from then
        noteOmitted(sequence, runtime.loss_reason)
        return false
    end
    local event = makeEvent(sequence, kind, payload, projection)
    local bytes = estimateEvent(kind, payload)
    local regularLimit = MAX_EVENTS - TERMINAL_EVENT_RESERVE
    local byteLimit = MAX_ESTIMATED_BYTES - TERMINAL_BYTE_RESERVE
    if #EsoWeaveDataSaved.encounter.events >= regularLimit
        or EsoWeaveDataSaved.encounter.estimated_bytes + bytes > byteLimit then
        noteOmitted(sequence, "capture-overflow")
        return false
    end
    table.insert(EsoWeaveDataSaved.encounter.events, event)
    EsoWeaveDataSaved.encounter.estimated_bytes = EsoWeaveDataSaved.encounter.estimated_bytes + bytes
    EsoWeaveDataSaved.encounter.stored_event_count = #EsoWeaveDataSaved.encounter.events
    if EsoWeaveDataSaved.encounter.first_sequence == 0 then
        EsoWeaveDataSaved.encounter.first_sequence = sequence
    end
    EsoWeaveDataSaved.encounter.ended_monotonic_ms = runtime.elapsed_ms
    return true
end

local function declareClockReset(rawSourceSequence)
    local projection = rawSourceSequence and {
        source_sequence = rawSourceSequence,
        projection_ordinal = 0,
    } or nil
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
    if #EsoWeaveDataSaved.encounter.events >= regularLimit
        or EsoWeaveDataSaved.encounter.estimated_bytes
            + estimateEvent("discontinuity", markerPayload) > byteLimit then
        setPartialReason("clock-reset")
        noteOmitted(nextSequence(), "capture-overflow")
        return
    end
    local missing = nextSequence()
    EsoWeaveDataSaved.encounter.omitted_event_count =
        EsoWeaveDataSaved.encounter.omitted_event_count + 1
    setPartialReason("clock-reset")
    local markerSequence = nextSequence()
    markerPayload.missing_sequence_from = missing
    markerPayload.missing_sequence_to = missing
    appendRegular(markerSequence, "discontinuity", markerPayload, projection)
end

local finishCapture

local function updateClock()
    local now = GetGameTimeMilliseconds()
    if now < runtime.last_raw_ms then
        local previous = runtime.last_raw_ms
        runtime.last_raw_ms = now
        runtime.last_boss_sample_raw = now
        runtime.last_performance_sample_raw = now
        local rawSourceSequence = rawLifecycle("clock-reset", true, previous, now)
        if rawSourceSequence then declareClockReset(rawSourceSequence) end
        finishCapture("clock-reset", false)
        return false
    end
    runtime.elapsed_ms = runtime.elapsed_ms + (now - runtime.last_raw_ms)
    runtime.last_raw_ms = now
    return true
end

local function rawCallback(sourceId, handler, ...)
    if not updateClock() then return end
    local argumentCount = select("#", ...)
    local sourceCode = select(1, ...)
    local sourceSequence = appendRaw("callback", sourceId, sourceCode,
        argumentCount, 0, false, ...)
    if not sourceSequence then return end
    handler({ source_sequence = sourceSequence, projection_ordinal = 0 }, ...)
end

local function record(projection, kind, payload)
    if not runtime then return end
    local sequence = nextSequence()
    appendRegular(sequence, kind, payload, projection)
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
        EVENT_MANAGER:UnregisterForEvent(MODULE_NAMESPACE, event)
    end
end

finishCapture = function(reason, requestedComplete)
    if not runtime then return end
    unregisterCaptureHandlers()
    if not updateClock() then return end

    local finishSourceSequence = rawLifecycle("capture-finish", true,
        reason, requestedComplete == true)
    local finishProjection = finishSourceSequence and {
        source_sequence = finishSourceSequence,
        projection_ordinal = 0,
    } or nil

    if runtime.loss_from then
        appendTerminal("discontinuity", {
            missing_sequence_from = runtime.loss_from,
            missing_sequence_to = runtime.loss_to,
            reason = runtime.loss_reason,
        }, finishProjection)
    end

    local complete = requestedComplete and not runtime.partial_reason
    local terminalReason = complete and reason or (runtime.partial_reason or reason)
    appendTerminal("encounter-end", {
        reason = terminalReason,
        complete = complete,
    }, finishProjection)

    EsoWeaveDataSaved.encounter.pending_partial_reason = nil
    EsoWeaveDataSaved.encounter.pending_loss_from = nil
    EsoWeaveDataSaved.encounter.pending_loss_to = nil
    EsoWeaveDataSaved.encounter.pending_loss_reason = nil
    EsoWeaveDataSaved.encounter.status = complete and "complete" or "partial"
    if complete then
        EsoWeaveDataSaved.encounter.partial_reason = nil
    else
        EsoWeaveDataSaved.encounter.partial_reason = terminalReason
    end
    EsoWeaveDataSaved.encounter.finished_at = tostring(GetTimeStamp())
    EsoWeaveDataSaved.encounter.ended_monotonic_ms = runtime.elapsed_ms
    EsoWeaveDataSaved.encounter.first_sequence =
        EsoWeaveDataSaved.encounter.events[1] and EsoWeaveDataSaved.encounter.events[1].sequence or 0
    EsoWeaveDataSaved.encounter.last_sequence = runtime.source_sequence
    EsoWeaveDataSaved.encounter.stored_event_count = #EsoWeaveDataSaved.encounter.events
    runtime = nil
    if complete then
        message("Capture complete. Use /reloadui, logout, or exit before desktop import.")
    else
        message("Capture partial (" .. terminalReason .. "). Use /reloadui, logout, or exit to flush it.")
    end
end

local function failCapture()
    if runtime then
        if runtime.raw_enabled then
            noteRawOmitted(nextRawSequence(), "callback-failed")
        end
        finishCapture("callback-failed", false)
    end
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
    projection, _, result, _, _, _, _, _, sourceType, _, targetType, hitValue, powerType,
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
        record(projection, "damage", common)
    elseif HEAL_RESULTS[result] then
        record(projection, "healing", common)
    elseif DEATH_RESULTS[result] then
        record(projection, "death", {
            actor = common.target_actor,
            source_actor = common.source_actor,
            source = "combat-result",
            result = result,
            ability_id = common.ability_id,
        })
    elseif result == ACTION_RESULT_RESURRECT then
        record(projection, "resurrection", {
            actor = common.target_actor,
            source_actor = common.source_actor,
            source = "combat-result",
            result = result,
            ability_id = common.ability_id,
        })
    end
end

local function handleEffectChanged(
    projection, _, changeType, _, _, unitTag, beginTime, endTime, stackCount, _, _,
    effectType, abilityType, statusEffectType, _, unitId, abilityId, sourceType)
    local actor = actorForUnitId(unitId)
    if actor == 0 then actor = actorForUnitTag(unitTag) end
    record(projection, "effect", {
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

local function handlePowerUpdate(projection, _, unitTag, _, powerType, value, maximum, effectiveMaximum)
    record(projection, "resource", {
        actor = actorForUnitTag(unitTag),
        power_type = powerType or 0,
        value = value or 0,
        maximum = maximum or 0,
        effective_maximum = effectiveMaximum or 0,
    })
end

local function handleActionSlotUsed(projection, _, slot)
    local rawAbilityId = GetSlotBoundId(slot)
    if not rawApiSample("GetSlotBoundId", 1, 1, slot, rawAbilityId) then return end
    local abilityId = rawAbilityId or 0
    record(projection, "cast", { slot = slot or 0, ability_id = abilityId })
    local currentQuickslot = GetCurrentQuickslot()
    if not rawApiSample("GetCurrentQuickslot", 0, 1, currentQuickslot) then return end
    if slot == currentQuickslot then
        local rawQuickslotAbilityId = GetSlotBoundId(
            slot, HOTBAR_CATEGORY_QUICKSLOT_WHEEL)
        if not rawApiSample("GetSlotBoundId", 2, 1, slot,
            HOTBAR_CATEGORY_QUICKSLOT_WHEEL, rawQuickslotAbilityId) then return end
        local quickslotAbilityId = rawQuickslotAbilityId or 0
        record(projection, "quickslot", {
            action = "used",
            slot = slot or 0,
            ability_id = quickslotAbilityId,
        })
    end
end

local function handleWeaponPairChanged(projection, _, activePair, locked)
    record(projection, "bar-change", { active_pair = activePair or 0, locked = locked == true })
end

local function handlePlayerDead(projection)
    record(projection, "death", { actor = actorForUnitTag("player"), source = "player-event" })
end

local function handlePlayerAlive(projection)
    record(projection, "resurrection", { actor = actorForUnitTag("player"), source = "player-event" })
end

local function handleQuickslotChanged(projection, _, slot)
    local rawAbilityId = GetSlotBoundId(slot, HOTBAR_CATEGORY_QUICKSLOT_WHEEL)
    if not rawApiSample("GetSlotBoundId", 2, 1, slot,
        HOTBAR_CATEGORY_QUICKSLOT_WHEEL, rawAbilityId) then return end
    local abilityId = rawAbilityId or 0
    record(projection, "quickslot", {
        action = "selected",
        slot = slot or 0,
        ability_id = abilityId,
    })
end

local function sampleBosses(parentProjection)
    for index = 1, 6 do
        local tag = "boss" .. tostring(index)
        local exists = DoesUnitExist(tag)
        if not rawApiSample("DoesUnitExist", 1, 1, tag, exists) then return end
        if exists == true then
            local value, maximum, effectiveMaximum = GetUnitPower(tag, POWERTYPE_HEALTH)
            local powerSourceSequence = rawApiSample("GetUnitPower", 2, 3,
                tag, POWERTYPE_HEALTH, value, maximum, effectiveMaximum)
            if not powerSourceSequence then return end
            local signature = tostring(value) .. ":" .. tostring(maximum) .. ":" .. tostring(effectiveMaximum)
            if runtime.boss_samples[index] ~= signature then
                runtime.boss_samples[index] = signature
                local projection = parentProjection or {
                    source_sequence = powerSourceSequence,
                    projection_ordinal = 0,
                }
                record(projection, "boss-health", {
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

local function handleBossesChanged(projection)
    sampleBosses(projection)
end

local function sampleUpdate()
    if not updateClock() then return end
    local now = GetGameTimeMilliseconds()
    if now - runtime.last_boss_sample_raw >= BOSS_SAMPLE_MS then
        runtime.last_boss_sample_raw = now
        sampleBosses()
    end
    if now - runtime.last_performance_sample_raw >= PERFORMANCE_SAMPLE_MS then
        runtime.last_performance_sample_raw = now
        local rawFramesPerSecond = GetFramerate()
        local frameSourceSequence = rawApiSample("GetFramerate", 0, 1,
            rawFramesPerSecond)
        if not frameSourceSequence then return end
        local rawLatency = GetLatency()
        if not rawApiSample("GetLatency", 0, 1, rawLatency) then return end
        local framesPerSecond = rawFramesPerSecond or 0
        local latency = rawLatency or 0
        record({ source_sequence = frameSourceSequence, projection_ordinal = 0 },
            "performance", {
            frames_per_second = math.floor(framesPerSecond + 0.5),
            latency_ms = latency,
        })
    end
end

local function registerCaptureHandlers()
    local handlers = {
        [EVENT_COMBAT_EVENT] = { "EVENT_COMBAT_EVENT", handleCombatEvent },
        [EVENT_EFFECT_CHANGED] = { "EVENT_EFFECT_CHANGED", handleEffectChanged },
        [EVENT_POWER_UPDATE] = { "EVENT_POWER_UPDATE", handlePowerUpdate },
        [EVENT_ACTION_SLOT_ABILITY_USED] = {
            "EVENT_ACTION_SLOT_ABILITY_USED", handleActionSlotUsed,
        },
        [EVENT_ACTIVE_WEAPON_PAIR_CHANGED] = {
            "EVENT_ACTIVE_WEAPON_PAIR_CHANGED", handleWeaponPairChanged,
        },
        [EVENT_PLAYER_DEAD] = { "EVENT_PLAYER_DEAD", handlePlayerDead },
        [EVENT_PLAYER_ALIVE] = { "EVENT_PLAYER_ALIVE", handlePlayerAlive },
        [EVENT_BOSSES_CHANGED] = { "EVENT_BOSSES_CHANGED", handleBossesChanged },
        [EVENT_ACTIVE_QUICKSLOT_CHANGED] = {
            "EVENT_ACTIVE_QUICKSLOT_CHANGED", handleQuickslotChanged,
        },
    }
    for event, registration in pairs(handlers) do
        local sourceId = registration[1]
        local handler = registration[2]
        EVENT_MANAGER:RegisterForEvent(MODULE_NAMESPACE, event, function(...)
            safely(rawCallback, sourceId, handler, ...)
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

local function beginCapture(...)
    local raw = GetGameTimeMilliseconds()
    local stamp = tostring(GetTimeStamp())
    local sessionId = "session-" .. stamp .. "-" .. tostring(raw)
    local encounterId = "encounter-" .. stamp .. "-" .. tostring(raw)
    local channel = EsoWeaveDataSaved.encounter.channel
    local apiVersion = GetAPIVersion()
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
        raw_enabled = true,
        raw_source_sequence = 0,
        raw_loss_from = nil,
        raw_loss_to = nil,
        raw_loss_reason = nil,
        api_version = apiVersion,
    }
    EsoWeaveDataSaved.encounter = {
        schema_version = SCHEMA_VERSION,
        addon_version = ADDON_VERSION,
        status = "capturing",
        channel = channel,
        source = {
            api_version = apiVersion,
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
        raw_first_sequence = 0,
        raw_last_sequence = 0,
        raw_observation_count = 0,
        raw_omitted_observation_count = 0,
        raw_loss = nil,
        estimated_bytes = BASE_ENVELOPE_BYTES,
        partial_reason = nil,
        warnings = {},
        events = {},
        raw_observations = {},
    }
    local argumentCount = select("#", ...)
    local sourceCode = select(1, ...)
    local sourceSequence, omittedSourceSequence = appendRaw(
        "callback", "EVENT_PLAYER_COMBAT_STATE",
        sourceCode, argumentCount, 0, false, ...)
    local projectionSourceSequence = sourceSequence or omittedSourceSequence
    local projection = projectionSourceSequence and {
        source_sequence = projectionSourceSequence,
        projection_ordinal = 0,
    } or nil
    appendRegular(nextSequence(), "encounter-start",
        { reason = "combat-started" }, projection)
    registerCaptureHandlers()
    message("Capture started. Exact local callback values can include names and identifiers.")
end

local function handleCombatState(...)
    local inCombat = select(2, ...)
    if runtime and not inCombat then
        if not updateClock() then return end
        appendRaw("callback", "EVENT_PLAYER_COMBAT_STATE", select(1, ...),
            select("#", ...), 0, false, ...)
        finishCapture("combat-ended", true)
        return
    end
    if runtime then
        if not updateClock() then return end
        appendRaw("callback", "EVENT_PLAYER_COMBAT_STATE", select(1, ...),
            select("#", ...), 0, false, ...)
        return
    end
    if EsoWeaveDataSaved.encounter.status ~= "armed" then return end
    if EsoWeaveDataSaved.encounter.wait_for_clean_boundary then
        if not inCombat then EsoWeaveDataSaved.encounter.wait_for_clean_boundary = false end
        return
    end
    if inCombat then beginCapture(...) end
end

local function handlePlayerDeactivated(...)
    if runtime then
        if not updateClock() then return end
        appendRaw("callback", "EVENT_PLAYER_DEACTIVATED", select(1, ...),
            select("#", ...), 0, false, ...)
        finishCapture("player-deactivated", false)
    end
end

local function arm(arguments)
    if EsoWeaveDataSaved.encounter.status ~= "idle" then
        message("Clear or disarm the current state before arming another capture.")
        return
    end
    local channel = string.match(arguments or "", "^(%S+)")
    if channel ~= "live" and channel ~= "pts" then
        message("Choose a channel: /ewencounter arm live|pts.")
        return
    end
    EsoWeaveDataSaved.encounter.status = "armed"
    EsoWeaveDataSaved.encounter.channel = channel
    EsoWeaveDataSaved.encounter.wait_for_clean_boundary = IsUnitInCombat("player") == true
    message("Armed for one " .. channel .. " encounter.")
end

local function disarm()
    if EsoWeaveDataSaved.encounter.status ~= "armed" then
        message("No armed capture is waiting.")
        return
    end
    EsoWeaveDataSaved.encounter = emptySaved()
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
    if runtime or EsoWeaveDataSaved.encounter.status == "armed" then
        message("Stop or disarm before clearing capture data.")
        return
    end
    if string.match(arguments or "", "^%s*confirm%s*$") == nil then
        message("Use /ewencounter clear confirm to delete the saved encounter envelope.")
        return
    end
    EsoWeaveDataSaved.encounter = emptySaved()
    message("Saved encounter envelope cleared locally.")
end

local function showStatus()
    local saved = EsoWeaveDataSaved.encounter
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
    local saved = EsoWeaveDataSaved.encounter
    if saved.status ~= "capturing" then return end
    saved.events = type(saved.events) == "table" and saved.events or {}
    saved.warnings = type(saved.warnings) == "table" and saved.warnings or {}
    saved.warnings.recovered_interruption =
        (saved.warnings.recovered_interruption or 0) + 1
    local lastEvent = saved.events[#saved.events]
    local sourceSequence = saved.last_sequence or (lastEvent and lastEvent.sequence) or 0
    local pendingReason = saved.pending_partial_reason
    if pendingReason ~= "capture-overflow" and pendingReason ~= "clock-reset"
        and pendingReason ~= "record-limit" and pendingReason ~= "byte-limit"
        and pendingReason ~= "string-limit" and pendingReason ~= "unsupported-value"
        and pendingReason ~= "callback-failed" then
        pendingReason = "player-deactivated"
    end
    local lossFrom = saved.pending_loss_from
    local lossTo = saved.pending_loss_to
    local lossReason = saved.pending_loss_reason
    if type(lossFrom) ~= "number" or lossFrom < 1 or lossFrom ~= math.floor(lossFrom)
        or type(lossTo) ~= "number" or lossTo < lossFrom
        or lossTo ~= math.floor(lossTo) or lossTo > sourceSequence
        or lossReason ~= "capture-overflow" then
        lossFrom = nil
        lossTo = nil
        lossReason = nil
    end
    runtime = {
        session_id = saved.session_id,
        encounter_id = saved.encounter_id,
        source_sequence = sourceSequence,
        elapsed_ms = saved.ended_monotonic_ms or (lastEvent and lastEvent.monotonic_ms) or 0,
        last_raw_ms = GetGameTimeMilliseconds(),
        last_boss_sample_raw = GetGameTimeMilliseconds(),
        last_performance_sample_raw = GetGameTimeMilliseconds(),
        actor_ids = {},
        actor_count = 0,
        boss_samples = {},
        partial_reason = pendingReason,
        loss_from = lossFrom,
        loss_to = lossTo,
        loss_reason = lossReason,
        raw_enabled = saved.schema_version == SCHEMA_VERSION,
        raw_source_sequence = saved.raw_last_sequence or 0,
        raw_loss_from = saved.raw_loss and saved.raw_loss.missing_sequence_from or nil,
        raw_loss_to = saved.raw_loss and saved.raw_loss.missing_sequence_to or nil,
        raw_loss_reason = saved.raw_loss and saved.raw_loss.reason or nil,
        api_version = saved.source and saved.source.api_version or GetAPIVersion(),
    }
    finishCapture("player-deactivated", false)
end

local function onLoaded(_, addonName)
    if addonName ~= ADDON_NAME then return end
    EVENT_MANAGER:UnregisterForEvent(MODULE_NAMESPACE, EVENT_ADD_ON_LOADED)
    if type(EsoWeaveDataSaved) ~= "table" then EsoWeaveDataSaved = {} end
    local saved = EsoWeaveDataSaved.encounter
    if type(saved) ~= "table" then
        EsoWeaveDataSaved.encounter = emptySaved()
    elseif saved.schema_version == 1 and saved.addon_version == 1
        and (saved.status == "complete" or saved.status == "partial") then
        -- Terminal v1 evidence remains byte-for-byte owned by the user.
    elseif saved.schema_version == 1 and saved.addon_version == 1
        and saved.status == "capturing" then
        recoverInterruptedSavedCapture()
    elseif saved.schema_version == 1 and saved.addon_version == 1
        and saved.status == "armed" then
        local channel = saved.channel
        local waitForCleanBoundary = saved.wait_for_clean_boundary == true
        EsoWeaveDataSaved.encounter = emptySaved()
        EsoWeaveDataSaved.encounter.status = "armed"
        EsoWeaveDataSaved.encounter.channel = channel
        EsoWeaveDataSaved.encounter.wait_for_clean_boundary = waitForCleanBoundary
    elseif saved.schema_version == 1 and saved.addon_version == 1
        and saved.status == "idle" then
        EsoWeaveDataSaved.encounter = emptySaved()
    elseif saved.schema_version ~= SCHEMA_VERSION
        or saved.addon_version ~= ADDON_VERSION then
        -- Unknown user-owned evidence is preserved until explicit clear.
    else
        recoverInterruptedSavedCapture()
    end
    SLASH_COMMANDS["/ewencounter"] = command
    EVENT_MANAGER:RegisterForEvent(MODULE_NAMESPACE .. "CombatState", EVENT_PLAYER_COMBAT_STATE, function(...)
        guarded(handleCombatState, ...)
    end)
    EVENT_MANAGER:RegisterForEvent(MODULE_NAMESPACE .. "Deactivate", EVENT_PLAYER_DEACTIVATED, function(...)
        guarded(handlePlayerDeactivated, ...)
    end)
    message("Loaded dormant. Use /ewencounter arm live|pts for one exact local capture.")
end

EVENT_MANAGER:RegisterForEvent(MODULE_NAMESPACE, EVENT_ADD_ON_LOADED, onLoaded)
