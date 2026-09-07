# Research: Safety Boundaries

## R1: Current weave authorization gap

`InputEngine::classify` checks game activity, focus, suspension, menu, life, roll, world, and travel before handing off a bare `Action`. The worker and `RealSink` see only life, roll, world, and travel. A focus, suspension, or menu transition after handoff can therefore leave queued or running work authorized.

**Decision**: Pair queued non-toggle actions with the authorization epoch observed at interception and extend the weave projection with all applicable gates.

**Rejected**: Checking three more booleans only. A close-reopen transition before dequeue would remain invisible.

## R2: Atomic ordering

The pixel worker already closes life/world/travel authorities before taking controller locks. Menu state currently reaches the shared input flag through later event routing, and suspension changes only the input engine.

**Decision**: Publish unsafe transitions before mutex acquisition and reopen only after controller-local synchronization. Use acquire/release ordering for the epoch and gate snapshots.

## R3: Weave cancellation semantics

`RealSink` already stops new Down operations after cancellation, permits matching Up operations for its held sets, polls during waits, and avoids cooldown consumption when nothing was admitted.

**Decision**: Preserve this mechanism and broaden its cancellation predicate to relevant gate bits plus epoch mismatch.

## R4: Fishing recovery

Fishing owns deadlines and synthesizes directly. Life/world/travel cancellation preserves the requested toggle and resumes only from fresh evidence; focus currently auto-casts on refocus; menu defers reel/recast.

**Decision**: Suspension follows the stricter life/world/travel pattern. It cancels state and deadlines, retains the request, records a reason, and never auto-casts on resume. A final gate-aware sink boundary prevents a check-to-emission race.

**Rejected**: Auto-cast on unsuspend. It creates surprise output unrelated to a fresh user or game action.

## R5: PixelBeacon status gap

`status` treats every manifest read failure as absent even if the target exists. `install_sized` then creates the directory and writes files unconditionally. UI projection folds readable unmanaged content into outdated managed content, exposing Update and Uninstall.

**Decision**: Inspect target shape without following links. Only a missing target is absent. Existing unproven content is unmanaged.

## R6: Writer-level ownership

UI state can become stale between render and click. Existing uninstall and API-version update verify the marker at mutation time, but install does not.

**Decision**: Centralize ownership inspection and require it inside every writer. UI action hiding is defense in depth and user guidance, not authorization.

## R7: Managed update flow

The app currently calls uninstall and then install regardless of uninstall success. Even for managed content, deletion before replacement creates needless loss risk.

**Decision**: Update through the guarded in-place install path. Keep the lower-level guard so stale intents and direct callers cannot bypass protection.

## R8: Target links and partial writes

Following an existing link could escape the AddOns subtree. Atomic replacement of two already-managed files would require a staging and rollback design beyond #94.

**Decision**: Treat linked targets as unmanaged and refuse them. Preserve partial-write recovery as out of scope, while avoiding the existing whole-directory deletion gap.

## R9: Documentation contract

S059 deliberately documented #92 and #94 as limitations and froze them in the coverage manifest.

**Decision**: Convert those deferred entries to covered evidence, update canonical safety prose, and adjust policy fixtures and semantic digest without weakening completeness checks.
