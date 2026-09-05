# Data Model: Negotiated Width-Aware Pixel Geometry

## Constants

- `LAYOUT_PROTOCOL_VERSION = 1`
- `LAYOUT_VERSION_CODE = 0x20`
- `MAX_LAYOUT_TOLERANCE = 0x0F`
- `LAYOUT_HEADER_BLOCKS = 3`
- `LEGACY_COLUMNS = 16`
- `MIN_LAYOUT_COLUMNS = 3`
- `MAX_LAYOUT_COLUMNS = 65535`
- H0 magic `(0x45, 0x53)`
- H1 marker `0x64`
- H2 marker `0x9C`

## `LayoutMode`

```text
Legacy
Negotiated { version: u8 }
```

Legacy is explicit compatibility, not a default geometry. Negotiated identifies
the validated protocol generation.

## `BusLayout`

```text
mode: LayoutMode
columns: u32
payload_offset: u32
```

Invariants:

- legacy is exactly `(columns = 16, payload_offset = 0)`
- negotiated version 1 has `columns in 3..=65535` and `payload_offset = 3`
- payload cell for signal `i` is `payload_offset + i`
- extent covers `payload_offset + NUM_BLOCKS` occupied cells
- every sample point lies inside the extent

Derived operations:

- `payload_point(block_px, index)`
- `rows()`
- `extent(block_px)`
- `fits(surface, block_px)`

## `LayoutFailure`

```text
Missing
InvalidBlockSize
UnsupportedVersion { observed: u8 }
CorruptMagic
CorruptHighByte
CorruptLowByte
ColumnsOutOfRange { observed: u32 }
ExceedsSurface { columns: u32, capacity: u32 }
ExtentExceedsSurface { extent: Size, surface: Size }
```

`CorruptMagic` is reserved for tests and recognized partial-magic cases. A cell
with no recognizable magic may become Legacy only when it is also a valid
legacy heartbeat; otherwise the state is Missing.

## `LayoutState`

```text
Unknown
Ready(BusLayout)
Unavailable(LayoutFailure)
```

Transitions are change-detected. `Unknown` is the pre-sample and reset state.
Unavailable state removes all payload authority until a later valid batch.

## `LayoutHeaderSamples`

```text
h0: Option<Rgb>
h1: Option<Rgb>
h2: Option<Rgb>
```

Decoding order:

1. Check H0 magic within tolerance.
2. If absent, accept Legacy only if H0 is a valid legacy heartbeat.
3. Validate the spaced protocol-version code within capture tolerance.
4. Validate H1 and H2 marker plus complement checksum.
5. Combine the high and low bytes.
6. Validate numeric bounds.
7. If measured, validate columns and occupied extent against the surface.

## `CaptureBatch`

The sampler keeps the latest prepared frame for a requested `Size`. The reader
tracks that prepared size for the current call only:

- initial unknown state prepares the three-cell header extent
- stable state prepares the complete cached-layout extent
- changed layout reuses the frame if contained
- changed layout that grows prepares the new complete extent once more

The prepared frame is never persisted across processes.
