# Contract: Native Binding Evidence

## Layout generation

- PixelBeacon addon version: 22
- Negotiated layout protocol: 6
- Protocol version wire code: `0xC0`
- Version 5 frozen payload count: 29
- Version 6 payload count: 40
- New cells: B29 through B39 in `NativeAction` order

## State and control byte

Reserved non-valid values:

| Value | State |
|---:|---|
| `0x00` | Unavailable |
| `0x01` | Unbound |
| `0x02` | Conflicting |
| `0x03` | Unsupported |

Values `0x04` through `0x0F` are invalid. Portable control values begin at `0x10`. The control registry is append-only within this protocol generation.

## RGB encoding

For action index `a` in 0 through 10, state or control byte `c`, and modifier mask `m`:

```text
R = high_nibble(c) * 17
G = low_nibble(c) * 17
check = (a * 5 + c * 3 + 7) mod 16
B = m * 16 + check
```

Non-valid states require `m = 0`. Valid controls require `c >= 0x10` and `m <= 0x0F`.

## Desktop decoding

1. Decode R and G only when each lies within the configured tolerance of exactly one multiple of 17.
2. Reconstruct `c` from the two nibbles.
3. Split B into `m` and `check` without tolerance.
4. Require `check == (a * 5 + c * 3 + 7) mod 16`.
5. For reserved states, require `m == 0`.
6. For valid values, require a known portable control code and modifier mask.
7. Return `Unavailable` on any failure.

Exact blue validation is intentional. A false negative is safe, while tolerance on the packed integrity byte could change either action identity or modifier authority.

## Discovery precedence

1. API or action lookup unavailable: `Unavailable`.
2. No distinct non-empty assignments: `Unbound`.
3. More than one distinct assignment: `Conflicting`.
4. Exactly one assignment with an unsupported primary or modifier set: `Unsupported`.
5. Exactly one supported assignment: `Valid`.

Exact duplicate slots count once. Modifier order does not affect equality.

## Read-only prohibition

PixelBeacon must not contain or declare:

- `CreateDefaultActionBind`
- `BindKeyToAction`
- `UnbindKeyFromAction`
- `UnbindAllKeysFromAction`
- `ResetAllBindsToDefault`
- `ResetKeyboardBindsToDefault`
- `Bindings.xml`
- custom `<Bindings>` or `<Action>` declarations
- binding-related SavedVariables

The static policy test checks the shipped addon and injected prohibited fixtures.

## Compatibility

Negotiated layout versions 1 through 5 and the explicit legacy layout have no binding cells. The desktop must not sample B29 through B39 for them and reports a default all-unavailable `NativeBindingSet`.
