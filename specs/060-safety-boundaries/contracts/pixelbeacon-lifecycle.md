# Contract: PixelBeacon Lifecycle Ownership

## Action matrix

| Target state | Install | Update | Uninstall | Block-size redeploy | API update | UI guidance |
| --- | --- | --- | --- | --- | --- | --- |
| Absent | Create managed copy | Not offered | Not offered | Skip | Skip | Install |
| Managed current | Guarded refresh allowed | Not normally offered | Remove allowed | Guarded refresh | Guarded update | Current, Uninstall |
| Managed outdated | Guarded refresh allowed | Guarded in-place refresh | Remove allowed | Guarded refresh | Guarded update | Update, Uninstall |
| Unmanaged/unverified | Refuse unchanged | Refuse unchanged | Refuse unchanged | Refuse or explicit safe skip unchanged | Refuse unchanged | No lifecycle action; resolve manually |
| AddOns root unavailable | Refuse | Refuse | Refuse | Refuse | Refuse | Fix environment or path settings |

## Ownership proof

Ownership is proven only when the target is a real directory and its manifest is readable UTF-8 containing the exact managed-marker line. An existing target that fails any part of this proof is unmanaged.

## Mutation boundary

Every public writer rechecks ownership at operation time. A prior UI status, cached result, or earlier successful inspection never authorizes mutation.

Fresh installation creates only an absent target. It must not follow an existing file link or directory link. If the target appears before creation, installation fails rather than adopting it.

## Update behavior

Managed Update writes the shipped manifest and Lua into the proven managed directory. It does not delete the directory first. Unrelated content in a managed directory remains subject to the existing marker ownership contract.

## Unmanaged presentation

Visible state: `Unmanaged (not modified)`.

Guidance: ESO Weave did not modify the existing PixelBeacon target. The operator must move or remove that target manually before using Install.

Install, Update, and Uninstall controls are absent or disabled in this state. This presentation does not replace the writer-level guard.
