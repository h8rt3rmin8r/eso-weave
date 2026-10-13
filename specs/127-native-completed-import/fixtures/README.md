# Native Representative Fixture

Derived from the first two completed combats in the first nonempty run of the
owner-provided local recording. The original remains outside Git.

The transform preserves record order/types/arity, flags, bracket groups and
source/target reference relationships. It replaces every quoted string with
synthetic text (including a comma and Unicode), maps nonzero numeric tokens to
synthetic identifiers/values, replaces decimal values, rebases all elapsed times,
and replaces header metadata/epoch. A synthetic END_LOG closes the selected
subset. No transformation mapping or original payload is published.

The fixture is normal version15 only. Synthetic tests cover unsupported versions,
layouts, escaping, unknown records, malformed groups and incomplete boundaries.
It is parsing evidence, not native/addon parity or installed-platform evidence.
