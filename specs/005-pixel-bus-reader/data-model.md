# Data Model: Pixel Bus Reader

Language-neutral entities. Concrete Rust types are fixed in the tasks phase.

## Rgb

A red-green-blue triple of 8-bit channels sampled from one beacon point.

## FishingSignal

`Waiting`, `Bite`, or `None`, decoded from the fishing block color.

## PixelBusEvent

One of `Heartbeat`, `SignalLost`, `FishingStarted`, `BiteDetected`,
`FishingStopped`, or `Latency(value)`.

## ReaderConfig

| Field | Default | Notes |
| --- | --- | --- |
| tolerance | 2 | Per-channel color match tolerance. |
| heartbeat_timeout_ms | 2000 | Absence past this after the last heartbeat raises SignalLost. |
| status_point | (8, 8) | Sample point for B0. |
| fishing_point | (24, 8) | Sample point for B1. |
| latency_point | (40, 8) | Sample point for B2. |
| interval_fishing_ms | 100 | Sampling interval while fishing is enabled. |
| interval_idle_ms | 1000 | Sampling interval otherwise. |

## Reader State

| Field | Type | Notes |
| --- | --- | --- |
| last_heartbeat_ms | Option u64 | Time of the last observed heartbeat. |
| signal_lost | bool | Whether SignalLost has been emitted and not yet cleared. |
| fishing | FishingSignal | Last decoded fishing signal, for transition detection. |

## Transitions

- Status present: Heartbeat; clear lost; record time; decode fishing and latency.
- Fishing into Waiting from not-Waiting: FishingStarted.
- Fishing into Bite from not-Bite: BiteDetected.
- Fishing into None from not-None: FishingStopped.
- Status absent past timeout and not lost: SignalLost (once); reset fishing to
  None.
