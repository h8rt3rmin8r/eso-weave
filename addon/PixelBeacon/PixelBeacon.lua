-- PixelBeacon: a minimal ESO screen-signal beacon managed by ESO Weave.
--
-- It renders up to four 16 by 16 physical-pixel blocks anchored to the top-left
-- of the client area, encoding load status (B0), fishing state (B1), server
-- latency (B2), and the active weapon bar with each bar's weapon class (B3). It
-- has no settings, no user interface beyond the blocks, no external libraries,
-- and no saved variables. Values follow the ESO Weave master specification
-- section 9.3 and the slice 014 weapon-bar block.
--
-- Fishing detection is poll-authoritative, mirroring the game's own reticle: a
-- periodic tick samples the interaction type for the waiting state, and the
-- lure-scoped bait-consumption inventory event is the sole bite signal (the
-- reel-in interact prompt is the standing cast prompt, not a bite indicator).

local ADDON_NAME = "PixelBeacon"
local BLOCK_PX = 16
local LATENCY_UPDATE_MS = 1000
local FISHING_UPDATE_MS = 100
local BITE_SAFETY_TIMEOUT_MS = 5000

-- Weapon-class codes, shared byte-for-byte with the ESO Weave pixel-bus reader.
local CLASS_NONE = 0
local CLASS_DUAL_WIELD = 1
local CLASS_TWO_HANDED = 2
local CLASS_SWORD_AND_SHIELD = 3
local CLASS_BOW = 4
local CLASS_DESTRUCTION_STAFF = 5
local CLASS_RESTORATION_STAFF = 6

-- The weapon-bar block marker (green channel), distinct from the latency marker.
local WEAPON_MARKER = 0x5A

-- The decoded weapon-bar state: active bar code (0 unknown, 1 front, 2 back) and
-- each bar's class code. Held across indeterminate reads (locked or none pair).
local weaponBar = { bar = 0, front = CLASS_NONE, back = CLASS_NONE }

local wm = WINDOW_MANAGER
local em = EVENT_MANAGER

-- Fishing state: "idle", "waiting" (cast active), or "bite".
local fishingState = "idle"

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

-- B3 Weapon bar ------------------------------------------------------------

-- Maps a weapon pair (main-hand and off-hand weapon types) to a normalized class
-- code from the named WEAPONTYPE_* constants, so the reader never needs the raw
-- game enum integers.
local function classifyWeaponPair(mainType, offType)
    if mainType == WEAPONTYPE_NONE then
        return CLASS_NONE
    elseif mainType == WEAPONTYPE_TWO_HANDED_SWORD
        or mainType == WEAPONTYPE_TWO_HANDED_AXE
        or mainType == WEAPONTYPE_TWO_HANDED_HAMMER then
        return CLASS_TWO_HANDED
    elseif mainType == WEAPONTYPE_BOW then
        return CLASS_BOW
    elseif mainType == WEAPONTYPE_FIRE_STAFF
        or mainType == WEAPONTYPE_FROST_STAFF
        or mainType == WEAPONTYPE_LIGHTNING_STAFF then
        return CLASS_DESTRUCTION_STAFF
    elseif mainType == WEAPONTYPE_HEALING_STAFF then
        return CLASS_RESTORATION_STAFF
    elseif offType == WEAPONTYPE_SHIELD then
        return CLASS_SWORD_AND_SHIELD
    else
        -- A one-handed melee weapon with another weapon (or nothing) in the off
        -- hand is treated as dual wield for timing purposes.
        return CLASS_DUAL_WIELD
    end
end

-- Recomputes the weapon-bar state from the game, holding the last good bar when
-- the pair is locked or none. Returns true when the stored state changed.
local function computeWeaponBar()
    local pair, locked = GetActiveWeaponPairInfo()
    local bar = weaponBar.bar
    if not locked then
        if pair == ACTIVE_WEAPON_PAIR_MAIN then
            bar = 1
        elseif pair == ACTIVE_WEAPON_PAIR_BACKUP then
            bar = 2
        end
        -- ACTIVE_WEAPON_PAIR_NONE leaves the last good bar unchanged.
    end

    local front = classifyWeaponPair(
        GetItemWeaponType(BAG_WORN, EQUIP_SLOT_MAIN_HAND),
        GetItemWeaponType(BAG_WORN, EQUIP_SLOT_OFF_HAND)
    )
    local back = classifyWeaponPair(
        GetItemWeaponType(BAG_WORN, EQUIP_SLOT_BACKUP_MAIN),
        GetItemWeaponType(BAG_WORN, EQUIP_SLOT_BACKUP_OFF)
    )

    if bar == weaponBar.bar and front == weaponBar.front and back == weaponBar.back then
        return false
    end
    weaponBar.bar = bar
    weaponBar.front = front
    weaponBar.back = back
    return true
end

-- Renders B3: green marker, red packs the front and back class nibbles, blue is
-- the active-bar code. Rendered only while the status block renders.
local function renderWeapon()
    if blocks.status:IsHidden() then
        blocks.weapon:SetHidden(true)
        return
    end
    local red = weaponBar.front * 16 + weaponBar.back
    blocks.weapon:SetCenterColor(channel(red), channel(WEAPON_MARKER), channel(weaponBar.bar), 1)
    blocks.weapon:SetHidden(false)
end

-- Reacts to a weapon-pair-changed event, which fires on nearly every attack:
-- re-render only when the decoded state actually changes.
local function onWeaponPairChanged()
    if computeWeaponBar() then
        renderWeapon()
    end
end

local function setFishingState(state)
    fishingState = state
    renderFishing()
end

-- Fishing detection ----------------------------------------------------------

local function isMenuOpen()
    -- A menu is open when neither the gameplay HUD nor the HUD UI scene is shown.
    return not (SCENE_MANAGER:IsShowing("hud") or SCENE_MANAGER:IsShowing("hudui"))
end

local function clearBiteTimer()
    em:UnregisterForUpdate(ADDON_NAME .. "BiteTimeout")
end

local function onBiteSafetyTimeout()
    -- Safety net unchanged: an unreeled bite reverts to waiting.
    clearBiteTimer()
    if fishingState == "bite" then
        setFishingState("waiting")
    end
end

local function onBite()
    setFishingState("bite")
    clearBiteTimer()
    em:RegisterForUpdate(ADDON_NAME .. "BiteTimeout", BITE_SAFETY_TIMEOUT_MS, onBiteSafetyTimeout)
end

-- The authoritative fishing poll. The game's own reticle samples the interaction
-- type every frame; an active cast holds INTERACTION_FISH for the whole
-- cast-wait-bite window. The tick tracks only the cast: the reel-in interact
-- prompt is the standing prompt for the entire cast (it is how a player reels
-- in early manually) and is never consulted, and the poll never demotes a
-- rendered bite; a bite ends on catch resolution, the safety timeout, or the
-- interaction ending.
local function onFishingTick()
    if GetInteractionType() ~= INTERACTION_FISH then
        if fishingState ~= "idle" then
            clearBiteTimer()
            setFishingState("idle")
        end
        return
    end
    if fishingState == "idle" then
        setFishingState("waiting")
    end
end

-- The sole bite signal: the equipped bait's stack decreases by one while a cast
-- is active and no menu is open (the game consumes the bait when the fish takes
-- it). The lure sound category scopes the decrease to bait, so unrelated
-- consumables are never reported as bites.
local function onInventorySlotUpdate(_, _, _, isNewItem, itemSoundCategory, _, stackCountChange)
    if isNewItem then
        -- A new item is gained (catch resolved): the bite is over.
        if fishingState == "bite" then
            clearBiteTimer()
            setFishingState("waiting")
        end
        return
    end
    if fishingState == "idle" or isMenuOpen() then
        return
    end
    if stackCountChange == -1 and itemSoundCategory == ITEM_SOUND_CATEGORY_LURE then
        onBite()
    end
end

local function onChatterEnd()
    -- Cleanup only; the fishing tick is authoritative and would converge anyway.
    clearBiteTimer()
    setFishingState("idle")
end

-- Initialization ------------------------------------------------------------

local function buildBlocks()
    root = wm:CreateTopLevelWindow(ADDON_NAME .. "Root")
    root:SetAnchor(TOPLEFT, GuiRoot, TOPLEFT, 0, 0)
    root:SetDimensions(physicalToUi(BLOCK_PX * 4), physicalToUi(BLOCK_PX))
    root:SetDrawLayer(DL_OVERLAY)

    blocks.status = createBlock("Status")
    blocks.fishing = createBlock("Fishing")
    blocks.latency = createBlock("Latency")
    blocks.weapon = createBlock("Weapon")

    positionBlock(blocks.status, 0)
    positionBlock(blocks.fishing, BLOCK_PX)
    positionBlock(blocks.latency, BLOCK_PX * 2)
    positionBlock(blocks.weapon, BLOCK_PX * 3)

    renderStatus()
    renderFishing()
    renderLatency()
    computeWeaponBar()
    renderWeapon()
end

local function onLatencyTick()
    renderStatus()
    renderLatency()
    -- A 1 Hz recompute picks up equipment changes; renders idempotently, so the
    -- read-back signal only changes on a real weapon or bar change.
    computeWeaponBar()
    renderWeapon()
end

local function onAddOnLoaded(_, name)
    if name ~= ADDON_NAME then
        return
    end
    em:UnregisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED)

    buildBlocks()

    em:RegisterForUpdate(ADDON_NAME .. "Latency", LATENCY_UPDATE_MS, onLatencyTick)
    em:RegisterForUpdate(ADDON_NAME .. "Fishing", FISHING_UPDATE_MS, onFishingTick)
    em:RegisterForEvent(ADDON_NAME .. "Inv", EVENT_INVENTORY_SINGLE_SLOT_UPDATE, onInventorySlotUpdate)
    em:RegisterForEvent(ADDON_NAME .. "Chatter", EVENT_CHATTER_END, onChatterEnd)

    -- Weapon-bar tracking: react immediately to a real bar swap, and re-baseline
    -- after each loading screen (the pair-changed event may not fire for the
    -- initial state).
    em:RegisterForEvent(ADDON_NAME .. "Bar", EVENT_ACTIVE_WEAPON_PAIR_CHANGED, onWeaponPairChanged)
    em:RegisterForEvent(ADDON_NAME .. "Activated", EVENT_PLAYER_ACTIVATED, function()
        computeWeaponBar()
        renderWeapon()
    end)
end

em:RegisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED, onAddOnLoaded)
