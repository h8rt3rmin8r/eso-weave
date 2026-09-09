# Research: External Encounter Model

## Evidence boundary

S069 describes what a future user-local capture and analysis path must preserve.
It does not claim live parity from synthetic data and does not implement addon,
import, database, UI, or recommendation behavior.

| Source | Pinned revision | Use |
| --- | --- | --- |
| ESOUI Combat Metrics | `6ec1deea4ef8801800dfe88ec79b1f94d0d6303b` | Stored fight families and derived metric behavior |
| ESOUI LibCombat | `80817e6929c7626832f9b9114d3b12bad8d642c1` | Callback families and loss-aware capture vocabulary |
| ESO live API source | `f76cf16c4e5be7b234d15dc7f676febffa64c5bb` | API 101050 event signatures and identifiers |

All three are primary published source snapshots. They are evidence for field
availability and existing practice, not authorization to copy or redistribute
game assets or third-party code.

## Findings and decisions

### Event coverage

Combat Metrics and LibCombat both preserve more than damage rows. Relevant
families include combat results, healing, effects, resources, skill timing,
death and resurrection, boss health, performance, messages, and quickslot use.
The live API exposes identifiers, unit IDs, timestamps, stack state, power
updates, action-slot use, death notifications, and combat state transitions.

Decision: the baseline contract requires encounter boundaries, damage, healing,
effect, resource, cast, bar-change, death, resurrection, boss-health,
performance, quickslot, and discontinuity events. Chat or arbitrary messages
are outside the privacy-minimized baseline.

### Transport

The existing Pixel Bus has intentionally narrow capacity and safety semantics.
Bulk encounter traffic would create coupling and loss pressure that its contract
does not promise to handle.

Decision: a future dedicated addon writes bounded, non-executing SavedVariables
records for an explicit importer. Pixel Bus may retain its existing observations
but is prohibited as the encounter transport.

### Clock, ordering, and loss

Wall clocks can move and callbacks may be batched. Sequence gaps, duplicate
records, reloads, and truncation must remain visible or derived metrics can look
precise while being wrong.

Decision: `(session_id, sequence)` identifies and orders raw events. Monotonic
milliseconds measure durations. A discontinuity record declares a missing range
or clock reset. Undeclared gaps, duplicate sequences, and backward monotonic time
are invalid. Declared loss degrades affected projections instead of being hidden.

### Identity and privacy

Stable account or character identity is not required for encounter-local
metrics and would increase privacy exposure.

Decision: actors receive encounter-local opaque IDs. Names, account handles,
chat, guild, location, and remote upload are omitted by default. Any later
opt-in identity feature needs a separate decision and retention control.

### Raw and derived retention

Combat Metrics constructs observations first and calculates summaries later.
This supports correction as formulas and catalog knowledge improve.

Decision: raw observations are immutable. Build snapshots, catalog join receipts,
and metric projections are replaceable derived records with explicit versions
and source ranges.

### Catalog joins

An encounter can contain an ability ID that a particular catalog snapshot does
not know. Dropping it would corrupt totals; rewriting raw data after discovery
would destroy provenance.

Decision: preserve every numeric ID and amount. A catalog join receipt records
known and unknown IDs for a catalog snapshot. Later resolution creates a new
receipt while the raw-content hash stays unchanged.

### Minimum viable projections

The deterministic spike calculates observed outgoing DPS, observed effective
HPS, ability damage share, effect uptime, and ordered cast sequence. Every
projection carries an algorithm version, event range, and quality state.

Decision: a loss marker yields usable observed values with degraded coverage.
They must not be presented as complete encounter truth.

### Synthetic spike and storage

The fixture exercises every required family, one declared loss range, and an
unknown ability that a later catalog resolves. Raw and gzip byte counts are
measured exactly from the fixture. Linear one-hour and 100-encounter values are
illustrative estimates only.

Decision: real retention guidance waits for representative live captures.

## Alternatives rejected

- **Send the full stream through Pixel Bus**: rejected because it changes the safety channel into a bulk telemetry transport.
- **Store summaries only**: rejected because formulas, loss handling, and catalog joins could not be reproduced.
- **Use wall-clock timestamps for ordering**: rejected because clock adjustments and batching can reorder observations.
- **Drop unknown IDs**: rejected because totals and later catalog reconciliation would become impossible.
- **Claim Combat Metrics parity from the dummy fixture**: rejected because only the same live parse can establish meaningful parity.

## Verification separation

Issue #131 is the dedicated release-verification owner for a representative
same-parse comparison with Combat Metrics. That issue records capture version,
Combat Metrics revision, metric tolerances, loss state, and storage measurements.
It is explicitly nonblocking for the repository-verifiable S069 design contract.
