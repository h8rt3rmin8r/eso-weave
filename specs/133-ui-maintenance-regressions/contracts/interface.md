# Interface and maintenance contracts

1. Toggling logs never adds/subtracts pane height from the user's chosen window. Necessary measured growth respects usable bounds; impossible-fit content remains reachable.
2. Saved UI visibility is the last explicit selection, including close immediately after a toggle.
3. Uninstall overlay changes neither dashboard bounds nor minimum size; dismissal has no lifecycle side effect.
4. Healthy/current status is concise; only known installed incompatibility adds an API warning.
5. Nonexistent automatic discovery roots are idle, not failed migrations. Real failure messages identify operation/category/outcome/action without private payloads. One unchanged failure produces one warning episode, with bounded retries and observable recovery.
6. Package ownership, race/link rejection, transactional recovery and all retained bytes remain covered.
