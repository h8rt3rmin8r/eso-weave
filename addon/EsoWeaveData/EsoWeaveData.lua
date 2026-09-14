local ADDON_NAME = "EsoWeaveData"
local BOOTSTRAP_NAMESPACE = ADDON_NAME .. "Bootstrap"
local SCHEMA_VERSION = 1
local ADDON_VERSION = 1

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
