# Runtime Safety Checklist: Life State Gate

**Purpose**: Protect every input-emitting path and the pixel-bus trust boundary

- [x] Alive is the only actionable state
- [x] Missing, corrupt, legacy, and lost signals map to Unknown
- [x] Physical interception passes blocked player input through unchanged
- [x] Application toggle hotkeys remain available while life-gated
- [x] Queued weave actions re-check life state before execution
- [x] Fishing timers cannot synthesize while life-gated
- [x] Auto-potion cannot synthesize while life-gated
- [x] Recovery requires fresh eligible work and never replays blocked work
- [x] Addon events and polling converge on one computation function
- [x] Cross-language constants and block count have one agreement test
- [x] Prior recursion, focus, hook-thread, and fishing SignalLost tests remain intact

