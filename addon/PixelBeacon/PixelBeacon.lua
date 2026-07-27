-- PixelBeacon: a minimal ESO screen-signal beacon managed by ESO Weave.
--
-- It renders up to nine square blocks (BLOCK_PX physical pixels on a side,
-- default 16; the companion sets this value on deploy) anchored to the top-left
-- of the client area, encoding load status (B0), fishing state (B1), server
-- latency (B2), the active weapon bar with each bar's weapon class (B3), and the
-- player's combat state (B4), and which native game UI surface is active (B5).
-- It has no settings, no user interface beyond the blocks, no external libraries,
-- and no saved variables. Values follow the ESO Weave master specification
-- section 10.3, the slice 014 weapon-bar block, the slice 031 combat block, and
-- the slice 032 menu block, and the slice 033 resource blocks (B6 health,
-- B7 stamina, B8 magicka).
--
-- Fishing detection is poll-authoritative, mirroring the game's own reticle: a
-- periodic tick samples the interaction type for the waiting state, and the
-- lure-scoped bait-consumption inventory event is the sole bite signal (the
-- reel-in interact prompt is the standing cast prompt, not a bite indicator).

local ADDON_NAME = "PixelBeacon"
local BLOCK_PX = 16
-- The block count, stated once. The root extent and every block placement derive
-- from it. The companion states the same number once as pixelbus::NUM_BLOCKS, and
-- its test suite parses this line to assert the two agree.
local NUM_BLOCKS = 10
-- The blocks in one row. Blocks wrap to the next row when a row is full, so the
-- beacon grows downward and its width is bounded forever at BLOCK_PX * COLUMNS.
-- The companion states the same number once as pixelbus::COLUMNS and its test
-- suite parses this line too, because a disagreement here would not degrade: it
-- would shift every block from row 1 onward, and the companion would read real
-- blocks that pass their marker and checksum checks while reporting each signal
-- as another signal's value.
local COLUMNS = 16
local LATENCY_UPDATE_MS = 1000
local FAST_UPDATE_MS = 100
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

-- The combat block marker (green channel) and its two state codes (red channel),
-- shared byte for byte with the companion decoder. Blue carries the complement
-- checksum, 255 minus red, so an unrelated color behind an absent block cannot
-- pass validation. The marker is at least 45 away from every other green on the
-- strip, and the two state codes are 192 apart.
local COMBAT_MARKER = 0x2D
local COMBAT_IN_RED = 0xE0
local COMBAT_OUT_RED = 0x20

-- The last rendered combat state, held so the block is redrawn only on a real
-- transition. nil until the first render.
local inCombat = nil

-- The movement block marker (green channel) and its state codes (red channel),
-- shared byte for byte with the companion decoder. Blue carries the complement
-- checksum, 255 minus red. The marker is 22 away from the nearest other green on
-- the grid, eleven times the reader's default tolerance, and the codes are 64
-- apart.
--
-- The code is two bits: bit 0 is mounted, bit 1 is sprint. Only the two codes
-- with the sprint bit clear are defined here, because the game exposes no sprint
-- observable to an addon and this addon therefore never emits one. The companion
-- reserves 0xA0 and 0xE0 for that axis and decodes them as unavailable. They are
-- deliberately absent from this file: a constant the addon never emits would have
-- no counterpart for the agreement check to compare against, and tests/beacon.rs
-- asserts their absence.
local MOVEMENT_MARKER = 0x43
local MOVEMENT_ON_FOOT_RED = 0x20
local MOVEMENT_MOUNTED_RED = 0x60

-- The last rendered mounted state, held so the block is redrawn only on a real
-- transition. nil until the first render.
local isMounted = nil

-- The menu block marker (green channel) and its surface code spacing, shared byte
-- for byte with the companion decoder. Red carries code * MENU_CODE_STEP, blue
-- carries the 255 minus red complement checksum. Codes are 24 apart, twelve times
-- the reader's default tolerance.
local MENU_MARKER = 0xD2
local MENU_CODE_STEP = 24
local MENU_CODE_MAX = 10

-- Surface codes, shared byte for byte with the companion.
local MENU_NONE = 0
local MENU_SYSTEM = 1
local MENU_MAP = 2
local MENU_INVENTORY = 3
local MENU_MAIL = 4
local MENU_CHARACTER = 5
local MENU_GUILD_STORE = 6
local MENU_CROWN_STORE = 7
local MENU_JOURNAL = 8
local MENU_CHAT_ENTRY = 9
local MENU_OTHER = 10

-- Scene name to surface code. A name missing from this table is NOT a problem:
-- the active/inactive decision is made from the game's UI mode before this table
-- is consulted, so an unlisted or renamed scene falls back to MENU_OTHER and
-- still gates. The table only improves what the companion displays.
local SCENE_CODES = {
    gameMenuInGame = MENU_SYSTEM,
    worldMap = MENU_MAP,
    inventory = MENU_INVENTORY,
    mailInbox = MENU_MAIL,
    mailSend = MENU_MAIL,
    mailManager = MENU_MAIL,
    stats = MENU_CHARACTER,
    skills = MENU_CHARACTER,
    championPerks = MENU_CHARACTER,
    tradinghouse = MENU_GUILD_STORE,
    market = MENU_CROWN_STORE,
    journal = MENU_JOURNAL,
}

-- The last rendered surface code, held so the block is redrawn only on a real
-- transition. nil until the first render.
local menuCode = nil

-- Resource block markers (green channel), shared byte for byte with the companion
-- decoder. Red carries the percentage 0 to 100 directly, NOT an index into a color
-- table: a capture that shifts red by one then reads one percent off, where a
-- table would land on whichever entry is nearest in color space and could be wrong
-- by any amount. Blue carries the 255 minus red complement checksum.
local HEALTH_MARKER = 0x16
local STAMINA_MARKER = 0x6D
local MAGICKA_MARKER = 0xBB

-- Published when a resource maximum is zero or unreadable. It passes the marker
-- and checksum checks and fails the range check, so the companion needs no special
-- case for it and it can never be read as a percentage.
local RESOURCE_UNAVAILABLE = 0xFF
local RESOURCE_MAX_PERCENT = 100

-- The last rendered percentages, held so each block is redrawn only on a real
-- change. nil until the first render.
local resourcePercents = { health = nil, stamina = nil, magicka = nil }

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

-- Places block `index` at its grid position: column first, then row. This is the
-- geometry contract shared byte for byte with the companion's
-- pixelbus::block_center. For index < COLUMNS the row is 0 and this reduces to
-- the single-row placement it replaced, which is why introducing the grid moved
-- no existing block.
local function positionBlock(control, index)
    local col = index % COLUMNS
    local row = math.floor(index / COLUMNS)
    control:ClearAnchors()
    control:SetAnchor(
        TOPLEFT,
        root,
        TOPLEFT,
        physicalToUi(BLOCK_PX * col),
        physicalToUi(BLOCK_PX * row)
    )
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

-- B4 Combat ------------------------------------------------------------------

-- Renders B4: the combat marker in green, the state code in red, and the
-- complement checksum in blue. Rendered only while the status block renders, and
-- never hidden to express a state: absence means an addon too old to draw it, so
-- the companion can tell "no combat information" apart from "not in combat".
local function renderCombat()
    if blocks.status:IsHidden() then
        blocks.combat:SetHidden(true)
        return
    end
    local red = inCombat and COMBAT_IN_RED or COMBAT_OUT_RED
    blocks.combat:SetCenterColor(channel(red), channel(COMBAT_MARKER), channel(255 - red), 1)
    blocks.combat:SetHidden(false)
end

-- Recomputes the combat state from the game, returning true when it changed. The
-- event carries the new state, but re-reading keeps one source of truth for both
-- the event path and the post-loading-screen re-baseline.
local function computeCombat()
    local current = IsUnitInCombat("player") and true or false
    if current == inCombat then
        return false
    end
    inCombat = current
    return true
end

-- Reacts to the combat state changing: re-render only on a real transition.
local function onCombatStateChanged()
    if computeCombat() then
        renderCombat()
    end
end

-- B9 Movement -----------------------------------------------------------------

-- Renders B9: the movement marker in green, the state code in red, and the
-- complement checksum in blue. Follows B4 exactly, including never hiding to
-- express a state, so absence means only an addon too old to draw it.
--
-- Only the mounted axis is published. The sprint codes (bit 1 of the two-bit
-- code) are reserved on the companion side and never emitted here, because the
-- game exposes no sprint state to an addon.
local function renderMovement()
    if blocks.status:IsHidden() then
        blocks.movement:SetHidden(true)
        return
    end
    local red = isMounted and MOVEMENT_MOUNTED_RED or MOVEMENT_ON_FOOT_RED
    blocks.movement:SetCenterColor(channel(red), channel(MOVEMENT_MARKER), channel(255 - red), 1)
    blocks.movement:SetHidden(false)
end

-- Recomputes the mounted state from the game, returning true when it changed.
-- The event carries the new state, but re-reading keeps one source of truth for
-- both the event path and the post-loading-screen re-baseline.
local function computeMovement()
    local current = IsMounted() and true or false
    if current == isMounted then
        return false
    end
    isMounted = current
    return true
end

-- Reacts to the mounted state changing: re-render only on a real transition.
local function onMountedStateChanged()
    if computeMovement() then
        renderMovement()
    end
end

-- B5 Menu ---------------------------------------------------------------------

-- Whether any native UI surface is active.
--
-- This deliberately does NOT use the scene test that isMenuOpen() uses below.
-- Opening chat does not hide the gameplay scenes, so a scene test reads "no menu"
-- while the player is typing, which is the single most common case this block
-- exists to cover. The game's own ZO_IngameSceneManager:ConsiderExitingUIMode
-- refuses to leave UI mode while chat text entry is open, so UI mode is the flag
-- that already means what we need. Chat entry is ORed in explicitly so the
-- guarantee does not depend on that internal behavior staying the same.
local function isChatEntryOpen()
    local chat = ZO_GetChatSystem and ZO_GetChatSystem()
    return chat ~= nil and chat.IsTextEntryOpen ~= nil and chat:IsTextEntryOpen()
end

local function isSurfaceActive()
    if IsGameCameraUIModeActive and IsGameCameraUIModeActive() then
        return true
    end
    return isChatEntryOpen() and true or false
end

-- Computes the surface code: the boolean first, the label second.
local function computeMenuCode()
    if not isSurfaceActive() then
        return MENU_NONE
    end
    if isChatEntryOpen() then
        return MENU_CHAT_ENTRY
    end
    local scene = SCENE_MANAGER and SCENE_MANAGER:GetCurrentScene()
    local name = scene and scene.GetName and scene:GetName()
    return (name and SCENE_CODES[name]) or MENU_OTHER
end

-- Renders B5: the menu marker in green, the surface code in red, and the
-- complement checksum in blue. Rendered only while the status block renders, and
-- never hidden to express a state.
local function renderMenu()
    if blocks.status:IsHidden() then
        blocks.menu:SetHidden(true)
        return
    end
    local red = (menuCode or MENU_NONE) * MENU_CODE_STEP
    blocks.menu:SetCenterColor(channel(red), channel(MENU_MARKER), channel(255 - red), 1)
    blocks.menu:SetHidden(false)
end

-- Recomputes the surface code, returning true when it changed.
local function updateMenu()
    local current = computeMenuCode()
    if current == menuCode then
        return false
    end
    menuCode = current
    return true
end

-- B6 to B8 Resources ----------------------------------------------------------

-- The percentage of a pool against its CURRENT maximum, or the unavailable value
-- when the maximum cannot be read or is zero. A percentage of a stale maximum is
-- meaningless, so the maximum is re-read every time rather than cached.
local function resourcePercent(mechanic)
    local current, maximum = GetUnitPower("player", mechanic)
    if current == nil or maximum == nil or maximum <= 0 then
        return RESOURCE_UNAVAILABLE
    end
    local percent = zo_floor((current / maximum) * RESOURCE_MAX_PERCENT + 0.5)
    if percent < 0 then
        percent = 0
    elseif percent > RESOURCE_MAX_PERCENT then
        percent = RESOURCE_MAX_PERCENT
    end
    return percent
end

local function renderResource(block, marker, percent)
    if blocks.status:IsHidden() then
        block:SetHidden(true)
        return
    end
    block:SetCenterColor(channel(percent), channel(marker), channel(255 - percent), 1)
    block:SetHidden(false)
end

-- Recomputes all three, returning true when any changed.
local function updateResources()
    local health = resourcePercent(COMBAT_MECHANIC_FLAGS_HEALTH)
    local stamina = resourcePercent(COMBAT_MECHANIC_FLAGS_STAMINA)
    local magicka = resourcePercent(COMBAT_MECHANIC_FLAGS_MAGICKA)
    if health == resourcePercents.health
        and stamina == resourcePercents.stamina
        and magicka == resourcePercents.magicka then
        return false
    end
    resourcePercents.health = health
    resourcePercents.stamina = stamina
    resourcePercents.magicka = magicka
    return true
end

local function renderResources()
    renderResource(blocks.health, HEALTH_MARKER, resourcePercents.health or RESOURCE_UNAVAILABLE)
    renderResource(blocks.stamina, STAMINA_MARKER, resourcePercents.stamina or RESOURCE_UNAVAILABLE)
    renderResource(blocks.magicka, MAGICKA_MARKER, resourcePercents.magicka or RESOURCE_UNAVAILABLE)
end

-- Reacts to a power update: re-render only on a real change.
local function onPowerUpdate()
    if updateResources() then
        renderResources()
    end
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

-- The fast tick. It drives fishing detection and publishes the menu gate.
--
-- The gate rides this rather than the one-second latency tick on purpose: the
-- companion samples the strip at its own fast cadence, so publishing slowly here
-- would make the addon the dominant source of latency and the gate would engage
-- long after the operator started typing.
local function onFastTick()
    if updateMenu() then
        renderMenu()
    end
    -- A backstop for the power events, so a missed update cannot strand a block
    -- on a stale percentage. Renders only on a real change, like every other block.
    if updateResources() then
        renderResources()
    end
    onFishingTick()
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
    -- The grid extent, derived rather than restated: as wide as the blocks in use
    -- require (never a full row's width for a partial row) and as tall as the
    -- rows they occupy.
    local columnsUsed = math.min(NUM_BLOCKS, COLUMNS)
    local rows = math.ceil(NUM_BLOCKS / COLUMNS)
    root:SetDimensions(
        physicalToUi(BLOCK_PX * columnsUsed),
        physicalToUi(BLOCK_PX * rows)
    )
    root:SetDrawLayer(DL_OVERLAY)

    blocks.status = createBlock("Status")
    blocks.fishing = createBlock("Fishing")
    blocks.latency = createBlock("Latency")
    blocks.weapon = createBlock("Weapon")
    blocks.combat = createBlock("Combat")
    blocks.menu = createBlock("Menu")
    blocks.health = createBlock("Health")
    blocks.stamina = createBlock("Stamina")
    blocks.magicka = createBlock("Magicka")
    blocks.movement = createBlock("Movement")

    -- Block indices, not pixel offsets: the grid decides where an index lands.
    positionBlock(blocks.status, 0)
    positionBlock(blocks.fishing, 1)
    positionBlock(blocks.latency, 2)
    positionBlock(blocks.weapon, 3)
    positionBlock(blocks.combat, 4)
    positionBlock(blocks.menu, 5)
    positionBlock(blocks.health, 6)
    positionBlock(blocks.stamina, 7)
    positionBlock(blocks.magicka, 8)
    positionBlock(blocks.movement, 9)

    renderStatus()
    renderFishing()
    renderLatency()
    computeWeaponBar()
    renderWeapon()
    computeCombat()
    renderCombat()
    updateMenu()
    renderMenu()
    updateResources()
    renderResources()
    computeMovement()
    renderMovement()
end

local function onLatencyTick()
    renderStatus()
    renderLatency()
    -- A 1 Hz recompute picks up equipment changes; renders idempotently, so the
    -- read-back signal only changes on a real weapon or bar change.
    computeWeaponBar()
    renderWeapon()
    -- A 1 Hz re-sync backstop for combat, idempotent like the weapon block, so a
    -- missed event cannot strand the block in a stale state.
    computeCombat()
    renderCombat()
    -- The same backstop for movement, which also keeps the block hidden in step
    -- with the status block when the beacon is hidden entirely.
    computeMovement()
    renderMovement()
end

local function onAddOnLoaded(_, name)
    if name ~= ADDON_NAME then
        return
    end
    em:UnregisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED)

    buildBlocks()

    em:RegisterForUpdate(ADDON_NAME .. "Latency", LATENCY_UPDATE_MS, onLatencyTick)
    em:RegisterForUpdate(ADDON_NAME .. "Fast", FAST_UPDATE_MS, onFastTick)
    em:RegisterForEvent(ADDON_NAME .. "Inv", EVENT_INVENTORY_SINGLE_SLOT_UPDATE, onInventorySlotUpdate)
    em:RegisterForEvent(ADDON_NAME .. "Chatter", EVENT_CHATTER_END, onChatterEnd)

    -- Weapon-bar tracking: react immediately to a real bar swap, and re-baseline
    -- after each loading screen (the pair-changed event may not fire for the
    -- initial state).
    em:RegisterForEvent(ADDON_NAME .. "Bar", EVENT_ACTIVE_WEAPON_PAIR_CHANGED, onWeaponPairChanged)

    -- Combat tracking: react immediately to a real transition, and re-baseline
    -- after each loading screen, because the combat event does not fire for a
    -- state that is already true when the world finishes loading.
    em:RegisterForEvent(ADDON_NAME .. "Combat", EVENT_PLAYER_COMBAT_STATE, onCombatStateChanged)

    -- Movement tracking: same shape as combat. The mount event is authoritative
    -- and instant, and the re-baseline covers a state already true when the world
    -- finishes loading (zoning while mounted, most obviously).
    em:RegisterForEvent(ADDON_NAME .. "Mount", EVENT_MOUNTED_STATE_CHANGED, onMountedStateChanged)

    -- Resource tracking: react to the game's own power updates, filtered to the
    -- player so other units' pools never drive our blocks.
    em:RegisterForEvent(ADDON_NAME .. "Power", EVENT_POWER_UPDATE, onPowerUpdate)
    em:AddFilterForEvent(ADDON_NAME .. "Power", EVENT_POWER_UPDATE, REGISTER_FILTER_UNIT_TAG, "player")

    em:RegisterForEvent(ADDON_NAME .. "Activated", EVENT_PLAYER_ACTIVATED, function()
        computeWeaponBar()
        renderWeapon()
        computeCombat()
        renderCombat()
        updateResources()
        renderResources()
        computeMovement()
        renderMovement()
    end)
end

em:RegisterForEvent(ADDON_NAME, EVENT_ADD_ON_LOADED, onAddOnLoaded)
