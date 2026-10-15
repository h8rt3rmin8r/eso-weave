local ADDON_NAME = "PixelBeacon"
local NAMESPACE = ADDON_NAME .. "DataBootstrap"

-- Keep the historical global table/schema, not the legacy disk filename.
-- PixelBeacon.txt owns SavedVariables/PixelBeacon.lua. The desktop separately
-- reads retained EsoWeaveData.lua; this addon neither loads nor rewrites it.
-- Unknown loaded roots remain untouched. Catalog owns only its snapshot/metadata.
local function onLoaded(_, addonName)
    if addonName ~= ADDON_NAME then return end
    EVENT_MANAGER:UnregisterForEvent(NAMESPACE, EVENT_ADD_ON_LOADED)
    if EsoWeaveDataSaved == nil then
        EsoWeaveDataSaved = { schema_version = 1, addon_version = 1 }
    end
end
EVENT_MANAGER:RegisterForEvent(NAMESPACE, EVENT_ADD_ON_LOADED, onLoaded)
