# Research: Pixel Bus Grid Wrap

**Feature**: `specs/035-grid-wrap/` | **Date**: 2026-07-27

Four questions, three of them arithmetic and one of them the reason this feature
exists in the shape it does.

## R1: Does the column count affect capture cost?

**Question**: The capture is a screen `BitBlt` running at up to 10 Hz. Does
choosing a wider or narrower grid make it cheaper or dearer?

**Finding**: No, to first order. The captured region is
`(block_px * min(N, C)) by (block_px * ceil(N / C))`. For any `N` that fills its
rows exactly, that is `block_px^2 * N` pixels regardless of `C`. For a partial
final row it is `block_px^2 * C * ceil(N / C)`, which rounds up to at most one
row of waste. Sixteen columns and sixty-four columns capture the same area for
the same block count.

**Decision**: Choose the column count on layout grounds and state explicitly that
cost is not a factor.

**Rationale**: Worth establishing before the choice rather than after, because
"wider is faster" or "narrower is cheaper" are both intuitive and both wrong, and
either would have been an easy justification to reach for.

**Consequence**: The one place arrangement does matter is the *shape* of the
capture. A very wide grid is a short wide rectangle and a very narrow one is a
tall thin rectangle; both cover the same area, and neither is meaningfully
cheaper to blit.

## R2: What column count?

**Question**: What value satisfies the constraints, and with how much margin?

**Finding**: Two constraints bind.

| Constraint | Source | Effect |
| --- | --- | --- |
| At least the current block count (9) | FR-009 | Otherwise the wrap moves existing squares and the no-change-today property is lost |
| One row at the largest block size (32) fits the narrowest supported client width (1024) | FR-010 | `C * 32 <= 1024`, so `C <= 32` |

That leaves `9 <= C <= 32`. Within it:

| Candidate | Row width at 32 px | Margin at 1024 | Rows for 256 blocks | Height at 32 px |
| --- | --- | --- | --- | --- |
| 12 | 384 | 2.7x | 22 | 704 |
| 16 | 512 | 2.0x | 16 | 512 |
| 24 | 768 | 1.3x | 11 | 352 |
| 32 | 1024 | 1.0x, none | 8 | 256 |

**Decision**: 16.

**Rationale**: It is the value that keeps both dimensions comfortable rather than
optimising one. 32 has no width margin at all at the largest block size, which
makes the compile-time bound exactly satisfied rather than satisfied, and an
exactly-satisfied bound is one patch away from being violated. 12 buys width
margin nobody needs at the cost of half again as many rows. 16 leaves the widest
grid at half the narrowest supported width and keeps a 256-block grid square,
which is also the arrangement least likely to overlap anything the operator cares
about in a corner of the screen.

Being a power of two is not a reason and is not relied on; the arithmetic is a
modulo and a division either way.

**Alternatives considered**: Making the column count configurable. Rejected
firmly: it is a value the addon and the application must agree on byte for byte,
and a user-editable setting is a way for them to disagree that no amount of
testing can close, since the addon reads its constant from the file the
application deploys and a hand-edited install would silently diverge.

## R3: The shared-constant mechanism already exists

**Question**: What enforces agreement, and does it need extending?

**Finding**: Slice 031 established `beacon::parse_lua_constant(source, name)`,
which reads `local NAME = <decimal or hex>` out of the addon source embedded in
the application binary, and `tests/beacon.rs` already asserts the addon's
`NUM_BLOCKS` equals `pixelbus::NUM_BLOCKS`. The function guards against a prefix
match on a longer name.

**Decision**: `COLUMNS` joins `NUM_BLOCKS` by adding one assertion. Nothing about
the mechanism changes.

**Rationale**: This is the second use of a facility built for exactly this, which
is the argument for having built it. The cost of enforcing the most dangerous
invariant in this feature is one line of test.

## R4: Why the column count is fixed, and what that means for slice 034

**Question**: Issue #3 argued that the render resolution had to be known before
the bus could wrap, and slice 034 built the detection on that basis. Was that
wrong?

**Finding**: The premise was wrong; the work was not wasted, but its role changes.

A derived column count requires the addon and the application to compute the
identical integer from independent measurements. The addon would use its
interface root scaled by the global UI scale; the application uses the window's
client rectangle. When those agree the scheme works. When they differ by one, the
result is not a visible failure: every square from the second row onward shifts by
one position, so the application reads real squares that pass their marker and
checksum checks and reports each signal as some other signal's value. Rounding,
UI-scale handling, overscan adjustment, and a mid-session resolution change are
four independent sources of that disagreement.

Compare the fixed constant: the two sides cannot disagree, because the build reads
one out of the other's source and asserts equality (R3).

**Decision**: Fixed constant. The measurement becomes a fit check.

**Rationale**: The comparison is not "derived is harder" but "derived fails
silently and plausibly". This project's whole marker-and-checksum discipline
exists to prevent exactly the failure mode a derived column count would
reintroduce at the layout layer, underneath the checksums, where they cannot see
it.

**What slice 034 is actually for, then**: confirming the grid fits inside the
client area. That is a real job that only a measurement can do, and the failure it
catches is the same species: a square drawn past the client edge is captured as
black, fails its marker check, and decodes as absent, which is exactly what a
missing addon looks like. Without the check an operator would debug an addon that
is installed, loaded, and drawing correctly.

This is the fifth consecutive slice to reverse part of the design its issue
proposed. In every case the issue predates a close reading of the code it
constrains, which is an argument for writing the reasoning down each time rather
than for writing fewer issues.
