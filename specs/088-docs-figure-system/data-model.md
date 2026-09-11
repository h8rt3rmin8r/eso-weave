# Data Model: Documentation Figure System

## FigurePlacement

| Field | Type | Rule |
| --- | --- | --- |
| page | repository-relative path | Must identify one canonical Markdown page |
| source | local URL | Must remain inside generated documentation |
| kind | enum | `screenshot`, `diagram`, `illustration`, `brand`, or `decorative` |
| alternative | string | Non-empty for meaningful placements; empty only for the decorative wordmark |
| caption | optional markup | Required for screenshot, illustration, and brand figures |
| intrinsic_width | positive integer | Resolved from attributes or decoded image |
| intrinsic_height | positive integer | Resolved from attributes or decoded image |
| interactive | boolean | True for every meaningful placement and false for the decorative wordmark |

## ExpansionControl

| Field | Type | Rule |
| --- | --- | --- |
| element | button | Exactly one per meaningful placement |
| accessible_name | string | `Expand image: <alternative>` |
| source_image | image | Retains the original local source and presentation classes |
| enhanced | boolean marker | Prevents duplicate wrapping on repeated initialization |
| caption_source | optional figure caption | Nearest caption in the owning figure |

State transitions:

1. `authored` to `enhanced` when the local theme recognizes a meaningful image.
2. `enhanced` to `invoking` when pointer, Enter, or Space activates the button.
3. `invoking` to `enhanced` after the shared dialog closes and focus returns.

## FigureDialog

| Field | Type | Rule |
| --- | --- | --- |
| element | native dialog | Exactly one per enhanced document |
| active_trigger | ExpansionControl | The exact focus-return target while open |
| close_control | button | Initial modal focus and obvious close action |
| accessible_label | string | Selected image alternative |
| visual_image | image | Decorative clone using the selected local source |
| caption_clone | optional figure caption | Cloned supporting markup and `aria-describedby` target |
| modal_state | enum | `closed` or `open` |

State transitions:

1. `closed` to `open`: copy source, label, dimensions, and caption; call `showModal`; focus close control.
2. `open` to `closed`: close button, Escape, or backdrop calls `close`.
3. On `close`: clear transient caption and sizing state, then focus `active_trigger`.

## CaptionTreatment

| Field | Type | Rule |
| --- | --- | --- |
| font_size | CSS length | `0.9em`, computed to 14.4 pixels from 16-pixel body copy |
| line_height | number | At least `1.5` for long-caption scanning |
| color | computed color | At least 4.5:1 against the component background |
| spacing | CSS box values | Visually attached to figure and distinct from body prose |
| emphasis | inherited strong element | Strong lead-ins remain semantically emphasized |

## FigureObservation

| Field | Type | Rule |
| --- | --- | --- |
| case_id | string | One of five representative cases |
| surface | string | `generated-loopback` |
| theme | enum | `navy` or `light` in browser matrix |
| viewport_width | integer | 320 or 1280 CSS pixels |
| input_method | enum | Pointer or keyboard evidence |
| modal | boolean | Native modal selector matches only while open |
| focus_target | string | Close control after open, exact trigger after close |
| contained | boolean | Dialog image remains inside viewport |
| aspect_ratio_error | number | At most 1.5 percent |
| upscaled | boolean | Must be false |
| caption_size | number | At least 14 CSS pixels when a caption exists |
| caption_contrast | number | At least 4.5 |
| modal_caption_size | number | At least 14 CSS pixels when a caption exists |
| modal_caption_contrast | number | At least 4.5 |
| brand_surface | optional enum | Matching `dark` or `light` surface for the brand case |
| duplicate_count | integer | Zero mdBook checkbox or clone implementations after enhancement |

## FigureReceipt

| Field | Type | Rule |
| --- | --- | --- |
| schema_version | integer | `1` |
| sentinel | string | Exact S088 pass sentinel |
| observations | FigureObservation array | Exactly 20 unique case, theme, viewport cells |
| keyboard_journeys | array | Enter and Space open, Tab and Shift+Tab containment, Escape close, and focus return |
| pointer_journeys | array | Trusted hit tests for trigger, close button, and backdrop close paths |
| zoom_observation | object | 200 percent browser scale, wrapping, caption association, and source plus modal containment |
| print_observation | object | Source image and caption visible with affordance, dialog, and legacy chrome hidden |
| no_script_observation | object | Static figures and captions visible with script resources blocked and interaction chrome absent |
| failures | string array | Empty for a passing receipt |
