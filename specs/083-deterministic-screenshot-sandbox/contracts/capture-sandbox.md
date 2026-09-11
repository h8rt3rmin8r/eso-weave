# Contract: Deterministic Documentation Capture

## Invocation contract

```powershell
cargo test --locked --test documentation_capture -- target/documentation-captures
```

The path after `--` is the only capture authority. Omitting it runs validation only. Extra arguments fail with usage guidance.

## Catalog contract

| Order | Scene | Required visible evidence |
| ---: | --- | --- |
| 1 | `first-launch` | Running companion, inactive game, PixelBeacon not installed |
| 2 | `healthy-system-state` | Active focused game, healthy signal, gameplay surface, observed resources |
| 3 | `pixelbeacon-lost` | Active game with explicitly lost PixelBeacon signal |
| 4 | `pixelbeacon-unmanaged` | Unmanaged addon warning and no destructive lifecycle action |
| 5 | `weaving-configuration` | Enabled slots, mixed weave types, and deterministic effective delays |
| 6 | `auto-potion-ready` | Auto Potion requested with ready quickslot and configured resource watches |
| 7 | `auto-potion-blocked` | Auto Potion requested and visibly blocked by unavailable beacon evidence |

## Variant contract

Every catalog row produces dark and light variants at narrow 760 by 1000 and wide 1280 by 900 logical points with one pixel per point. Filenames and receipt order are stable.

## Isolation contract

The target imports no platform hook, input synthesis, GDI/X11 sampling, addon lifecycle mutation, configuration loader, user-directory resolver, random generator, system clock, or native-window runner. It calls only the pure frame seam and development renderer.

The model uses `config_dir = None`, a deterministic beacon path override, and no background workers. The action receiver remains under target control and must be empty after every render.

## Filesystem contract

The destination must resolve below the repository root, cannot equal that root, and cannot traverse a symlink. Only exact named outputs and fixture files are created. The harness never recursively deletes the destination or touches an existing unrelated file.

## Completion contract

`capture-manifest.json` is written only after every requested PNG succeeds. A nonzero exit leaves no receipt that claims a partial run is complete.
