# Data Model: Deterministic Screenshot Sandbox

## Capture scene

| Field | Meaning | Constraint |
| --- | --- | --- |
| `id` | Stable filename identity | Lowercase ASCII kebab case |
| `title` | Human-readable purpose | Fixed non-personal text |
| `kind` | Fixture constructor selector | One of seven closed variants |
| `order` | Manifest and execution position | Unique 1 through 7 |
| `expected` | Pre-render view contract | Scene-specific stable assertions |

## Capture theme

| ID | Application value | Egui value |
| --- | --- | --- |
| `dark` | `Theme::Dark` | `egui::Theme::Dark` |
| `light` | `Theme::Light` | `egui::Theme::Light` |

## Capture viewport

| ID | Width | Height | Purpose |
| --- | ---: | ---: | --- |
| `narrow` | 760 | 1000 | Documentation-width stacked application view |
| `wide` | 1280 | 900 | Wide two-column dashboard view |

## Capture variant

The Cartesian product of scene, theme, and viewport. Exactly 7 times 2 times 2 equals 28 variants. Ordering is scene, then theme, then viewport.

## Fixture root

The validated output destination contains `.fixture-data/{scene}`. A scene may create only its own synthetic status-classification files below that root. It stores no user configuration, SavedVariables, log, catalog, or real addon data.

## Capture receipt

| Field | Meaning |
| --- | --- |
| `scene` | Stable scene ID |
| `title` | Scene purpose |
| `theme` | `dark` or `light` |
| `viewport` | `narrow` or `wide` |
| `width` | PNG pixel width |
| `height` | PNG pixel height |
| `file` | Relative PNG filename |

## Capture manifest

The manifest contains schema `1`, generator `S083 deterministic screenshot sandbox`, logical pixel scale `1`, and the 28 ordered receipts. It is serialized as UTF-8 without BOM, pretty JSON, LF, and a trailing newline.

## State transitions

1. Parse and validate the optional output authority.
2. Validate static catalog and isolation contracts.
3. Without authority, exit successfully.
4. With authority, build one fixture for each scene.
5. Validate the scene's `AppView` contract.
6. Render each theme and viewport variant.
7. Assert no input action was emitted.
8. Write the exact PNG.
9. After all variants succeed, stage and publish the complete manifest.
