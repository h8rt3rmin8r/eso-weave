# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

## [Unreleased]

## [0.10.0] - 2026-07-27

### Added

- The application shows what is in the active quickslot and whether it is ready:
  the remaining quickslot cooldown and the slotted item's identity, published by
  PixelBeacon as four new blocks (B16 to B19) and shown as two new rows in the
  Status region. Nothing acts on the values; they are observables, like combat
  state. Addon version advances to 12 (issue #19).

  **This takes the beacon grid onto a second row for the first time.** The block
  count goes from sixteen to twenty against a fixed column count of sixteen, so
  the overlay in the corner of the game client is now two squares tall (256 by 32
  physical pixels at the default size) where it was one. No existing block moved.
  The application now reports the overlay's footprint beside the Block size
  setting and in its log, and the README documents it.

  The reason to add this signal rather than another is that the feature which
  follows it, drinking a potion automatically when a resource runs low, cannot
  safely fire anything without all three facts: that the quickslot holds a potion,
  that it is off cooldown, and which item it is. That consumer is a separate slice
  because it synthesizes a keypress and therefore lands on a constitution
  NON-NEGOTIABLE surface; publishing the observable first means that when a potion
  fires at the wrong moment, the reading underneath it is already known good.

- The application shows how long each skill slot has left before it can be used
  again, published by PixelBeacon as six new blocks (B10 to B15) covering the five
  skills and the ultimate. The Skills grid gains a Cooldown column. Nothing acts
  on the values; they are observables, like combat state. Addon version advances
  to 11 (issue #18).

  The reason to add this signal rather than another is that the weave engine
  currently guesses at exactly this information: the global cooldown is a fixed
  setting and the per-weapon heavy-attack delays are, by the admission of the
  comment beside them, community estimates never validated in game. The game knows
  the real answer per slot and had never been asked.

- The application shows whether the player is mounted, published by PixelBeacon
  as a tenth block (B9). Nothing acts on the value; it is an observable, like
  combat state. Addon version advances to 10 (issue #11).

### Decisions

- 2026-07-27: The quickslot cooldown is read with
  `GetSlotCooldownInfo(GetCurrentQuickslot(), HOTBAR_CATEGORY_QUICKSLOT_WHEEL)`,
  not the `remainingCooldown` return of `GetItemLinkOnUseAbilityInfo` that issue
  #19 proposed. Three reasons, in order of weight: the slot rather than the item
  link is authoritative on whether the thing can be used right now, because
  potions share a cooldown; it is the same call the skill cooldown blocks already
  make, so the quantization contract is shared by construction rather than
  reimplemented beside itself; and the signature turned out to take an explicit
  nilable hotbar category, which is the doubt that sent the issue to the item link
  in the first place. `GetItemLinkOnUseAbilityInfo` is still used, for its
  `hasAbility` return only.
- 2026-07-27: The quickslot publishes the item's identity, not what the potion
  restores. The restore types are not available as structured data: they appear
  only inside a localized human-readable ability description that the game's own
  interface consumes as tooltip text. Parsing it would be a locale-dependent
  heuristic baked into a colour contract shared byte for byte between two
  codebases, which is the same class of construct the sprint verification rejected
  in slice 036. The identity is machine-readable, gives the consumer swap
  detection, and leaves restore awareness addable later in the companion without
  touching the bus contract.
- 2026-07-27: The decoded quickslot carries a cooldown and an optional identity,
  and answers "is there a potion" as the cooldown not being unknown, rather than
  storing that as a third field the way issue #19 proposed. The three-field shape
  is equivalent but admits states that cannot exist, such as a flag claiming a
  potion while the cooldown says unknown. Making those unrepresentable matters
  more here than elsewhere, because the next slice acts on this value by
  synthesizing a keypress.
- 2026-07-27: "Empty quickslot", "not a potion", "no readable cooldown", and
  "unreadable block" are one outcome, not four. The reserved payload and an absent
  block are indistinguishable by construction, so naming them apart would invent a
  distinction the transport cannot carry and tempt the consumer to branch on it.
  This follows the skill cooldown blocks, which settled the same question one
  slice earlier.
- 2026-07-27: The compile-time assertion that the block count does not exceed the
  column count, which slice 037 deliberately left at its limit to fail here, is
  **replaced** rather than relaxed. It did exactly its job: the build stopped at
  the edit that raised the count. Three assertions succeed it, stating that the
  grid is exactly two rows, that the first row is full, and that the last row is
  partial, so the slice that adds a twenty-first block is told just as clearly.
  `grid_rows` and `grid_position` became `const fn` so the assertion calls the
  real function instead of open-coding its arithmetic. Separately, the bound
  `COLUMNS >= NUM_BLOCKS` in the column-count test was restated as
  `COLUMNS >= BLOCKS_AT_WRAP`: its stated justification was always about the block
  count at the moment the wrap shipped, and it was written in terms of the current
  count only because the two were the same number at the time.
- 2026-07-27: The doubled overlay is reported, not managed. The anchor is not
  moved and the square size is not adjusted automatically; both would change
  shared geometry on one side of a byte-for-byte contract, and an origin
  disagreement is exactly the failure neither side can detect. What the
  application owes instead is that the footprint is knowable: a derived caption
  beside the Block size setting, which is where an operator stands when they want
  the overlay smaller, and a debug log line at sampler start, which is the record
  that explains a field report afterwards.
- 2026-07-27: A latent defect in the live-log boundary was found and fixed while
  landing this slice. The no-overlap bound constrained the log panel's *inner*
  height while the boundary is about the space the panel occupies, so the panel's
  frame overhead (two 8-point margins plus the separator stroke) was never
  budgeted. It has been wrong since slice 030 and was unreachable until now: the
  bound only binds when the content comes within that overhead of filling the
  window, and until two status rows were added the content never did. The fix
  derives the overhead from the margin constant and the style rather than a
  literal.
- 2026-07-27: The cooldown blocks cover six slots, not the seven the application
  displays. Synergy gets no block. This was settled by checking the game's own
  action bar, which iterates its slots from the first normal slot index through
  the ultimate slot index; Synergy is outside that range because it is a
  contextual prompt rather than an action slot, so there is no cooldown to read in
  any state. A seventh permanently-unavailable block would have cost a square, a
  marker, a permanently muted interface cell, and would have pushed the grid onto
  a second row to carry nothing. The Synergy row simply shows a dash.
- 2026-07-27: The six cooldown markers are `0x0B`, `0x21`, `0x4E`, `0x92`, `0xC6`,
  and `0xE8`, the midpoints of the six widest gaps left in the block-centre green
  registry. This puts the minimum separation across the whole registry at 11,
  five and a half times the default tolerance, which is tighter than the 22 a
  single added marker achieved and is the honest price of adding six at once: with
  seventeen values in a 256-wide channel and the incumbents fixed, 11 is the best
  achievable minimum. Six distinct markers rather than one shared marker, because
  six adjacent blocks carrying the same kind of value are exactly where an
  off-by-one geometry error would otherwise decode a neighbour's cooldown as this
  slot's, silently and plausibly.
- 2026-07-27: The block count now equals the column count exactly: the grid fills
  one row completely and the next block added anywhere wraps onto a second row.
  The compile-time assertion that the count does not exceed the column count is
  deliberately left in force rather than relaxed. It now sits at its limit with no
  margin, which is precisely when it becomes valuable, and the slice that adds the
  seventeenth block is the one that should be told.
- 2026-07-27: Issue #11 asked for one block covering mounted and sprinting, and
  made verifying the sprint observable a blocking entry condition. The
  verification is done and conclusive: the game exposes no sprint state to an
  addon. `IsUnitSprinting`, `IsPlayerSprinting`, and `EVENT_SPRINT_STATE_CHANGED`
  return zero hits against both the indexed API database and the live
  `esoui/esoui` source, and the only four `Sprint` references in that source are
  the `SPECIAL_MOVE_SPRINT` keybind actions (which call into the engine and
  expose no state), the `IN_WORLD_UI_SETTING_TOGGLE_SPRINT` preference, and a
  `sprintf`. The same evidence shows sprint is toggled on gamepad but held on
  keyboard, with a preference letting keyboard users opt into toggle, so any
  heuristic reconstruction would have to model three input semantics. The slice
  therefore ships the mounted axis alone, which issue #11 names as the sanctioned
  fallback, and sprint becomes a follow-up. This is the entry condition that
  unblocked build plan 010's last slice.
- 2026-07-27: The movement marker is `0x43`. It is the midpoint of the widest gap
  left in the block-center green registry (`0x2D` to `0x5A`), 22 from its nearest
  neighbour, eleven times the default tolerance. `0xE8` tied on separation and
  lost the tiebreak because unrelated screen content clusters at the channel
  extremes and `0xE8` sits 23 from `0xFF`. The nibble-swap convention
  (`0xA5`/`0x5A`, `0x2D`/`0xD2`) is not continued: it was a mnemonic, the
  resource markers already abandoned it, and `0x34` would sit 7 from `0x2D`.
- 2026-07-27: The movement code is two bits (bit 0 mounted, bit 1 sprint) with
  four evenly spaced reds, of which `0xA0` and `0xE0` are reserved for the
  deferred sprint axis and never emitted. Reserving them costs nothing now and
  means a future sprint feature adds its axis without a second block, a second
  marker, or recolouring either live value. They are defined and rejection-tested
  on the companion side only: a constant the addon never emits would have no
  counterpart for the cross-language agreement check, and exempting it would
  weaken the mechanism that makes that check trustworthy. Later slices adding a
  block inherit both the reservation pattern and the naming rule that a signal is
  named for the concept it will grow into, not the axis that ships first.

## [0.9.0] - 2026-07-27

### Added

- The beacon blocks now wrap into a grid instead of extending one ever-widening
  row, so the number of signals PixelBeacon can publish is no longer bounded by
  the width of the screen. At the current nine blocks the grid is exactly the row
  it replaces, block for block and pixel for pixel, which is asserted by test:
  this is a change to the contract and not to what anything draws or reads. The
  application also warns if a block size and block count combination would put
  part of the grid outside the game's client area, because a block drawn past the
  edge reads as absent and looks exactly like a missing addon. Addon version
  advances to 9 (issue #16).

- The application now knows how big the game's render surface is, which physical
  display it is on, how that display is scaled, and where the surface sits on it,
  resolved from the operating system without reading a single pixel of the beacon
  and kept current as the window is moved, resized, or switched between windowed
  and fullscreen. The game's stored video settings are parsed as a cross-check and
  as a pre-launch fallback, and can never override a live reading. Nothing acts on
  the result yet: it is the out-of-band input the future grid-wrap layout needs,
  because the bus cannot be used to locate the bus. No addon change and no
  manifest version change (issue #3).

- The application shows the player's Health, Stamina, and Magicka as percentages
  of their current maximums, published by PixelBeacon as three new blocks (B6 to
  B8). Nothing acts on the values; they are an observable, like combat state.
  Addon version advances to 8 (issue #2).

- The application stops interfering while a native game menu or text field is
  open. PixelBeacon publishes which UI surface is active as a sixth block (B5),
  and while any is up the application suspends key interception and starts no new
  weave or fishing interact, so typing in chat, composing a mail, renaming an item,
  or searching the guild store is no longer disturbed. The application's own
  suspend and fishing hotkeys keep working, exactly as they do while manually
  suspended. Addon version advances to 7 (issue #10).
- The strip is now sampled at the fast cadence whenever the application can
  intercept, not only during a fishing session, because a gate that engages a
  second late does not solve the problem it exists for. A suspended application
  still samples slowly.

- The PixelBeacon addon publishes the player's combat state as a fifth beacon
  block (B4), and the companion decodes it and shows it beside the weapon-bar
  readout. Driven by `EVENT_PLAYER_COMBAT_STATE` with a re-baseline from
  `IsUnitInCombat("player")` after each loading screen, so it is instant on a
  transition and correct after a zone. Nothing in the application acts on the
  value; it is an observable only. Addon version advances to 6 (issue #9).
- The block count is now stated once on each side of the pixel-bus contract, with
  `tests/beacon.rs` parsing the addon source embedded in the binary to assert the
  two sides agree on the strip length and on every combat color. The "shared byte
  for byte" discipline the weapon-class codes have always claimed is now enforced
  by the suite rather than by review.
- A block-center color registry (`pixelbus::BLOCK_CENTER_GREENS`) with a test
  asserting every marker is separated from every other by more than the default
  match tolerance, so a future block cannot introduce a colliding marker without
  the build failing and naming the collision.

### Decisions

- 2026-07-27: The beacon grid's column count is a fixed constant (16) stated once
  on each side of the contract and asserted equal by the build, not a value each
  side derives from the live client width. This reverses the premise of issue #3,
  which justified out-of-band display detection partly as the input to the wrap,
  and it demotes that descriptor to a fit check. The argument is about failure
  modes rather than difficulty: a derived count requires the addon (from its
  interface root, scaled) and the application (from the window's client
  rectangle) to produce the identical integer from independent measurements, and
  a disagreement of one does not degrade. It shifts every block from the second
  row onward, so the application reads real blocks that pass their marker and
  checksum checks and reports each signal as another signal's value, with the
  error sitting underneath the validation built to catch precisely that.
  Rounding, UI-scale handling, overscan, and a mid-session resolution change are
  four independent ways to cause it, none of which announce themselves. 16 was
  chosen because it is at least the block count, so no block moved when the grid
  landed, and one row at the maximum block size is 512 pixels, half the narrowest
  supported client width. Capture cost played no part: the captured area depends
  on the block count and block size, not on how the blocks are arranged.
- 2026-07-27: The game's stored window-mode value is reported exactly as read and
  is never mapped to a named window mode. Issue #3 records that the integer's
  meaning is unconfirmed and asks that it be verified before shipping; instead the
  design's dependence on it was removed, because the mapping is needed for exactly
  one decision (which of two stored resolution pairs is live) and the operating
  system answers that decision authoritatively whenever a window exists. Guessing
  would produce a confident wrong answer on precisely the installs where the two
  pairs differ, and on the one install measured they differ by a lot. The
  consequence is that a configured descriptor is produced only when both stored
  pairs are identical, which is the sole case where the mapping is irrelevant.
  When both a measurement and a stored reading are available, the pair the
  measurement matched is logged alongside the raw mode value, so the evidence the
  issue asked someone to gather by toggling modes and diffing the file now
  accumulates from ordinary use. Nothing acts on that inference.
- 2026-07-27: Display detection extends the existing `SurfaceSampler` seam with a
  defaulted `display()` method rather than introducing a second trait. The
  boundary is the same one that trait already draws (here is where the operating
  system starts), both platform backends already hold the handle it needs, and a
  parallel trait would have meant a second resolution path, a second boxed object
  in the worker, and a second mock. Later work adding a platform backend inherits
  the default, so the addition breaks nothing.
- 2026-07-27: The Linux probe uses only the core X protocol and reports no scale
  factor, so the display it names is the X screen rather than the head the window
  is on. RandR would give per-monitor rectangles and is the better long-term
  answer, but this repository compiles Linux only in the release pipeline, so an
  unverifiable dependency change is a poor trade for a value the wrap layout does
  not need. A scale derived from the screen's millimetre dimensions was rejected:
  drivers routinely fabricate those, and a plausible wrong scale is worse than an
  honest unknown.
- 2026-07-27: The resource blocks encode the percentage numerically in the payload
  channel rather than as an index into a hundred-entry colour table, reversing what
  issue #2 specifies and dissolving the deliverable it names as gating. The issue
  rejected a numeric channel as "more fragile at 1-step resolution", but the
  latency block has encoded a number in a channel since the first slice and decodes
  correctly in the field, and more importantly the two encodings fail differently:
  a lookup table maps a one-step channel error onto whichever entry is nearest in
  colour space, which can be any percentage at all, while a numeric channel maps it
  onto one percent. Unbounded error is acceptable for a discrete state and not for
  an ordered quantity. The guarantee is now provable by enumeration over the full
  publishable range rather than by inspecting a hundred colours by eye, and the 5
  percent fallback the issue allows is unnecessary.
- 2026-07-27: A resource payload slightly above 100 clamps to 100 rather than being
  rejected. Full is the ordinary out-of-combat state, so rejecting it on any upward
  capture drift would have made the most common value the least stable reading on
  the strip.
- 2026-07-27: Resource changes log at TRACE, breaking the DEBUG pattern the combat
  and menu blocks set. Those change a few times a minute; three resources at 1
  percent granularity change many times a second, and at DEBUG they would bury the
  live log, which is the tool that diagnosed every field defect this project has
  had.
- 2026-07-27: Marker selection has stopped being free. Ten greens now occupy the
  channel and the nibble-swap mnemonic that produced `0xA5`/`0x5A` and
  `0x2D`/`0xD2` is abandoned, because the remaining swap partners land badly
  (`0xD6` is four away from the menu marker). The three resource markers `0x16`,
  `0x6D`, and `0xBB` sit in the widest remaining gaps, giving a minimum separation
  of 19. A future block has roughly that much headroom, and the registry test is
  what will say so.
- 2026-07-27: The menu gate reads the game's UI-mode state, not the addon's
  existing HUD scene test, reversing what issue #10 proposed. Opening chat does not
  hide the gameplay scenes, so the scene test reads "no menu" while the player is
  typing in the most common in-game text field, which is the case the gate exists
  to cover. Verified against the live `esoui/esoui` source: the game's own
  `ZO_IngameSceneManager:ConsiderExitingUIMode` declines to leave UI mode while
  chat text entry is open. Chat entry is ORed in explicitly so the guarantee does
  not rest on that internal behavior staying the same.
- 2026-07-27: The gate covers two synthesis paths, not one. The weave path is
  gated at the interception decision; the fishing controller synthesizes on its own
  timers in response to beacon events and never passes through that decision, so
  its autonomous reel and recast are deferred separately. They are deferred and
  retried rather than dropped, so the controller's state never advances past an
  interact the game did not receive. Gating only the interception decision would
  have left a reel free to land in a chat message, and waiting for a bite is
  exactly when someone opens chat. The operator-initiated first cast is not
  deferred: the fishing hotkey is exempt from the gate, and the cast is the direct
  and immediate result of pressing it.
- 2026-07-27: Both gates default to inactive and are set through methods rather
  than constructor parameters. Inactive reproduces the pre-feature behavior, so
  every existing safety test keeps passing with no edit at all, and every failure
  mode (an addon too old to draw the block, a sample that fails validation, a lost
  signal) lands on the safe value by construction rather than by handling. The
  one-way property is proven by an exhaustive cross product over the interception
  decision's inputs rather than by chosen scenarios, because the risk it guards
  against is the combination nobody thought to try.
- 2026-07-27: The fast sampling cadence now applies whenever the application can
  intercept, not unconditionally. Making it unconditional would have left
  `interval_idle_ms` with no effect, which is the mirror image of the defect where
  `interval_fishing_ms` was dead and every fishing session sampled once a second. A
  suspended application intercepts and synthesizes nothing, so it has no gate to
  keep current; both settings keep a real meaning and the extra capture cost is
  paid only while the application is working.
- 2026-07-27: The combat block uses green marker `0x2D`, red state codes `0xE0`
  (in combat) and `0x20` (out of combat), and a blue complement checksum, adopting
  the latency block's marker-and-checksum validation rather than the weapon
  block's exact code match. The weapon block compares its blue channel exactly
  against `0`, `1`, and `2`, codes one apart under a default tolerance of 2, so it
  carries no margin and is safe only because the capture path happens to return
  exact values; repeating that in a new block would build in a known fragility.
  The marker is at least 45 from every other block-center green. Its nibble swap
  `0xD2` is reserved as the next block's marker, continuing the `0xA5` and `0x5A`
  pairing.
- 2026-07-27: `PixelBusReader::observe` now takes a `BlockSamples` struct with a
  derived `Default` instead of four positional `Option<Rgb>` arguments. Adding a
  block becomes one new field, and existing constructions using
  `..Default::default()` keep compiling, so the three following PixelBeacon slices
  do not each rewrite every call site. Named fields also remove a latent hazard:
  four arguments of the same type allowed two blocks to be transposed silently.
- 2026-07-27: The combat block clears to unavailable on any sample that does not
  decode, deliberately diverging from the weapon-bar block, which holds its last
  decoded value while the beacon is alive and clears only on signal loss. Holding
  would let a stale "in combat" survive an addon downgrade or a mid-session
  reload, which is the false reading the tri-state exists to prevent. The cost, a
  one-sample flap on a transient misread, is nil while nothing consumes the value;
  a consumer needing hysteresis adds it at the consumer. Both blocks' behavior is
  asserted in the same test so the divergence cannot drift unnoticed.
- 2026-07-27: The decoded combat state is stored on the weave engine beside the
  latency and weapon-bar state, because that is already the shared home for
  beacon-derived observables and is already behind the mutex both the reader
  thread and the interface thread take. Nothing reads it for any decision, and
  `tests/weave_engine.rs` asserts the engine behaves identically for all three
  values, so wiring combat into timing later has to break that test deliberately
  rather than happen by accident.

## [0.8.1] - 2026-07-25

### Fixed

- The window can be shrunk to its content minimum in a single continuous drag
  again, on both axes. The enforced minimum was measured from the central panel's
  own rectangle, which is the window size less the frame margins, so the window
  was pinned at approximately its own current size and each drag gesture yielded
  only about one text line before locking. The minimum is now intrinsic: the
  widest content-sized block and the height the laid-out content occupies, a
  function of the controls, the theme, and the scale only, never of the window
  (issue #12).
- The live-log pane can no longer be dragged over the Skills controls. The
  boundary is now enforced on every frame rather than computed from the inflated
  content height, the height egui commits after a drag is clamped before it is
  stored or persisted, and a window too short for both the content and six log
  lines now compresses the log rather than covering the controls (issue #13).
- The settings modal grows with the window again. Its height was never set, only
  its width, so it inherited the roughly half-window space its centered area left
  and stayed frozen at its 400 point floor no matter how large the window grew,
  showing about 22 percent of the settings body. The rendered rectangle now equals
  the computed extent on both axes, and 51 percent of the body is visible at the
  maximum (issue #14).

### Changed

- The egui rendering layer is no longer excluded from the tested surface. Its
  sizing behavior is covered by `tests/app_ui_sizing.rs`, which drives the real
  frame body through a headless egui harness and asserts rendered geometry: the
  intrinsic extent, the minimum pushed across a simulated resize gesture, the log
  pane boundary under drag and resize, and the modal's rendered rectangle. Every
  prior window-sizing defect shipped with a fully green suite because the tested
  part (the pure arithmetic) was never the broken part.

### Decisions

- 2026-07-25: Added `egui_kittest` 0.35 as a dev-dependency with
  `default-features = false`. It has five feature flags and zero enabled by
  default, so the test build gains no GPU, windowing, or image stack; the three
  crates added are `egui_kittest`, `kittest`, and `accesskit_consumer`. This
  follows the `ureq` precedent from slice 018 for recording a new dependency. The
  alternative, extracting still more arithmetic into pure functions, was rejected:
  it is exactly the strategy that produced three consecutive green-suite failures.
- 2026-07-25: The settings modal's configured maximum height stays at 880 points.
  FR-017 permits raising it only if half the settings body is not visible at the
  maximum; the measurement is 820 of 1612 points, or 51 percent, so the bar is met
  and the constant is left alone. The margin is thin: adding settings rows will
  push it below half and require revisiting.

## [0.8.0] - 2026-07-25

### Added

- Advanced Pixel Beacon "Block size" setting: the physical-pixel size of each
  beacon square is now configurable (even, 2 to 32; default 16), so the overlay
  can be shrunk on screen. Changing it re-deploys a managed PixelBeacon at the new
  size and takes full effect after an in-game `/reloadui` and an app restart. An
  unmanaged or absent addon folder is never modified (issue #1).

### Changed

- The pixel-bus block geometry is now derived from a single `block_px` value on
  both sides of the bus: the reader's four block-center sample points and the
  Windows screen-capture region are computed from it, and the deployed addon's
  `BLOCK_PX` is written to match at install time (mirroring the manifest's
  API-version templating). At the default size the reader points (8,8), (24,8),
  (40,8), (56,8) and the 64 by 16 capture are byte-for-byte unchanged.

### Fixed

- The window no longer keeps a permanent empty band below the controls. The
  enforced minimum height was latched at the 480 by 420 boot floor even after the
  real content measured shorter; it now hugs the measured content once the layout
  is stable and can shrink when a control row is removed (issue #8).
- The live-log pane is resizable again and reserves no phantom band. Its available
  height is computed against the true content height (not the inflated floor), and
  the enforced minimum open-window height reserves one extra line of drag room so
  the pane is never frozen while the window can still shrink and compress the log
  to six lines (issue #8).
- Enlarging the window with the log open now shares the added height between the
  central area and the log pane in proportion to what each already occupies,
  instead of giving it all to the central area (issue #8).
- Opening then closing the log is height-neutral even when the pane was resized:
  close shrinks the window by the pane's actual height rather than a fixed minimum,
  leaving no residual empty band (issue #8).

### Decisions

- 2026-07-25: Slice 029 (window sizing model rebuild) closes issue #8, a breaking
  UI regression from slice 027. The permanent running-max `content_min` (seeded at
  the boot floor, never released) is replaced by a stable-measured model:
  `content_min_size(measured, boot_floor, stable)` returns the boot floor until
  `measurement_stable` (two consecutive frames equal within 0.5pt) holds, then the
  measured extent per dimension (which may shrink). The log pane's range is
  computed against the true `content_extent.y`; the enforced minimum open-window
  height reserves `open_log_reserve = log_min_height + one row` so the pane is
  resizable at the minimum (max one row above min) while the window stays
  shrinkable. Window-height changes are split proportionally by the live pane
  fraction (`split_log_height`), driving the egui bottom panel to the computed
  height on resize frames and reading the user's drag back otherwise. Open/close is
  height-neutral by the pane's actual height, with the persisted `log_panel_height`
  as the single source of truth. All sizing math is pure and unit-tested; the egui
  glue is validated by build and a desk run. Consequence of the drag-room reserve:
  at the absolute minimum open window with the log dragged to six lines, the
  central area gains one row (~14pt) of slack, a deliberate tradeoff far smaller
  than the fixed dead band it replaces. Issue-driven work, so no `docs/plans/` row
  (same convention as slices 027 and 028). Details in
  `specs/029-window-sizing-rebuild/`.

- 2026-07-25: Slice 028 (pixel-bus block size single source of truth) closes
  issue #1. `block_px` becomes the sole stored geometry value on `ReaderConfig`;
  the four read points are computed methods (`block_center`) and the Windows
  capture region is `capture_dims`, so the reader and addon can no longer drift.
  The addon Lua is templated at deploy (`render_lua` rewrites only the
  `local BLOCK_PX` line, preserving the managed marker), matching the existing
  `rewrite_api_version` pattern; no codegen was added. A block-size change drives
  a managed-only re-deploy (`redeploy_for_block_size`): `ManagedUpToDate` or
  `ManagedVersionMismatch` are re-written, `Unmanaged` and `NotInstalled` are
  skipped and reported, so the safety-critical managed-marker guarantee is
  upheld. Reader geometry applies at the next app start, like the existing
  tolerance and interval settings (the pixel-bus worker owns its config by move);
  a settings-apply log line states the `/reloadui` plus restart requirement.
  Supported sizes are even integers 2 to 32; `sanitize_block_px` corrects an
  invalid value (odd rounds down, out-of-range clamps) with a non-fatal notice.
  The default stays 16, so existing and fresh installs are unchanged; the
  minimum reliably readable size is an owed in-game validation (quickstart
  OV-1..OV-3), not a merge blocker, and the default is not lowered until then.
  This is issue-driven work, so it carries no `docs/plans/` build-plan row (the
  same convention as slice 027). Details in `specs/028-pixelbus-block-size/`.

## [0.7.0] - 2026-07-25

### Added

- Tracked two Claude skills in the repository under `.claude/skills/`:
  `shruggie-speckit` (runs a spec-kit feature slice under autopilot, driving
  the installed `/speckit-*` commands end to end and halting once before push)
  and `gh-fix-ci` (inspects failing GitHub Actions PR checks with `gh`,
  summarizes the failure, and drafts a fix plan before implementing).

### Fixed

- The smallest allowed window now fits every interactive control. Previously,
  at the minimum height the bottom Skills row was clipped (a regression from
  adding the Weapon Bar row) and at the minimum width the Pixel Beacon
  Install/Update/Uninstall row was cut off. The window minimum size is now
  derived from the actual laid-out content extent and pushed at runtime, so it
  fits the content and tracks new rows automatically instead of relying on a
  hand-tuned constant (issue #4).
- The live log viewer no longer covers the Skills area. Enabling it grows the
  window height by the log pane's minimum instead of squeezing the controls,
  the pane now shows at least six lines of text at its minimum height, the
  window is wider while the pane is open so log lines wrap less, and the resize
  bar is hard-clamped so its top can never cross into the Skills area.
  Disabling the viewer shrinks the window back by the same amount, so toggling
  it is height-neutral (issue #5).
- The "Settings saved" confirmation no longer appears for window moves, window
  resizes, or log-pane resizes. Those still persist silently (window geometry
  and log height are saved as before); the confirmation now appears only for a
  meaningful settings change such as a toggle or a form-field edit (issue #6).

### Changed

- Buttons, toggle switches, and dropdown menus are about 20 percent shorter,
  applied consistently through the shared control style, with control text kept
  fully legible in both themes (issue #7).

### Decisions

- 2026-07-24: Slice 027 (UI window-sizing and layout hardening) bundles issues
  #4, #5, #6, #7. The window minimum size is derived from the measured
  central-panel content extent (pushed via `ViewportCommand::MinInnerSize`)
  rather than a fixed constant, so the floor tracks content and cannot silently
  under-size when a UI row is added. Control heights were reduced by the full 20
  percent (`interact_size.y` 22.0 to 17.6, `button_padding.y` 5.0 to 4.0), bounded
  by a legibility floor of the font line height plus 3 points so text is never
  clipped. The log viewer enforces a wider minimum width while open
  (`LOG_WIDTH_BONUS = 100` points). The save-confirmation toast is gated on a new
  meaningful-change signal in the save scheduler; window geometry and log height
  are marked through silent layout paths so persistence is unchanged while the
  toast stays quiet. All behavior is verified at the desk without the game.
  Details in `specs/027-window-sizing-hardening/`.

## [0.6.2] - 2026-07-13

### Fixed

- The app no longer reels in immediately after casting. Version 5 of the
  addon (v0.6.1's version 4) misread the reel-in interact prompt as a bite:
  that prompt is shown for the entire time the line is in the water (it is
  how a player reels in early manually), so every cast registered a bite on
  the first poll tick, reeled 100 ms later, recast, and looped at roughly
  two casts per second, consuming bait each cycle. The prompt is no longer
  consulted at all; the sole bite signal is the equipped bait's stack
  decreasing (the game consumes the bait when the fish takes it), the same
  trigger both proven reference implementations use. A cast now waits as
  long as the fish takes.

### Decisions

- 2026-07-13: Corrected the 2026-07-13 slice 025 decision. The reticle
  action matching `SI_GAMECAMERAACTIONTYPE17` ("Reel In") is the standing
  cast prompt, not a bite indicator, proven by the v0.6.1 field log (bite
  fired 200 to 800 ms after every cast, on the first poll tick). The
  fishing-detection contract (specification section 10.2) now names the
  lure-scoped bait-consumption inventory event as the sole bite signal,
  matching InfoPanel 1.63 and fishyboteso's FishingStateMachine (which
  never compares the action string). Evidence and citations in
  `specs/026-fishing-bite-signal/research.md`.

## [0.6.1] - 2026-07-13

### Added

- Fishing engine transition logging. The fishing controller now narrates every
  state transition at debug level under `eso_weave::fishing`: the cast interact
  and its confirmation window, cast detected, bite detected, reel and recast
  interacts, and every disable with its stop reason. A failed fishing session is
  now diagnosable from the log alone.

### Fixed

- Fishing casts are now detected. PixelBeacon's fishing detection previously
  hinged on `EVENT_CLIENT_INTERACT_RESULT`, which the game's interface registers
  as an error-alert channel that never fires on a clean successful cast, so the
  waiting signal was never rendered and every session timed out to Idle with no
  cast detected. The addon (version 4) now polls the game's interaction state on
  a 100 ms tick, mirroring the game's own reticle: `GetInteractionType() ==
  INTERACTION_FISH` drives the waiting signal, the reticle action matching the
  localized reel-in string is the primary bite signal, and bait consumption
  (now scoped to `ITEM_SOUND_CATEGORY_LURE`, fixing a false-bite defect where
  any single-stack decrease counted) remains the secondary bite signal.

### Decisions

- 2026-07-13: Retired `EVENT_CLIENT_INTERACT_RESULT` from the PixelBeacon
  fishing-detection contract and replaced it with poll-authoritative detection
  (specification section 10.2 updated). Verified against the official interface
  source (github.com/esoui/esoui, branch live, pushed 2026-06-29, APIVersion
  101050): the event's sole official consumer is the alert-text error handler
  (alerthandlers.lua:1387), while the game's reticle polls `GetInteractionType()`
  every frame (reticle.lua:310) and the string table defines the reel-in action
  (en_client.lua:3258). Citations recorded in
  `specs/025-fishing-interaction-detection/research.md`.

## [0.6.0] - 2026-07-13

### Added

- Window geometry persistence. The application window now records its position,
  size, and maximized state as they change and restores them on the next launch,
  reopening where and how it was last left, including on the same monitor of a
  multi-monitor desktop. A recorded position that is no longer on any connected
  monitor (a disconnected or reconfigured display) falls back to a visible default
  rather than opening off-screen, and a degenerate size is clamped to the usable
  range. Geometry is written to `state.json` and a change made immediately before
  closing is flushed on exit.
- An Update control for the PixelBeacon addon, beside Install and Uninstall. It is
  greyed while the addon is not installed and enabled whenever it is installed
  (even when current); pressing it reinstalls the addon (uninstall then install)
  for a clean managed copy. The managed-marker uninstall safety rule is unchanged,
  so an unmanaged folder is never deleted.

### Fixed

- The Weapon Bar line is now drawn in the same grid as the Status, Fishing, and
  Pixel Beacon rows, so its title and state align with them instead of hanging out
  of alignment.
- Dropdowns on the main window (the skill Weave selector and the live-log level
  filter) now keep a constant width regardless of the option selected, so
  selecting an option no longer shifts the rows below.
- The Skills delay column now reads `Delay (ms)` and renders each delay in a
  right-aligned field wide enough for four digits, in both states: an editable
  field when the override is on, and a matching greyed read-only field showing the
  inherited value when it is off, so toggling the override no longer changes the
  cell width or appearance.
- The F2 key is now selectable in the keybindings. It was missing from the key
  list even though it is the default Toggle Fishing binding, so that binding could
  not be seen or changed; F2 now appears and the binding shows it.
- Keybindings now display friendly key names (Number 1 through Number 5, E, R, X,
  Q, Space, F1, F2) instead of raw internal strings; the stored key values are
  unchanged.
- The live-log verbosity dropdown and the settings Log level are now one setting:
  changing either updates the other and the captured verbosity. Hiding the live
  log panel no longer has any effect on the logging level.
- The settings modal now sizes both its width and height to the current window
  each time it opens and as the window resizes, growing in pixels but taking a
  progressively smaller fraction up to a maximum, so it looks right from a small
  window up to a QHD ultrawide display and no longer conforms to a stale size.
- The Settings saved confirmation now uses a green success color so it is easy to
  notice, kept legible in both light and dark themes.
- Fishing could report Casting and then revert to Idle (no cast detected) because
  the Windows sampler read the game window device context with GetPixel, which on a
  hardware-accelerated (DirectX) surface returns the GDI front buffer (black or
  stale pixels), so the PixelBeacon signal was never read. The Windows sampler now
  captures a small strip of the composited desktop at the window's top-left and
  reads the beacon blocks from it, so accelerated content is read as displayed. The
  live log also gains clearer diagnostics: a message when the beacon heartbeat is
  acquired or lost, and the decoded fishing signal and heartbeat age on the verbose
  per-sample trace, so a single session shows whether the signal is being read.

### Decisions

- 2026-07-13: Window geometry is stored as session state in `state.json` (session
  schema 2 to 3, additive forward migration), not in the settings file, because it
  is automatically captured runtime state; this honors the constitution's
  configuration-holds-user-settings-only constraint. eframe's built-in window
  persistence was rejected to avoid a second persistence store and to keep the
  project's JSON-with-schema-version convention.
- 2026-07-13: Added the `Win32_UI_HiDpi` feature to the windows-sys dependency for
  `GetDpiForSystem`, used to convert the physical virtual-screen bounds to egui
  points for the off-screen recovery check. windows-sys is not a pinned artifact.
- 2026-07-13: The Windows pixel-bus capture changed from a window device-context
  `GetPixel` to a screen-composited `BitBlt` of the beacon strip, so DirectX-
  rendered content is read from the desktop framebuffer rather than the GDI front
  buffer. The master specification section 10.3 is updated to match. The addon
  interaction-detection contract and the keypress synthesis are deliberately left
  unchanged so the capture fix stays isolable; the end-to-end result requires an
  in-game validation run (see `specs/024-fishing-capture-hardening/quickstart.md`).

## [0.5.0] - 2026-07-13

### Added

- ESO API version check automation. On startup, off the GUI thread, the app
  fetches the live ESO game client version from the official esoui/esoui GitHub
  live branch as a bump-detection signal, keeps the on-disk PixelBeacon manifest
  `## APIVersion:` current (marker-gated, never downgrading, preserving all other
  lines), and warns in the live log when the client has moved ahead of this build
  so the player knows to update. A compiled default API version guarantees a valid
  manifest with no network and no stored value; the last known API version and
  last seen game version persist in `state.json`.

### Fixed

- Buttons grew slightly on hover and shifted the whole window up and down. The
  egui theme set a wider border stroke on the hovered state, which fed the widget
  inner margin and reflowed the layout. All widget states now use the same
  size-affecting inputs (zero interaction expansion and a 1.0 border stroke width),
  so hovering a control changes only its color, never its size.
- The settings modal filled only part of its width and its scrollbar floated in
  the middle instead of at the right edge. The modal scroll area now disables
  horizontal auto-shrink, so the body fills the modal width and the scrollbar sits
  at the far right edge.

### Documentation

- The master specification is rewritten as `docs/ESO-Weave-Specification-v0.2.0.md`,
  documenting the system as built in a declarative voice with expanded mermaid
  diagrams (system architecture, concurrency and ownership, input interception,
  weave sequence, fishing state machine, pixel-bus pipeline, beacon lifecycle, API
  version check, GUI layout, and config and state persistence). Every repository
  reference is repointed to v0.2.0 and the superseded v0.1.0 file is removed.
- The README fishing usage now includes bait selection: without bait selected in
  game the F2 cast fails and fishing never starts. Bait is added as a
  prerequisite, as an explicit step in "Using it", and as a troubleshooting check.

### Decisions

- 2026-07-12: The master specification is superseded by
  `docs/ESO-Weave-Specification-v0.2.0.md`. Per the constitution, a new master-spec
  version lapses the standing Build-Phase Autopilot authorization; this rewrite was
  produced under an explicit operator kickoff, and the standing autopilot
  authorization is re-affirmed against v0.2.0. The version is bumped (not a v0.1.0
  in-place edit) so the document maturity matches its filename, at the cost of
  repointing the path references, which are updated in the same change.
- 2026-07-12: Added a networked dependency, `ureq` (blocking, rustls TLS), to
  support the startup ESO API version check. No async runtime is introduced; the
  check runs on a `std::thread`. `Cargo.toml` is not a pinned artifact; this entry
  records the added networked dependency per the constitution. The chosen version
  source is the esoui/esoui GitHub `live` branch head commit, because the exact
  numeric API version is only published behind bot challenges a plain client
  cannot pass, whereas GitHub reliably reports the live game version string as the
  bump-detection signal. The numeric value written to the manifest resolves as the
  maximum of the stored last known value and the compiled default.

## [0.4.3] - 2026-07-13

### Documentation

- The README now has dedicated Fishing and Weaving usage sections. Fishing
  documents the interaction model (the F2 hotkey casts for you; do not cast
  first), the PixelBeacon prerequisites (installed, enabled, not out of date,
  beacon visible, window focused), the status progression, the interact key and
  timings, and troubleshooting for the early-stop symptom. Weaving documents the
  single-bar overview, the skill slots and defaults, the weave types, and the
  default timings (global cooldown 500 ms, light 50 ms, heavy 1000 ms, bash
  125 ms); multi-bar weaving is noted as out of scope. The Disclaimer is moved to
  be the next-to-last section, immediately before the License.

### Fixed

- Fishing would start (status "active") and then revert to Idle within a few
  seconds, and a caught fish was never reeled in. Three causes are addressed.
  First, the embedded PixelBeacon manifest declared a stale API version
  (`101044`), so the game flagged the addon out of date and, unless the player had
  enabled out-of-date addons, did not load it at all; with no beacon rendered the
  app never saw the cast or bite signal. The manifest now declares
  `## APIVersion: 101050 101054` (the current live value plus a future value, the
  game's supported two-value form) and bumps `## Version`/`## AddOnVersion` to 3 so
  an existing on-disk install is classified as outdated and refreshed. Second, the
  pixel-bus worker loop always slept at the idle interval (1000 ms) and never used
  the fishing interval (100 ms), so it sampled the beacon and ticked the fishing
  state machine only once per second and missed the transient cast and bite pulses
  and the reel window; the loop now polls at the fishing cadence while a session is
  active. Third, fishing deadlines were stamped on the GUI clock but evaluated on a
  separate worker clock; both now share one monotonic origin. The default arm
  timeout is raised from 5000 ms to 8000 ms for margin. The safety behaviors (no
  blocking on the hook thread, focus-scoped suppression, SignalLost cancels any
  pending interact, and managed-marker-gated uninstall) are unchanged and stay
  tested.

### Added

- The Fishing status now reads in plain language (Casting, Fishing (waiting for a
  bite), Reeling in, Recasting) instead of internal state names, and when the
  routine returns to idle it explains why: Idle (no cast detected) after an arm
  timeout, Idle (signal lost) on signal loss, or a plain Idle when the player
  stopped it. The reason persists until fishing is next started and colors a
  fault-stop as a warning, so an early stop is diagnosable at a glance.

### Decisions

- 2026-07-12: The PixelBeacon manifest `## APIVersion` is set to `101050 101054`,
  closing open item R4 (the live API version could not be confirmed offline when
  the value was last left at 101044). The current live value (game Update 50) is
  declared so the game loads the addon, and a future value is declared using the
  supported two-value form to keep the addon current across several future updates.
  `## Version`/`## AddOnVersion` are bumped to 3 so existing installs refresh; the
  managed-marker line is unchanged so marker-gated safe uninstall is unaffected.
- 2026-07-12: The pixel-bus worker selects its poll interval from the live fishing
  state (fishing interval while active, idle interval otherwise) through a pure,
  unit-tested helper, rather than always polling fast (wasteful) or redesigning the
  loop to be event-driven (out of scope). The GUI and worker share one monotonic
  clock origin so fishing deadlines are stamped and evaluated on one timeline. The
  arm-timeout default rises to 8000 ms as a provisional value pending in-game
  validation.

## [0.4.2] - 2026-07-12

### Fixed

- The default F1 (suspend) and F2 (fishing) hotkeys had no effect in-game. The
  input engine hands their actions off on the action channel, which is drained
  only by the weave worker, and the weave engine maps both toggle actions to no
  operation; the real suspend and fishing state is owned by the GUI intent path
  (`AppModel::apply_intent`), so the hotkeys never reached it. The weave worker
  now forwards the two application-level toggle actions to the GUI over a
  dedicated channel, and the GUI drains them each frame and applies them through
  the same intent path as the Status and Fishing buttons. A hotkey and its button
  now share one state, one persistence mark, and one display update. The
  safety-critical `InputEngine::classify` path (recursion breaking, focus-scoped
  suppression, non-blocking hand-off) is unchanged, and toggles still only take
  effect while the game window is focused.

### Added

- Pixel-bus reader diagnostics for weapon-bar detection. The reader now logs a
  DEBUG line when a weapon bar is first detected (with the decoded bar and
  classes) and when it is cleared on signal loss, and a TRACE line with the raw
  sampled block bytes on every observation. This lets the operator confirm
  in-game whether the weapon-bar signal is present and decoding, and tell a
  present heartbeat with a non-decoding B3 (a stale or misrendered addon) apart
  from no heartbeat at all, without a debugger. Nothing weapon-related logs at the
  default level on an idle sample. The decode path and the B3 encoding are
  unchanged; this slice wires up detection visibility only, with no effect on
  weave timing or skill weaving. The live pixel signal is validated in-game (an
  explicit operator follow-up).

### Decisions

- 2026-07-11: Hotkey suspend and fishing toggles are routed from the weave worker
  to the GUI intent path over a dedicated `std::sync::mpsc` channel, rather than
  mutating the shared state from the worker or splitting the action channel inside
  the safety-critical `InputEngine`. Reusing `AppModel::apply_intent` makes a
  hotkey and its button provably identical in state, persistence, and display, and
  leaves the most-tested core untouched. Worst-case reflection latency is the
  existing 250 ms idle repaint cadence.
- 2026-07-11: Weapon-bar reader diagnostics are layered by log level (DEBUG for
  detected and cleared transitions, TRACE for raw per-sample bytes) so detection
  is diagnosable in-game without emitting per-sample log lines at the default
  level, and without changing decode behavior.

## [0.4.1] - 2026-07-11

### Fixed

- Main-window emphasis labels (the Status, Fishing, Pixel Beacon (Addon), and
  Weapon Bar titles, and the Skills column headers Skill, Enabled, Weave,
  Override, Delay) were nearly unreadable on the dark base: they rendered in the
  dark ink used for text on a gold button. egui derives bold (`.strong()`) text
  color from the active-widget text color, which the brand theme sets to
  `gold_text`, so every strong label inherited it. Emphasis labels now go through
  a new `widgets::label_strong` helper that draws them in the primary text color
  (Inter SemiBold at body size), which the palette legibility test already
  guarantees is readable in both themes. Presentation layer only; no
  safety-critical surface changed.

## [0.4.0] - 2026-07-11

### Added

- Weapon-Bar-Aware Adaptive Timing (S014): the app now detects which weapon bar is
  active and each bar's weapon class and applies per-bar skill-delay timing. The
  PixelBeacon addon gains a fourth pixel block (B3) that encodes the active bar and
  a normalized weapon-class code computed in Lua from the game weapon-type
  constants (so the reader never needs the raw enum integers), edge-detected so
  per-attack redraws do not churn and re-baselined after loading screens. The
  pixel-bus reader decodes it, the weave engine keeps a front and back timing
  profile selected by the active bar, and an "auto timing from weapon" preference
  fills each bar's heavy-attack delay from weapon-class presets (dual wield fastest
  through staves and bow slowest). The main window shows the detected bar and
  weapon classes, and the settings expose the auto-timing toggle and a back-bar
  timing group. Closes research item R1 with a new timing appendix
  (`docs/ESO-Weave-Specification-v0.2.0.md` Appendix A). The exact preset values
  and the pixel signal require in-game validation (an explicit follow-up).
- GUI Ergonomics, Information Design, and Auto-Save (S013): a substantial rework
  of the main window. Two-state controls (suspend and resume, fishing, per-skill
  enabled and override, and every boolean setting) are now colorized toggle
  switches; sections use real headings from the bundled Inter SemiBold weight; the
  status region is renamed (Status, Fishing, Pixel Beacon (Addon)), spread across a
  full-width grid, and shows a normalized, color-coded state field; the Skills grid
  has labeled columns (Skill, Enabled, Weave, Override, Delay) and shows the
  inherited default (muted) instead of a literal zero when no override is set, with
  the override targeting the delay for the row's weave type. The Settings screen is
  now a full-frame modal over a dimmed backdrop that closes on an outside click,
  Escape, or the close control, reorganized into labeled clusters (Appearance,
  Combat timing, Fishing, Pixel Beacon and bus, Logging, Keybindings) with no
  underscores in any label and a short inline help line under every option; the
  previously hidden beacon AddOns-folder override and game environment are now
  surfaced. The live log moved into a resizable bottom panel with a darker
  terminal-like fill and a monospace font. All persistence is now automatic and
  coalesced (no Apply or Save control anywhere): main-window skill edits, the live
  suspend and fishing intents, and the log-panel height are all persisted and
  restored across restarts, with a gentle bottom-right save confirmation. Hover
  tooltips and inline help cover the controls, section titles, and Skills columns.

### Decisions

- 2026-07-11: Weapon-Bar-Aware Adaptive Timing (S014) changes pinned contract
  surfaces. The pixel-bus contract (`specs/004-pixelbeacon-addon/contracts/pixel-bus.md`)
  gains the B3 weapon-bar block at x=48 (sample (56,8)): green `0x5A` marker
  (distinct from the latency marker `0xA5`), red packing the front and back
  weapon-class nibbles, blue the active-bar code; the marker is matched within
  tolerance while the data channels are read exactly. The reader contract
  (`specs/005-pixel-bus-reader/contracts/reader.md`) gains `decode_weapon_bar`, the
  `ActiveBar`/`WeaponClass`/`WeaponBarSignal` types, the `WeaponBar` event (emitted
  only on change and only with a heartbeat), and the fourth sample point, and
  `observe` takes a `b3` argument. The PixelBeacon manifest
  (`addon/PixelBeacon/PixelBeacon.txt`) bumps `## Version` and `## AddOnVersion` to
  2 (single-sourced into the app's embedded-version check); the managed-marker line
  is unchanged so safe uninstall still verifies it, and `## APIVersion` is left at
  101044 because the weapon-bar API predates it and the live value cannot be
  confirmed offline (tracked under R4). The weapon-class codes are shared
  byte-for-byte between the addon Lua and the reader. The R1 appendix in the master
  specification records the evidence-based defaults and marks R1 closed. Session
  state is unaffected. In-game validation of the pixel signal and the exact preset
  timings (including one-hand-and-shield) is owed.
- 2026-07-11: GUI Ergonomics and Auto-Save (S013) persists live session state
  (the suspend and fishing on/off intents) so the app restores the state it was
  closed in. Because the constitution requires the configuration file to hold user
  settings only, with no session, runtime, or derived state, this session state is
  written to a separate `state.json` in the config directory, never to
  `config.json`. Folding it into `config.json` was rejected as a constitution
  violation; not persisting it was rejected because the operator requested the
  restore. Restoring under the focus-scoped input invariant is safe: a restored
  running or fishing-on state performs no input until the game window is focused,
  and the fishing intent restores as a clean re-arm rather than a transient
  sub-state. The log-panel height is a user layout preference and is kept in the
  config UI section. No pinned artifact is changed by this slice.

## [0.3.0] - 2026-07-11

### Added

- Brand and UX Polish (S012): a documented "Arcane gold on ink" brand standard
  (`docs/brand/ESO-Weave-Brand-v1.md`) applied across the app and installers. A new
  woven-caret brand mark (gold and teal on an ink badge) replaces the antique
  two-fish gold mark and is regenerated at every size from SVG masters under
  `assets/brand/` by `assets/brand/generate.sh`. The application window and the
  Windows executable now carry the mark (a `build.rs` embeds the exe icon on
  Windows), and the app is themed for both dark (default) and light modes with the
  bundled Inter typeface, aligned skill columns, and a pointer cursor on every
  clickable control. The installer license page is rendered as clean proportional
  text, the wizard uses branded artwork, and the desktop shortcut is now an opt-in
  Custom Setup feature that is off by default. Adds a GitHub social-share image
  (`assets/eso-weave-social.png`).

### Decisions

- 2026-07-11: Brand and UX Polish (S012) changes the pinned packaging artifacts.
  `wix/main.wxs` switches the wizard from `WixUI_InstallDir` to `WixUI_FeatureTree`,
  adds the `WixUIBannerBmp` and `WixUIDialogBmp` branded-artwork variables, and
  moves the desktop shortcut into its own `Level="2"` (off-by-default) `Feature`,
  nested under the application feature, so it is opt-in via the Custom Setup step,
  while the application feature is `Absent="disallow"` and configurable for the
  install location. The shortcut `Target` values use the resolved path
  `[APPLICATIONFOLDER]eso-weave.exe` instead of the `[#EsoWeaveExe]` file key so
  the opt-in shortcut in a child feature does not trip ICE69 (a cross-feature file
  reference); this was confirmed by building the MSI locally with WiX 3.11. A single
  checkbox on the install page was rejected because it requires replacing the entire
  built-in WixUI dialog set, which cannot be validated without a local WiX build.
  `packaging/windows/License.rtf` is regenerated from `LICENSE` as proportional
  (Segoe UI) RTF with headings and spacing, text preserved verbatim. New pinned
  wizard bitmaps `packaging/windows/banner.bmp` (493x58) and
  `packaging/windows/dialog.bmp` (493x312) are added, and the pinned Linux and
  AppImage icons (`packaging/linux/eso-weave.png`,
  `packaging/appimage/AppDir/eso-weave.png`) are regenerated from the new mark. The
  pinned `.gitattributes` adds `*.bmp binary` so the wizard bitmaps are never line
  normalized. All packaging rasters are reproduced by `assets/brand/generate.sh`
  (ImageMagick 7). Rationale is in `specs/012-brand-ux-polish/`.

## [0.2.0] - 2026-07-11

### Added

- Installer and First-Run Experience (S011): the Windows MSI now presents a guided
  WixUI wizard (welcome, license, install location, progress, finish) with a
  license acceptance gate, adds a desktop shortcut alongside the Start Menu entry,
  and offers a de-elevated "Launch ESO Weave" checkbox on the finish page that
  never launches on a silent install. The application is built for the Windows
  subsystem on release, so it no longer flashes a console window, and a startup
  panic hook shows a native message box and writes a log line so a first-run
  failure is never silent. Adds `packaging/windows/License.rtf` and a bin-local
  `startup` module behind a testable `Notifier` seam; the README documents the
  shortcut and log locations.
- README: the `assets/eso-weave-banner.png` banner now heads the README, and the
  static version badge is bumped automatically by the release rollover so it no
  longer drifts from the released version.

### Decisions

- 2026-07-11: Installer and First-Run Experience (S011) changes the pinned
  packaging artifact `wix/main.wxs` (adding the WixUI_InstallDir wizard, the
  `WixUILicenseRtf` variable, a desktop shortcut component, and the ExitDialog
  launch custom action) and adds `packaging/windows/License.rtf` (the repository
  Apache-2.0 license rendered as RTF for the wizard license page). The
  launch-on-finish uses the WixUI ExitDialog with `WixShellExec` and
  `Impersonate="yes"`, which runs in the InstallUISequence as the invoking user for
  a de-elevated launch; a silent install has no UI sequence and never launches.
  cargo-wix links WixUIExtension and WixUtilExtension by default (verified in the
  cargo-wix linker source), so the pinned `.github/workflows/release.yml` is
  unchanged. The WixShellExec custom action takes no `Return` attribute (WiX
  CNDL0038 forbids `Return` without `ExeCommand`). Rationale is in
  `specs/011-installer-first-run/research.md`.
- 2026-07-11: Automate the README version badge in the pinned `release.toml` with a
  `[[pre-release-replacements]]` entry that rewrites the static shields.io badge
  version on every `cargo release`, and correct `docs/releasing.md` (both pinned
  artifacts) which had described the badge as dynamically read from the latest
  GitHub Release. The badge is static, so it needs the rollover to stay in sync.

## [0.1.1] - 2026-07-11

### Added

- Foundations (S001): a single Rust crate with the Config Store (settings-only
  JSON, corruption fallback with `.invalid` preservation, forward migration) and
  the Logging subsystem (runtime-selectable level, always-on ring buffer,
  optional monthly file sink, input-privacy guarantee).
- Input Engine (S002): a platform-agnostic engine core with focused-window-only
  interception, injected-input recursion breaking, a non-blocking bounded
  hand-off, suspend with suspend-exempt toggles, and a conflict-rejecting
  keybinding model persisted as an additive settings section, behind an
  `InputBackend` seam with a mock plus Windows (low-level hook, SendInput) and
  Linux (evdev grab, uinput) backends.
- Weave Engine (S003): seven skill slots with four weave types, a pure
  sequence builder, global timing with per-slot overrides, monotonic-clock
  cooldown gating, inactive-slot pass-through fed to the Input Engine, and
  additive `skills` and `timing` settings sections, executed through a testable
  `WeaveSink` seam. Adds mouse synthesis (primary and secondary) to the input
  backends.
- PixelBeacon addon (S004): the embedded in-game Lua companion under
  `addon/PixelBeacon/`, rendering the three pixel-bus blocks (status heartbeat,
  fishing state, latency with marker and checksum) at constant physical-pixel
  geometry and detecting a bite from bait consumption, with the managed marker
  line in its manifest. No Rust changes.
- Pixel Bus Reader (S005): pure decoders (status heartbeat, fishing signal,
  checksum-validated latency) with per-channel tolerance and a `PixelBusReader`
  state machine that emits typed events and raises SignalLost on heartbeat
  timeout against an injected clock, behind a `SurfaceSampler` seam with a mock
  plus thin GDI (Windows) and X11 (Linux) samplers.
- Beacon Manager (S006): on-disk lifecycle of the embedded PixelBeacon addon
  (embedded manifest and Lua, single-sourced embedded version), pure four-state
  classification, install confined to the `PixelBeacon` subtree of an injected
  AddOns root, and a marker-gated uninstall that deletes only when the managed
  marker line is verified present in the on-disk manifest. AddOns discovery sits
  behind thin backends (Windows Documents known folder; Linux Steam
  `libraryfolders.vdf` plus Proton app id 306130 compatdata), with a manual path
  override and a selectable `live`/`pts` environment persisted as an additive
  `beacon` settings section, plus a best-effort running-game probe feeding the
  `/reloadui` reminder. No new crates.
- Fishing Controller (S007): a pure, non-blocking fishing state machine (Disabled,
  Armed, Waiting, Reeling, Recast) driven by detector events and an injected clock,
  with configurable arm/reel/recast timing persisted as an additive `fishing`
  settings section. On SignalLost it disables fishing and cancels any pending
  interact rather than blind-firing. A `BiteDetector` trait (with a stub) and a v1
  `PixelBusDetector` adapt the Pixel Bus Reader events (dropping Latency), and the
  interact key is synthesized through a `FishingSink` seam over the input backend
  (mock plus real), with `Key::E` added as the default interact key. No new crates.
- Latency-Adaptive Delays (S008): an opt-in weave enhancement that scales the
  `d_weave` and `d_bash` delays by server latency using
  `effective_delay = base + clamp(round(k * latency), 0, 300)` (k default 0.25),
  leaving `d_heavy` and `global_cooldown` untouched. The computation lives in the
  pure weave sequence builder; `sequence_for` delegates to the adapted builder with
  the feature disabled, so existing weave timing is byte-for-byte unchanged unless
  the feature is enabled with live latency. The engine takes latency in via
  `set_latency(Option<u16>)` (clearing on signal loss reverts to base delays), and
  the enabled flag and `k` persist as an additive `latency` settings section. Off by
  default. No new crates.
- Graphical User Interface (S009): an eframe/egui main window that integrates and
  controls every subsystem, built around a testable application view-model (status
  and beacon-light derivation, UI-intent handling, the settings-to-config mapping
  for all of section 10.3, and the reader-event routing) separated from the egui
  rendering. Status region (Suspend/Resume, Go Fish/Stop, a PixelBeacon status light
  with exact-condition tooltip, Install, confirm-gated Uninstall), skills region
  (per-slot active, weave type, and delay override), a colorized live log panel over
  the ring buffer with pause-scroll and a level filter, and an in-app settings
  surface for every section-10.3 category. A worker loop pumps the pixel bus reader
  and routes its events (latency to the weave engine, signal loss to weave and
  fishing, fishing events to the controller) without blocking the UI thread. Adds the
  `eframe`/`egui` dependency (glow backend) and additive `pixelbus` and `ui` settings
  sections.
- Packaging and Distribution (S010): the artifacts that complete the pinned release
  pipeline, a WiX MSI source (`wix/main.wxs`) and `assets/icon.ico`, cargo-deb
  metadata in `Cargo.toml` with a desktop entry, icon, and a packaged `/dev/uinput`
  udev rule, an AppImage `AppDir`, the `scripts/changelog-section.sh` and
  `scripts/linux-build-deps.sh` scripts, `release.toml` for cargo-release, and a
  Linux evdev-permission section in the README. The MSI installs only the
  application and never writes to game or Documents directories; the version stays
  single-sourced from `Cargo.toml`.

### Changed

### Fixed

### Decisions

- 2026-07-11: Pin the Rust toolchain to 1.96.0 via `rust-toolchain.toml` (a
  pinned artifact; this dated entry records its creation) and adopt serde,
  serde_json, tracing, tracing-subscriber, dirs, time, and thiserror for the
  foundations slice. Rationale is recorded in
  `specs/001-foundations/research.md`.
- 2026-07-11: Adopt target-specific dependencies for the Input Engine backends:
  `windows-sys` (Windows) for the low-level hook, SendInput, and timer
  resolution, and `evdev` plus `x11rb` (Linux) for the keyboard grab, uinput
  synthesis, and X11 focus. The Linux backend is type-checked and clippy-clean on
  the linux target; its runtime is validated on a Linux host. Rationale is in
  `specs/002-input-engine/research.md`.
- 2026-07-11: Beacon Manager (S006) single-sources the embedded addon version by
  parsing the embedded `PixelBeacon.txt` manifest at runtime rather than
  declaring a separate version constant, so the file written on install and the
  version verify compares can never drift. Beacon settings (AddOns path override
  and `live`/`pts` environment) reuse the additive opaque config-section pattern
  (like `timing` and `skills`), requiring no config `schema_version` bump. Enable
  the `windows-sys` `Win32_System_Diagnostics_ToolHelp` feature for the
  best-effort running-game process probe; no new crates. Rationale is in
  `specs/006-beacon-manager/research.md`.
- 2026-07-11: Fishing Controller (S007) is a non-blocking, event-and-tick-driven
  state machine with all delays and timeouts modeled as deadlines against an
  injected clock, so it is pure and fully unit-tested. Its interact sink is a
  dedicated key-only `FishingSink` over the input engine's `InputBackend` (not the
  weave engine's `WeaveSink`), keeping the fishing module dependent only on the
  input engine and the reader. Fishing settings reuse the additive opaque
  config-section pattern (no `schema_version` bump), and a `Key::E` variant was
  added to the input engine as the default interact key (its Windows and Linux
  scan-code mappings included). Rationale is in
  `specs/007-fishing-controller/research.md`.
- 2026-07-11: Latency-Adaptive Delays (S008) computes the effective delay in the
  pure weave sequence builder, exactly where the per-slot-resolved delays are
  consumed, so the scaling respects per-slot overrides and stays unit-testable.
  `sequence_for` delegates to `sequence_for_adapted` with the feature disabled,
  structurally guaranteeing no regression to existing weave timing. The adaptation
  config (enabled flag and `k`, valid finite in `[0.0, 4.0]`) persists as a new
  additive `latency` settings section (no `schema_version` bump); the transient
  current latency is runtime state fed via `set_latency` and never written to the
  config file. Rationale is in
  `specs/008-latency-adaptive-delays/research.md`.
- 2026-07-11: Packaging (S010) creates the pinned artifacts the release pipeline
  references for the first time: `scripts/changelog-section.sh` and
  `scripts/linux-build-deps.sh` (the shared changelog extractor and Linux build
  dependency source), `release.toml` (cargo-release: version bump, CHANGELOG roll,
  `release: vX.Y.Z` commit, tag, push), the WiX MSI source `wix/main.wxs`, the
  AppImage `AppDir` under `packaging/appimage/`, and the udev rule and desktop entry
  under `packaging/linux/`. It also adds `[package.metadata.deb]` to `Cargo.toml`
  and generates `assets/icon.ico` from the logo art with ImageMagick. The pinned
  `.github/workflows/release.yml`, `docs/releasing.md`, and `rust-toolchain.toml`
  are not modified, and no release tag is cut. The MSI never writes to game or
  Documents directories. Rationale is in `specs/010-packaging-and-ci/research.md`.
- 2026-07-11: The GUI (S009) adds the `eframe`/`egui` 0.35 dependency with the glow
  backend (`default-features = false`, features `glow`, `default_fonts`, `x11`,
  `wayland`), the spec-named GUI framework; the glow backend is lighter than wgpu and
  builds on both targets. The correctness-bearing logic lives in a testable
  `app` view-model separated from the egui rendering, which is validated with a
  manual checklist because a native window cannot be exercised in the automated
  environment. The input hook thread keeps its own message pump (the S002 contract)
  while eframe owns the main thread; the subsystems are shared across the
  interception, weave-worker, and pixel-bus worker threads via a `SharedBackend`
  adapter so synthesis stays self-originated. Theme and always-on-top, and pixel bus
  sampling tolerance and intervals, persist as additive `ui` and `pixelbus` settings
  sections (no `schema_version` bump). Rationale is in `specs/009-gui/research.md`.
