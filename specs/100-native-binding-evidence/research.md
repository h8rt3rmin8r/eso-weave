# Research: Native ESO Binding Evidence

## R1. Read-only ESO discovery surface

**Decision**: Resolve each action with `GetActionIndicesFromName`, inspect `GetMaxBindingsPerAction()` slots with `GetActionBindingInfo`, and refresh on `EVENT_KEYBINDINGS_LOADED`, `EVENT_KEYBINDING_SET`, and `EVENT_KEYBINDING_CLEARED` plus the existing periodic backstop.

**Evidence**: The live ESO UI API documentation exposes all five read surfaces and events, and declares the binding result as one primary `KeyCode` plus four modifier `KeyCode` values. The live base-game binding declaration confirms the required action names: `ACTION_BUTTON_3` through `ACTION_BUTTON_9`, `USE_SYNERGY`, `SPECIAL_MOVE_ATTACK`, `SPECIAL_MOVE_BLOCK`, and `GAME_CAMERA_INTERACT`.

**Sources**:

- [ESO UI API documentation](https://github.com/esoui/esoui/blob/live/ESOUIDocumentation.txt)
- [ESO base-game binding declarations](https://github.com/esoui/esoui/blob/live/esoui/ingame/globals/bindings.xml)

**Rejected**: Reading saved binding files. That would be filesystem-dependent, would not guarantee current in-memory state, and is unnecessary when the game provides a read-only runtime API.

## R2. Conflict and duplicate semantics

**Decision**: Inspect every advertised binding slot, ignore `KEY_INVALID`, normalize modifier order, and deduplicate exact raw chords. Zero distinct chords is unbound, one is evaluated for portable support, and more than one is conflicting.

**Rationale**: Choosing the first slot would silently create authority that the user did not provide. Exact duplicates do not introduce ambiguity, while any two distinct assignments do.

**Rejected**: Prefer the first keyboard binding or prefer a supported binding over an unsupported one. Either choice hides ambiguity and can make automation emit a control the operator did not intend as authoritative.

## R3. Portable control registry

**Decision**: Define a stable repository-owned control code registry for ordinary keyboard keys, mouse buttons 1 through 5, and mouse-wheel directions. Map ESO `KEY_*` constants to that registry inside PixelBeacon. Modifier primaries, gamepad keys, combined keys, hold keys, and unknown keys publish unsupported.

**Rationale**: ESO runtime numeric key codes are not a desktop wire contract. A portable registry keeps later Windows and Linux mapping explicit and versioned while still rejecting controls that cannot be synthesized safely.

**Rejected**: Transport localized key names. Names are variable-width, layout-sensitive, and language-sensitive. The parent issue explicitly forbids variable-width string transport.

## R4. One-cell fixed-width encoding

**Decision**: Append one RGB cell for each of the eleven actions. Red and green carry the high and low nibbles of an 8-bit control or state code, each expanded by 17 for color-distance tolerance. Blue carries the four modifier bits in its high nibble and an action-and-control check nibble in its low nibble.

**Rationale**: The complete binding set costs exactly eleven cells. Expanded nibbles tolerate bounded channel drift without allowing an adjacent control code. The check nibble rejects cells moved between action positions. Reserved control codes represent the four non-valid states.

**Rejected**: Two or three cells per action. That would add 22 or 33 blocks for information that fits safely in one. A shared marker with adjacent raw control codes was also rejected because tolerance could turn one control into another.

## R5. Compatibility generation

**Decision**: Advance the negotiated layout protocol from version 5 to version 6, freeze version 5 at 29 payload blocks, and make version 6 carry 40. Layouts before version 6 decode all binding facts as unavailable while preserving their existing telemetry.

**Rationale**: Payload count is part of the negotiated geometry. Explicit version counts prevent the reader from sampling nonexistent blocks from an older addon.

## R6. Snapshot integration

**Decision**: Add a `NativeBindingSet` to reader state, expose it through a getter, and emit one `PixelBusEvent::Bindings` only when the complete set changes or is cleared by signal loss.

**Rationale**: One coherent set avoids eleven events for a binding refresh and provides the natural boundary for #207 and #208. S100 publishes and decodes the set but does not route it to automation.

## R7. Read-only policy guard

**Decision**: Add a static addon policy test that rejects binding mutation API names, reset functions, `Bindings.xml`, custom binding declarations, and binding-related SavedVariables.

**Rationale**: Code review alone does not preserve a permanent authority boundary. A repository test makes future drift visible in CI.

## R8. S099 bootstrap wording

**Decision**: Change the fallback notice to state that the trusted workflow is using its checked-in bootstrap policy because the base checkout did not provide the policy file.

**Rationale**: On protected-main push runs, the old notice incorrectly said the policy was not yet on main even though the fallback was the current protected workflow content. This is a wording correction only and receives a dated changelog decision because the workflow is pinned policy.
