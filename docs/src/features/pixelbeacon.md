# PixelBeacon

ESO exposes its Lua API to addons but provides no direct real-time subscription
for an external desktop application. PixelBeacon is the minimal bridge used by
ESO Weave. It renders solid-color telemetry blocks and has no settings, libraries,
saved variables, or interface beyond those blocks.

PixelBeacon is embedded in the ESO Weave binary and managed from System and
State. The overlay must remain visible at the top-left of the ESO client because
anything drawn over it can make the signal unavailable.

At the default block size, protocol version 5 occupies 512 by 16 physical pixels:
three layout-header cells and twenty-nine payload cells. PixelBeacon calculates
how many complete blocks fit the current client width. Lowering Block Size in
Settings reduces the footprint. A changed size takes effect after the addon is
redeployed, `/reloadui` is run, and ESO Weave restarts.

The overlay cannot be moved. Its origin is part of the shared protocol; changing
only one side would create an undetectable geometry disagreement.

## AddOns directory discovery

On Windows, ESO Weave resolves the Documents known folder through the shell API,
then locates `Elder Scrolls Online\<environment>\AddOns`. On Linux, it reads Steam
library metadata, finds app id `306130`, and resolves the path in the Proton
prefix. The `live` environment is the default and `pts` can be selected. A manual
override is also available.

Discovery never writes outside the resolved AddOns directory.

## Installation and update

Install writes embedded files only beneath `AddOns/PixelBeacon/` and renders the
manifest with the resolved ESO API version. Installing over a managed older copy
is an update. If the AddOns directory does not exist, installation refuses rather
than creating it.

The on-disk manifest is classified as:

| State | Meaning |
| --- | --- |
| Not Installed | No manifest is present |
| Unmanaged | A manifest exists without the managed marker |
| Installed (current) | The marker and embedded version match |
| Installed (outdated) | The marker exists but versions differ |

Removal deletes the PixelBeacon directory only when the manifest contains
`## X-ESO-Weave-Managed: true`. An unmanaged or unreadable directory is never
deleted. The same managed-marker gate applies to every manifest edit.

When ESO is running during an install, update, or removal, use `/reloadui` or
relog before expecting the change in game.

## API version upkeep

The manifest carries the addon version, one or more ESO API versions, and the
managed marker. At startup, ESO Weave uses the greater of its stored last-known
API version and a compiled default. A valid manifest therefore requires neither
network access nor prior state.

For a managed installation with an older primary API token, ESO Weave rewrites
only the `## APIVersion` line, retains greater tokens, and drops lesser ones. A
background startup check reads the official `esoui/esoui` live client version as
a bump-detection signal. It never guesses an undisclosed numeric API version,
never downgrades the manifest, and does not block startup on network failure.

## Fishing and quickslot diagnostics

Fishing state is polled from ESO's interaction state every 100 ms. Bait
consumption while a cast is active is the sole bite signal. PixelBeacon does not
infer a bite from the standing interact prompt or a timer.

Run `/pbquickslot` in ESO for one bounded snapshot of quickslot facts. Run
`/pbquickslot watch` to report only changed snapshots, and run it again to stop.
The diagnostic intentionally omits localized item names and descriptions.
Quickslot classification first shipped in PixelBeacon version 13.

See [Pixel Bus Protocol](../reference/pixel-bus-protocol.md) for the wire contract.
