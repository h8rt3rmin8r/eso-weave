local ADDON_NAME = "PixelBeacon"
local NAMESPACE = ADDON_NAME .. "DataBootstrap"

-- Keep the historical shared SavedVariables identity. Unknown roots remain
-- untouched, and Catalog owns only its completed snapshot and bounded metadata.
local function onLoaded(_, addonName)
    if addonName ~= ADDON_NAME then return end
    EVENT_MANAGER:UnregisterForEvent(NAMESPACE, EVENT_ADD_ON_LOADED)
    if EsoWeaveDataSaved == nil then
        EsoWeaveDataSaved = { schema_version = 1, addon_version = 1 }
    end
end
EVENT_MANAGER:RegisterForEvent(NAMESPACE, EVENT_ADD_ON_LOADED, onLoaded)
