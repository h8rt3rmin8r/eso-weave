# Research: Controller Binding Consumption

## Decision 1: Share one autonomous chord executor

**Decision**: Implement chord ownership once in the input layer and wrap it with controller-specific sinks.

**Rationale**: Fishing and Auto Potion need identical native lookup, epoch, modifier, momentary-primary, cleanup, and backend-failure rules. A shared implementation prevents divergent safety behavior.

**Alternatives rejected**: Duplicate the S101 real-sink logic in both modules (unsafe drift); call the weave sink directly (imports weave sequencing policy and the roll gate into unrelated controllers).

## Decision 2: Reuse one non-roll autonomous epoch

**Decision**: Generalize the existing Fishing epoch and gates for both controllers.

**Rationale**: The current shared atomic gates already invalidate Fishing before controller locks. Binding replacement needs the same path, and Auto Potion has the same autonomous race. Neither controller currently treats roll-dodge as an authority.

**Alternatives rejected**: Independent epochs per controller (more mutation sites without stronger guarantees); reuse weave epoch (would add roll policy and couple autonomous work to combat admission).

## Decision 3: Treat legacy keys as ignored input

**Decision**: Remove typed fields and serialized output while allowing serde to ignore old JSON members.

**Rationale**: No value from a legacy gameplay key is safe to migrate into native authority. Silent compatibility preserves startup and the next normal save cleans the document.

**Alternatives rejected**: Warn and default (suggests a fallback still exists); schema rejection (breaks existing users); map old values into live bindings (would override ESO authority).

## Decision 4: Account only for emitted Auto Potion attempts

**Decision**: Change the sink seam to one complete Quickslot attempt returning success.

**Rationale**: Retry time represents an input attempt. Marking it when binding admission or primary down fails suppresses later valid recovery without any action having occurred.

**Alternatives rejected**: Preserve two void key transitions (cannot distinguish admission or synthesis failure); record every rule trigger (misstates actual side effects).

## Decision 5: Show live authority read-only

**Decision**: Replace both editable settings controls with formatted read-only binding states and remediation.

**Rationale**: The removed controls need a visible explanation, and the input engine already owns the coherent snapshot used at runtime.

**Alternatives rejected**: Remove rows entirely (poor discoverability); cache a second UI binding copy (creates another coherence problem).
