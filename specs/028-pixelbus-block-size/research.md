# Research: Pixel-Bus Block Size Single Source of Truth

Phase 0 findings. Each decision resolves an unknown or records a rationale used
by the plan.

## Decision 1: Where the single source of truth lives

- **Decision**: `block_px: u32` on `ReaderConfig` is the sole stored geometry
  value; the four block-center read points become methods computed from it, and
  the Windows capture region is computed from it. The addon derives its geometry
  from its own `BLOCK_PX`, which the companion writes to match at deploy time.
- **Rationale**: The current code stores the four points as literals in
  `ReaderConfig::Default` and the capture size as `CAPTURE_W`/`CAPTURE_H` consts,
  independent of the addon's `BLOCK_PX = 16`. Nothing couples them, so a change to
  one silently mismatches the others. Making the points derived (not stored)
  turns the single source of truth into a structural guarantee rather than a
  convention a maintainer must remember.
- **Alternatives considered**:
  - Keep the four point fields and a `block_px` field, recomputing points in a
    constructor. Rejected: two representations can still drift if a field is set
    directly.
  - A shared constants file generated for both Rust and Lua (codegen). Rejected:
    heavier than needed; the addon already derives everything from one `BLOCK_PX`,
    so only that one line must be written to match, and templating (below)
    already exists in the codebase for the manifest.

## Decision 2: How the addon receives the block size

- **Decision**: Add `render_lua(block_px)` that rewrites only the
  `local BLOCK_PX = N` line of the embedded Lua, and have `install` write the
  rendered Lua. This mirrors `render_manifest` / `rewrite_api_version`, which
  already rewrite only the `## APIVersion:` line of the manifest and preserve
  every other line including the managed marker.
- **Rationale**: The pattern is established, tested, and preserves the managed
  marker by construction. The addon's `buildBlocks` already uses `BLOCK_PX * 4`
  and `positionBlock(..., BLOCK_PX * N)`, so rewriting the single constant is
  sufficient for the drawn geometry to follow.
- **Alternatives considered**:
  - Ship the Lua verbatim and pass the size some other way (saved variable, a
    second file). Rejected: the addon deliberately has no saved variables and no
    settings; a templated constant is the minimal, in-contract change.

## Decision 3: When a size change takes effect (live vs restart)

- **Decision**: Reader geometry (the derived points and capture dimensions)
  applies at the next app start, exactly like the existing `tolerance` and
  sampling-interval settings. The addon re-deploy happens immediately on apply
  (for managed installs), and a notice tells the user the new size takes effect
  after `/reloadui` in game and an ESO Weave restart.
- **Rationale**: The pixel-bus worker takes `reader_config` by value at startup
  (`main.rs`) and `App::reload_from_settings` does not push updated reader config
  to that worker; today changing `tolerance` or the intervals already only takes
  effect on restart. Matching that behavior keeps the change small and avoids
  adding a live-reconfiguration channel to the worker, which is out of scope.
- **Alternatives considered**:
  - Add a live-update channel so the worker rebuilds its reader and sampler on
    apply. Rejected for this slice: larger surface area, and it would be the
    first setting to do so; can be a separate future improvement covering all
    reader settings uniformly.

## Decision 4: Re-deploy trigger and safety

- **Decision**: On settings apply, if the applied `block_px` differs from what is
  deployed and `status(addons_root)` is managed (`ManagedUpToDate` or
  `ManagedVersionMismatch`), re-run `install` with the new size. If the status is
  `Unmanaged` or `NotInstalled`, do not write; emit a notice. Uninstall's
  managed-marker verification is unchanged.
- **Rationale**: `install` only writes inside the `PixelBeacon` subfolder and
  never deletes, so re-deploying a managed folder is safe; refusing unmanaged
  folders upholds the constitution's non-negotiable that an unmanaged folder is
  never modified out from under the user.
- **Alternatives considered**:
  - Re-deploy unconditionally. Rejected: would overwrite a user's hand-installed
    or foreign PixelBeacon, violating the managed-only guarantee.
  - Only flag "re-deploy required" and require the user to click Install.
    Rejected: the issue asks the setting to drive the re-deploy; automatic
    managed re-deploy is the least-friction in-contract behavior.

## Decision 5: Supported sizes and validation

- **Decision**: Supported block sizes are even integers 2..=32 inclusive; default
  16. Validation (`sanitize_block_px`) corrects an invalid value to the nearest
  supported even value with a non-fatal notice: an odd value rounds down to the
  next even, a below-range value clamps to 2, an above-range value clamps to 32,
  a missing/wrong-typed value falls back to the default.
- **Rationale**: Even sizes keep `block_px / 2` an integer so the sampled center
  is always a whole pixel. The upper bound 32 keeps the capture region small
  (32 * 4 = 128 wide worst case) and comfortably includes the current 16; the
  lower bound 2 is the smallest even square. This matches the config-loader
  discipline already used for the sampling intervals (`checked_interval`).
- **Alternatives considered**:
  - Restrict to powers of two {2,4,8,16,32}. Rejected: no correctness benefit
    over "even in range" and needlessly forbids sizes like 6 or 12; the
    even-and-bounded rule is simpler to validate and explain.
  - Allow odd sizes and sample `floor(size/2)`. Rejected: an off-center sample is
    more likely to land on a blended edge pixel at small sizes.

## Decision 6: Minimum reliable size is an owed in-game validation

- **Decision**: This slice enables smaller sizes and proves the geometry by
  construction (unit tests), but does not lower the default. Determining the
  smallest size that the surface samplers read reliably (capture-path filtering,
  UI scale, per-channel tolerance) is recorded as an owed in-game validation in
  [quickstart.md](quickstart.md), consistent with how prior slices tracked owed
  field validations (026 T007, 027 T025).
- **Rationale**: Reliability at 2-4 px is empirical and needs a live game across
  the GDI and X11 backends; it cannot be settled by automated tests. Keeping the
  default at 16 means zero risk to existing users while the smaller sizes become
  available for opt-in field testing.
- **Alternatives considered**:
  - Lower the default to 4 now. Rejected: unvalidated, could silently break
    sampling for users on a filtering-heavy capture path.

## Platform note: Linux sampler needs no change

`X11Sampler` reads each point with a 1x1 `get_image` at whatever coordinates it
is given, so it has no capture-region constant to derive; it automatically reads
the new derived points. Only the Windows GDI sampler, which captures a fixed
strip, needs its capture dimensions derived from `block_px`.
