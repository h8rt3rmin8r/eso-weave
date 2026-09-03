# Contract: Game Context and Dormancy

## Surface decoding

```text
decode_menu(sample, tolerance) -> optional<MenuSurface>
```

- Valid marker, checksum, and code -> observed surface, including None.
- Any invalid component -> unavailable.
- No nearest-state fallback.

## Game Context projection

`GameContext` is derived from runtime, focus, freshness, and surface using the
precedence in `data-model.md`. Only one combination produces Gameplay:

```text
Active + Focused + Fresh + Observed(None)
```

Every named surface and Other menu presents its reviewed label. Unknown and
Signal unavailable never present as Gameplay.

## Input behavior

- Runtime not Active: input engine, fishing, and auto-potion are safety-blocked.
- Focus not Focused: interception and synthesis are safety-blocked.
- Fresh observed named surface: existing menu gates apply to all relevant
  consumers.
- Signal unavailable: Game Context is unavailable. Existing PixelBeacon-optional
  weaving behavior remains; fishing and auto-potion retain their fail-closed
  signal-loss policies.

## Dormant view

While runtime is not Active, these values display `Game not active` rather than
stale data or dashes:

- weapon bar and weapon classes
- combat
- movement
- Health, Stamina, and Magicka
- quickslot and quickslot item
- skill cooldowns
- Game Context uses `Not detected`
- requested automation remains visible but its effective presentation is
  dormant where S041 exposes it

## Help text

The Game Context explanation is identical on pointer hover and keyboard focus.
It states:

- the value combines game activity, focus, signal freshness, and surface
- Gameplay requires a fresh valid no-menu observation
- unavailable means the required evidence is not authoritative
- menu and text-entry states gate input
