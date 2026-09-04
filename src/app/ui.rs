//! egui rendering for the main window.
//!
//! This layer reads the [`AppModel`] view and raises intents. It was long treated
//! as carrying no correctness-bearing logic and excluded from the tested surface,
//! validated only by a manual checklist. That was wrong, and it cost four releases:
//! the window-sizing defects of issues #4, #5, #8, #12, #13, and #14 all lived
//! here, in the glue between the pure helpers in [`crate::app`] and egui, while
//! every one of those helpers' own tests stayed green.
//!
//! The sizing behavior is therefore covered by `tests/app_ui_sizing.rs`, which
//! drives [`EsoWeaveApp::frame_ui`] through a headless egui harness and asserts
//! rendered geometry: the intrinsic content extent, the minimum pushed to the
//! viewport across a simulated resize gesture, the log pane's never-overlap
//! boundary under drag and resize, and the settings modal's rendered rectangle.
//! What remains manual is appearance rather than geometry (color, wording, hover
//! affordances) plus the real window-manager drag, which only a desk run exercises.
//!
//! Rendering uses a central panel (menu bar, status region, and skills), an
//! optional resizable bottom panel for the live log, and a settings modal, keeping
//! to a small, stable set of egui widgets plus a few brand presentation helpers.

use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use eframe::egui;

use crate::app::log_view::build_log_view;
use crate::app::settings_form::{SettingsForm, UiPrefs};
use crate::app::{
    app_toggle_intent, modal_extent, override_edit_for, strings, widgets, AppModel, SkillEdit,
    StatusLine, UiIntent,
};
use crate::beacon::api_check::ApiCheckOutcome;
use crate::config::state::WindowGeometry;
use crate::config::{LevelName, Theme};
use crate::input::{Action, Key};
use crate::weave::WeaveType;

/// Adds the pointer (hand) cursor to an interactive widget's hover state, so
/// every clickable control signals that it is clickable.
trait Clickable {
    fn clickable(self) -> Self;
}

impl Clickable for egui::Response {
    fn clickable(self) -> Self {
        self.on_hover_cursor(egui::CursorIcon::PointingHand)
    }
}

/// A gold-filled primary action button (dark text on the brand accent), for the
/// main affirmative controls. Secondary and destructive actions stay neutral.
fn primary_button(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    text: &str,
) -> egui::Response {
    let button =
        egui::Button::new(egui::RichText::new(text).color(palette.gold_text)).fill(palette.gold);
    ui.add(button).clickable()
}

const WEAVE_TYPES: [WeaveType; 4] = [
    WeaveType::LightAttack,
    WeaveType::HeavyAttack,
    WeaveType::BashAttack,
    WeaveType::BlockCasting,
];

const KEYS: [Key; 13] = [
    Key::Digit1,
    Key::Digit2,
    Key::Digit3,
    Key::Digit4,
    Key::Digit5,
    Key::E,
    Key::R,
    Key::X,
    Key::Q,
    Key::Space,
    Key::F1,
    Key::F2,
    Key::F3,
];

const LEVELS: [LevelName; 6] = [
    LevelName::Off,
    LevelName::Error,
    LevelName::Warn,
    LevelName::Info,
    LevelName::Debug,
    LevelName::Trace,
];

/// Fixed width (points) for the app's dropdowns, sized to the longest option in
/// use (the weave types) with breathing room, so the resting field never changes
/// width with the selection and the rows below never shift. Shared by the main
/// window and the settings modal.
const COMBO_WIDTH: f32 = 150.0;

/// Fixed width (points) for a skill delay field: enough to show four digits
/// comfortably, right-aligned, in both the editable and greyed read-only states.
const DELAY_FIELD_WIDTH: f32 = 56.0;

/// The boot minimum inner size (points) used before the content extent has been
/// measured. Mirrors `MIN_SIZE` in `main.rs`; once the first frames lay out, the
/// measured content extent (issue #4) raises the real minimum above this floor.
const BOOT_MIN_SIZE: egui::Vec2 = egui::vec2(480.0, 420.0);

/// Extra minimum width (points) enforced while the live log viewer is open, so log
/// lines wrap less than at the base content width (issue #5, FR-006).
const LOG_WIDTH_BONUS: f32 = 100.0;

/// The settings modal frame's inner margin (points) on each edge. Set explicitly
/// rather than inherited, so the modal's outer rendered rectangle can be made to
/// equal its computed extent exactly (issue #14, FR-014).
const MODAL_FRAME_MARGIN: f32 = 8.0;

/// The log text row height (points) used to size the six-line log minimum. Read
/// from the monospace text style (its size is the same in either theme), falling
/// back to a sensible default.
fn log_row_height(ctx: &egui::Context) -> f32 {
    ctx.style_of(egui::Theme::Dark)
        .text_styles
        .get(&egui::TextStyle::Monospace)
        .map(|f| f.size)
        .unwrap_or(14.0)
}

/// The width (points) of the live-log panel's top separator stroke.
///
/// The bottom panel draws this stroke above the height egui is given, so the
/// panel's outer rect is this much taller than the height it was asked for. The
/// never-overlap boundary is about the outer rect, so the stroke has to come out
/// of the bound. Read from the style rather than written as 1.0, so a theme that
/// draws a thicker separator stays correct. Mirrors [`log_row_height`], which
/// reads the dark style for the same reason: the value is identical in either
/// theme.
fn log_panel_separator(ctx: &egui::Context) -> f32 {
    ctx.style_of(egui::Theme::Dark)
        .visuals
        .widgets
        .noninteractive
        .bg_stroke
        .width
}

/// A dropdown preset to a fixed width, so its resting field does not track the
/// selected option (which would reflow the rows below on selection or hover).
fn combo(
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    selected_text: impl Into<egui::WidgetText>,
) -> egui::ComboBox {
    egui::ComboBox::from_id_salt(id_salt)
        .width(COMBO_WIDTH)
        .selected_text(selected_text)
}

/// The eframe application: renders the main window from the [`AppModel`].
pub struct EsoWeaveApp {
    model: AppModel,
    /// Hotkey toggles (suspend, fishing) forwarded from the weave worker, drained
    /// each frame and applied through the same intent path as the GUI buttons.
    toggle_rx: Receiver<Action>,
    /// Startup API-version-check outcomes forwarded from the check thread, drained
    /// each frame and persisted through the model's session save path.
    api_rx: Receiver<ApiCheckOutcome>,
    ui_prefs: UiPrefs,
    applied_prefs: Option<(Theme, bool)>,
    log_height: f32,
    log_panel_open: bool,
    settings_open: bool,
    settings_draft: Option<SettingsForm>,
    settings_applied: Option<SettingsForm>,
    confirm_uninstall: bool,
    toast: Option<widgets::Toast>,
    /// The last window geometry captured from the viewport, used to detect a
    /// change and to keep the normal geometry while maximized. Seeded from the
    /// restored session so an unchanged restored window is not re-saved.
    last_geometry: Option<WindowGeometry>,
    /// The skill slot currently being edited in the Delay column and the digits
    /// typed so far, so the model value does not clobber in-progress input.
    delay_edit: Option<(u8, String)>,
    /// The enforced central-content extent (points): the boot floor until the
    /// measurement is stable, then the measured extent (which may shrink). Drives
    /// the window minimum inner size so it hugs the real content and tracks new or
    /// removed rows, with no permanent dead band (issue #8).
    content_extent: egui::Vec2,
    /// The previous frame's measured content extent, for the two-frame stability
    /// gate that lets the measured extent supersede the boot floor.
    prev_measured: Option<egui::Vec2>,
    /// The previous frame's window inner height (points), used to detect a
    /// window-height change and split it proportionally between the central pane
    /// and the open log pane (issue #8).
    prev_window_h: Option<f32>,
    /// Set when the log pane is opened so the next log frame seeds the pane to the
    /// persisted height (the single source of truth) rather than any egui-remembered
    /// size, so `default_size` cannot fight the restore.
    log_reseed: bool,
    /// The last minimum inner size pushed to the viewport, so the command is sent
    /// only when the target changes rather than every frame.
    last_min_sent: Option<egui::Vec2>,
    /// The widest content-sized block laid out this frame (points), accumulated as
    /// the central content is built and reset at the start of each frame. Blocks
    /// that expand to fill the available width are deliberately excluded, which is
    /// what keeps the intrinsic extent independent of the window (issue #12).
    content_width: f32,
    /// The live-log pane's rendered top edge this frame, and the central content's
    /// rendered bottom edge. Recorded so the never-overlap invariant can be
    /// asserted per frame rather than inferred from the arithmetic (issue #13).
    last_log_top: Option<f32>,
    last_content_bottom: Option<f32>,
    /// The settings modal's rendered size this frame, and the settings body's total
    /// laid-out height at the modal's inner width. Recorded so the modal's rendered
    /// extent can be compared against its computed extent (issue #14).
    last_modal_size: Option<egui::Vec2>,
    last_modal_target: Option<egui::Vec2>,
    last_settings_body_height: Option<f32>,
    /// The height of the settings body actually visible without scrolling.
    last_settings_body_visible: Option<f32>,
}

impl EsoWeaveApp {
    /// Creates the app over the view-model, the hotkey-toggle receiver, and the
    /// API-version-check outcome receiver.
    pub fn new(
        model: AppModel,
        toggle_rx: Receiver<Action>,
        api_rx: Receiver<ApiCheckOutcome>,
        restored_geometry: Option<WindowGeometry>,
    ) -> Self {
        let ui_prefs = model.ui_prefs();
        let log_height = ui_prefs.log_panel_height as f32;
        Self {
            model,
            toggle_rx,
            api_rx,
            ui_prefs,
            applied_prefs: None,
            log_height,
            log_panel_open: false,
            settings_open: false,
            settings_draft: None,
            settings_applied: None,
            confirm_uninstall: false,
            toast: None,
            last_geometry: restored_geometry,
            delay_edit: None,
            content_extent: BOOT_MIN_SIZE,
            prev_measured: None,
            prev_window_h: None,
            log_reseed: false,
            last_min_sent: None,
            content_width: 0.0,
            last_log_top: None,
            last_content_bottom: None,
            last_modal_size: None,
            last_modal_target: None,
            last_settings_body_height: None,
            last_settings_body_visible: None,
        }
    }

    /// The height of the settings body visible without scrolling on the last frame
    /// the modal was open. Compared against the body's total height for FR-017.
    pub fn last_settings_body_visible(&self) -> Option<f32> {
        self.last_settings_body_visible
    }

    /// The size the modal's growth rule called for on the last frame it was open.
    /// The rendered size must equal this (issue #14, contract C5).
    pub fn last_modal_target(&self) -> Option<egui::Vec2> {
        self.last_modal_target
    }

    /// The settings modal's rendered size from the last frame, or `None` when the
    /// modal is closed. Compared against `modal_extent` by the rendered-frame tests
    /// (issue #14, contract C5).
    pub fn last_modal_size(&self) -> Option<egui::Vec2> {
        self.last_modal_size
    }

    /// The settings body's total laid-out height at the modal's inner width, from
    /// the last frame the modal was open (contract C6).
    pub fn last_settings_body_height(&self) -> Option<f32> {
        self.last_settings_body_height
    }

    /// The live-log pane's rendered top edge and the central content's rendered
    /// bottom edge from the last frame, or `None` when the log is closed. The
    /// never-overlap invariant is `log_top >= content_bottom` (issue #13,
    /// contract C4). Exposed for the rendered-frame sizing tests.
    pub fn last_log_top(&self) -> Option<f32> {
        self.last_log_top
    }

    /// See [`Self::last_log_top`].
    pub fn last_content_bottom(&self) -> Option<f32> {
        self.last_content_bottom
    }

    /// Records a content-sized block's width toward this frame's intrinsic extent.
    /// Only blocks that size to their content may be recorded; anything that
    /// expands to fill the available width would reintroduce the window-tracking
    /// measurement that issue #12 reported (FR-007).
    fn note_content_width(&mut self, width: f32) {
        self.content_width = self.content_width.max(width);
    }

    /// Captures the current window geometry from the egui viewport and raises an
    /// intent when it changes. While maximized, the last normal position and size
    /// are kept and only the maximized flag is set, so unmaximizing returns to the
    /// prior geometry. Windows snap (half-screen) is a normal move/resize and is
    /// captured as normal geometry.
    fn capture_geometry(&mut self, ctx: &egui::Context, intents: &mut Vec<UiIntent>) {
        let (outer, inner, maximized) = ctx.input(|i| {
            let vp = i.viewport();
            (vp.outer_rect, vp.inner_rect, vp.maximized.unwrap_or(false))
        });
        let candidate = if maximized {
            match self.last_geometry {
                Some(prev) => WindowGeometry {
                    maximized: true,
                    ..prev
                },
                // No normal geometry known yet; nothing meaningful to record.
                None => return,
            }
        } else {
            let (Some(outer), Some(inner)) = (outer, inner) else {
                return;
            };
            WindowGeometry {
                x: outer.min.x.round() as i32,
                y: outer.min.y.round() as i32,
                width: inner.width().round() as u32,
                height: inner.height().round() as u32,
                maximized: false,
            }
        };
        if self.last_geometry != Some(candidate) {
            self.last_geometry = Some(candidate);
            intents.push(UiIntent::SetWindowGeometry(candidate));
        }
    }

    /// Drains any hotkey toggles received since the last frame and applies each
    /// through the model's intent path, so a hotkey and its button share one
    /// state, one persistence mark, and one display update. Each toggle is mapped
    /// against the live fishing state and applied immediately, so two presses in a
    /// single frame compose correctly.
    fn drain_hotkey_toggles(&mut self) {
        while let Ok(action) = self.toggle_rx.try_recv() {
            if let Some(intent) =
                app_toggle_intent(action, self.model.fishing_on(), self.model.auto_potion_on())
            {
                self.model.apply_intent(intent);
            }
        }
    }

    /// Drains any API-version-check outcomes received since the last frame and
    /// applies each to the model, which persists the updated cache through the
    /// coalesced session save path.
    fn drain_api_checks(&mut self) {
        while let Ok(outcome) = self.api_rx.try_recv() {
            self.model.apply_api_check(outcome);
        }
    }

    fn apply_prefs(&mut self, ctx: &egui::Context) {
        // Only the theme and always-on-top drive a re-apply; the log height is a
        // layout preference that must not churn the theme while the user drags.
        let key = (self.ui_prefs.theme, self.ui_prefs.always_on_top);
        if self.applied_prefs == Some(key) {
            return;
        }
        crate::app::theme::apply(ctx, self.ui_prefs.theme);
        let level = if self.ui_prefs.always_on_top {
            egui::WindowLevel::AlwaysOnTop
        } else {
            egui::WindowLevel::Normal
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(level));
        self.applied_prefs = Some(key);
    }
}

impl eframe::App for EsoWeaveApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // The eframe frame handle is unused, so the whole body lives in an
        // inherent method taking only the `Ui`. That is the seam the headless
        // rendered-frame tests drive through `egui_kittest::Harness::new_ui`
        // (slice 030, contract C7); without it this layer cannot be tested at all.
        self.frame_ui(ui);
    }
}

impl EsoWeaveApp {
    /// The content extent enforced this frame (points). Exposed for the
    /// rendered-frame sizing tests, which assert it is independent of the window
    /// size (slice 030, contract C1).
    pub fn content_extent(&self) -> egui::Vec2 {
        self.content_extent
    }

    /// The last minimum inner size pushed to the viewport, or `None` if none has
    /// been sent. Exposed for the rendered-frame sizing tests (contract C2).
    pub fn last_min_sent(&self) -> Option<egui::Vec2> {
        self.last_min_sent
    }

    /// The current live-log pane height (points). Exposed for the rendered-frame
    /// sizing tests (contract C4).
    pub fn log_height(&self) -> f32 {
        self.log_height
    }

    /// Opens or closes the live-log pane directly, bypassing the menu. Exposed so
    /// the rendered-frame sizing tests can reach the log-open cases.
    pub fn set_log_panel_open(&mut self, open: bool) {
        self.log_panel_open = open;
        self.log_reseed = true;
    }

    /// Shows or hides the uninstall confirmation row, which is the app's one
    /// transient control row. Exposed so the rendered-frame sizing tests can prove
    /// the enforced minimum grows for a new row and shrinks again when it goes
    /// (FR-004).
    pub fn set_confirm_uninstall(&mut self, confirm: bool) {
        self.confirm_uninstall = confirm;
    }

    /// Opens or closes the settings modal directly, bypassing the menu. Exposed so
    /// the rendered-frame sizing tests can reach the modal cases (contract C5).
    pub fn set_settings_open(&mut self, open: bool) {
        if open {
            let form = self.model.settings_form();
            self.settings_applied = Some(form.clone());
            self.settings_draft = Some(form);
        }
        self.settings_open = open;
    }

    /// Renders one frame. Reachable without an `eframe::Frame`, a window, or a
    /// GPU, so `tests/app_ui_sizing.rs` can assert the rendered geometry that the
    /// pure sizing helpers alone never proved (slice 030).
    pub fn frame_ui(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        self.apply_prefs(&ctx);
        // Apply any hotkey toggles before deriving the view, so a press taken this
        // frame is reflected immediately.
        self.drain_hotkey_toggles();
        self.drain_api_checks();
        let extreme_bg = ui.visuals().extreme_bg_color;

        let mut intents: Vec<UiIntent> = Vec::new();
        let mut exit = false;

        // Record the window geometry each frame; the intent is raised only on a
        // change and coalesced into a single settle-write by the model.
        self.capture_geometry(&ctx, &mut intents);

        // The live log lives in a resizable bottom panel (drag handle and resize
        // cursor come for free), added before the central panel. It is clamped so
        // it never overlaps the interactive area or shrinks away, and its height is
        // persisted as a layout preference.
        if self.log_panel_open {
            let window_h = ctx.content_rect().height();
            let row_h = log_row_height(&ctx);
            let content_h = self.content_extent.y;
            // Minimum shows six lines of log; maximum stops before the interactive
            // (Skills) area, computed against the true measured content height so no
            // phantom band is reserved (issue #8). Both bounds are shared helpers.
            let min_h = crate::app::log_min_height(row_h);
            // The non-overlap bound is unconditional: in a window too short for both
            // the content and a six-line log, the controls win and the pane gives up
            // its readable floor rather than covering them (issue #13, FR-010).
            //
            // The panel's frame overhead is subtracted because the bound constrains
            // the *inner* height egui is given, while the boundary is about the
            // space the panel actually occupies. The frame adds an inner margin on
            // the top and bottom edges plus the separator stroke above them, so a
            // pane granted the whole remaining budget as inner height overruns the
            // content by that overhead.
            //
            // This was latent from slice 030 until slice 038: the bound only binds
            // when the content comes within the overhead of filling the window, and
            // until four quickslot blocks added two status rows the content never
            // did. Derived from the margin constant and the style rather than
            // written as a number, so a theme change cannot silently reintroduce it.
            let frame_overhead = 2.0 * crate::app::LOG_FRAME_MARGIN + log_panel_separator(&ctx);
            let no_overlap = (crate::app::log_max_height_no_overlap(window_h, content_h)
                - frame_overhead)
                .max(0.0);
            let min_h = min_h.min(no_overlap);
            let max_h = (window_h - content_h).max(min_h).min(no_overlap);
            // Force the pane height on two kinds of frame: the first frame after
            // opening (seed from the persisted height), and any frame where the
            // window height changed while open (split the change proportionally
            // between the central and log panes). On all other frames egui owns the
            // height and the user can drag it freely.
            let forced = if self.log_reseed {
                self.log_reseed = false;
                Some(crate::app::clamp_log_height(
                    self.log_height,
                    window_h,
                    row_h,
                    content_h,
                ))
            } else if let Some(prev) = self.prev_window_h {
                if (window_h - prev).abs() > 0.5 {
                    Some(crate::app::split_log_height(
                        prev,
                        window_h,
                        self.log_height,
                        content_h,
                        min_h,
                    ))
                } else {
                    None
                }
            } else {
                None
            };
            // Whatever produced the forced height, it can never breach the boundary.
            let forced = forced.map(|h| h.clamp(0.0, no_overlap));
            let (panel_min, panel_max, start) = match forced {
                Some(h) => (h, h, h),
                None => (
                    min_h,
                    max_h,
                    crate::app::clamp_log_height(self.log_height, window_h, row_h, content_h)
                        .clamp(0.0, no_overlap),
                ),
            };
            let resp = egui::Panel::bottom("log_panel")
                .resizable(true)
                .min_size(panel_min)
                .max_size(panel_max)
                .default_size(start)
                .frame(
                    egui::Frame::new()
                        .fill(extreme_bg)
                        .inner_margin(egui::Margin::same(crate::app::LOG_FRAME_MARGIN as i8)),
                )
                .show(ui, |ui| {
                    self.log_view(ui, &mut intents);
                });
            // Re-clamp what egui committed BEFORE it is stored or persisted. In
            // v0.8.0 the committed height was trusted, so a drag past the boundary
            // was both rendered and remembered across restarts (issue #13, FR-011).
            let committed = resp.response.rect.height();
            let bounded = crate::app::clamp_log_height(committed, window_h, row_h, content_h)
                .clamp(0.0, no_overlap);
            if (committed - bounded).abs() > 0.5 {
                // egui let the drag past the boundary; force the bounded height on
                // the next frame rather than letting the overlap persist.
                self.log_reseed = true;
            }
            if (bounded - self.log_height).abs() > 0.5 {
                self.log_height = bounded;
                intents.push(UiIntent::SetLogHeight(bounded.round() as u32));
            }
            self.last_log_top = Some(resp.response.rect.top());
        } else {
            self.last_log_top = None;
            self.last_content_bottom = None;
        }

        // Measured content extent of the central panel this frame, captured inside
        // the closure and used after to drive the window minimum size (issue #8).
        //
        // The measurement is INTRINSIC: the height comes from a scope whose rect
        // grows only to what the content allocated, and the width from the widest
        // content-sized block. Taking the panel's own `min_rect` instead (as v0.8.0
        // did) returns the window size less the frame margins on both axes, so the
        // enforced minimum pinned the window at its own current size and the window
        // could not be shrunk in a single gesture (issue #12, FR-001).
        let mut measured = self.content_extent;
        self.content_width = 0.0;
        egui::CentralPanel::default().show(ui, |ui| {
            let scope = ui.scope(|ui| {
                // Menu bar.
                let menu_bar = egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button(strings::MENU_FILE, |ui| {
                        if ui
                            .button(strings::MENU_SETTINGS)
                            .on_hover_text(strings::MENU_SETTINGS_TOOLTIP)
                            .clickable()
                            .clicked()
                        {
                            let form = self.model.settings_form();
                            self.settings_applied = Some(form.clone());
                            self.settings_draft = Some(form);
                            self.settings_open = true;
                        }
                        if ui.button(strings::MENU_EXIT).clickable().clicked() {
                            exit = true;
                        }
                    })
                    .response
                    .clickable();
                    ui.menu_button(strings::MENU_VIEW, |ui| {
                        if ui
                            .checkbox(&mut self.log_panel_open, strings::MENU_LOG_TOGGLE)
                            .on_hover_text(strings::MENU_LOG_TOGGLE_TOOLTIP)
                            .clickable()
                            .changed()
                        {
                            intents.push(UiIntent::ToggleLogPanel(self.log_panel_open));
                            // Height-neutral toggle (issue #8, FR-009): on open, grow the
                            // window by the log height actually about to show (the
                            // persisted height, clamped) and seed the pane to it; on
                            // close, shrink by the pane's actual current height, not a
                            // fixed minimum, so a user-enlarged log leaves no residual
                            // band.
                            if let Some(inner) = ctx.input(|i| i.viewport().inner_rect) {
                                let cur = inner.size();
                                let row_h = log_row_height(&ctx);
                                let window_h = ctx.content_rect().height();
                                let new_h = if self.log_panel_open {
                                    self.log_reseed = true;
                                    let shown = crate::app::clamp_log_height(
                                        self.log_height,
                                        window_h,
                                        row_h,
                                        self.content_extent.y,
                                    );
                                    cur.y + shown
                                } else {
                                    (cur.y - self.log_height).max(self.content_extent.y)
                                };
                                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(
                                    egui::vec2(cur.x, new_h),
                                ));
                            }
                        }
                    })
                    .response
                    .clickable();
                });
                // The menu bar spans the available width (measured, not assumed: it
                // reports 1168 in a 1200 point window), so like the separator it is
                // full-width chrome and contributes height only. Its buttons need far
                // less width than the grids below, so nothing is clipped by excluding
                // it (FR-007).
                let _ = menu_bar;
                ui.separator();

                self.main_view(ui, &mut intents);
            });
            // The scope's rect grows only to what the content allocated, so its
            // height is the content height rather than the panel height.
            let extent =
                crate::app::intrinsic_extent(self.content_width, scope.response.rect.height());
            measured = egui::vec2(extent.0, extent.1);
            if self.log_panel_open {
                self.last_content_bottom = Some(scope.response.rect.bottom());
            }
        });

        // Update the enforced content extent: the boot floor until the measurement
        // is stable (two consecutive close frames), then the measured extent, which
        // may shrink, so the minimum hugs the real content with no permanent dead
        // band (issue #8). Then push it as the window minimum inner size. While the
        // log viewer is open the minimum is widened and grown by the open log
        // reserve (six lines plus one row of drag room) so the controls and a
        // resizable log always fit.
        let measured_tuple = (measured.x, measured.y);
        let stable = crate::app::measurement_stable(
            self.prev_measured.map(|v| (v.x, v.y)),
            measured_tuple,
            0.5,
        );
        let extent = crate::app::content_min_size(
            measured_tuple,
            (BOOT_MIN_SIZE.x, BOOT_MIN_SIZE.y),
            stable,
        );
        self.content_extent = egui::vec2(extent.0, extent.1);
        self.prev_measured = Some(measured);
        let target_min = if self.log_panel_open {
            egui::vec2(
                self.content_extent.x + LOG_WIDTH_BONUS,
                self.content_extent.y + crate::app::open_log_reserve(log_row_height(&ctx)),
            )
        } else {
            self.content_extent
        };
        // Cap at the display work area so a small display at a high scale factor
        // cannot produce a window that is unpositionable or unresizable (FR-008).
        // Making the central content scroll instead was rejected: a filling scroll
        // area would reintroduce the window-tracking measurement this slice removes.
        let target_min = ctx
            .input(|i| i.viewport().monitor_size)
            .map(|work| {
                let capped =
                    crate::app::cap_to_work_area((target_min.x, target_min.y), (work.x, work.y));
                egui::vec2(capped.0, capped.1)
            })
            .unwrap_or(target_min);
        // Sent only when the value itself changes, never in response to a window
        // geometry change (FR-005). This is what makes a mid-gesture relaxation of
        // the minimum impossible, which is the mechanism the ratchet depended on.
        if self.last_min_sent != Some(target_min) {
            ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(target_min));
            self.last_min_sent = Some(target_min);
        }
        // The window grows to fit content that no longer fits, but never shrinks
        // back: taking away a size the user chose would be its own defect (FR-009).
        if let Some(inner) = ctx.input(|i| i.viewport().inner_rect) {
            let cur = inner.size();
            if let Some((w, h)) =
                crate::app::window_growth_request((target_min.x, target_min.y), (cur.x, cur.y))
            {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(w, h)));
            }
        }
        // Record this frame's window height for the next frame's proportional split.
        self.prev_window_h = Some(ctx.content_rect().height());

        if self.settings_open {
            self.settings_modal(&ctx, &mut intents);
        }

        for intent in intents {
            self.model.apply_intent(intent);
        }

        // Coalesced auto-save: flush any settled changes. The confirmation toast
        // fires only for a meaningful settings change, not for a pure layout write
        // (window move/resize or log-pane resize), which persists silently
        // (issue #6).
        let now = Instant::now();
        if self.model.maybe_flush(now).notify {
            self.toast = Some(widgets::Toast::new(strings::SAVED_TOAST, now));
        }
        let mut clear_toast = false;
        if let Some(toast) = &self.toast {
            if toast.expired(now) {
                clear_toast = true;
            } else {
                let palette = crate::app::theme::palette(self.ui_prefs.theme);
                toast.show(&ctx, &palette, now);
                ctx.request_repaint();
            }
        }
        if clear_toast {
            self.toast = None;
        }

        // On the Exit menu item or a window-manager close request, force the
        // final geometry to disk before the window goes away, so a resize made in
        // the last moments is not lost to the settle-delayed scheduler.
        let close_requested = ctx.input(|i| i.viewport().close_requested());
        if exit || close_requested {
            self.model.flush_session_now();
        }
        if exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        ctx.request_repaint_after(Duration::from_millis(250));
    }

    fn main_view(&mut self, ui: &mut egui::Ui, intents: &mut Vec<UiIntent>) {
        let view = self.model.view();

        let palette = crate::app::theme::palette(self.ui_prefs.theme);

        if self.confirm_uninstall {
            let row = ui.horizontal(|ui| {
                ui.label("Remove the PixelBeacon addon?");
                if ui
                    .button("Uninstall")
                    .on_hover_text(strings::BEACON_UNINSTALL_TOOLTIP)
                    .clickable()
                    .clicked()
                {
                    intents.push(UiIntent::UninstallBeacon);
                    self.confirm_uninstall = false;
                }
                if ui
                    .button("Cancel")
                    .on_hover_text("Keep the addon installed.")
                    .clickable()
                    .clicked()
                {
                    self.confirm_uninstall = false;
                }
            });
            self.note_content_width(row.response.rect.width());
            ui.separator();
        }

        // Status region: title, colorized state, and control, aligned in a grid
        // that spans the same width as the Skills grid below.
        let status = egui::Grid::new("status")
            .num_columns(3)
            .spacing([12.0, 8.0])
            .min_col_width(110.0)
            .show(ui, |ui| {
                status_cells(ui, &palette, &view.status_line);
                let mut running = !view.suspended;
                if widgets::toggle_switch(ui, &mut running, &palette)
                    .on_hover_text(strings::SUSPEND_TOOLTIP)
                    .clickable()
                    .changed()
                {
                    intents.push(UiIntent::ToggleSuspend);
                }
                ui.end_row();

                status_cells(ui, &palette, &view.fishing_line);
                let mut fishing_on = view.fishing_active;
                if widgets::toggle_switch(ui, &mut fishing_on, &palette)
                    .on_hover_text(strings::FISHING_TOGGLE_TOOLTIP)
                    .clickable()
                    .changed()
                {
                    intents.push(UiIntent::SetFishing(fishing_on));
                }
                ui.end_row();

                status_cells(ui, &palette, &view.beacon_line);
                ui.horizontal(|ui| {
                    if primary_button(ui, &palette, "Install")
                        .on_hover_text(strings::BEACON_INSTALL_TOOLTIP)
                        .clicked()
                    {
                        intents.push(UiIntent::InstallBeacon);
                    }
                    if ui
                        .add_enabled(view.uninstall_enabled, egui::Button::new("Update"))
                        .on_hover_text(strings::BEACON_UPDATE_TOOLTIP)
                        .clickable()
                        .clicked()
                    {
                        intents.push(UiIntent::UpdateBeacon);
                    }
                    if ui
                        .add_enabled(view.uninstall_enabled, egui::Button::new("Uninstall"))
                        .on_hover_text(strings::BEACON_UNINSTALL_TOOLTIP)
                        .clickable()
                        .clicked()
                    {
                        self.confirm_uninstall = true;
                    }
                });
                ui.end_row();

                status_cells(ui, &palette, &view.installation_line);
                ui.end_row();

                status_cells(ui, &palette, &view.runtime_line);
                ui.end_row();

                // Weapon bar (from the updated Pixel Beacon addon), rendered as a
                // grid row so its title and state align with the rows above.
                widgets::label_strong(ui, &palette, strings::WEAPON_BAR_TITLE)
                    .on_hover_text(strings::WEAPON_BAR_TOOLTIP);
                let wb = &view.weapon_bar;
                let color = crate::app::theme::status_color(&palette, wb.role);
                let text = if wb.detected {
                    format!("{} (front {}, back {})", wb.active_bar, wb.front, wb.back)
                } else {
                    wb.active_bar.to_string()
                };
                ui.label(egui::RichText::new(text).color(color))
                    .on_hover_text(strings::WEAPON_BAR_TOOLTIP);
                ui.end_row();

                // Combat state (from the updated Pixel Beacon addon), rendered as
                // a grid row so it aligns with the weapon-bar row above it.
                widgets::label_strong(ui, &palette, strings::COMBAT_TITLE)
                    .on_hover_text(strings::COMBAT_TOOLTIP);
                let cb = &view.combat;
                let combat_color = crate::app::theme::status_color(&palette, cb.role);
                ui.label(egui::RichText::new(cb.state).color(combat_color))
                    .on_hover_text(strings::COMBAT_TOOLTIP);
                ui.end_row();

                // Movement state, beside combat: the same kind of decoded
                // player-state observable, read the same way.
                widgets::label_strong(ui, &palette, strings::MOVEMENT_TITLE)
                    .on_hover_text(strings::MOVEMENT_TOOLTIP);
                let mo = &view.movement;
                let movement_color = crate::app::theme::status_color(&palette, mo.role);
                ui.label(egui::RichText::new(mo.state).color(movement_color))
                    .on_hover_text(strings::MOVEMENT_TOOLTIP);
                ui.end_row();

                // Game menu gate: when this is active the application is
                // deliberately not intercepting, so the operator can see why
                // their weaves are not firing.
                let mv = &view.menu;
                game_context_cells(ui, &palette, mv);
                ui.end_row();

                // Resource pools, one grid row each so they align with the rows
                // above. Display only; nothing in the application reads them.
                for (title, view) in [
                    (strings::HEALTH_TITLE, &view.resources.health),
                    (strings::STAMINA_TITLE, &view.resources.stamina),
                    (strings::MAGICKA_TITLE, &view.resources.magicka),
                ] {
                    widgets::label_strong(ui, &palette, title)
                        .on_hover_text(strings::RESOURCE_TOOLTIP);
                    let color = crate::app::theme::status_color(&palette, view.role);
                    ui.label(egui::RichText::new(view.text.clone()).color(color))
                        .on_hover_text(strings::RESOURCE_TOOLTIP);
                    ui.end_row();
                }

                // Auto-potion: the one row in this region that describes
                // something the application *does* rather than something it
                // reads, which is why it is worth showing at all.
                widgets::label_strong(ui, &palette, strings::AUTO_POTION_TITLE)
                    .on_hover_text(strings::AUTO_POTION_TOOLTIP);
                let (potion_text, potion_role) = if view.auto_potion_active
                    && !crate::potion::EXPLICIT_QUICKSLOT_AUTOMATION_ENABLED
                {
                    (
                        strings::AUTO_POTION_PENDING,
                        crate::app::StatusRole::Warning,
                    )
                } else if view.auto_potion_active && !view.game_active {
                    ("On (game not active)", crate::app::StatusRole::Warning)
                } else if view.auto_potion_active {
                    (strings::AUTO_POTION_ON, crate::app::StatusRole::Active)
                } else {
                    (strings::AUTO_POTION_OFF, crate::app::StatusRole::Muted)
                };
                let potion_color = crate::app::theme::status_color(&palette, potion_role);
                ui.label(egui::RichText::new(potion_text).color(potion_color))
                    .on_hover_text(strings::AUTO_POTION_TOOLTIP);
                // The control sits in the third column, like suspend and fishing,
                // so the hotkey and the switch reach the same state by the same
                // intent path.
                let mut potion_on = view.auto_potion_active;
                if widgets::toggle_switch(ui, &mut potion_on, &palette)
                    .on_hover_text(strings::AUTO_POTION_TOGGLE_TOOLTIP)
                    .clickable()
                    .changed()
                {
                    intents.push(UiIntent::SetAutoPotion(potion_on));
                }
                ui.end_row();

                // Classification is independent from availability and cooldown.
                // That keeps an apparently ready cooldown from claiming that an
                // empty or unreadable slot contains a potion.
                for (title, tooltip, field) in [
                    (
                        strings::QUICKSLOT_TITLE,
                        strings::QUICKSLOT_TOOLTIP,
                        &view.quickslot.state,
                    ),
                    (
                        strings::QUICKSLOT_AVAILABILITY_TITLE,
                        strings::QUICKSLOT_AVAILABILITY_TOOLTIP,
                        &view.quickslot.availability,
                    ),
                    (
                        strings::QUICKSLOT_COOLDOWN_TITLE,
                        strings::QUICKSLOT_COOLDOWN_TOOLTIP,
                        &view.quickslot.cooldown,
                    ),
                ] {
                    widgets::label_strong(ui, &palette, title).on_hover_text(tooltip);
                    let color = crate::app::theme::status_color(&palette, field.role);
                    ui.label(egui::RichText::new(field.text.clone()).color(color))
                        .on_hover_text(tooltip);
                    ui.end_row();
                }
            });

        self.note_content_width(status.response.rect.width());
        ui.separator();
        let skills_title =
            widgets::heading(ui, strings::SKILLS_TITLE).on_hover_text(strings::SKILLS_TOOLTIP);
        self.note_content_width(skills_title.rect.width());
        // A single grid so the label, enabled toggle, weave selector, override
        // toggle, and delay align in labeled columns across every row. When a row
        // has no override, the Delay cell shows the inherited default (muted) so a
        // row never displays a meaningless zero.
        let skills = egui::Grid::new("skills")
            .num_columns(5)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                for (header, tip) in strings::SKILL_COLUMNS {
                    widgets::label_strong(ui, &palette, header).on_hover_text(tip);
                }
                ui.end_row();

                for row in &view.skills {
                    ui.label(&row.label);

                    let mut active = row.active;
                    if widgets::toggle_switch(ui, &mut active, &palette)
                        .on_hover_text(strings::SKILL_COLUMNS[1].1)
                        .clickable()
                        .changed()
                    {
                        intents.push(UiIntent::EditSkill(row.index, SkillEdit::Active(active)));
                    }

                    let mut weave_type = row.weave_type;
                    combo(("weave", row.index), weave_type_name(weave_type))
                        .show_ui(ui, |ui| {
                            for candidate in WEAVE_TYPES {
                                ui.selectable_value(
                                    &mut weave_type,
                                    candidate,
                                    weave_type_name(candidate),
                                );
                            }
                        })
                        .response
                        .on_hover_text(strings::SKILL_COLUMNS[2].1)
                        .clickable();
                    if weave_type != row.weave_type {
                        intents.push(UiIntent::EditSkill(
                            row.index,
                            SkillEdit::WeaveType(weave_type),
                        ));
                    }

                    let mut has_override = row.is_override;
                    if widgets::toggle_switch(ui, &mut has_override, &palette)
                        .on_hover_text(strings::SKILL_COLUMNS[3].1)
                        .clickable()
                        .changed()
                    {
                        let value = if has_override {
                            Some(row.effective_delay)
                        } else {
                            None
                        };
                        intents.push(UiIntent::EditSkill(
                            row.index,
                            override_edit_for(row.weave_type, value),
                        ));
                    }

                    // Delay (ms): a right-aligned, four-digit field in both states so
                    // the column keeps one width when Override is toggled. Editable
                    // when overriding (buffered so typing is not clobbered by the
                    // model value), greyed and read-only otherwise.
                    let tip = strings::SKILL_COLUMNS[4].1;
                    if row.is_override {
                        let mut buf = match &self.delay_edit {
                            Some((idx, s)) if *idx == row.index => s.clone(),
                            _ => row.effective_delay.to_string(),
                        };
                        let resp = ui
                            .add(
                                egui::TextEdit::singleline(&mut buf)
                                    .desired_width(DELAY_FIELD_WIDTH)
                                    .horizontal_align(egui::Align::RIGHT),
                            )
                            .on_hover_text(tip);
                        if resp.changed() {
                            let filtered: String =
                                buf.chars().filter(|c| c.is_ascii_digit()).take(4).collect();
                            let value: u32 = filtered.parse().unwrap_or(0);
                            self.delay_edit = Some((row.index, filtered));
                            intents.push(UiIntent::EditSkill(
                                row.index,
                                override_edit_for(row.weave_type, Some(value)),
                            ));
                        }
                        if resp.lost_focus() {
                            self.delay_edit = None;
                        }
                    } else {
                        let mut buf = row.effective_delay.to_string();
                        ui.add_enabled(
                            false,
                            egui::TextEdit::singleline(&mut buf)
                                .desired_width(DELAY_FIELD_WIDTH)
                                .horizontal_align(egui::Align::RIGHT),
                        )
                        .on_hover_text(tip);
                    }

                    // Cooldown: read-only, from the beacon. The Synergy row always
                    // shows the muted placeholder, because the game exposes no
                    // cooldown for a contextual prompt and it has no block.
                    let cd = &row.cooldown;
                    let cd_color = crate::app::theme::status_color(&palette, cd.role);
                    ui.label(egui::RichText::new(&cd.text).color(cd_color))
                        .on_hover_text(strings::SKILL_COLUMNS[5].1);
                    ui.end_row();
                }
            });
        self.note_content_width(skills.response.rect.width());
    }

    fn log_view(&mut self, ui: &mut egui::Ui, intents: &mut Vec<UiIntent>) {
        let filter = self.model.view().log_filter;
        let palette = crate::app::theme::palette(self.ui_prefs.theme);
        ui.horizontal(|ui| {
            widgets::label_strong(ui, &palette, strings::LOG_TITLE)
                .on_hover_text(strings::LOG_TOOLTIP);
            let mut selected = filter;
            combo("log_filter", level_name(selected))
                .show_ui(ui, |ui| {
                    for level in LEVELS {
                        ui.selectable_value(&mut selected, level, level_name(level));
                    }
                })
                .response
                .on_hover_text(strings::LOG_FILTER_TOOLTIP)
                .clickable();
            if selected != filter {
                intents.push(UiIntent::SetLogFilter(selected));
            }
        });
        let events = self.model.log_handle().recent(1000);
        let rows = build_log_view(&events, filter);
        // A terminal-like panel: monospace rows over the darker panel fill set by
        // the enclosing bottom panel, keeping the per-level colors.
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for row in rows {
                    let color = egui::Color32::from_rgb(row.color.r, row.color.g, row.color.b);
                    ui.label(egui::RichText::new(row.text).monospace().color(color));
                }
            });
    }

    /// Renders settings as a full-frame modal over a dimmed backdrop. Changes are
    /// applied and persisted automatically (coalesced), with no explicit save.
    /// The modal closes on an outside click, on Escape, or on the close control.
    fn settings_modal(&mut self, ctx: &egui::Context, intents: &mut Vec<UiIntent>) {
        let palette = crate::app::theme::palette(self.ui_prefs.theme);
        let mut draft = match self.settings_draft.take() {
            Some(draft) => draft,
            None => {
                self.settings_open = false;
                return;
            }
        };
        let screen = ctx.content_rect();
        let mut close = false;

        // Size the modal from the current window each frame: both axes grow with
        // the window but occupy a progressively smaller fraction, bounded to a
        // maximum (so it looks right from the minimum window up to a QHD ultrawide
        // display) and never exceeding the window.
        let modal_w = modal_extent(screen.width(), 460.0, 1040.0, 0.92);
        // The maximum height rose from 880 to 1120 in slice 039. The settings body
        // grew by an auto-potion group of five settings plus a keybinding row for
        // the new toggle, taking it past the FR-017 bound that at least half the
        // body is visible at the modal maximum. Slice 030 recorded that margin as
        // thin and that any added settings row would need the maximum raised; this
        // is that. It is still capped at 92 percent of the window, so a small
        // display is unaffected.
        let modal_h = modal_extent(screen.height(), 400.0, 1120.0, 0.92);
        // The room above the body (heading, separator, close row) is measured from
        // the laid-out chrome rather than reserved as a constant: the old fixed 52
        // points understated the real chrome by about half, so the modal overshot
        // its computed height at both ends of the range (issue #14).

        // The modal's own frame margin sits outside the inner Ui, so the inner size
        // is the target less that margin on each edge. Setting it explicitly is what
        // makes the OUTER rendered rectangle equal the computed extent; using the
        // default frame left a constant overshoot on both axes (issue #14).
        let modal_frame = egui::Frame::popup(&ctx.style_of(match self.ui_prefs.theme {
            Theme::Light => egui::Theme::Light,
            Theme::Dark => egui::Theme::Dark,
        }))
        .inner_margin(egui::Margin::same(MODAL_FRAME_MARGIN as i8));
        // The margin and the frame's stroke both sit outside the inner Ui, so both
        // come off the target. Reading the stroke from the frame keeps this exact if
        // the style changes, rather than encoding today's one-point border.
        let edge = MODAL_FRAME_MARGIN + modal_frame.stroke.width;
        let inner_w = (modal_w - 2.0 * edge).max(0.0);
        let inner_h = (modal_h - 2.0 * edge).max(0.0);
        let modal = egui::Modal::new(egui::Id::new("eso_weave_settings"))
            .frame(modal_frame)
            .show(ctx, |ui| {
                ui.set_width(inner_w);
                // The height must be set as explicitly as the width. Without it the
                // modal inherited whatever vertical space its centered area happened to
                // leave, which is about half the window, so the body scrolled almost
                // immediately no matter how large the window grew (issue #14, FR-014).
                ui.set_height(inner_h);
                let modal_top = ui.min_rect().top();
                ui.horizontal(|ui| {
                    widgets::heading(ui, strings::MENU_SETTINGS);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clickable().clicked() {
                            close = true;
                        }
                    });
                });
                ui.separator();
                // Do not shrink to content width: fill the modal's inner width so the
                // body spans the modal and the vertical scrollbar sits at the far right
                // edge (matching the log-panel scroll area).
                let chrome = ui.cursor().top() - modal_top;
                let body_max_h = (inner_h - chrome).max(80.0);
                let body = egui::ScrollArea::vertical()
                    .max_height(body_max_h)
                    // Pin the scrolled viewport to the reserved height as well, so the
                    // body fills the modal instead of collapsing to whatever space the
                    // area inherited.
                    .min_scrolled_height(body_max_h)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        settings_body(ui, &palette, &mut draft);
                    });
                (body.content_size.y, body_max_h)
            });
        self.last_modal_size = Some(modal.response.rect.size());
        self.last_modal_target = Some(egui::vec2(modal_w, modal_h));
        self.last_settings_body_height = Some(modal.inner.0);
        self.last_settings_body_visible = Some(modal.inner.1);
        if modal.should_close() {
            close = true;
        }

        // Auto-apply: any change to the draft is applied live and persisted
        // (coalesced through the save scheduler), with no explicit save action.
        if self.settings_applied.as_ref() != Some(&draft) {
            intents.push(UiIntent::ApplySettings(Box::new(draft.clone())));
            self.ui_prefs = draft.ui;
            self.settings_applied = Some(draft.clone());
        }

        if close {
            self.settings_open = false;
            self.settings_applied = None;
        } else {
            self.settings_draft = Some(draft);
        }
    }
}

/// Renders the clustered settings body into the modal. Each option carries a
/// human-readable label (no underscore) and a short inline help line.
fn settings_body(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    draft: &mut SettingsForm,
) {
    widgets::heading(ui, strings::CLUSTER_APPEARANCE);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        setting(ui, palette, &strings::SET_THEME, |ui| {
            combo("set_theme", theme_name(draft.ui.theme))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut draft.ui.theme, Theme::Dark, "Dark");
                    ui.selectable_value(&mut draft.ui.theme, Theme::Light, "Light");
                })
                .response
                .clickable();
        });
        setting(ui, palette, &strings::SET_ALWAYS_ON_TOP, |ui| {
            widgets::toggle_switch(ui, &mut draft.ui.always_on_top, palette);
        });
    });
    ui.add_space(6.0);

    widgets::heading(ui, strings::CLUSTER_COMBAT_TIMING);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        setting(ui, palette, &strings::SET_GLOBAL_COOLDOWN, |ui| {
            ui.add(egui::DragValue::new(
                &mut draft.weave.timing.global_cooldown,
            ));
        });
        setting(ui, palette, &strings::SET_D_WEAVE, |ui| {
            ui.add(egui::DragValue::new(&mut draft.weave.timing.d_weave));
        });
        setting(ui, palette, &strings::SET_D_HEAVY, |ui| {
            ui.add(egui::DragValue::new(&mut draft.weave.timing.d_heavy));
        });
        setting(ui, palette, &strings::SET_D_BASH, |ui| {
            ui.add(egui::DragValue::new(&mut draft.weave.timing.d_bash));
        });
        setting(ui, palette, &strings::SET_AUTO_TIMING, |ui| {
            widgets::toggle_switch(ui, &mut draft.weave.auto_timing, palette);
        });
        if !draft.weave.auto_timing {
            ui.add_space(4.0);
            widgets::muted_help(
                ui,
                palette,
                "Back bar delays (used when auto timing is off)",
            );
            setting(ui, palette, &strings::SET_D_WEAVE, |ui| {
                ui.add(egui::DragValue::new(&mut draft.weave.timing_back.d_weave));
            });
            setting(ui, palette, &strings::SET_D_HEAVY, |ui| {
                ui.add(egui::DragValue::new(&mut draft.weave.timing_back.d_heavy));
            });
            setting(ui, palette, &strings::SET_D_BASH, |ui| {
                ui.add(egui::DragValue::new(&mut draft.weave.timing_back.d_bash));
            });
        }
        setting(ui, palette, &strings::SET_LATENCY_ENABLED, |ui| {
            widgets::toggle_switch(ui, &mut draft.latency.enabled, palette);
        });
        setting(ui, palette, &strings::SET_LATENCY_K, |ui| {
            ui.add(egui::DragValue::new(&mut draft.latency.k).speed(0.05));
        });
    });
    ui.add_space(6.0);

    widgets::heading(ui, strings::CLUSTER_FISHING);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        setting(ui, palette, &strings::SET_ARM_TIMEOUT, |ui| {
            ui.add(egui::DragValue::new(&mut draft.fishing.arm_timeout_ms));
        });
        setting(ui, palette, &strings::SET_REEL_DELAY, |ui| {
            ui.add(egui::DragValue::new(&mut draft.fishing.reel_delay_ms));
        });
        setting(ui, palette, &strings::SET_RECAST_DELAY, |ui| {
            ui.add(egui::DragValue::new(&mut draft.fishing.recast_delay_ms));
        });
    });
    ui.add_space(6.0);

    widgets::heading(ui, strings::CLUSTER_BEACON);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        setting(ui, palette, &strings::SET_BEACON_PATH, |ui| {
            let mut text = draft
                .beacon
                .path_override
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_default();
            if ui.text_edit_singleline(&mut text).changed() {
                let trimmed = text.trim();
                draft.beacon.path_override = if trimmed.is_empty() {
                    None
                } else {
                    Some(std::path::PathBuf::from(trimmed))
                };
            }
        });
        setting(ui, palette, &strings::SET_BEACON_ENV, |ui| {
            combo("set_env", env_name(draft.beacon.environment))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut draft.beacon.environment,
                        crate::beacon::Environment::Live,
                        "Live",
                    );
                    ui.selectable_value(
                        &mut draft.beacon.environment,
                        crate::beacon::Environment::Pts,
                        "PTS",
                    );
                })
                .response
                .clickable();
        });
        setting(ui, palette, &strings::SET_BLOCK_PX, |ui| {
            combo("set_block_px", draft.reader.block_px.to_string())
                .show_ui(ui, |ui| {
                    for size in [2u32, 4, 8, 16, 32] {
                        ui.selectable_value(&mut draft.reader.block_px, size, size.to_string());
                    }
                })
                .response
                .clickable();
        });
        // The overlay's footprint at the size being edited, so the operator can see
        // what the setting does to the thing on their screen without measuring it.
        // Derived from the drafted value rather than the one in effect, because the
        // question being asked here is what it will become.
        widgets::muted_help(
            ui,
            palette,
            &crate::app::grid_footprint_caption(draft.reader.block_px),
        );
        setting(ui, palette, &strings::SET_TOLERANCE, |ui| {
            ui.add(egui::DragValue::new(&mut draft.reader.tolerance));
        });
        setting(ui, palette, &strings::SET_INTERVAL_FISHING, |ui| {
            ui.add(egui::DragValue::new(&mut draft.reader.interval_fishing_ms));
        });
        setting(ui, palette, &strings::SET_INTERVAL_IDLE, |ui| {
            ui.add(egui::DragValue::new(&mut draft.reader.interval_idle_ms));
        });
    });
    ui.add_space(6.0);

    widgets::heading(ui, strings::CLUSTER_AUTO_POTION);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        // Each resource is an independent enable plus threshold, which is what
        // makes the OR rule visible in the interface rather than implied: a
        // shared number would suggest the three are compared together.
        for (s, watch) in [
            (&strings::SET_POTION_HEALTH, &mut draft.potion.health),
            (&strings::SET_POTION_MAGICKA, &mut draft.potion.magicka),
            (&strings::SET_POTION_STAMINA, &mut draft.potion.stamina),
        ] {
            setting(ui, palette, s, |ui| {
                ui.checkbox(&mut watch.enabled, "").clickable();
                ui.add(egui::DragValue::new(&mut watch.threshold).range(0..=100));
            });
        }
        setting(ui, palette, &strings::SET_POTION_KEY, |ui| {
            combo("set_potion_key", draft.potion.quickslot_key.display_name())
                .show_ui(ui, |ui| {
                    for key in KEYS {
                        ui.selectable_value(
                            &mut draft.potion.quickslot_key,
                            key,
                            key.display_name(),
                        );
                    }
                })
                .response
                .clickable();
        });
        setting(ui, palette, &strings::SET_POTION_RETRY, |ui| {
            ui.add(egui::DragValue::new(&mut draft.potion.retry_interval_ms));
        });
    });
    ui.add_space(6.0);

    widgets::heading(ui, strings::CLUSTER_LOGGING);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        setting(ui, palette, &strings::SET_LOG_LEVEL, |ui| {
            combo("set_log_level", level_name(draft.logging.level))
                .show_ui(ui, |ui| {
                    for level in LEVELS {
                        ui.selectable_value(&mut draft.logging.level, level, level_name(level));
                    }
                })
                .response
                .clickable();
        });
        setting(ui, palette, &strings::SET_FILE_LOGGING, |ui| {
            widgets::toggle_switch(ui, &mut draft.logging.file_enabled, palette);
        });
    });
    ui.add_space(6.0);

    widgets::heading(ui, strings::CLUSTER_KEYBINDINGS);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        for action in Action::ALL {
            let current = draft.bindings.key_for(action);
            let mut selected = current;
            ui.horizontal(|ui| {
                ui.label(action_label(action));
                combo(("bind", action.as_str()), selected.display_name())
                    .show_ui(ui, |ui| {
                        for key in KEYS {
                            ui.selectable_value(&mut selected, key, key.display_name());
                        }
                    })
                    .response
                    .clickable();
            });
            if selected != current {
                let _ = draft.bindings.rebind(action, selected);
            }
        }
    });
}

/// Renders one settings option: a label with a tooltip, the control, and a small
/// muted inline help line beneath it.
fn setting(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    s: &strings::Setting,
    add: impl FnOnce(&mut egui::Ui),
) {
    ui.horizontal(|ui| {
        ui.label(s.label).on_hover_text(s.help);
        add(ui);
    });
    widgets::muted_help(ui, palette, s.help);
}

/// A human-readable, underscore-free label for a bindable action.
fn action_label(action: Action) -> &'static str {
    match action {
        Action::Skill1 => "Skill 1",
        Action::Skill2 => "Skill 2",
        Action::Skill3 => "Skill 3",
        Action::Skill4 => "Skill 4",
        Action::Skill5 => "Skill 5",
        Action::Ultimate => "Ultimate",
        Action::Synergy => "Synergy",
        Action::ToggleSuspend => "Toggle suspend",
        Action::ToggleFishing => "Toggle fishing",
        Action::ToggleAutoPotion => "Toggle auto-potion",
    }
}

/// The display name for a game environment.
fn env_name(env: crate::beacon::Environment) -> &'static str {
    match env {
        crate::beacon::Environment::Live => "Live",
        crate::beacon::Environment::Pts => "PTS",
    }
}

/// Renders the first two cells of a status grid row: the section title, then the
/// colorized, normalized state field. The caller adds the third (control) cell.
fn status_cells(ui: &mut egui::Ui, palette: &crate::app::theme::Palette, line: &StatusLine) {
    widgets::label_strong(ui, palette, line.title).on_hover_text(line.tooltip);
    let color = crate::app::theme::status_color(palette, line.role);
    ui.label(egui::RichText::new(&line.state_text).color(color))
        .on_hover_text(line.tooltip);
}

/// Renders focusable Game Context cells. Hover uses the ordinary delayed
/// tooltip, while keyboard focus shows the identical text immediately.
fn game_context_cells(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    view: &crate::app::MenuView,
) {
    let title = ui
        .add(
            egui::Label::new(
                egui::RichText::new(strings::MENU_TITLE)
                    .strong()
                    .color(palette.text),
            )
            .sense(egui::Sense::focusable_noninteractive()),
        )
        .on_hover_text(strings::MENU_TOOLTIP);
    if title.has_focus() {
        title.show_tooltip_text(strings::MENU_TOOLTIP);
    }
    let color = crate::app::theme::status_color(palette, view.role);
    let state = ui
        .add(
            egui::Label::new(egui::RichText::new(view.state).color(color))
                .sense(egui::Sense::focusable_noninteractive()),
        )
        .on_hover_text(strings::MENU_TOOLTIP);
    if state.has_focus() {
        state.show_tooltip_text(strings::MENU_TOOLTIP);
    }
}

fn weave_type_name(weave_type: WeaveType) -> &'static str {
    match weave_type {
        WeaveType::LightAttack => "Light Attack",
        WeaveType::HeavyAttack => "Heavy Attack",
        WeaveType::BashAttack => "Bash Attack",
        WeaveType::BlockCasting => "Block Casting",
    }
}

fn theme_name(theme: Theme) -> &'static str {
    match theme {
        Theme::Dark => "Dark",
        Theme::Light => "Light",
    }
}

fn level_name(level: LevelName) -> &'static str {
    match level {
        LevelName::Off => "OFF",
        LevelName::Error => "ERROR",
        LevelName::Warn => "WARN",
        LevelName::Info => "INFO",
        LevelName::Debug => "DEBUG",
        LevelName::Trace => "TRACE",
    }
}
