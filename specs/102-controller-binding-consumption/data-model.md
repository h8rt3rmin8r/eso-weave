# Data Model: Controller Binding Consumption

## AutonomousAuthority

- safety gates: game, focus, suspension, menu, life, world, travel
- epoch: monotonic atomic generation
- bindings: shared coherent `NativeBindingSet`
- physical modifiers: shared atomic `ModifierSet`

Invariant: any gate closure or binding-set replacement advances the epoch before later work can be admitted.

## NativeActionAttempt

- action: `Interact` or `Quickslot`
- admitted epoch: captured controller generation
- chord: copied valid `NativeChord`
- generated modifiers: ordered owned subset
- primary emitted: boolean success boundary

Invariant: the chord comes from the requested action in the same admitted generation. No field supplies a fallback.

## FishingConfig

- arm timeout milliseconds
- reel delay milliseconds
- recast delay milliseconds

Removed: desktop Interact key.

## AutoPotionConfig

- health, magicka, stamina watch enablement and thresholds
- retry interval milliseconds

Removed: desktop Quickslot key.

## BindingPresentation

- action name
- state: valid chord, unavailable, unbound, conflicting, or unsupported
- remediation: change or verify the control in ESO and restore live PixelBeacon evidence

Invariant: presentation derives from the same coherent binding snapshot used by synthesis.
