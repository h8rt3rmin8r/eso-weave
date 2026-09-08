# First Launch

This walkthrough establishes a safe, observable baseline before any automation
is enabled. Keep ESO Weave visible beside ESO while completing the checks.

## 1. Confirm the desktop application

Launch ESO Weave from its installed shortcut or package. The window contains a
Live HUD, System and State, Skills, and an optional Live Log. **ESO Weave** should
show **Active** unless you intentionally suspended it.

Open **Settings** from the File menu and confirm the theme and platform-specific
options are usable. Changes are saved automatically. All Fishing controls and
the Pixel Bus tolerance and sampling intervals apply live. Only Pixel Bus Block
Size remains staged for an ESO reload
or relog plus an ESO Weave restart; see
[Settings application timing](../reference/settings.md#application-timing).

## 2. Confirm ESO discovery

Start the ESO launcher and game. In **System and State**, the **Game** row combines
runtime and installation evidence. A normal installed game progresses from
**Launcher open** to **Active** and names one provider: ESO Store, Steam, Epic
Games, or Steam Proton.

If the row says **Not detected**, **Unknown**, or **Multiple installs detected**,
do not enable automation. Follow [Game discovery and runtime](troubleshooting.md#game-discovery-and-runtime).

## 3. Install PixelBeacon

PixelBeacon is the bundled addon that displays the color-block telemetry overlay.
In **System and State**, choose **Install** or **Update** beside **PixelBeacon
Status**. If ESO is already running, enter `/reloadui` in ESO or relog.

At ESO's character selection, enable the PixelBeacon addon if the game has not
enabled it automatically. The small block overlay must remain visible at the
top-left of the game client. Do not cover that area with another window or
overlay.

If an existing PixelBeacon target was installed or edited outside ESO Weave,
System and State shows **Unmanaged (not modified)** and offers no lifecycle
buttons. Move or remove only that exact target manually before using **Install**.
ESO Weave does not overwrite an unproven target.

## 4. Verify fresh telemetry

Focus the ESO game window and enter the world. Wait for loading to finish. A
healthy baseline has:

| Field | Expected observation |
| --- | --- |
| Game | **Active** with the detected provider |
| World State | **Active** |
| PixelBeacon Status | **Installed (current)** |
| PixelBeacon Signal | **Signal detected** |
| Game Context | **Gameplay** when no native menu or chat field is open |
| Life State | **Alive** |
| Travel | **Inactive** |

Health, Stamina, Magicka, and Ultimate should become numeric when their current
protocol fields are available. A zero resource is a valid numeric reading;
**Signal unavailable**, **Not detected**, and **Unknown** are not zero.

Open and close an ESO menu to confirm **Game Context** changes away from and back
to **Gameplay**. This is an observation test, not an invitation to automate.

## 5. Configure one feature

Start with only one feature:

- [Configure one weaving slot](../features/weaving.md#configure-one-slot).
- [Prepare and start Fishing](../features/fishing.md#before-starting).
- [Configure Auto Potion](../features/auto-potion.md#configure-auto-potion).

Use the [Status Reference](../reference/status-reference.md) for exact meanings.
If the expected baseline does not appear, continue to
[Troubleshooting](troubleshooting.md) before enabling input generation.
