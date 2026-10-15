local ADDON_NAME = "EsoWeaveData"
local BOOTSTRAP_NAMESPACE = ADDON_NAME .. "Bootstrap"
local SCHEMA_VERSION = 1
local ADDON_VERSION = 1

-- Player replies bypass debug interception. Initialization buffering stays local
-- and bounded; the selected desktop/game environment does not affect routing.
local MESSAGE_RETRY = ADDON_NAME .. "UserMessages"
local MAX_PENDING_MESSAGES = 64
local pending = {}
local omitted = 0
EsoWeaveDataMessages = {}

local function chatReady()
    if not CHAT_ROUTER or type(CHAT_ROUTER.AddSystemMessage) ~= "function" then return false end
    if type(ZO_GetChatSystem) == "function" then
        local ok, system = pcall(ZO_GetChatSystem)
        if not ok or not system then return false end
    end
    return true
end

function EsoWeaveDataMessages.Flush()
    if not chatReady() then return false end
    while #pending > 0 do
        local ok = pcall(CHAT_ROUTER.AddSystemMessage, CHAT_ROUTER, pending[1])
        if not ok then return false end
        table.remove(pending, 1)
    end
    if omitted > 0 then
        local ok = pcall(CHAT_ROUTER.AddSystemMessage, CHAT_ROUTER,
            "[ESO Weave Data] " .. tostring(omitted) .. " replies exceeded the startup message limit. Use /ewencounter help or /ewcollect help to repeat guidance.")
        if not ok then return false end
        omitted = 0
    end
    EVENT_MANAGER:UnregisterForUpdate(MESSAGE_RETRY)
    return true
end

function EsoWeaveDataMessages.User(module, text)
    local admitted = #pending < MAX_PENDING_MESSAGES
    if admitted then
        table.insert(pending, "[ESO Weave Data: " .. module .. "] " .. text)
    else
        omitted = math.min(omitted + 1, 1000000)
    end
    local delivered = EsoWeaveDataMessages.Flush()
    if not delivered then EVENT_MANAGER:RegisterForUpdate(MESSAGE_RETRY, 250, EsoWeaveDataMessages.Flush) end
    return admitted and delivered
end

local function onLoaded(_, addonName)
    if addonName ~= ADDON_NAME then return end
    EVENT_MANAGER:UnregisterForEvent(BOOTSTRAP_NAMESPACE, EVENT_ADD_ON_LOADED)
    if type(EsoWeaveDataSaved) ~= "table" then
        EsoWeaveDataSaved = {}
    end
    EsoWeaveDataSaved.schema_version = SCHEMA_VERSION
    EsoWeaveDataSaved.addon_version = ADDON_VERSION
end

EVENT_MANAGER:RegisterForEvent(BOOTSTRAP_NAMESPACE, EVENT_ADD_ON_LOADED, onLoaded)
