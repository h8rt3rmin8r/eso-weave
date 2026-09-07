# Quickstart: Validate S054

## Focused behavior

```powershell
cargo test --test app_view_model dashboard
cargo test --test app_ui_sizing dashboard
cargo test --test app_ui_sizing system_state
cargo test --test app_ui_sizing resource
cargo test --test app_strings
```

Expected: collapse-aware selection, equal card geometry, symmetric growth,
stable controls, flexible values, group spacing, and exact labels all pass.

## Full merge gate

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
git diff --check
```

## Text and encoding hygiene

```powershell
$forbiddenText = [string][char]0x2014 + '|' + [string][char]0x2013 + '|' + [string][char]0xfffd
rg -n $forbiddenText specs/054-dashboard-layout-cohesion src/app tests README.md CHANGELOG.md docs
```

Expected: no forbidden punctuation or replacement characters, all changed text
is UTF-8 without BOM and uses LF endings, and no generated or secret file appears.

## Maintainer review

Inspect expanded narrow and wide layouts, collapse at wide width, all addon
states, long live values, open-log transitions, and normal/high display scales.
Confirm both cards feel paired, controls form one column, and full values remain
available without clutter.
