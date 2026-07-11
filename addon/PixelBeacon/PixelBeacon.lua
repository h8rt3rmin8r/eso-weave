-- PixelBeacon: a minimal ESO screen-signal beacon managed by ESO Weave.
--
-- It renders up to three 16 by 16 physical-pixel blocks anchored to the top-left
-- of the client area, encoding load status (B0), fishing state (B1), and server
-- latency (B2). It has no settings, no user interface beyond the blocks, no
-- external libraries, and no saved variables. Values follow the ESO Weave master
-- specification section 9.3.

local ADDON_NAME = "PixelBeacon"
local ADDON_VERSION = 1
local BLOCK_PX = 16
local LATENCY_UPDATE_MS = 1000
local BITE_SAFETY_TIMEOUT_MS = 5000

local wm = WINDOW_MANAGER
local em = EVENT_MANAGER

-- Fishing state: "idle", "waiting" (cast active), or "bite".
local fishingState = "idle"
local fishingInteractionActive = false

local root
local blocks = {}

-- Converts an 8-bit channel to the 0 to 1 range the API expects.
local function channel(value)
    return value / 255
end

-- Converts a physical-pixel measurement to UI units so block geometry is constant
-- in physical pixels regardless of the user's UI scale.
local function physicalToUi(px)
    local scale = GetUIGlobalScale()
    if scale == nil or scale == 0 then
        scale = 1
    end
    return px / scale
end

local function positionBlock(control, xPhysical)
    control:ClearAnchors()
    control:SetAnchor(TOPLEFT, root, TOPLEFT, physicalToUi(xPhysical), 0)
    local dimension = physicalToUi(BLOCK_PX)
    control:SetDimensions(dimension, dimension)
end

local function createBlock(suffix)
    local control = wm:CreateControl(ADDON_NAME .. suffix, root, CT_BACKDROP)
    control:SetEdgeTexture("", 1, 1, 0)
    control:SetEdgeColor(0, 0, 0, 0)
    return control
end

-- B0 Status: solid magenta whenever the addon is loaded and rendering.
local function renderStatus()
    blocks.status:SetCenterColor(channel(0xFF), channel(0x00), channel(0xFF), 1)
    blocks.status:SetHidden(false)
end

-- B1 Fishing: waiting color, bite color, or hidden when idle.
local function renderFishing()
    if fishingState == "waiting" then
        blocks.fishing:SetCenterColor(channel(0x00), channel(0x80), channel(0xFF), 1)
        blocks.fishing:SetHidden(false)
    elseif fishingState == "bite" then
        blocks.fishing:SetCenterColor(channel(0x00), channel(0xFF), channel(0x00), 1)
        blocks.fishing:SetHidden(false)
    else
        blocks.fishing:SetHidden(true)
    end
end

-- B2 Latency: encodes GetLatency() with a marker and a checksum, rendered only
-- while the status block renders.
local function renderLatency()
    if blocks.status:IsHidden() then
        blocks.latency:SetHidden(true)
        return
    end
    local latency = GetLatency()
    if latency < 0 then
        latency = 0
    elseif latency > 1020 then
        latency = 1020
    end
    local red = zo_floor(latency / 4)
    local green = 0xA5
    local blue = 255 - red
    blocks.latency:SetCenterColor(channel(red), channel(green), channel(blue), 1)
    blocks.latency:SetHidden(false)
end

local function setFishingState(state)
    fishingState = state
    renderFishing()
end

-- Bite detection ------------------------------------------------------------

local function isMenuOpen()
    -- A menu is open when neither the gameplay HUD nor the HUD UI scene is shown.
    return not (SCENE_MANAGER:IsShowing("hud") or SCENE_MANAGER:IsShowing("hudui"))
end

local function clearBite()
    if fishingState == "bite" then
        setFishingState("waiting")
    end
    em:UnregisterForUpdate(ADDON_NAME .. "BiteTimeout")
end

local function onBiteSafetyTimeout()
    em:UnregisterForUpdate(ADDON_NAME .. "BiteTimeout")
    clearBite()
end

local function onBite()
    setFishingState("bite")
    em:UnregisterForUpdate(ADDON_NAME .. "BiteTimeout")
    em:RegisterForUpdate(ADDON_NAME .. "BiteTimeout", BITE_SAFETY_TIMEOUT_MS, onBiteSafetyTimeout)
end

-- A stack-count decrease of one on the equipped bait while a fishing interaction
-- is active, and no menu is open, is a bite.
local function onInventorySlotUpdate(_, bagId, slotId, isNewItem, _, _, stackCountChange)
    if isNewItem then
        -- A new item is gained (catch resolved): clear any bite.
        clearBite()
        return
    end
    if not fishingInteractionActive or isMenuOpen() then
        return
    end
    if stackCountChange == -1 then
        onBite()
    end
end

local function onInteractResult()
    -- A fishing interaction is active while the game camera is interacting with a
    -- fishing node.
    fishingInteractionActive = IsGameCameraActive()
        and GetInteractionType() == INTERACTION_FISH
    if fishingInteractionActive then
        setFishingState("waiting")
    end
end

local function onChatterEnd()
    fishingInteractionActive = false
    setFishingState("idle")
    clearBite()
end

-- Initialization ------------------------------------------------------------

local function buildBlocks()
    root = wm:CreateTopLevelWindow(ADDON_NAME .. "Root")
    root:SetAnchor(TOPLEFT, GuiRoot, TOPLEFT, 0, 0)
    root:SetDimensions(physicalToUi(BLOCK_PX * 3), physicalToUi(BLOCK_PX))
    root:SetDrawLayer(DL_OVERLAY)

    blocks.status = createBlock("Status")
    blocks.fishing = createBlock("Fishing")
    blocks.latency = createBlock("Latency")

    positionBlock(blocks.status, 0)
    positionBlock(blocks.fishing, BLOCK_PX)
    positionBlock(blocks.latency, BLOCK_PX * 2)

    renderStatus()
    renderFishing()
    renderLatency()
end

local function onLatencyTick()
    renderStatus()
    renderLatency()
end

local function onAddOnLoaded(_, name)
    if name ~= ADDON_NAME then
        return
    end
    em:UnregisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED)

    buildBlocks()

    em:RegisterForUpdate(ADDON_NAME .. "Latency", LATENCY_UPDATE_MS, onLatencyTick)
    em:RegisterForEvent(ADDON_NAME .. "Inv", EVENT_INVENTORY_SINGLE_SLOT_UPDATE, onInventorySlotUpdate)
    em:RegisterForEvent(ADDON_NAME .. "Interact", EVENT_CLIENT_INTERACT_RESULT, onInteractResult)
    em:RegisterForEvent(ADDON_NAME .. "Chatter", EVENT_CHATTER_END, onChatterEnd)
end

em:RegisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED, onAddOnLoaded)
