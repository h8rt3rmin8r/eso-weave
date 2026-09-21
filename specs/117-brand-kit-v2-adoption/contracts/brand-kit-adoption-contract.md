# Contract: Brand Kit Adoption

## Source authority

The implementation consumes exact package `eso-weave-brand-1.0.0-bb2.0.0` with archive SHA-256 `b37ac1459666ae33d772229bd5247699c2c845971eabed267ff85465b68a1ba1`. No `latest` alias or unpinned replacement is valid.

## Runtime roles

| Role | Dark | Light |
| --- | --- | --- |
| Background | `#0E1116` | `#F8F8F6` |
| Card | `#171C24` | `#FFFFFF` |
| Overlay | `#0A0D12` | `#FFFFFF` |
| Secondary | `#1D2430` | `#F0EFED` |
| Hover | `#252E3B` | `#F0EFED` |
| Primary text | `#FFFFFF` | `#0A0A0A` |
| Muted text | `#9A9A9A` | `#6B6B6B` |
| Primary action | `#2DD4BF` | `#986000` |
| On primary action | `#000000` | `#FFFFFF` |
| Emphasis | `#2DD4BF` | `#986000` |
| Destructive | `#E9505F` | `#C0293A` |
| On destructive | `#000000` | `#FFFFFF` |
| Border and input | `#262626` | `#E5E5E5` |
| Focus | `#2DD4BF` | `#986000` |

The light background/card and destructive values follow `brand.json` and Interface Canon. They intentionally override conflicting generated egui adapter values.

## Geometry and state

- interaction target minimum: 44 logical points
- focus width: 2 logical points
- focus offset: 2 logical points where the renderer exposes an offset
- default border: 1 logical point
- selected border: 2 logical points
- disabled opacity: 0.48
- hover opacity: 0.92
- control spacing: 12 logical points
- component spacing: 24 logical points
- control radius: 8 logical points
- card/dialog radius: 12 logical points

## Typography

- display: Inter Medium or SemiBold
- body: Inter Regular or Medium
- mono metadata: Geist Mono Regular
- no synthesized undeclared weight
- local bytes only, with default framework fallback after the approved family

## Identity and assets

- full and reduced masters remain byte-identical
- reduced mark threshold: 32 pixels
- horizontal minimum: 160 pixels
- stacked minimum: 144 pixels
- wordmark minimum: 120 pixels
- no artificial crossing overlay, knockout, outline, or substrate separator
- classic Win32 builds use the kit's generated classic ICO
- Linux and installer reference assets retain the already-current official bytes

## Verification

The contract passes only when:

1. adoption metadata and versions are exact;
2. every declared repository artifact matches its SHA-256;
3. runtime semantic token values match this table;
4. all required contrast pairs pass;
5. interactive test allocations meet 44 by 44 points;
6. local CI-parity and documentation checks pass.
