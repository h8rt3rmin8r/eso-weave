# S131 Synthetic Native Fixtures

The three v15 fixtures are manually specified synthetic records derived from
the pinned static producer writers in
`specs/131-native-v15-format-coverage/contracts/native-v15-grammar.md`.
They are not captured game output and contain synthetic identities only.

Normal/inline uses comma separators. Both verbose files contain literal TAB
separators, including inside PLAYER_INFO arrays. They cover flat repeated unit
fields, a self-target marker, interrupted casts, effect synergy ID zero and
remove-track ID, scribed standalone metadata, the actual no-enchantment
`enchantCP` label and standalone metadata coexisting with inline events.
The inline name contains a literal backslash and comma. Test mutations cover
null/default metadata, missing optional fields, malformed labels and limits.
