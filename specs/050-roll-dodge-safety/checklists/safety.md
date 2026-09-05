# Safety Checklist: Roll-Dodge Weave Gate

- [x] Unknown fails closed for generated weave work
- [x] Physical player input remains pass-through while gated
- [x] Application toggles remain available
- [x] Pending requests are dropped, never replayed
- [x] Mid-sequence gate closure permits releases but blocks new down events
- [x] Watchdog recovery is fixed, bounded, and subordinate to real completion
- [x] Death, zoning, process exit, and signal loss invalidate stale state
- [x] Older protocols cannot fabricate B23 from ordinary screen content
- [x] Fishing and auto-potion behavior is outside this slice
