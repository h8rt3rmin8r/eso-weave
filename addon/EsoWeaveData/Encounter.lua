local ADDON_NAME = "EsoWeaveData"
local MODULE_NAMESPACE = ADDON_NAME .. "Encounter"
local ADDON_VERSION = 3
local SCHEMA_VERSION = 2
local CONTROLLER_ADDON_VERSION = 4
local STATE_SCHEMA_VERSION = 1

local MAX_EVENTS = 100000
local MAX_RAW_OBSERVATIONS = 100000
local MAX_ESTIMATED_BYTES = 33554432
local MAX_ACTORS = 4096
local MAX_EXACT_INTEGER = 9007199254740991
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
local MAX_SESSION_ENCOUNTERS = 1024
local MAX_INTERRUPTION_MARKERS = 1024
local OUTER_FAILURE_BYTE_RESERVE = 1024

if type(EsoWeaveEncounterTestLimits) == "table" then
    MAX_EVENTS = EsoWeaveEncounterTestLimits.max_events or MAX_EVENTS
    MAX_RAW_OBSERVATIONS = EsoWeaveEncounterTestLimits.max_raw_observations
        or MAX_RAW_OBSERVATIONS
    MAX_ESTIMATED_BYTES = EsoWeaveEncounterTestLimits.max_estimated_bytes or MAX_ESTIMATED_BYTES
    MAX_ACTORS = EsoWeaveEncounterTestLimits.max_actors or MAX_ACTORS
    MAX_STRING_BYTES = EsoWeaveEncounterTestLimits.max_string_bytes or MAX_STRING_BYTES
    MAX_SESSION_ENCOUNTERS = EsoWeaveEncounterTestLimits.max_session_encounters
        or MAX_SESSION_ENCOUNTERS
    MAX_INTERRUPTION_MARKERS = EsoWeaveEncounterTestLimits.max_interruptions
        or MAX_INTERRUPTION_MARKERS
end
MAX_RAW_OBSERVATIONS = math.max(4, MAX_RAW_OBSERVATIONS)
MAX_ESTIMATED_BYTES = math.max(8192, MAX_ESTIMATED_BYTES)

local UPDATE_NAMESPACE = MODULE_NAMESPACE .. "Samples"
local runtime = nil
local controllerInvalid = false

local function state()
    return EsoWeaveDataSaved.encounter
end

local function capture()
    return runtime and runtime.capture or nil
end

local function touch()
    local saved = state()
    saved.revision = (saved.revision or 0) + 1
end

local function ordinalKey(ordinal)
    return string.format("%010d", ordinal)
end

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

local function normalizationProfile(apiVersion)
    return {
        version = 1,
        api_version = apiVersion,
        player_combat_unit_type = COMBAT_UNIT_TYPE_PLAYER,
        health_power_type = POWERTYPE_HEALTH,
        quickslot_category = HOTBAR_CATEGORY_QUICKSLOT_WHEEL,
        damage_results = {
            ACTION_RESULT_DAMAGE,
            ACTION_RESULT_CRITICAL_DAMAGE,
            ACTION_RESULT_DOT_TICK,
            ACTION_RESULT_DOT_TICK_CRITICAL,
            ACTION_RESULT_BLOCKED_DAMAGE,
            ACTION_RESULT_DAMAGE_SHIELDED,
        },
        healing_results = {
            ACTION_RESULT_HEAL,
            ACTION_RESULT_CRITICAL_HEAL,
            ACTION_RESULT_HOT_TICK,
            ACTION_RESULT_HOT_TICK_CRITICAL,
        },
        death_results = { ACTION_RESULT_DIED, ACTION_RESULT_DIED_XP },
        resurrect_result = ACTION_RESULT_RESURRECT,
    }
end

local function message(text)
    d("[ESO Weave Encounter] " .. text)
end

local function emptyController(selectedMode, selectedChannel)
    return {
        state_schema_version = STATE_SCHEMA_VERSION,
        addon_version = CONTROLLER_ADDON_VERSION,
        selected_mode = selectedMode == "continuous" and "continuous" or "single",
        selected_channel = selectedChannel,
        requested_mode = nil,
        active_mode = nil,
        state = "stopped",
        current_encounter_id = nil,
        session = nil,
        current = nil,
        records = {},
        interruptions = {},
        stop_reason = "never-started",
        failure = nil,
        revision = 1,
    }
end

local function addWarning(name)
    local warnings = capture().warnings
    warnings[name] = (warnings[name] or 0) + 1
    touch()
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
        capture().pending_partial_reason = reason
        touch()
    end
end

local function nextSequence()
    runtime.source_sequence = runtime.source_sequence + 1
    capture().last_sequence = runtime.source_sequence
    touch()
    return runtime.source_sequence
end

local function nextRawSequence()
    runtime.raw_source_sequence = runtime.raw_source_sequence + 1
    capture().raw_last_sequence = runtime.raw_source_sequence
    if capture().raw_first_sequence == 0 then
        capture().raw_first_sequence = runtime.raw_source_sequence
    end
    touch()
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
    local saved = capture()
    saved.raw_omitted_observation_count = saved.raw_omitted_observation_count + 1
    saved.raw_loss = {
        missing_sequence_from = runtime.raw_loss_from,
        missing_sequence_to = runtime.raw_loss_to,
        reason = runtime.raw_loss_reason,
    }
    touch()
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
    local saved = capture()
    local recordLimit = terminal and MAX_RAW_OBSERVATIONS
        or (MAX_RAW_OBSERVATIONS - RAW_TERMINAL_RESERVE)
    local byteLimit = terminal
        and (MAX_ESTIMATED_BYTES - OUTER_FAILURE_BYTE_RESERVE)
        or (MAX_ESTIMATED_BYTES - TERMINAL_BYTE_RESERVE
            - OUTER_FAILURE_BYTE_RESERVE)
    if state().session.aggregate_raw_observation_count >= recordLimit then
        noteRawOmitted(sequence, "record-limit")
        runtime.hard_failure = "storage-pressure"
        return nil, sequence
    end
    if state().session.aggregate_estimated_bytes + bytes > byteLimit then
        noteRawOmitted(sequence, "byte-limit")
        runtime.hard_failure = "storage-pressure"
        return nil, sequence
    end
    table.insert(saved.raw_observations, observation)
    saved.estimated_bytes = saved.estimated_bytes + bytes
    state().session.aggregate_estimated_bytes =
        state().session.aggregate_estimated_bytes + bytes
    state().session.aggregate_raw_observation_count =
        state().session.aggregate_raw_observation_count + 1
    saved.raw_observation_count = #saved.raw_observations
    if saved.raw_first_sequence == 0 then saved.raw_first_sequence = sequence end
    touch()
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
    local saved = capture()
    if state().session.aggregate_event_count >= MAX_EVENTS
        or state().session.aggregate_estimated_bytes + bytes
            > MAX_ESTIMATED_BYTES - OUTER_FAILURE_BYTE_RESERVE then
        addWarning("terminal_reserve_exhausted")
        runtime.terminal_failed = true
        return false
    end
    table.insert(saved.events, event)
    saved.estimated_bytes = saved.estimated_bytes + bytes
    state().session.aggregate_estimated_bytes =
        state().session.aggregate_estimated_bytes + bytes
    state().session.aggregate_event_count = state().session.aggregate_event_count + 1
    saved.stored_event_count = #saved.events
    if saved.first_sequence == 0 then
        saved.first_sequence = sequence
    end
    saved.ended_monotonic_ms = runtime.elapsed_ms
    touch()
    return true
end

local function noteOmitted(sequence, reason)
    if not runtime.loss_from then
        runtime.loss_from = sequence
        runtime.loss_reason = reason
    end
    runtime.loss_to = sequence
    capture().pending_loss_from = runtime.loss_from
    capture().pending_loss_to = runtime.loss_to
    capture().pending_loss_reason = runtime.loss_reason
    capture().omitted_event_count = capture().omitted_event_count + 1
    setPartialReason(reason)
    touch()
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
        - OUTER_FAILURE_BYTE_RESERVE
    local saved = capture()
    if state().session.aggregate_event_count >= regularLimit
        or state().session.aggregate_estimated_bytes + bytes > byteLimit then
        noteOmitted(sequence, "capture-overflow")
        runtime.hard_failure = "storage-pressure"
        return false
    end
    table.insert(saved.events, event)
    saved.estimated_bytes = saved.estimated_bytes + bytes
    state().session.aggregate_estimated_bytes =
        state().session.aggregate_estimated_bytes + bytes
    state().session.aggregate_event_count = state().session.aggregate_event_count + 1
    saved.stored_event_count = #saved.events
    if saved.first_sequence == 0 then
        saved.first_sequence = sequence
    end
    saved.ended_monotonic_ms = runtime.elapsed_ms
    touch()
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
        - OUTER_FAILURE_BYTE_RESERVE
    if state().session.aggregate_event_count >= regularLimit
        or state().session.aggregate_estimated_bytes
            + estimateEvent("discontinuity", markerPayload) > byteLimit then
        setPartialReason("clock-reset")
        noteOmitted(nextSequence(), "capture-overflow")
        return
    end
    local missing = nextSequence()
    capture().omitted_event_count = capture().omitted_event_count + 1
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
        runtime.hard_failure = "clock-reset"
        return false
    end
    runtime.elapsed_ms = runtime.elapsed_ms + (now - runtime.last_raw_ms)
    runtime.last_raw_ms = now
    return true
end

local function failSession(reason, encounterOrdinal)
    local saved = state()
    saved.requested_mode = nil
    saved.active_mode = nil
    saved.state = "failed"
    saved.current_encounter_id = nil
    saved.current = nil
    saved.stop_reason = nil
    saved.failure = {
        reason = reason,
        occurred_at = tostring(GetTimeStamp()),
        encounter_ordinal = encounterOrdinal,
    }
    if saved.session then
        saved.session.status = "failed"
        saved.session.finished_at = tostring(GetTimeStamp())
    end
    touch()
    message("Capture failed (" .. reason .. "). Retained encounter data was preserved.")
end

local function retainCapture(finished, ordinal)
    local saved = state()
    saved.records[ordinalKey(ordinal)] = { ordinal = ordinal, capture = finished }
    saved.current = nil
    saved.current_encounter_id = nil
    saved.session.completed_encounter_count =
        saved.session.completed_encounter_count + 1
    if finished.status == "partial" then
        saved.session.degraded_encounter_count =
            saved.session.degraded_encounter_count + 1
    end
    touch()
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
    if type(unitId) ~= "number" or unitId <= 0
        or unitId > MAX_EXACT_INTEGER or unitId ~= math.floor(unitId) then
        return 0
    end
    return actorForKey("unit:" .. string.format("%.0f", unitId))
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
    if reason ~= "clock-reset" and not updateClock()
        and runtime.hard_failure ~= "clock-reset" then return end

    local finished = runtime.capture
    local mode = runtime.mode
    local ordinal = runtime.ordinal

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

    finished.pending_partial_reason = nil
    finished.pending_loss_from = nil
    finished.pending_loss_to = nil
    finished.pending_loss_reason = nil
    finished.status = complete and "complete" or "partial"
    if complete then
        finished.partial_reason = nil
    else
        finished.partial_reason = terminalReason
    end
    finished.finished_at = tostring(GetTimeStamp())
    finished.ended_monotonic_ms = runtime.elapsed_ms
    finished.first_sequence = finished.events[1] and finished.events[1].sequence or 0
    finished.last_sequence = runtime.source_sequence
    finished.stored_event_count = #finished.events
    local terminalFailed = runtime.terminal_failed == true
    runtime = nil
    return finished, mode, ordinal, terminalFailed
end

local function finishAndRetain(reason, requestedComplete)
    local finished, mode, ordinal, terminalFailed = finishCapture(reason, requestedComplete)
    if not finished then return nil, nil, nil end
    if terminalFailed then
        failSession("terminal-reserve-exhausted", ordinal)
        return nil, mode, ordinal
    end
    retainCapture(finished, ordinal)
    return finished, mode, ordinal
end

local function hardFailActive(reason)
    if not runtime then
        failSession(reason, nil)
        return
    end
    local terminalReason = runtime.partial_reason or reason
    local _, _, ordinal = finishAndRetain(terminalReason, false)
    if state().state ~= "failed" then failSession(reason, ordinal) end
end

local function failCapture()
    if runtime then
        if runtime.raw_enabled then
            noteRawOmitted(nextRawSequence(), "callback-failed")
        end
        setPartialReason("callback-failed")
        hardFailActive("callback-failed")
    end
end

local function safely(callback, ...)
    if not runtime then return end
    local ok = pcall(callback, ...)
    if not ok then
        failCapture()
    elseif runtime and runtime.hard_failure then
        hardFailActive(runtime.hard_failure)
    end
end

local function guarded(callback, ...)
    local ok = pcall(callback, ...)
    if not ok and runtime then
        failCapture()
    elseif runtime and runtime.hard_failure then
        hardFailActive(runtime.hard_failure)
    end
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

local function estimatedOpeningBytes(startedMidCombat, ...)
    local sourceKind = startedMidCombat and "api-sample" or "callback"
    local sourceId = startedMidCombat and "IsUnitInCombat"
        or "EVENT_PLAYER_COMBAT_STATE"
    local argumentCount = startedMidCombat and 1 or select("#", ...)
    local returnCount = startedMidCombat and 1 or 0
    local sourceCode = startedMidCombat and nil or select(1, ...)
    local values
    if startedMidCombat then
        values = encodeRawValues(MAX_STRING_BYTES, "player", true)
    else
        values = encodeRawValues(MAX_STRING_BYTES, ...)
    end
    local rawBytes = 0
    if values then
        rawBytes = BASE_EVENT_BYTES + estimateValue({
            session_id = state().session.session_id,
            encounter_id = string.rep("0", 128),
            sequence = 1,
            monotonic_ms = 0,
            api_version = GetAPIVersion(),
            source_kind = sourceKind,
            source_id = sourceId,
            source_code = sourceCode,
            source_version = 1,
            argument_count = argumentCount,
            return_count = returnCount,
            values = values,
        }, 0)
    end
    local reason = startedMidCombat and "started-mid-combat" or "combat-started"
    return BASE_ENVELOPE_BYTES + rawBytes
        + estimateEvent("encounter-start", { reason = reason })
end

local function canBeginCapture(startedMidCombat, ...)
    local saved = state()
    local session = saved.session
    return session
        and session.completed_encounter_count < MAX_SESSION_ENCOUNTERS
        and session.aggregate_event_count <= MAX_EVENTS - 3
        and session.aggregate_raw_observation_count <= MAX_RAW_OBSERVATIONS - 4
        and session.aggregate_estimated_bytes
            + estimatedOpeningBytes(startedMidCombat == true, ...)
            + TERMINAL_BYTE_RESERVE
            <= MAX_ESTIMATED_BYTES - OUTER_FAILURE_BYTE_RESERVE
end

local function beginCapture(startedMidCombat, ...)
    if not canBeginCapture(startedMidCombat, ...) then
        failSession("storage-pressure", nil)
        return
    end
    local saved = state()
    local session = saved.session
    local raw = GetGameTimeMilliseconds()
    local stamp = tostring(GetTimeStamp())
    local ordinal = session.next_encounter_ordinal
    local sessionSuffix = string.match(session.session_id,
        "^session%-%d+%-(%d+)$") or tostring(raw)
    local encounterId = "encounter-" .. session.started_at .. "-"
        .. sessionSuffix .. string.format("%04d", ordinal)
    local channel = session.channel
    local apiVersion = GetAPIVersion()
    local working = {
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
        normalization_profile = normalizationProfile(apiVersion),
        session_id = session.session_id,
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
    runtime = {
        capture = working,
        mode = session.mode,
        ordinal = ordinal,
        session_id = session.session_id,
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
        hard_failure = nil,
        terminal_failed = false,
    }
    saved.current = working
    saved.current_encounter_id = encounterId
    saved.state = "capturing"
    saved.active_mode = saved.requested_mode
    saved.stop_reason = nil
    session.next_encounter_ordinal = ordinal + 1
    session.aggregate_estimated_bytes =
        session.aggregate_estimated_bytes + BASE_ENVELOPE_BYTES
    touch()

    local sourceSequence, omittedSourceSequence
    local startReason
    if startedMidCombat then
        setPartialReason("started-mid-combat")
        sourceSequence, omittedSourceSequence = appendRaw(
            "api-sample", "IsUnitInCombat", nil, 1, 1, false,
            "player", true)
        startReason = "started-mid-combat"
    else
        local argumentCount = select("#", ...)
        local sourceCode = select(1, ...)
        sourceSequence, omittedSourceSequence = appendRaw(
            "callback", "EVENT_PLAYER_COMBAT_STATE",
            sourceCode, argumentCount, 0, false, ...)
        startReason = "combat-started"
    end
    local projectionSourceSequence = sourceSequence or omittedSourceSequence
    local projection = projectionSourceSequence and {
        source_sequence = projectionSourceSequence,
        projection_ordinal = 0,
    } or nil
    appendRegular(nextSequence(), "encounter-start",
        { reason = startReason }, projection)
    if runtime and runtime.hard_failure then
        hardFailActive(runtime.hard_failure)
        return
    end
    registerCaptureHandlers()
    message("Capture started.")
end

local function stopSession(reason)
    local saved = state()
    saved.requested_mode = nil
    saved.active_mode = nil
    saved.state = "stopped"
    saved.current_encounter_id = nil
    saved.current = nil
    saved.stop_reason = reason
    saved.failure = nil
    if saved.session then
        saved.session.status = "stopped"
        saved.session.finished_at = tostring(GetTimeStamp())
    end
    touch()
end

local function appendInterruption(reason)
    local saved = state()
    if #saved.interruptions >= MAX_INTERRUPTION_MARKERS then
        failSession("interruption-limit", nil)
        return false
    end
    local sequence = #saved.interruptions + 1
    saved.interruptions[sequence] = {
        sequence = sequence,
        occurred_at = tostring(GetTimeStamp()),
        after_encounter_ordinal = saved.session.completed_encounter_count,
        reason = reason,
    }
    saved.session.interruption_count = sequence
    touch()
    return true
end

local function afterCombatFinished(mode, finished)
    local saved = state()
    if mode == "single" then
        stopSession(finished.status == "complete" and "single-complete" or "single-partial")
        message("Single encounter capture stopped. Flush SavedVariables before desktop import.")
        return
    end
    if not canBeginCapture(false) then
        failSession("storage-pressure", nil)
        return
    end
    saved.state = "waiting"
    saved.active_mode = "continuous"
    touch()
    message("Encounter retained. Continuous capture is waiting for the next encounter.")
end

local function handleCombatState(...)
    if controllerInvalid then return end
    local inCombat = select(2, ...)
    if runtime and not inCombat then
        if not updateClock() then return end
        appendRaw("callback", "EVENT_PLAYER_COMBAT_STATE", select(1, ...),
            select("#", ...), 0, false, ...)
        local pendingFailure = runtime.hard_failure
        local finished, mode, ordinal = finishAndRetain("combat-ended", true)
        if pendingFailure then
            failSession(pendingFailure, ordinal)
        elseif finished then
            afterCombatFinished(mode, finished)
        end
        return
    end
    if runtime then
        if not updateClock() then return end
        appendRaw("callback", "EVENT_PLAYER_COMBAT_STATE", select(1, ...),
            select("#", ...), 0, false, ...)
        return
    end
    local saved = state()
    if saved.state == "waiting" and saved.requested_mode and inCombat then
        beginCapture(false, ...)
    end
end

local function handlePlayerDeactivated(...)
    if controllerInvalid then return end
    if runtime then
        if not updateClock() then return end
        appendRaw("callback", "EVENT_PLAYER_DEACTIVATED", select(1, ...),
            select("#", ...), 0, false, ...)
        local pendingFailure = runtime.hard_failure
        local _, mode, ordinal = finishAndRetain("player-deactivated", false)
        if pendingFailure then
            failSession(pendingFailure, ordinal)
            return
        end
        if not appendInterruption("player-deactivated") then return end
        if mode == "single" then
            stopSession("single-interrupted")
        else
            state().active_mode = nil
            state().state = "interrupted"
            touch()
        end
    elseif state().requested_mode and state().state == "waiting" then
        if not appendInterruption("player-deactivated") then return end
        if state().session.mode == "single" then
            stopSession("single-interrupted")
        else
            state().active_mode = nil
            state().state = "interrupted"
            touch()
        end
    end
end

local function selectMode(arguments)
    local mode = string.match(arguments or "", "^(%S+)")
    if mode ~= "single" and mode ~= "continuous" then
        message("Choose one mode: /ewencounter mode single|continuous.")
        return
    end
    local saved = state()
    if saved.requested_mode then
        message("Turn capture off before changing mode.")
        return
    end
    if saved.session or next(saved.records) ~= nil then
        message("Import and clear the retained session before changing mode.")
        return
    end
    saved.selected_mode = mode
    touch()
    message("Selected " .. mode .. " encounter capture.")
end

local function selectChannel(arguments)
    local channel = string.match(arguments or "", "^(%S+)")
    if channel ~= "live" and channel ~= "pts" then
        message("Choose a channel: /ewencounter channel live|pts.")
        return
    end
    local saved = state()
    if saved.requested_mode then
        message("Turn capture off before changing channel.")
        return
    end
    if saved.session or next(saved.records) ~= nil then
        message("Import and clear the retained session before changing channel.")
        return
    end
    saved.selected_channel = channel
    touch()
    message("Selected the " .. channel .. " encounter channel.")
end

local function startSession()
    local saved = state()
    if saved.state == "failed" then
        message("Clear retained failed state before starting another session.")
        return
    end
    if saved.session or next(saved.records) ~= nil then
        message("Import and clear the retained session before starting another.")
        return
    end
    if saved.selected_channel ~= "live" and saved.selected_channel ~= "pts" then
        message("Choose live or pts before enabling capture.")
        return
    end
    message("Enabling local encounter capture. Exact callback values can include names and identifiers.")
    local raw = GetGameTimeMilliseconds()
    local stamp = tostring(GetTimeStamp())
    local mode = saved.selected_mode
    saved.session = {
        session_id = "session-" .. stamp .. "-" .. tostring(raw),
        mode = mode,
        channel = saved.selected_channel,
        status = "active",
        started_at = stamp,
        finished_at = nil,
        next_encounter_ordinal = 1,
        completed_encounter_count = 0,
        degraded_encounter_count = 0,
        aggregate_estimated_bytes = 0,
        aggregate_event_count = 0,
        aggregate_raw_observation_count = 0,
        interruption_count = 0,
    }
    saved.requested_mode = mode
    saved.active_mode = mode
    saved.state = "waiting"
    saved.stop_reason = nil
    saved.failure = nil
    touch()
    if IsUnitInCombat("player") == true then beginCapture(true) end
end

local function toggle()
    local saved = state()
    if saved.state == "failed" then
        message("Capture is failed. Import and clear retained evidence before retrying.")
    elseif saved.requested_mode then
        if runtime then
            local finished = finishAndRetain("user-stopped", false)
            if not finished or state().state == "failed" then return end
        end
        stopSession("user-disabled")
        message("Encounter capture stopped.")
    elseif saved.state == "stopped" then
        startSession()
    else
        message("Encounter capture state cannot be toggled.")
    end
end

local function clear(arguments)
    local saved = state()
    if runtime or (saved.state ~= "stopped" and saved.state ~= "failed") then
        message("Stop capture before clearing encounter data.")
        return
    end
    if string.match(arguments or "", "^%s*confirm%s*$") == nil then
        message("Use /ewencounter clear confirm to delete the saved encounter envelope.")
        return
    end
    local selectedMode = saved.selected_mode
    local selectedChannel = saved.selected_channel
    EsoWeaveDataSaved.encounter = emptyController(selectedMode, selectedChannel)
    state().stop_reason = "cleared"
    message("Saved encounter session cleared locally.")
end

local function showStatus()
    local saved = state()
    local session = saved.session
    local lastInterruption = saved.interruptions[#saved.interruptions]
    message("Selected mode: " .. tostring(saved.selected_mode)
        .. ", requested mode: " .. tostring(saved.requested_mode or "none")
        .. ", active mode: " .. tostring(saved.active_mode or "none")
        .. ", channel: " .. tostring(saved.selected_channel or "none")
        .. ", state: " .. tostring(saved.state)
        .. ", current encounter: " .. tostring(saved.current_encounter_id or "none")
        .. ", session: " .. tostring(session and session.status or "none")
        .. ", encounters: " .. tostring(session and session.completed_encounter_count or 0)
        .. ", interruptions: " .. tostring(session and session.interruption_count or 0)
        .. ", last interruption: "
        .. tostring(lastInterruption and lastInterruption.reason or "none")
        .. (saved.failure and ", failure: " .. saved.failure.reason or "") .. ".")
end

local function command(arguments)
    local verb, rest = string.match(arguments or "", "^(%S*)%s*(.-)%s*$")
    if controllerInvalid then
        if verb == "clear" and string.match(rest or "", "^%s*confirm%s*$") then
            local saved = state()
            local selectedMode = saved.selected_mode == "continuous"
                and "continuous" or "single"
            local selectedChannel = (saved.selected_channel == "live"
                or saved.selected_channel == "pts") and saved.selected_channel or nil
            EsoWeaveDataSaved.encounter = emptyController(selectedMode, selectedChannel)
            state().stop_reason = "cleared"
            controllerInvalid = false
            message("Invalid saved encounter state was explicitly cleared.")
        elseif verb == "status" then
            message("Hard failure: state-invalid. Saved encounter state is inactive and preserved.")
        else
            message("Saved encounter state is invalid and preserved. Use status or clear confirm.")
        end
        return
    end
    if state().state_schema_version ~= STATE_SCHEMA_VERSION
        or state().addon_version ~= CONTROLLER_ADDON_VERSION then
        message("Encounter state uses an unsupported version and was preserved.")
        return
    end
    if verb == "mode" then
        selectMode(rest)
    elseif verb == "channel" then
        selectChannel(rest)
    elseif verb == "toggle" then
        toggle()
    elseif verb == "arm" then
        if rest ~= "live" and rest ~= "pts" then
            message("Choose a channel: /ewencounter arm live|pts.")
            return
        end
        selectMode("single")
        selectChannel(rest)
        if not state().requested_mode then toggle() end
    elseif verb == "disarm" then
        if state().requested_mode then toggle() else message("No encounter capture is active.") end
    elseif verb == "stop" then
        if state().requested_mode then toggle() else message("No encounter capture is active.") end
    elseif verb == "status" then
        showStatus()
    elseif verb == "clear" then
        clear(rest)
    else
        message("Use /ewencounter mode single|continuous, channel live|pts, toggle, status, clear confirm, or help.")
    end
end

local function sessionFromCapture(saved, status, nextOrdinal)
    return {
        session_id = saved.session_id,
        mode = "single",
        channel = saved.channel,
        status = status,
        started_at = saved.started_at,
        finished_at = status == "active" and nil or saved.finished_at,
        next_encounter_ordinal = nextOrdinal,
        completed_encounter_count = status == "active" and 0 or 1,
        degraded_encounter_count = status == "active" and 0
            or (saved.status == "partial" and 1 or 0),
        aggregate_estimated_bytes = saved.estimated_bytes or BASE_ENVELOPE_BYTES,
        aggregate_event_count = saved.stored_event_count or #(saved.events or {}),
        aggregate_raw_observation_count = saved.raw_observation_count or 0,
        interruption_count = 0,
    }
end

local function wrapLegacyTerminal(saved)
    local controller = emptyController("single", saved.channel)
    controller.session = sessionFromCapture(saved, "stopped", 2)
    controller.records[ordinalKey(1)] = { ordinal = 1, capture = saved }
    controller.stop_reason = saved.status == "complete" and "single-complete" or "single-partial"
    return controller
end

local function wrapLegacyCurrent(saved)
    local controller = emptyController("single", saved.channel)
    controller.requested_mode = "single"
    controller.active_mode = "single"
    controller.state = "capturing"
    controller.current_encounter_id = saved.encounter_id
    controller.session = sessionFromCapture(saved, "active", 2)
    controller.current = saved
    controller.stop_reason = nil
    return controller
end

local function recoverInterruptedSavedCapture()
    local controller = state()
    local saved = controller.current
    if controller.state ~= "capturing" or type(saved) ~= "table" then return end
    saved.events = type(saved.events) == "table" and saved.events or {}
    saved.raw_observations = type(saved.raw_observations) == "table"
        and saved.raw_observations or {}
    saved.warnings = type(saved.warnings) == "table" and saved.warnings or {}
    saved.warnings.recovered_interruption =
        (saved.warnings.recovered_interruption or 0) + 1
    local lastEvent = saved.events[#saved.events]
    local sourceSequence = saved.last_sequence or (lastEvent and lastEvent.sequence) or 0
    local pendingReason = saved.pending_partial_reason
    if pendingReason ~= "capture-overflow" and pendingReason ~= "clock-reset"
        and pendingReason ~= "record-limit" and pendingReason ~= "byte-limit"
        and pendingReason ~= "string-limit" and pendingReason ~= "unsupported-value"
        and pendingReason ~= "callback-failed"
        and pendingReason ~= "started-mid-combat" then
        pendingReason = saved.schema_version == 1
            and "player-deactivated" or "runtime-interrupted"
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
        capture = saved,
        mode = controller.session.mode,
        ordinal = controller.session.next_encounter_ordinal - 1,
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
        hard_failure = nil,
        terminal_failed = false,
    }
    local _, mode, ordinal = finishAndRetain(pendingReason, false)
    if state().state == "failed" then return end
    if not appendInterruption("runtime-interrupted") then return end
    if mode == "single" then
        stopSession("single-interrupted")
    else
        state().active_mode = nil
        state().state = "interrupted"
        touch()
    end
end

local function isInteger(value, minimum, maximum)
    return type(value) == "number" and value == math.floor(value)
        and value >= minimum and (maximum == nil or value <= maximum)
end

local function hasOnlyKeys(value, allowed)
    if type(value) ~= "table" then return false end
    for key in pairs(value) do
        if not allowed[key] then return false end
    end
    return true
end

local OUTER_KEYS = {
    state_schema_version = true, addon_version = true, selected_mode = true,
    selected_channel = true, requested_mode = true, active_mode = true,
    state = true, current_encounter_id = true, session = true, current = true,
    records = true, interruptions = true, stop_reason = true, failure = true,
    revision = true,
}

local SESSION_KEYS = {
    session_id = true, mode = true, channel = true, status = true,
    started_at = true, finished_at = true, next_encounter_ordinal = true,
    completed_encounter_count = true, degraded_encounter_count = true,
    aggregate_estimated_bytes = true, aggregate_event_count = true,
    aggregate_raw_observation_count = true, interruption_count = true,
}

local RECORD_KEYS = { ordinal = true, capture = true }
local INTERRUPTION_KEYS = {
    sequence = true, occurred_at = true, after_encounter_ordinal = true,
    reason = true,
}
local FAILURE_KEYS = {
    reason = true, occurred_at = true, encounter_ordinal = true,
}
local FAILURE_REASONS = {
    ["storage-pressure"] = true, ["callback-failed"] = true,
    ["clock-reset"] = true, ["terminal-reserve-exhausted"] = true,
    ["interruption-limit"] = true, ["state-invalid"] = true,
}
local STOP_REASONS = {
    ["never-started"] = true, ["single-complete"] = true,
    ["single-partial"] = true, ["single-interrupted"] = true,
    ["user-disabled"] = true, ["cleared"] = true,
}

local function validIdentifier(value)
    return type(value) == "string" and #value >= 1 and #value <= 128
        and string.match(value, "^[%w._:-]+$") ~= nil
end

local function validTimestamp(value)
    if type(value) ~= "string" or #value < 1 or #value > 20
        or string.match(value, "^%d+$") == nil then return false end
    return #value < 20 or value <= "18446744073709551615"
end

local function timestampBefore(left, right)
    local normalizedLeft = string.gsub(left, "^0+", "")
    local normalizedRight = string.gsub(right, "^0+", "")
    if normalizedLeft == "" then normalizedLeft = "0" end
    if normalizedRight == "" then normalizedRight = "0" end
    if #normalizedLeft ~= #normalizedRight then
        return #normalizedLeft < #normalizedRight
    end
    return normalizedLeft < normalizedRight
end

local SOURCE_KEYS = {
    api_version = true, game_version = true, locale = true, platform = true,
}
local EVENT_KEYS = {
    session_id = true, encounter_id = true, sequence = true,
    monotonic_ms = true, kind = true, payload = true,
    source_sequence = true, projection_ordinal = true,
}
local RAW_KEYS = {
    session_id = true, encounter_id = true, sequence = true,
    monotonic_ms = true, api_version = true, source_kind = true,
    source_id = true, source_code = true, source_version = true,
    argument_count = true, return_count = true, values = true,
}
local RAW_VALUE_KEYS = {
    position = true, value_type = true, boolean = true, string = true,
    sign = true, significand = true, exponent = true,
}
local PROFILE_KEYS = {
    version = true, api_version = true, player_combat_unit_type = true,
    health_power_type = true, quickslot_category = true,
    damage_results = true, healing_results = true, death_results = true,
    resurrect_result = true,
}
local V2_CAPTURE_KEYS = {
    schema_version = true, addon_version = true, status = true, channel = true,
    source = true, normalization_profile = true, session_id = true,
    encounter_id = true, started_at = true, finished_at = true,
    started_monotonic_ms = true, ended_monotonic_ms = true,
    first_sequence = true, last_sequence = true, stored_event_count = true,
    omitted_event_count = true, raw_first_sequence = true,
    raw_last_sequence = true, raw_observation_count = true,
    raw_omitted_observation_count = true, raw_loss = true,
    estimated_bytes = true, partial_reason = true, warnings = true,
    events = true, raw_observations = true, pending_partial_reason = true,
    pending_loss_from = true, pending_loss_to = true,
    pending_loss_reason = true,
}
local WARNING_KEYS = {
    actor_limit = true, terminal_reserve_exhausted = true,
    recovered_interruption = true,
}
local PARTIAL_REASONS = {
    ["capture-overflow"] = true, ["clock-reset"] = true,
    ["user-stopped"] = true, ["player-deactivated"] = true,
    ["runtime-interrupted"] = true,
    ["callback-failed"] = true, ["started-mid-combat"] = true,
    ["unsupported-value"] = true, ["record-limit"] = true,
    ["byte-limit"] = true, ["string-limit"] = true,
}
local RAW_LOSS_REASONS = {
    ["record-limit"] = true, ["byte-limit"] = true,
    ["string-limit"] = true, ["unsupported-value"] = true,
    ["callback-failed"] = true,
}

local function safeSourceToken(value, maximum)
    return type(value) == "string" and #value >= 1 and #value <= maximum
        and string.match(value, "^[%w._+%-]+$") ~= nil
end

local function isDenseArray(value)
    if type(value) ~= "table" then return false end
    local count = 0
    for key in pairs(value) do
        if not isInteger(key, 1, MAX_EXACT_INTEGER) then return false end
        count = count + 1
    end
    for index = 1, count do
        if value[index] == nil then return false end
    end
    return true
end

local function exactPayload(payload, required, optional)
    if type(payload) ~= "table" then return false end
    local allowed = {}
    for _, key in ipairs(required) do
        allowed[key] = true
        if payload[key] == nil then return false end
    end
    for _, key in ipairs(optional or {}) do allowed[key] = true end
    for key in pairs(payload) do
        if type(key) ~= "string" or not allowed[key] then return false end
    end
    return true
end

local function numericPayload(payload, required, optional)
    if not exactPayload(payload, required, optional) then return false end
    for key, value in pairs(payload) do
        if not isInteger(value, 0, MAX_EXACT_INTEGER) then return false end
    end
    return true
end

local function validEventPayload(kind, payload)
    if kind == "encounter-start" then
        return exactPayload(payload, { "reason" })
            and (payload.reason == "combat-started"
                or payload.reason == "started-mid-combat")
    end
    if kind == "encounter-end" then
        return exactPayload(payload, { "complete", "reason" })
            and type(payload.complete) == "boolean"
            and (PARTIAL_REASONS[payload.reason] == true
                or payload.reason == "combat-ended")
    end
    if kind == "damage" or kind == "healing" then
        return numericPayload(payload,
            { "ability_id", "amount", "result", "source_actor", "target_actor" },
            { "damage_type", "overflow", "power_type", "source_type", "target_type" })
    end
    if kind == "effect" then
        return numericPayload(payload,
            { "ability_id", "change_type", "stack_count", "target_actor" },
            { "ability_type", "begin_ms", "effect_type", "end_ms",
                "source_type", "status_effect_type" })
    end
    if kind == "resource" then
        return numericPayload(payload,
            { "actor", "effective_maximum", "maximum", "power_type", "value" })
    end
    if kind == "cast" then
        return numericPayload(payload, { "ability_id", "slot" })
    end
    if kind == "bar-change" then
        return exactPayload(payload, { "active_pair", "locked" })
            and isInteger(payload.active_pair, 0, MAX_EXACT_INTEGER)
            and type(payload.locked) == "boolean"
    end
    if kind == "death" or kind == "resurrection" then
        if payload.source == "player-event" then
            return numericPayload({ actor = payload.actor }, { "actor" })
                and exactPayload(payload, { "actor", "source" })
        end
        if payload.source == "combat-result" then
            return exactPayload(payload,
                { "ability_id", "actor", "result", "source_actor", "source" })
                and isInteger(payload.ability_id, 0, MAX_EXACT_INTEGER)
                and isInteger(payload.actor, 0, MAX_ACTORS)
                and isInteger(payload.result, 0, MAX_EXACT_INTEGER)
                and isInteger(payload.source_actor, 0, MAX_ACTORS)
        end
        return isInteger(payload.source, 0, MAX_ACTORS)
            and exactPayload(payload, { "actor", "source" })
            and isInteger(payload.actor, 0, MAX_ACTORS)
    end
    if kind == "boss-health" then
        return numericPayload(payload,
            { "actor", "boss_index", "effective_maximum", "maximum", "value" })
            and payload.actor <= MAX_ACTORS and payload.boss_index >= 1
            and payload.boss_index <= 6
    end
    if kind == "performance" then
        return numericPayload(payload, { "frames_per_second", "latency_ms" })
    end
    if kind == "quickslot" then
        return exactPayload(payload, { "ability_id", "action", "slot" })
            and isInteger(payload.ability_id, 0, MAX_EXACT_INTEGER)
            and isInteger(payload.slot, 0, MAX_EXACT_INTEGER)
            and (payload.action == "selected" or payload.action == "used")
    end
    if kind == "discontinuity" then
        return exactPayload(payload,
            { "missing_sequence_from", "missing_sequence_to", "reason" })
            and isInteger(payload.missing_sequence_from, 1, MAX_EXACT_INTEGER)
            and isInteger(payload.missing_sequence_to,
                payload.missing_sequence_from, MAX_EXACT_INTEGER)
            and (payload.reason == "capture-overflow"
                or payload.reason == "clock-reset")
    end
    return false
end

local function validEvent(inner, event)
    if not hasOnlyKeys(event, EVENT_KEYS)
        or event.session_id ~= inner.session_id
        or event.encounter_id ~= inner.encounter_id
        or not isInteger(event.sequence, 1, MAX_EXACT_INTEGER)
        or not isInteger(event.monotonic_ms, 0, MAX_EXACT_INTEGER)
        or not isInteger(event.source_sequence, 1, MAX_EXACT_INTEGER)
        or not isInteger(event.projection_ordinal, 0, MAX_EXACT_INTEGER)
        or not validEventPayload(event.kind, event.payload) then
        return false
    end
    for _, key in ipairs({ "actor", "source_actor", "target_actor" }) do
        if event.payload[key] ~= nil and event.payload[key] > MAX_ACTORS then
            return false
        end
    end
    return true
end

local function validRawValue(value, position)
    if not hasOnlyKeys(value, RAW_VALUE_KEYS) or value.position ~= position then
        return false
    end
    if value.value_type == "nil" then
        return value.boolean == nil and value.string == nil and value.sign == nil
            and value.significand == nil and value.exponent == nil
    end
    if value.value_type == "boolean" then
        return type(value.boolean) == "boolean" and value.string == nil
            and value.sign == nil and value.significand == nil and value.exponent == nil
    end
    if value.value_type == "string" then
        return type(value.string) == "string" and #value.string <= MAX_STRING_BYTES
            and value.boolean == nil and value.sign == nil
            and value.significand == nil and value.exponent == nil
    end
    if value.value_type ~= "number" or value.boolean ~= nil or value.string ~= nil
        or (value.sign ~= -1 and value.sign ~= 1)
        or type(value.significand) ~= "string"
        or string.match(value.significand, "^%d+$") == nil
        or (#value.significand > 1 and string.sub(value.significand, 1, 1) == "0")
        or not isInteger(value.exponent, -1074, 1023) then
        return false
    end
    local significand = tonumber(value.significand)
    return significand ~= nil and significand <= MAX_EXACT_INTEGER
        and (significand ~= 0 or value.exponent == 0)
        and significand * (2 ^ value.exponent) < math.huge
end

local function rawCountsMatch(observation)
    local arguments, returns = observation.argument_count, observation.return_count
    local kind, id = observation.source_kind, observation.source_id
    if kind == "callback" then
        if returns ~= 0 then return false end
        local minimums = {
            EVENT_PLAYER_COMBAT_STATE = 2, EVENT_PLAYER_DEACTIVATED = 1,
            EVENT_COMBAT_EVENT = 18, EVENT_EFFECT_CHANGED = 17,
            EVENT_POWER_UPDATE = 7, EVENT_ACTION_SLOT_ABILITY_USED = 2,
            EVENT_ACTIVE_QUICKSLOT_CHANGED = 2,
            EVENT_ACTIVE_WEAPON_PAIR_CHANGED = 3, EVENT_PLAYER_DEAD = 1,
            EVENT_PLAYER_ALIVE = 1, EVENT_BOSSES_CHANGED = 2,
        }
        return minimums[id] ~= nil and arguments >= minimums[id]
            and isInteger(observation.source_code, 0, MAX_EXACT_INTEGER)
    end
    if observation.source_code ~= nil then return false end
    if kind == "lifecycle" then
        return (id == "clock-reset" or id == "capture-finish")
            and arguments == 2 and returns == 0
    end
    if kind ~= "api-sample" then return false end
    if id == "IsUnitInCombat" then return arguments == 1 and returns == 1 end
    if id == "GetSlotBoundId" then
        return (arguments == 1 or arguments == 2) and returns == 1
    end
    if id == "GetCurrentQuickslot" or id == "GetFramerate"
        or id == "GetLatency" then return arguments == 0 and returns == 1 end
    if id == "DoesUnitExist" then return arguments == 1 and returns == 1 end
    return id == "GetUnitPower" and arguments == 2 and returns == 3
end

local function validRawObservation(inner, observation)
    if not hasOnlyKeys(observation, RAW_KEYS)
        or observation.session_id ~= inner.session_id
        or observation.encounter_id ~= inner.encounter_id
        or not isInteger(observation.sequence, 1, MAX_EXACT_INTEGER)
        or not isInteger(observation.monotonic_ms, 0, MAX_EXACT_INTEGER)
        or observation.api_version ~= inner.source.api_version
        or observation.source_version ~= 1
        or type(observation.source_id) ~= "string" or #observation.source_id < 1
        or #observation.source_id > 128
        or string.match(observation.source_id, "^[%w_%-]+$") == nil
        or not isInteger(observation.argument_count, 0, MAX_RAW_VALUES)
        or not isInteger(observation.return_count, 0, MAX_RAW_VALUES)
        or not isDenseArray(observation.values)
        or #observation.values ~= observation.argument_count + observation.return_count
        or #observation.values > MAX_RAW_VALUES or not rawCountsMatch(observation) then
        return false
    end
    for position, value in ipairs(observation.values) do
        if not validRawValue(value, position) then return false end
    end
    if observation.source_kind == "callback" then
        local first = observation.values[1]
        if first.value_type ~= "number" then return false end
        local encoded = first.sign * tonumber(first.significand) * (2 ^ first.exponent)
        if encoded ~= observation.source_code then return false end
    end
    return true
end

local function sourceMatches(kind, sourceId)
    if kind == "encounter-start" then
        return sourceId == "EVENT_PLAYER_COMBAT_STATE" or sourceId == "IsUnitInCombat"
    end
    if kind == "encounter-end" then return sourceId == "capture-finish" end
    if kind == "damage" or kind == "healing" then return sourceId == "EVENT_COMBAT_EVENT" end
    if kind == "effect" then return sourceId == "EVENT_EFFECT_CHANGED" end
    if kind == "resource" then return sourceId == "EVENT_POWER_UPDATE" end
    if kind == "cast" then return sourceId == "EVENT_ACTION_SLOT_ABILITY_USED" end
    if kind == "bar-change" then return sourceId == "EVENT_ACTIVE_WEAPON_PAIR_CHANGED" end
    if kind == "death" then
        return sourceId == "EVENT_COMBAT_EVENT" or sourceId == "EVENT_PLAYER_DEAD"
    end
    if kind == "resurrection" then
        return sourceId == "EVENT_COMBAT_EVENT" or sourceId == "EVENT_PLAYER_ALIVE"
    end
    if kind == "boss-health" then
        return sourceId == "EVENT_BOSSES_CHANGED" or sourceId == "GetUnitPower"
    end
    if kind == "performance" then return sourceId == "GetFramerate" end
    if kind == "quickslot" then
        return sourceId == "EVENT_ACTION_SLOT_ABILITY_USED"
            or sourceId == "EVENT_ACTIVE_QUICKSLOT_CHANGED"
    end
    return kind == "discontinuity"
        and (sourceId == "clock-reset" or sourceId == "capture-finish")
end

local function sameArray(left, right)
    if not isDenseArray(left) or #left ~= #right then return false end
    for index, value in ipairs(right) do
        if left[index] ~= value then return false end
    end
    return true
end

local function validProfile(inner)
    local profile = inner.normalization_profile
    local expected = normalizationProfile(inner.source.api_version)
    return type(profile) == "table" and hasOnlyKeys(profile, PROFILE_KEYS)
        and profile.version == expected.version
        and profile.api_version == expected.api_version
        and profile.player_combat_unit_type == expected.player_combat_unit_type
        and profile.health_power_type == expected.health_power_type
        and profile.quickslot_category == expected.quickslot_category
        and profile.resurrect_result == expected.resurrect_result
        and sameArray(profile.damage_results, expected.damage_results)
        and sameArray(profile.healing_results, expected.healing_results)
        and sameArray(profile.death_results, expected.death_results)
end

local function validV2Evidence(inner, terminal)
    if not hasOnlyKeys(inner, V2_CAPTURE_KEYS)
        or not isDenseArray(inner.raw_observations)
        or type(inner.source) ~= "table" or not hasOnlyKeys(inner.source, SOURCE_KEYS)
        or not isInteger(inner.source.api_version, 1, MAX_EXACT_INTEGER)
        or not safeSourceToken(inner.source.game_version, 128)
        or not safeSourceToken(inner.source.locale, 16)
        or not safeSourceToken(inner.source.platform, 32) then
        return false
    end
    local profileValid = (inner.addon_version == 2
        and inner.normalization_profile == nil) or validProfile(inner)
    if not profileValid or type(inner.warnings) ~= "table" then
        return false
    end
    for name, count in pairs(inner.warnings) do
        if not WARNING_KEYS[name] or not isInteger(count, 1, MAX_EXACT_INTEGER) then
            return false
        end
    end
    if not validTimestamp(inner.started_at)
        or not isInteger(inner.started_monotonic_ms, 0, MAX_EXACT_INTEGER)
        or not isInteger(inner.ended_monotonic_ms, 0, MAX_EXACT_INTEGER)
        or not isInteger(inner.first_sequence, 1, MAX_EXACT_INTEGER)
        or not isInteger(inner.last_sequence, inner.first_sequence, MAX_EXACT_INTEGER)
        or not isInteger(inner.omitted_event_count, 0, MAX_EXACT_INTEGER)
        or not isDenseArray(inner.events) or #inner.events < 1
        or #inner.events ~= inner.stored_event_count then return false end
    local previousSequence, previousTime, omitted = 0, 0, 0
    local rawBySequence, projectionOrdinals = {}, {}
    for _, observation in ipairs(inner.raw_observations) do
        if not validRawObservation(inner, observation)
            or rawBySequence[observation.sequence] ~= nil then return false end
        rawBySequence[observation.sequence] = observation
    end
    local previousSource = 0
    for index, event in ipairs(inner.events) do
        if not validEvent(inner, event) or event.monotonic_ms < previousTime
            or event.source_sequence < previousSource then return false end
        if terminal then
            if previousSequence > 0 and event.sequence ~= previousSequence + 1 then
                if event.kind ~= "discontinuity"
                    or event.payload.missing_sequence_from ~= previousSequence + 1
                    or event.payload.missing_sequence_to + 1 ~= event.sequence then
                    return false
                end
                omitted = omitted + event.payload.missing_sequence_to
                    - event.payload.missing_sequence_from + 1
            elseif previousSequence > 0 and event.kind == "discontinuity" then
                return false
            end
        elseif event.sequence ~= index or event.kind == "discontinuity"
            or event.kind == "encounter-end" then return false end
        local source = rawBySequence[event.source_sequence]
        local declaredLostStart = source == nil and event.kind == "encounter-start"
            and event.monotonic_ms == 0 and event.source_sequence == 1
            and type(inner.raw_loss) == "table"
            and inner.raw_loss.missing_sequence_from == 1
            and inner.raw_loss.missing_sequence_to >= 1
        if not declaredLostStart and (source == nil
            or source.monotonic_ms ~= event.monotonic_ms
            or not sourceMatches(event.kind, source.source_id)) then return false end
        local expectedOrdinal = projectionOrdinals[event.source_sequence] or 0
        if event.projection_ordinal ~= expectedOrdinal then return false end
        projectionOrdinals[event.source_sequence] = expectedOrdinal + 1
        previousSequence, previousTime = event.sequence, event.monotonic_ms
        previousSource = event.source_sequence
    end
    if inner.events[1].kind ~= "encounter-start"
        or inner.events[1].sequence ~= 1 or inner.events[1].monotonic_ms ~= 0
        or inner.first_sequence ~= 1
        or inner.last_sequence ~= #inner.events + inner.omitted_event_count
        or inner.ended_monotonic_ms ~= inner.events[#inner.events].monotonic_ms
        or (terminal and omitted ~= inner.omitted_event_count) then return false end

    if not isInteger(inner.raw_first_sequence, 1, MAX_EXACT_INTEGER)
        or not isInteger(inner.raw_last_sequence, inner.raw_first_sequence,
            MAX_EXACT_INTEGER)
        or not isInteger(inner.raw_omitted_observation_count, 0, MAX_EXACT_INTEGER)
        or not isDenseArray(inner.raw_observations) or #inner.raw_observations < 1
        or #inner.raw_observations ~= inner.raw_observation_count
        or inner.raw_last_sequence ~= #inner.raw_observations
            + inner.raw_omitted_observation_count then return false end
    local rawLoss = inner.raw_loss
    if inner.raw_omitted_observation_count == 0 then
        if rawLoss ~= nil then return false end
    elseif type(rawLoss) ~= "table" or not hasOnlyKeys(rawLoss, {
        missing_sequence_from = true, missing_sequence_to = true, reason = true,
    }) or not isInteger(rawLoss.missing_sequence_from, 1, inner.raw_last_sequence)
        or not isInteger(rawLoss.missing_sequence_to,
            rawLoss.missing_sequence_from, inner.raw_last_sequence)
        or rawLoss.missing_sequence_to - rawLoss.missing_sequence_from + 1
            ~= inner.raw_omitted_observation_count
        or not RAW_LOSS_REASONS[rawLoss.reason] then return false end
    local expectedRaw, crossedLoss, priorRawTime = 1, false, 0
    for _, observation in ipairs(inner.raw_observations) do
        if observation.sequence ~= expectedRaw then
            if crossedLoss or rawLoss == nil
                or rawLoss.missing_sequence_from ~= expectedRaw
                or rawLoss.missing_sequence_to + 1 ~= observation.sequence then
                return false
            end
            crossedLoss = true
        end
        if not validRawObservation(inner, observation)
            or observation.monotonic_ms < priorRawTime then return false end
        expectedRaw = observation.sequence + 1
        priorRawTime = observation.monotonic_ms
    end
    if terminal then
        local lastRaw = inner.raw_observations[#inner.raw_observations]
        if expectedRaw ~= inner.raw_last_sequence + 1
            or lastRaw.source_kind ~= "lifecycle"
            or lastRaw.source_id ~= "capture-finish"
            or (rawLoss ~= nil and not crossedLoss) then return false end
    elseif rawLoss == nil then
        if expectedRaw ~= inner.raw_last_sequence + 1 then return false end
    elseif expectedRaw ~= rawLoss.missing_sequence_from then return false end
    return true
end

local function validLegacyRecoverableCurrent(inner)
    if type(inner) ~= "table" or inner.schema_version ~= 1
        or inner.addon_version ~= 1 or inner.status ~= "capturing"
        or inner.channel ~= "live" and inner.channel ~= "pts"
        or inner.privacy_profile ~= "anonymous-local-v1"
        or not validIdentifier(inner.session_id) or not validIdentifier(inner.encounter_id)
        or type(inner.source) ~= "table" or not hasOnlyKeys(inner.source, SOURCE_KEYS)
        or not isInteger(inner.source.api_version, 1, MAX_EXACT_INTEGER)
        or not safeSourceToken(inner.source.game_version, 128)
        or not safeSourceToken(inner.source.locale, 16)
        or not safeSourceToken(inner.source.platform, 32)
        or not validTimestamp(inner.started_at) or inner.finished_at ~= ""
        or not isInteger(inner.started_monotonic_ms, 0, MAX_EXACT_INTEGER)
        or not isInteger(inner.ended_monotonic_ms, 0, MAX_EXACT_INTEGER)
        or not isInteger(inner.first_sequence, 1, MAX_EXACT_INTEGER)
        or not isInteger(inner.last_sequence, inner.first_sequence, MAX_EXACT_INTEGER)
        or not isInteger(inner.stored_event_count, 1, MAX_EVENTS)
        or not isInteger(inner.omitted_event_count, 0, MAX_EXACT_INTEGER)
        or not isInteger(inner.estimated_bytes, 0, MAX_ESTIMATED_BYTES)
        or type(inner.warnings) ~= "table" or not isDenseArray(inner.events)
        or #inner.events ~= inner.stored_event_count
        or inner.last_sequence ~= #inner.events + inner.omitted_event_count then
        return false
    end
    for name, count in pairs(inner.warnings) do
        if not WARNING_KEYS[name] or not isInteger(count, 1, MAX_EXACT_INTEGER) then
            return false
        end
    end
    local previousTime = 0
    for index, event in ipairs(inner.events) do
        if not hasOnlyKeys(event, EVENT_KEYS)
            or event.source_sequence ~= nil or event.projection_ordinal ~= nil
            or event.session_id ~= inner.session_id
            or event.encounter_id ~= inner.encounter_id
            or event.sequence ~= index
            or not isInteger(event.monotonic_ms, previousTime, MAX_EXACT_INTEGER)
            or event.kind == "encounter-end" or event.kind == "discontinuity"
            or not validEventPayload(event.kind, event.payload) then return false end
        previousTime = event.monotonic_ms
    end
    if inner.events[1].kind ~= "encounter-start"
        or inner.events[1].monotonic_ms ~= 0
        or inner.ended_monotonic_ms < previousTime then return false end
    local lossFrom, lossTo, lossReason = inner.pending_loss_from,
        inner.pending_loss_to, inner.pending_loss_reason
    return lossFrom == nil and lossTo == nil and lossReason == nil
            and inner.omitted_event_count == 0
        or lossReason == "capture-overflow"
            and isInteger(lossFrom, #inner.events + 1, inner.last_sequence)
            and lossTo == inner.last_sequence
            and lossTo - lossFrom + 1 == inner.omitted_event_count
end

local function validRecoverableCurrent(inner)
    local pendingReason = inner.pending_partial_reason
    local lossFrom, lossTo, lossReason = inner.pending_loss_from,
        inner.pending_loss_to, inner.pending_loss_reason
    local pendingLossValid = lossFrom == nil and lossTo == nil and lossReason == nil
        and inner.omitted_event_count == 0
        or lossReason == "capture-overflow"
            and isInteger(lossFrom, #inner.events + 1, inner.last_sequence)
            and lossTo == inner.last_sequence
            and lossTo - lossFrom + 1 == inner.omitted_event_count
    return validV2Evidence(inner, false)
        and (pendingReason == nil or PARTIAL_REASONS[pendingReason] == true)
        and pendingLossValid
end

local function validTerminalCapture(saved, inner, session, ordinal)
    if type(inner) ~= "table"
        or (inner.status ~= "complete" and inner.status ~= "partial")
        or inner.session_id ~= session.session_id
        or inner.channel ~= session.channel
        or not validIdentifier(inner.encounter_id)
        or not isInteger(inner.stored_event_count, 0, MAX_EVENTS)
        or not isInteger(inner.raw_observation_count or 0, 0, MAX_RAW_OBSERVATIONS)
        or not isInteger(inner.estimated_bytes, 0, MAX_ESTIMATED_BYTES)
        or not validTimestamp(inner.started_at) or not validTimestamp(inner.finished_at)
        or timestampBefore(inner.finished_at, inner.started_at)
        or not isDenseArray(inner.events)
        or #inner.events ~= inner.stored_event_count then
        return false
    end
    if inner.schema_version == 1 and inner.addon_version == 1 then
        return session.mode == "single" and ordinal == 1
    end
    local currentFormat = inner.schema_version == SCHEMA_VERSION
        and (inner.addon_version == 2 or inner.addon_version == ADDON_VERSION)
        and type(inner.raw_observations) == "table"
        and #inner.raw_observations == inner.raw_observation_count
        and validV2Evidence(inner, true)
    if inner.addon_version == 2 then
        return currentFormat and session.mode == "single" and ordinal == 1
    end
    if not currentFormat then return false end
    local first, last = inner.events[1], inner.events[#inner.events]
    if first.kind ~= "encounter-start" or last.kind ~= "encounter-end" then return false end
    if inner.status == "complete" then
        return inner.partial_reason == nil and inner.omitted_event_count == 0
            and inner.raw_omitted_observation_count == 0 and inner.raw_loss == nil
            and last.payload.complete == true and last.payload.reason == "combat-ended"
    end
    if not PARTIAL_REASONS[inner.partial_reason]
        or last.payload.complete ~= false or last.payload.reason ~= inner.partial_reason then
        return false
    end
    local evidence = inner.partial_reason == "user-stopped"
        or inner.partial_reason == "player-deactivated"
        or inner.partial_reason == "runtime-interrupted"
            and isInteger(inner.warnings.recovered_interruption, 1, MAX_EXACT_INTEGER)
        or inner.raw_loss and inner.raw_loss.reason == inner.partial_reason
    for _, event in ipairs(inner.events) do
        if event.kind == "discontinuity"
            and event.payload.reason == inner.partial_reason then evidence = true end
    end
    for _, observation in ipairs(inner.raw_observations) do
        if inner.partial_reason == "clock-reset"
            and observation.source_kind == "lifecycle"
            and observation.source_id == "clock-reset" then evidence = true end
    end
    if inner.partial_reason == "started-mid-combat" then
        local opening = inner.raw_observations[1]
        evidence = first.payload.reason == "started-mid-combat"
            and opening.source_kind == "api-sample"
            and opening.source_id == "IsUnitInCombat"
            and opening.values[1].value_type == "string"
            and opening.values[1].string == "player"
            and opening.values[2].value_type == "boolean"
            and opening.values[2].boolean == true
    end
    return evidence == true
end

local function validController(saved)
    local function invalid(_) return false end
    if not hasOnlyKeys(saved, OUTER_KEYS)
        or saved.state_schema_version ~= STATE_SCHEMA_VERSION
        or saved.addon_version ~= CONTROLLER_ADDON_VERSION
        or (saved.selected_mode ~= "single" and saved.selected_mode ~= "continuous")
        or (saved.selected_channel ~= nil and saved.selected_channel ~= "live"
            and saved.selected_channel ~= "pts")
        or type(saved.records) ~= "table"
        or not isDenseArray(saved.interruptions)
        or not isInteger(saved.revision, 1, MAX_EXACT_INTEGER)
        or (saved.state ~= "stopped" and saved.state ~= "waiting"
            and saved.state ~= "capturing" and saved.state ~= "interrupted"
            and saved.state ~= "failed") then
        return invalid("outer")
    end

    if saved.session == nil then
        return saved.state == "stopped" and saved.current == nil
            and saved.current_encounter_id == nil and saved.requested_mode == nil
            and saved.active_mode == nil and saved.failure == nil
            and STOP_REASONS[saved.stop_reason] == true
            and next(saved.records) == nil and next(saved.interruptions) == nil
    end
    local session = saved.session
    if not hasOnlyKeys(session, SESSION_KEYS)
        or not validIdentifier(session.session_id)
        or (session.mode ~= "single" and session.mode ~= "continuous")
        or (session.channel ~= "live" and session.channel ~= "pts")
        or session.mode ~= saved.selected_mode
        or session.channel ~= saved.selected_channel
        or (session.status ~= "active" and session.status ~= "stopped"
            and session.status ~= "failed")
        or not validTimestamp(session.started_at)
        or (session.status == "active" and session.finished_at ~= nil)
        or (session.status ~= "active" and not validTimestamp(session.finished_at))
        or (session.status ~= "active"
            and timestampBefore(session.finished_at, session.started_at))
        or not isInteger(session.completed_encounter_count, 0, MAX_SESSION_ENCOUNTERS)
        or not isInteger(session.degraded_encounter_count, 0,
            session.completed_encounter_count)
        or not isInteger(session.aggregate_event_count, 0, MAX_EVENTS)
        or not isInteger(session.aggregate_raw_observation_count, 0,
            MAX_RAW_OBSERVATIONS)
        or not isInteger(session.aggregate_estimated_bytes, 0,
            MAX_ESTIMATED_BYTES)
        or not isInteger(session.interruption_count, 0, MAX_INTERRUPTION_MARKERS)
        or not isInteger(session.next_encounter_ordinal, 1,
            MAX_SESSION_ENCOUNTERS + 1) then
        return invalid("session")
    end

    local eventCount, rawCount, byteCount, degradedCount = 0, 0, 0, 0
    local encounterIds = {}
    for ordinal = 1, session.completed_encounter_count do
        local record = saved.records[ordinalKey(ordinal)]
        if not hasOnlyKeys(record, RECORD_KEYS) or record.ordinal ~= ordinal
            or not validTerminalCapture(saved, record.capture, session, ordinal) then
            return invalid("record")
        end
        local inner = record.capture
        if encounterIds[inner.encounter_id] then return invalid("record-identity") end
        encounterIds[inner.encounter_id] = true
        eventCount = eventCount + inner.stored_event_count
        rawCount = rawCount + (inner.raw_observation_count or 0)
        byteCount = byteCount + inner.estimated_bytes
        if inner.status == "partial" then degradedCount = degradedCount + 1 end
    end
    local recordCount = 0
    for key in pairs(saved.records) do
        recordCount = recordCount + 1
        if type(key) ~= "string" then return false end
    end
    if recordCount ~= session.completed_encounter_count
        or degradedCount ~= session.degraded_encounter_count then
        return invalid("record-count")
    end
    if session.mode == "single" then
        if session.completed_encounter_count > 1 then
            return invalid("single-record-count")
        end
        if session.completed_encounter_count > 0
            and saved.state ~= "stopped" and saved.state ~= "failed" then
            return invalid("single-terminal-state")
        end
    end

    for sequence = 1, session.interruption_count do
        local marker = saved.interruptions[sequence]
        if not hasOnlyKeys(marker, INTERRUPTION_KEYS)
            or marker.sequence ~= sequence
            or not validTimestamp(marker.occurred_at)
            or not isInteger(marker.after_encounter_ordinal, 0,
                session.completed_encounter_count)
            or (marker.reason ~= "player-deactivated"
                and marker.reason ~= "runtime-interrupted") then
            return invalid("interruption")
        end
    end
    local interruptionCount = 0
    for key in pairs(saved.interruptions) do
        interruptionCount = interruptionCount + 1
        if not isInteger(key, 1, session.interruption_count) then
            return invalid("interruption-key")
        end
    end
    if interruptionCount ~= session.interruption_count then
        return invalid("interruption-count")
    end

    if saved.state == "capturing" then
        local inner = saved.current
        if type(inner) ~= "table" or inner.schema_version ~= SCHEMA_VERSION
            or inner.addon_version ~= ADDON_VERSION or inner.status ~= "capturing"
            or inner.session_id ~= session.session_id or inner.channel ~= session.channel
            or not validIdentifier(inner.encounter_id)
            or inner.encounter_id ~= saved.current_encounter_id
            or encounterIds[inner.encounter_id] == true
            or not isInteger(inner.stored_event_count, 0, MAX_EVENTS)
            or not isInteger(inner.raw_observation_count, 0, MAX_RAW_OBSERVATIONS)
            or not isInteger(inner.estimated_bytes, 0, MAX_ESTIMATED_BYTES)
            or type(inner.events) ~= "table"
            or #inner.events ~= inner.stored_event_count
            or type(inner.raw_observations) ~= "table"
            or #inner.raw_observations ~= inner.raw_observation_count
            or not validRecoverableCurrent(inner)
            or saved.requested_mode ~= session.mode
            or saved.active_mode ~= saved.requested_mode
            or session.status ~= "active"
            or session.next_encounter_ordinal
                ~= session.completed_encounter_count + 2 then
            return invalid("current")
        end
        eventCount = eventCount + inner.stored_event_count
        rawCount = rawCount + inner.raw_observation_count
        byteCount = byteCount + inner.estimated_bytes
    elseif saved.current ~= nil or saved.current_encounter_id ~= nil
        or session.next_encounter_ordinal ~= session.completed_encounter_count + 1 then
        return invalid("current-absence")
    end

    if eventCount ~= session.aggregate_event_count
        or rawCount ~= session.aggregate_raw_observation_count
        or byteCount ~= session.aggregate_estimated_bytes then
        return invalid("aggregate")
    end
    if saved.state == "capturing" then
        return saved.stop_reason == nil and saved.failure == nil
    end
    if saved.state == "waiting" then
        return session.status == "active" and saved.requested_mode == session.mode
            and saved.active_mode == saved.requested_mode and saved.failure == nil
            and saved.stop_reason == nil
    end
    if saved.state == "interrupted" then
        return session.status == "active" and saved.requested_mode == session.mode
            and saved.active_mode == nil and saved.failure == nil
            and saved.stop_reason == nil and session.interruption_count > 0
    end
    if saved.state == "failed" then
        return session.status == "failed" and saved.requested_mode == nil
            and saved.active_mode == nil and saved.stop_reason == nil
            and hasOnlyKeys(saved.failure, FAILURE_KEYS)
            and FAILURE_REASONS[saved.failure.reason] == true
            and validTimestamp(saved.failure.occurred_at)
            and (saved.failure.encounter_ordinal == nil
                or isInteger(saved.failure.encounter_ordinal, 1,
                    session.next_encounter_ordinal))
    end
    return saved.state == "stopped" and session.status == "stopped"
        and saved.requested_mode == nil and saved.active_mode == nil
        and saved.failure == nil and STOP_REASONS[saved.stop_reason] == true
end

local function resumeInterrupted()
    local saved = state()
    if saved.state ~= "interrupted" or not saved.requested_mode then return end
    if saved.session.mode == "single" then
        stopSession("single-interrupted")
        return
    end
    saved.state = "waiting"
    saved.active_mode = saved.requested_mode
    touch()
    if IsUnitInCombat("player") == true then beginCapture(true) end
end

local function onLoaded(_, addonName)
    if addonName ~= ADDON_NAME then return end
    EVENT_MANAGER:UnregisterForEvent(MODULE_NAMESPACE, EVENT_ADD_ON_LOADED)
    if type(EsoWeaveDataSaved) ~= "table" then EsoWeaveDataSaved = {} end
    local saved = EsoWeaveDataSaved.encounter
    if type(saved) ~= "table" then
        EsoWeaveDataSaved.encounter = emptyController("single", nil)
    elseif saved.state_schema_version ~= nil then
        if validController(saved) then
            if saved.state == "capturing" then recoverInterruptedSavedCapture() end
            if saved.state == "waiting" and saved.session.mode == "single" then
                if appendInterruption("runtime-interrupted") then
                    stopSession("single-interrupted")
                end
            end
            resumeInterrupted()
        elseif saved.state_schema_version == STATE_SCHEMA_VERSION
            and saved.addon_version == CONTROLLER_ADDON_VERSION then
            controllerInvalid = true
        end
    elseif (saved.schema_version == 1 and saved.addon_version == 1)
        or (saved.schema_version == 2 and (saved.addon_version == 2
            or saved.addon_version == 3)) then
        if saved.status == "complete" or saved.status == "partial" then
            EsoWeaveDataSaved.encounter = wrapLegacyTerminal(saved)
        elseif saved.status == "capturing" then
            local validLegacy = saved.schema_version == 1
                and validLegacyRecoverableCurrent(saved)
                or saved.schema_version == 2 and validV2Evidence(saved, false)
            if validLegacy then
                EsoWeaveDataSaved.encounter = wrapLegacyCurrent(saved)
                recoverInterruptedSavedCapture()
            else
                controllerInvalid = true
            end
        elseif saved.status == "armed" then
            local channel = saved.channel
            EsoWeaveDataSaved.encounter = emptyController("single", channel)
            startSession()
        elseif saved.status == "idle" then
            EsoWeaveDataSaved.encounter = emptyController("single", saved.channel)
        end
    else
        -- Unknown user-owned evidence is preserved until explicit clear.
    end
    SLASH_COMMANDS["/ewencounter"] = command
    EVENT_MANAGER:RegisterForEvent(MODULE_NAMESPACE .. "CombatState", EVENT_PLAYER_COMBAT_STATE, function(...)
        guarded(handleCombatState, ...)
    end)
    EVENT_MANAGER:RegisterForEvent(MODULE_NAMESPACE .. "Deactivate", EVENT_PLAYER_DEACTIVATED, function(...)
        guarded(handlePlayerDeactivated, ...)
    end)
    if controllerInvalid then
        message("Loaded with state-invalid saved encounter evidence preserved and inactive.")
    else
        message("Loaded. Select single or continuous mode, choose a channel, then use /ewencounter toggle.")
    end
end

EVENT_MANAGER:RegisterForEvent(MODULE_NAMESPACE, EVENT_ADD_ON_LOADED, onLoaded)
