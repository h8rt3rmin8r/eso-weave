# Quickstart: Encounter Capture Modes

## Single encounter

1. In ESO, run `/ewencounter mode single`.
2. Run `/ewencounter channel live` or `/ewencounter channel pts`.
3. Run `/ewencounter toggle` before or during the encounter.
4. Single mode stops at the next combat exit. A mid-combat start is truthfully partial.
5. Reload UI or exit ESO through a supported flush boundary before desktop import.

## Continuous session

1. In ESO, run `/ewencounter mode continuous` and select the channel.
2. Run `/ewencounter toggle` once.
3. Play any number of encounters within the declared aggregate bounds.
4. Run `/ewencounter toggle` again to stop between fights or during a fight.
5. Inspect `/ewencounter status` for controlled counts, interruption, or failure.

## Desktop import

Open Encounter History and import the selected environment. ESO Weave validates
all terminal records before one atomic write and reports imported and
already-present counts. Any controller status is last-saved disk evidence, not a
live control channel.

## Clearing

After successful import and while capture is stopped or failed, run
`/ewencounter clear confirm` inside ESO. The desktop never deletes or rewrites
the shared SavedVariables file.
