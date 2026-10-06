# Player Messaging Contract

| Concept | Explanation |
| --- | --- |
| Application logs | ESO Weave diagnostic events for troubleshooting, separate from combat recordings |
| Native combat logs | ESO's own combat-log files; separate provisional import route |
| Encounter recording | ESO Weave Data records one fight or multiple fights until stopped |
| SavedVariables | ESO's saved addon data, written on /reloadui, logout or exit |
| Imported history | Copies accepted into desktop history on this computer |
| Catalog | Game-data definitions used to identify recorded ability/effect IDs; collection is a separate maintainer workflow |
| Current status | /ewencounter status inside ESO |
| Last saved recording state | Desktop snapshot from last file save; may lag current activity |

## Point-of-use actions

Install ESO Weave Data in the selected environment, enable it in ESO's Add-Ons menu and reload. Choose `/ewencounter mode single` or `continuous`, `/ewencounter channel live` or `pts`, then `/ewencounter toggle`. Single stops after one fight; continuous waits between fights until stopped or failed. Use `/ewencounter status` for current status. Stop with toggle when appropriate, save with `/reloadui`, logout or exit, then File > Encounter History > Import Saved Capture. Refresh rereads desktop history, not ESO data.

Unknown-version evidence remains preserved and unusable; do not suggest clear because current guard refuses it. Invalid same-version evidence may be explicitly cleared. Do not require proof of import or send commands from desktop. Explain partial mid-combat starts and interruptions.

## Deletion and privacy

Explain exact effects of clear/delete/uninstall before action. Stop retains recordings. Separate module clear, desktop delete and package removal; retain all current ownership tests. Catalog start replaces prior inactive collection; cancel preserves an incomplete unusable snapshot.

No raw callback values appear in messages, logs, public evidence or default UI. Keep explained technical identity available in secondary details. Figure/text show ordered data, validation/loss/replay, original identity, compatible kind-specific catalog, versioned metrics and separate recommendation gates. Unknown IDs, omissions, incompatible versions and provisional native input remain qualified.
