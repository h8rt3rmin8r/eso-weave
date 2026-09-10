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

use crate::app::encounter_history::{
    metric_presentation, quality_label, EncounterHistoryWorker, HistoryEvent,
};
use crate::app::log_view::build_log_view;
use crate::app::settings_form::{SettingsForm, UiPrefs};
use crate::app::{
    app_toggle_intent, beacon_primary_action, effective_dashboard_layout, modal_extent,
    override_edit_for, strings, widgets, AppModel, AppView, BeaconPrimaryAction, DashboardLayout,
    ResourceTheme, SkillEdit, StatusLine, UiIntent,
};
use crate::beacon::api_check::ApiCheckOutcome;
use crate::catalog::Channel;
use crate::catalog_pipeline::CandidateSummary;
use crate::catalog_update::{
    candidate_compatibility, resolve_availability, AvailabilityInput, CaptureFingerprint,
    CatalogResolution, CatalogUpdateService, CatalogUpdateWorker, CheckFreshness, LiveUpdateState,
    UpdateProgress, UpdateStage, WorkerEvent,
};
use crate::config::state::WindowGeometry;
use crate::config::{LevelName, Theme};
use crate::documentation::{BrowserOpener, DocumentationService, NativeBrowser};
use crate::encounter::{
    EncounterHistoryService, EncounterIdentity, EncounterProjection, EncounterSummary,
    HistoryDiagnostic, HistoryDiagnosticKind, MetricResult,
};
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
fn lifecycle_button(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    text: &str,
    primary: bool,
) -> egui::Response {
    let mut button = egui::Button::new(if primary {
        egui::RichText::new(text).color(palette.gold_text)
    } else {
        egui::RichText::new(text)
    });
    if primary {
        button = button.fill(palette.gold);
    }
    ui.add_sized(
        [LIFECYCLE_BUTTON_WIDTH, ui.spacing().interact_size.y],
        button,
    )
    .clickable()
}

fn dashboard_frame(palette: &crate::app::theme::Palette) -> egui::Frame {
    egui::Frame::new()
        .fill(palette.panel)
        .stroke(egui::Stroke::new(1.0, palette.stroke))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 6,
            bottom: 6,
        })
}

const WEAVE_TYPES: [WeaveType; 4] = [
    WeaveType::LightAttack,
    WeaveType::HeavyAttack,
    WeaveType::BashAttack,
    WeaveType::BlockCasting,
];

const KEYS: [Key; 13] = Key::ALL;

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

const DASHBOARD_NARROW_GAP: f32 = 4.0;
const DASHBOARD_LABEL_WIDTH: f32 = 118.0;
const DASHBOARD_RESOURCE_GAP: f32 = 6.0;
const LIFECYCLE_BUTTON_WIDTH: f32 = 76.0;
const LIFECYCLE_BUTTON_GAP: f32 = 4.0;
const DASHBOARD_INTERACTION_WIDTH: f32 = 2.0 * LIFECYCLE_BUTTON_WIDTH + LIFECYCLE_BUTTON_GAP;
const DASHBOARD_FRAME_VERTICAL_OVERHEAD: f32 = 14.0;

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
    system_state_expanded: bool,
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
    /// Responsive dashboard mode and section rectangles from the latest frame.
    last_dashboard_layout: Option<DashboardLayout>,
    dashboard_rects: Option<(egui::Rect, egui::Rect)>,
    /// Width inputs needed to project a pending responsive transition before the
    /// bottom log panel consumes space on the next frame.
    last_dashboard_available_width: Option<f32>,
    previous_frame_available_width: Option<f32>,
    pending_responsive_content_height: Option<f32>,
    /// One-frame guard for disclosure-driven dashboard reflow. The bottom panel
    /// must wait until the new stacked extent has been measured directly.
    dashboard_state_reflow_pending: bool,
    documentation: DocumentationService,
    documentation_opener: Box<dyn BrowserOpener>,
    documentation_error: Option<String>,
    catalog_worker: Option<CatalogUpdateWorker>,
    catalog_update_open: bool,
    catalog_candidates: Vec<CandidateSummary>,
    catalog_selected: Option<String>,
    catalog_acknowledged_candidate: Option<String>,
    catalog_progress: Option<UpdateProgress>,
    catalog_update_message: Option<String>,
    catalog_notice_dismissed: bool,
    catalog_observed_live: Option<crate::catalog::version::GameVersion>,
    catalog_check_freshness: CheckFreshness,
    catalog_live_state: LiveUpdateState,
    catalog_status_ready: bool,
    collector_waiting_fingerprint: Option<CaptureFingerprint>,
    collector_status: Option<crate::collector::lifecycle::CollectorStatus>,
    collector_status_requested: bool,
    encounter_history_worker: Option<EncounterHistoryWorker>,
    encounter_history_open: bool,
    encounter_history_initialized: bool,
    encounter_history_busy: bool,
    encounter_history: Vec<EncounterSummary>,
    encounter_history_selected: Option<EncounterIdentity>,
    encounter_history_detail: Option<Result<EncounterProjection, HistoryDiagnostic>>,
    encounter_history_diagnostic: Option<HistoryDiagnostic>,
    encounter_history_message: Option<String>,
    encounter_history_catalog_refresh_pending: bool,
    encounter_delete_confirmation: Option<EncounterDeleteConfirmation>,
}

#[derive(Debug, Clone)]
enum EncounterDeleteConfirmation {
    One(EncounterIdentity),
    All,
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
            system_state_expanded: ui_prefs.system_state_expanded,
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
            last_dashboard_layout: None,
            dashboard_rects: None,
            last_dashboard_available_width: None,
            previous_frame_available_width: None,
            pending_responsive_content_height: None,
            dashboard_state_reflow_pending: false,
            documentation: DocumentationService::new(),
            documentation_opener: Box::new(NativeBrowser),
            documentation_error: None,
            catalog_worker: None,
            catalog_update_open: false,
            catalog_candidates: Vec::new(),
            catalog_selected: None,
            catalog_acknowledged_candidate: None,
            catalog_progress: None,
            catalog_update_message: None,
            catalog_notice_dismissed: false,
            catalog_observed_live: None,
            catalog_check_freshness: CheckFreshness::Offline,
            catalog_live_state: LiveUpdateState::OfflineStaleCheck,
            catalog_status_ready: false,
            collector_waiting_fingerprint: None,
            collector_status: None,
            collector_status_requested: false,
            encounter_history_worker: None,
            encounter_history_open: false,
            encounter_history_initialized: false,
            encounter_history_busy: false,
            encounter_history: Vec::new(),
            encounter_history_selected: None,
            encounter_history_detail: None,
            encounter_history_diagnostic: None,
            encounter_history_message: None,
            encounter_history_catalog_refresh_pending: false,
            encounter_delete_confirmation: None,
        }
    }

    /// Enables the background catalog status and user-initiated update worker.
    pub fn with_catalog_updates(mut self, service: CatalogUpdateService) -> Self {
        let worker = CatalogUpdateWorker::spawn(service);
        let _ = worker.startup();
        self.catalog_worker = Some(worker);
        self
    }

    /// Enables the local encounter-history worker. The worker remains idle until
    /// the user opens the history surface or requests an operation.
    pub fn with_encounter_history(mut self, service: EncounterHistoryService) -> Self {
        self.encounter_history_worker = Some(EncounterHistoryWorker::spawn(service));
        self
    }

    pub fn encounter_history_busy(&self) -> bool {
        self.encounter_history_busy
    }

    pub fn encounter_history_count(&self) -> usize {
        self.encounter_history.len()
    }

    /// Replaces the operating-system browser adapter. Production uses
    /// [`NativeBrowser`]; deterministic UI tests provide a recording or failing
    /// adapter without launching an external process.
    pub fn with_documentation_opener(mut self, opener: Box<dyn BrowserOpener>) -> Self {
        self.documentation_opener = opener;
        self
    }

    /// Latest documentation start or browser-launch error, if one is visible.
    pub fn documentation_error(&self) -> Option<&str> {
        self.documentation_error.as_deref()
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

    /// Whether the System and State disclosure is currently expanded.
    pub fn system_state_expanded(&self) -> bool {
        self.system_state_expanded
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
            self.catalog_status_ready = true;
            self.catalog_observed_live = outcome.last_seen_game_version;
            self.catalog_check_freshness = if outcome.fresh {
                CheckFreshness::Fresh
            } else {
                CheckFreshness::Offline
            };
            self.model.apply_api_check(outcome);
        }
        self.refresh_catalog_availability();
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

    /// Responsive dashboard mode selected for the latest rendered frame.
    pub fn last_dashboard_layout(&self) -> Option<DashboardLayout> {
        self.last_dashboard_layout
    }

    /// Live HUD and System and State rectangles from the latest frame.
    pub fn dashboard_rects(&self) -> Option<(egui::Rect, egui::Rect)> {
        self.dashboard_rects
    }

    /// Projects the content height for an imminent wide-to-narrow transition.
    ///
    /// The bottom panel is allocated before the central panel, so it cannot use
    /// the current frame's measurement. Window-width delta is sufficient to
    /// project the dashboard's available width because the surrounding margins
    /// are stable. The stacked dashboard adds the shorter section's height plus
    /// its inter-section gap to the previous wide measurement.
    fn projected_content_height(&self, frame_available_width: f32) -> f32 {
        let projected_dashboard_width = match (
            self.last_dashboard_available_width,
            self.previous_frame_available_width,
        ) {
            (Some(dashboard_width), Some(previous_frame_width)) => {
                dashboard_width + frame_available_width - previous_frame_width
            }
            _ => return self.content_extent.y,
        };
        if self.last_dashboard_layout == Some(DashboardLayout::Wide)
            && effective_dashboard_layout(projected_dashboard_width, self.system_state_expanded)
                == DashboardLayout::Narrow
        {
            if let Some((live, system)) = self.dashboard_rects {
                return self.content_extent.y
                    + live.height().min(system.height())
                    + DASHBOARD_NARROW_GAP;
            }
        }
        self.content_extent.y
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

    /// Opens or closes the catalog update modal for interaction tests.
    pub fn set_catalog_update_open(&mut self, open: bool) {
        self.catalog_update_open = open;
    }

    pub fn catalog_update_open(&self) -> bool {
        self.catalog_update_open
    }

    fn drain_catalog_updates(&mut self) {
        loop {
            let event = match self
                .catalog_worker
                .as_ref()
                .map(CatalogUpdateWorker::try_recv)
            {
                Some(Ok(event)) => event,
                _ => break,
            };
            match event {
                WorkerEvent::StartupComplete {
                    candidates,
                    resolution,
                    recovered_staging,
                    discovery_failure,
                } => {
                    self.catalog_status_ready = true;
                    self.catalog_candidates = candidates;
                    self.choose_default_catalog_candidate();
                    let warning = self.install_active_catalog(resolution);
                    let mut messages = Vec::new();
                    if let Some(warning) = warning {
                        messages.push(warning);
                    }
                    if recovered_staging > 0 {
                        messages.push(format!(
                            "Recovered {recovered_staging} interrupted catalog staging operation(s)."
                        ));
                    }
                    if let Some(failure) = discovery_failure {
                        messages.push(format!("Candidate discovery failed: {}", failure.message));
                    }
                    self.catalog_update_message =
                        (!messages.is_empty()).then(|| messages.join(" "));
                }
                WorkerEvent::DiscoveryComplete(candidates) => {
                    self.catalog_status_ready = true;
                    self.catalog_candidates = candidates;
                    self.choose_default_catalog_candidate();
                    self.catalog_update_message = Some(if self.catalog_candidates.is_empty() {
                        "No verified candidates were found in the catalog import folders.".into()
                    } else {
                        "Catalog candidate review refreshed.".into()
                    });
                }
                WorkerEvent::CaptureBoundaryRecorded(fingerprint) => {
                    self.collector_waiting_fingerprint = Some(fingerprint);
                    self.catalog_progress = Some(UpdateProgress {
                        stage: UpdateStage::WaitingForCapture,
                        completed_bytes: 0,
                        total_bytes: 0,
                        elapsed_millis: 0,
                        message: "Waiting for user/game. In ESO, run /reloadui, log out, or exit to flush SavedVariables."
                            .into(),
                    });
                    self.catalog_update_message = Some(
                        "Capture boundary recorded privately. ESO Weave is not reading in-memory game state."
                            .into(),
                    );
                }
                WorkerEvent::CollectorCandidateBuilt(candidate) => {
                    if !self
                        .catalog_candidates
                        .iter()
                        .any(|existing| existing.candidate_sha256 == candidate.candidate_sha256)
                    {
                        self.catalog_candidates.push(candidate.clone());
                    }
                    self.catalog_selected = Some(candidate.candidate_sha256);
                    self.catalog_progress = Some(UpdateProgress {
                        stage: UpdateStage::Complete,
                        completed_bytes: candidate.total_bytes,
                        total_bytes: candidate.total_bytes,
                        elapsed_millis: 0,
                        message: "Local collector candidate built and verified. Review it before installation."
                            .into(),
                    });
                    self.catalog_update_message = Some(
                        "The local candidate passed S073 verification. Installation remains a separate explicit action."
                            .into(),
                    );
                }
                WorkerEvent::CollectorState { status, message } => {
                    if let Some(status) = status {
                        self.collector_status = Some(status);
                    }
                    self.catalog_update_message = Some(message);
                }
                WorkerEvent::Progress(progress) => {
                    self.catalog_progress = Some(progress);
                }
                WorkerEvent::InstallComplete {
                    outcome,
                    resolution,
                } => {
                    let warning = self.install_active_catalog(*resolution);
                    self.catalog_progress = Some(UpdateProgress {
                        stage: UpdateStage::Complete,
                        completed_bytes: outcome.candidate.total_bytes,
                        total_bytes: outcome.candidate.total_bytes,
                        elapsed_millis: 0,
                        message: "Catalog installed and selected. Restart safety verified.".into(),
                    });
                    self.catalog_update_message = outcome.receipt_warning.or(warning).or_else(|| {
                        Some(format!(
                            "Installed Live catalog {}. The previous catalog remains available for rollback.",
                            outcome.candidate.catalog_version
                        ))
                    });
                }
                WorkerEvent::RollbackComplete {
                    outcome,
                    resolution,
                } => {
                    let warning = self.install_active_catalog(*resolution);
                    self.catalog_progress = Some(UpdateProgress {
                        stage: UpdateStage::Complete,
                        completed_bytes: 0,
                        total_bytes: 0,
                        elapsed_millis: 0,
                        message: "Previous verified catalog restored.".into(),
                    });
                    self.catalog_update_message = outcome
                        .receipt_warning
                        .or(warning)
                        .or_else(|| Some("Catalog rollback completed.".into()));
                }
                WorkerEvent::Failed(failure) => {
                    self.catalog_progress = Some(UpdateProgress {
                        stage: failure.stage,
                        completed_bytes: 0,
                        total_bytes: 0,
                        elapsed_millis: 0,
                        message: failure.message.clone(),
                    });
                    self.catalog_update_message =
                        Some(format!("{}: {}", failure.code, failure.message));
                }
                WorkerEvent::ReceiptWriteFailed => {
                    self.catalog_update_message = Some(
                        "receipt-write-failed: The operation stayed safe, but its redacted receipt could not be stored."
                            .into(),
                    );
                }
                WorkerEvent::Cancelled => {
                    self.catalog_progress = Some(UpdateProgress {
                        stage: UpdateStage::Cancelled,
                        completed_bytes: 0,
                        total_bytes: 0,
                        elapsed_millis: 0,
                        message: "Catalog operation cancelled. The prior selection is unchanged."
                            .into(),
                    });
                    self.catalog_update_message = Some(
                        "Catalog operation cancelled. The prior catalog remains active.".into(),
                    );
                }
            }
        }
        if self.catalog_update_open
            && !self.collector_status_requested
            && self
                .catalog_worker
                .as_ref()
                .is_some_and(|worker| !worker.is_busy())
        {
            if let (Some(worker), Some(root)) = (
                self.catalog_worker.as_ref(),
                self.model.catalog_collector_addons_root(),
            ) {
                self.collector_status_requested = worker.inspect_collector(root);
            }
        }
        self.request_catalog_changed_history_detail();
        self.refresh_catalog_availability();
    }

    fn install_active_catalog(&mut self, resolution: CatalogResolution) -> Option<String> {
        let CatalogResolution {
            access,
            path,
            warning,
            ..
        } = resolution;
        if let Some(worker) = &self.encounter_history_worker {
            worker.set_catalog_path(path);
        }
        self.model.set_catalog(access);
        if self.encounter_history_selected.is_some() {
            self.encounter_history_detail = None;
            self.encounter_history_catalog_refresh_pending = true;
        }
        warning
    }

    fn refresh_catalog_availability(&mut self) {
        let active = self.model.catalog().release().ok().flatten();
        let live_candidates = self
            .catalog_candidates
            .iter()
            .filter(|candidate| candidate.channel == Channel::Live)
            .cloned()
            .collect();
        let pts_candidates = self
            .catalog_candidates
            .iter()
            .filter(|candidate| candidate.channel == Channel::Pts)
            .cloned()
            .collect();
        self.catalog_live_state = resolve_availability(AvailabilityInput {
            active,
            observed_live: self.catalog_observed_live,
            freshness: self.catalog_check_freshness,
            live_candidates,
            pts_candidates,
            collector_capture_required: false,
            supported_schema: crate::catalog::schema::SCHEMA_VERSION,
        })
        .live_state;
    }

    fn choose_default_catalog_candidate(&mut self) {
        let selected_exists = self.catalog_selected.as_ref().is_some_and(|selected| {
            self.catalog_candidates
                .iter()
                .any(|candidate| &candidate.candidate_sha256 == selected)
        });
        if !selected_exists {
            self.catalog_selected = self
                .catalog_candidates
                .iter()
                .find(|candidate| candidate.channel == Channel::Live)
                .map(|candidate| candidate.candidate_sha256.clone());
        }
    }

    fn request_history_refresh(&mut self) {
        if self.encounter_history_busy {
            return;
        }
        match self
            .encounter_history_worker
            .as_ref()
            .map(EncounterHistoryWorker::refresh)
        {
            Some(Ok(())) => {
                self.encounter_history_busy = true;
                self.encounter_history_initialized = true;
                self.encounter_history_diagnostic = None;
            }
            Some(Err(_)) => {
                self.encounter_history_message =
                    Some("Encounter history is already processing another request.".into());
            }
            None => {}
        }
    }

    fn request_history_detail(&mut self, identity: EncounterIdentity) {
        if self.encounter_history_busy {
            return;
        }
        match self
            .encounter_history_worker
            .as_ref()
            .map(|worker| worker.load_detail(identity.clone()))
        {
            Some(Ok(())) => {
                self.encounter_history_selected = Some(identity);
                self.encounter_history_detail = None;
                self.encounter_history_catalog_refresh_pending = false;
                self.encounter_history_busy = true;
            }
            Some(Err(_)) => {
                self.encounter_history_message =
                    Some("Encounter history is already processing another request.".into());
            }
            None => {}
        }
    }

    fn request_catalog_changed_history_detail(&mut self) {
        if !self.encounter_history_catalog_refresh_pending || self.encounter_history_busy {
            return;
        }
        let Some(identity) = self.encounter_history_selected.clone() else {
            self.encounter_history_catalog_refresh_pending = false;
            return;
        };
        match self
            .encounter_history_worker
            .as_ref()
            .map(|worker| worker.load_detail(identity))
        {
            Some(Ok(())) => {
                self.encounter_history_catalog_refresh_pending = false;
                self.encounter_history_busy = true;
            }
            Some(Err(_)) => {}
            None => self.encounter_history_catalog_refresh_pending = false,
        }
    }

    fn drain_encounter_history(&mut self) {
        while let Some(Ok(event)) = self
            .encounter_history_worker
            .as_ref()
            .map(EncounterHistoryWorker::try_receive)
        {
            self.encounter_history_busy = false;
            match event {
                HistoryEvent::Snapshot {
                    encounters,
                    message,
                    ..
                } => {
                    self.encounter_history = encounters;
                    self.encounter_history_message = message;
                    self.encounter_history_diagnostic = None;
                    let selection_still_exists = self
                        .encounter_history_selected
                        .as_ref()
                        .is_some_and(|selected| {
                            self.encounter_history.iter().any(|summary| {
                                summary.session_id == selected.session_id
                                    && summary.encounter_id == selected.encounter_id
                            })
                        });
                    if !selection_still_exists {
                        self.encounter_history_selected = None;
                        self.encounter_history_detail = None;
                        self.encounter_history_catalog_refresh_pending = false;
                    }
                }
                HistoryEvent::Detail { identity, result } => {
                    if self.encounter_history_selected.as_ref() == Some(&identity) {
                        self.encounter_history_detail = Some(result.map(|projection| *projection));
                    }
                }
                HistoryEvent::Failed { diagnostic, .. } => {
                    self.encounter_history_message = None;
                    self.encounter_history_diagnostic = Some(diagnostic);
                }
            }
        }
        self.request_catalog_changed_history_detail();
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
        self.drain_catalog_updates();
        self.drain_encounter_history();
        if self
            .catalog_worker
            .as_ref()
            .is_some_and(CatalogUpdateWorker::is_busy)
        {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
        if self.encounter_history_busy {
            ctx.request_repaint_after(Duration::from_millis(50));
        }
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
        let frame_available_width = ui.available_width();
        let window_rect = ctx.content_rect();
        let window_h = window_rect.height();
        let frame_overhead = 2.0 * crate::app::LOG_FRAME_MARGIN + log_panel_separator(&ctx);
        let state_reflow_pending = self.dashboard_state_reflow_pending;
        let projected_content_h = if state_reflow_pending {
            self.content_extent.y
        } else {
            self.projected_content_height(frame_available_width)
        };
        if projected_content_h > self.content_extent.y + 0.5 {
            self.pending_responsive_content_height = Some(
                self.pending_responsive_content_height
                    .unwrap_or_default()
                    .max(projected_content_h),
            );
        }
        let projected_content_h = self
            .pending_responsive_content_height
            .unwrap_or(projected_content_h)
            .max(projected_content_h);
        // If a responsive transition makes the content taller than the current
        // frame, preserve the open preference but defer the log for this frame.
        // The minimum-size update below grows the window before the pane returns;
        // drawing it now would cover the newly stacked dashboard or Skills.
        let render_log_panel = self.log_panel_open
            && !state_reflow_pending
            && window_h + 0.5 >= projected_content_h + frame_overhead;
        if render_log_panel {
            let row_h = log_row_height(&ctx);
            let content_h = projected_content_h;
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
            } else if content_h > self.content_extent.y + 0.5 {
                // A width-only resize can switch the dashboard to its taller
                // stacked layout without entering the window-height branch.
                // Force the projected bound now; egui otherwise retains the
                // previous panel size until after it has covered content.
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
                        if ui
                            .button(strings::MENU_CATALOG_UPDATE)
                            .on_hover_text(strings::MENU_CATALOG_UPDATE_TOOLTIP)
                            .clickable()
                            .clicked()
                        {
                            self.catalog_update_open = true;
                            ui.close();
                        }
                        if ui
                            .button(strings::MENU_ENCOUNTER_HISTORY)
                            .on_hover_text(strings::MENU_ENCOUNTER_HISTORY_TOOLTIP)
                            .clickable()
                            .clicked()
                        {
                            self.encounter_history_open = true;
                            if !self.encounter_history_initialized {
                                self.request_history_refresh();
                            }
                            ui.close();
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
                    ui.menu_button(strings::MENU_HELP, |ui| {
                        if ui
                            .button(strings::MENU_DOCUMENTATION)
                            .on_hover_text(strings::MENU_DOCUMENTATION_TOOLTIP)
                            .clickable()
                            .clicked()
                        {
                            ui.close();
                            match self
                                .documentation
                                .open_with(self.documentation_opener.as_ref())
                            {
                                Ok(_) => self.documentation_error = None,
                                Err(err) => {
                                    tracing::warn!(target: "eso_weave::documentation", "{err}");
                                    self.documentation_error = Some(format!(
                                        "ESO Weave could not open the offline guide. {err}"
                                    ));
                                }
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

                if !self.catalog_notice_dismissed
                    && self.catalog_status_ready
                    && (self.catalog_live_state != LiveUpdateState::CatalogCurrent
                        || !self.catalog_candidates.is_empty()
                        || self.catalog_update_message.is_some())
                {
                    ui.horizontal_wrapped(|ui| {
                        let text = live_state_label(self.catalog_live_state);
                        ui.label(text);
                        if ui.button("Review").clickable().clicked() {
                            self.catalog_update_open = true;
                        }
                        if ui.button("Dismiss").clickable().clicked() {
                            self.catalog_notice_dismissed = true;
                        }
                    });
                    ui.separator();
                }

                self.main_view(ui, &mut intents);
            });
            // The scope's rect grows only to what the content allocated, so its
            // height is the content height rather than the panel height.
            let extent =
                crate::app::intrinsic_extent(self.content_width, scope.response.rect.height());
            measured = egui::vec2(extent.0, extent.1);
            if render_log_panel {
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
        let mut extent = if let Some(previous) = self.prev_measured {
            (
                if (previous.x - measured.x).abs() <= 0.5 {
                    measured.x
                } else {
                    self.content_extent.x
                },
                if (previous.y - measured.y).abs() <= 0.5 {
                    measured.y
                } else {
                    self.content_extent.y
                },
            )
        } else {
            crate::app::content_min_size(measured_tuple, (BOOT_MIN_SIZE.x, BOOT_MIN_SIZE.y), stable)
        };
        if state_reflow_pending {
            extent.1 = measured.y;
            self.pending_responsive_content_height = None;
            self.dashboard_state_reflow_pending = false;
        }
        self.content_extent = egui::vec2(extent.0, extent.1);
        if self
            .pending_responsive_content_height
            .is_some_and(|pending| self.content_extent.y + 0.5 >= pending)
        {
            self.pending_responsive_content_height = None;
        }
        self.prev_measured = Some(measured);
        let responsive_content_height = self
            .pending_responsive_content_height
            .unwrap_or(self.content_extent.y)
            .max(self.content_extent.y);
        let target_min = if self.log_panel_open {
            egui::vec2(
                self.content_extent.x + LOG_WIDTH_BONUS,
                responsive_content_height + crate::app::open_log_reserve(log_row_height(&ctx)),
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
        self.previous_frame_available_width = Some(frame_available_width);

        if self.settings_open {
            self.settings_modal(&ctx, &mut intents);
        }

        if self.catalog_update_open {
            self.catalog_update_modal(&ctx);
        }

        if self.encounter_history_open {
            self.encounter_history_window(&ctx);
        }

        if self.encounter_delete_confirmation.is_some() {
            self.encounter_delete_confirmation(&ctx);
        }

        if let Some(message) = self.documentation_error.clone() {
            egui::Window::new("Documentation unavailable")
                .id(egui::Id::new("documentation_error"))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(&ctx, |ui| {
                    ui.label(message);
                    if ui.button("Close").clickable().clicked() {
                        self.documentation_error = None;
                    }
                });
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

        let dashboard_width = ui.available_width();
        self.last_dashboard_available_width = Some(dashboard_width);
        let layout = effective_dashboard_layout(dashboard_width, self.system_state_expanded);
        self.last_dashboard_layout = Some(layout);
        let (live_rect, system_rect) = match layout {
            DashboardLayout::Narrow => {
                let live = Self::live_hud(ui, &palette, &view);
                ui.add_space(DASHBOARD_NARROW_GAP);
                let shared_height = self.system_state_expanded.then_some(live.height());
                let system = self.system_and_state(ui, &palette, &view, intents, shared_height);
                (live, system)
            }
            DashboardLayout::Wide => {
                let mut rects = None;
                let gap = ui.spacing().item_spacing.x;
                let usable_width = ui.available_width() - gap;
                let column_width = usable_width / 2.0;
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(column_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            let live = Self::live_hud(ui, &palette, &view);
                            rects = Some((live, egui::Rect::NOTHING));
                        },
                    );
                    ui.allocate_ui_with_layout(
                        egui::vec2(column_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            let live = rects.expect("Live HUD renders before operational state").0;
                            let system = self.system_and_state(
                                ui,
                                &palette,
                                &view,
                                intents,
                                Some(live.height()),
                            );
                            rects = Some((live, system));
                        },
                    );
                });
                rects.expect("dashboard columns render both sections")
            }
        };
        self.dashboard_rects = Some((live_rect, system_rect));
        // This is the compact layout's intrinsic width. The wide containers
        // expand with the window and must never feed the enforced minimum.
        self.note_content_width(572.0);
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

    fn live_hud(
        ui: &mut egui::Ui,
        palette: &crate::app::theme::Palette,
        view: &AppView,
    ) -> egui::Rect {
        dashboard_frame(palette)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                widgets::heading(ui, strings::LIVE_HUD_TITLE);
                widgets::resource_group(
                    ui,
                    palette,
                    &[
                        (
                            strings::HEALTH_TITLE,
                            &view.resources.health,
                            ResourceTheme::Health,
                        ),
                        (
                            strings::STAMINA_TITLE,
                            &view.resources.stamina,
                            ResourceTheme::Stamina,
                        ),
                        (
                            strings::MAGICKA_TITLE,
                            &view.resources.magicka,
                            ResourceTheme::Magicka,
                        ),
                    ],
                    0.0,
                );
                widgets::ultimate_meter(ui, palette, strings::ULTIMATE_TITLE, &view.ultimate);
                ui.add_space(DASHBOARD_RESOURCE_GAP);
                ui.spacing_mut().item_spacing.y = 2.0;
                game_context_row(ui, palette, &view.menu);
                dashboard_metric_row(
                    ui,
                    palette,
                    dashboard_metric(
                        strings::COMBAT_TITLE,
                        view.combat.state,
                        view.combat.role,
                        strings::COMBAT_TOOLTIP,
                    ),
                    0.0,
                    |_| {},
                );
                dashboard_metric_row(
                    ui,
                    palette,
                    dashboard_metric(
                        strings::MOVEMENT_TITLE,
                        view.movement.state,
                        view.movement.role,
                        strings::MOVEMENT_TOOLTIP,
                    ),
                    0.0,
                    |_| {},
                );
                dashboard_metric_row(
                    ui,
                    palette,
                    dashboard_metric(
                        strings::ROLL_DODGE_TITLE,
                        view.roll_dodge.state,
                        view.roll_dodge.role,
                        strings::ROLL_DODGE_TOOLTIP,
                    ),
                    0.0,
                    |_| {},
                );
                dashboard_metric_row(
                    ui,
                    palette,
                    dashboard_metric(
                        strings::LIFE_TITLE,
                        view.life.state,
                        view.life.role,
                        strings::LIFE_TOOLTIP,
                    ),
                    0.0,
                    |_| {},
                );

                let weapon = if view.weapon_bar.detected {
                    format!(
                        "{} | front {} | back {}",
                        view.weapon_bar.active_bar, view.weapon_bar.front, view.weapon_bar.back
                    )
                } else {
                    view.weapon_bar.active_bar.to_string()
                };
                dashboard_metric_row(
                    ui,
                    palette,
                    dashboard_metric(
                        strings::WEAPON_BAR_TITLE,
                        &weapon,
                        view.weapon_bar.role,
                        strings::WEAPON_BAR_TOOLTIP,
                    ),
                    0.0,
                    |_| {},
                );

                let quickslot = if view.quickslot.state.text == "Game not active" {
                    view.quickslot.state.text.clone()
                } else {
                    format!(
                        "{} | {} | {}",
                        view.quickslot.state.text,
                        view.quickslot.availability.text,
                        view.quickslot.cooldown.text
                    )
                };
                dashboard_metric_row(
                    ui,
                    palette,
                    dashboard_metric(
                        strings::QUICKSLOT_TITLE,
                        &quickslot,
                        view.quickslot.state.role,
                        strings::QUICKSLOT_TOOLTIP,
                    ),
                    0.0,
                    |_| {},
                );
            })
            .response
            .rect
    }

    fn system_and_state(
        &mut self,
        ui: &mut egui::Ui,
        palette: &crate::app::theme::Palette,
        view: &AppView,
        intents: &mut Vec<UiIntent>,
        expanded_outer_height: Option<f32>,
    ) -> egui::Rect {
        dashboard_frame(palette)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                if let Some(outer_height) = expanded_outer_height {
                    ui.set_min_height((outer_height - DASHBOARD_FRAME_VERTICAL_OVERHEAD).max(0.0));
                }
                let disclosure = ui
                    .scope(|ui| {
                        ui.visuals_mut().collapsing_header_frame = true;
                        egui::CollapsingHeader::new(
                            egui::RichText::new(strings::SYSTEM_STATE_TITLE).heading(),
                        )
                        .id_salt("system_state_disclosure")
                        .default_open(self.system_state_expanded)
                        .show_background(true)
                        .show(ui, |ui| {
                            // The bottom log is allocated before this disclosure.
                            // On a collapsed-to-expanded click, wait until the next
                            // guarded frame to reveal the body so content cannot
                            // grow underneath a log sized for the collapsed frame.
                            if !self.system_state_expanded {
                                return;
                            }
                            ui.spacing_mut().item_spacing.y = 2.0;
                            let game_summary = format!(
                                "{} | {}",
                                view.runtime_line.state_text, view.installation_line.state_text
                            );
                            let game_role = match view.installation_line.role {
                                crate::app::StatusRole::Warning | crate::app::StatusRole::Error => {
                                    view.installation_line.role
                                }
                                _ => view.runtime_line.role,
                            };
                            dashboard_metric_row(
                                ui,
                                palette,
                                dashboard_metric(
                                    strings::GAME_TITLE,
                                    &game_summary,
                                    game_role,
                                    strings::GAME_RUNTIME_TOOLTIP,
                                ),
                                0.0,
                                |_| {},
                            );
                            dashboard_metric_row(
                                ui,
                                palette,
                                dashboard_metric(
                                    strings::TRAVEL_TITLE,
                                    view.travel.state,
                                    view.travel.role,
                                    strings::TRAVEL_TOOLTIP,
                                ),
                                0.0,
                                |_| {},
                            );
                            dashboard_metric_row(
                                ui,
                                palette,
                                dashboard_metric(
                                    strings::WORLD_TITLE,
                                    view.world.state,
                                    view.world.role,
                                    strings::WORLD_TOOLTIP,
                                ),
                                0.0,
                                |_| {},
                            );

                            let mut running = !view.suspended;
                            dashboard_status_row(
                                ui,
                                palette,
                                &view.status_line,
                                DASHBOARD_INTERACTION_WIDTH,
                                |ui| {
                                    if widgets::toggle_switch_named(
                                        ui,
                                        &mut running,
                                        palette,
                                        strings::STATUS_TITLE,
                                    )
                                    .on_hover_text(strings::SUSPEND_TOOLTIP)
                                    .clickable()
                                    .changed()
                                    {
                                        intents.push(UiIntent::ToggleSuspend);
                                    }
                                },
                            );

                            dashboard_status_row(
                                ui,
                                palette,
                                &view.beacon_line,
                                DASHBOARD_INTERACTION_WIDTH,
                                |ui| {
                                    ui.spacing_mut().item_spacing.x = LIFECYCLE_BUTTON_GAP;
                                    let primary_intent =
                                        match beacon_primary_action(view.beacon_condition) {
                                            Some(BeaconPrimaryAction::Install) => {
                                                lifecycle_button(ui, palette, "Install", true)
                                                    .on_hover_text(strings::BEACON_INSTALL_TOOLTIP)
                                                    .clicked()
                                                    .then_some(UiIntent::InstallBeacon)
                                            }
                                            Some(BeaconPrimaryAction::Update) => {
                                                lifecycle_button(ui, palette, "Update", true)
                                                    .on_hover_text(strings::BEACON_UPDATE_TOOLTIP)
                                                    .clicked()
                                                    .then_some(UiIntent::UpdateBeacon)
                                            }
                                            None => None,
                                        };
                                    if let Some(intent) = primary_intent {
                                        intents.push(intent);
                                    }
                                    if view.uninstall_enabled
                                        && lifecycle_button(ui, palette, "Uninstall", false)
                                            .on_hover_text(strings::BEACON_UNINSTALL_TOOLTIP)
                                            .clicked()
                                    {
                                        self.confirm_uninstall = true;
                                    }
                                },
                            );

                            dashboard_status_row(
                                ui,
                                palette,
                                &view.beacon_signal_line,
                                0.0,
                                |_| {},
                            );

                            dashboard_status_row(ui, palette, &view.catalog_line, 0.0, |_| {});

                            let mut fishing_on = view.fishing_active;
                            dashboard_status_row(
                                ui,
                                palette,
                                &view.fishing_line,
                                DASHBOARD_INTERACTION_WIDTH,
                                |ui| {
                                    if widgets::toggle_switch_named(
                                        ui,
                                        &mut fishing_on,
                                        palette,
                                        strings::FISHING_TITLE,
                                    )
                                    .on_hover_text(strings::FISHING_TOGGLE_TOOLTIP)
                                    .clickable()
                                    .changed()
                                    {
                                        intents.push(UiIntent::SetFishing(fishing_on));
                                    }
                                },
                            );

                            let mut potion_on = view.auto_potion_requested;
                            dashboard_metric_row(
                                ui,
                                palette,
                                dashboard_metric(
                                    strings::AUTO_POTION_TITLE,
                                    &view.auto_potion.text,
                                    view.auto_potion.role,
                                    strings::AUTO_POTION_TOOLTIP,
                                ),
                                DASHBOARD_INTERACTION_WIDTH,
                                |ui| {
                                    if widgets::toggle_switch_named(
                                        ui,
                                        &mut potion_on,
                                        palette,
                                        strings::AUTO_POTION_TITLE,
                                    )
                                    .on_hover_text(strings::AUTO_POTION_TOGGLE_TOOLTIP)
                                    .clickable()
                                    .changed()
                                    {
                                        intents.push(UiIntent::SetAutoPotion(potion_on));
                                    }
                                },
                            );
                        })
                    })
                    .inner;
                disclosure
                    .header_response
                    .clone()
                    .on_hover_text(strings::SYSTEM_STATE_TOOLTIP)
                    .clickable();
                if disclosure.header_response.changed() {
                    self.system_state_expanded = !self.system_state_expanded;
                    self.dashboard_state_reflow_pending = true;
                    intents.push(UiIntent::SetSystemStateExpanded(self.system_state_expanded));
                }
            })
            .response
            .rect
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

    fn catalog_update_modal(&mut self, ctx: &egui::Context) {
        let busy = self
            .catalog_worker
            .as_ref()
            .is_some_and(CatalogUpdateWorker::is_busy);
        let modal = egui::Modal::new(egui::Id::new("eso_weave_catalog_update")).show(ctx, |ui| {
            let modal_width = 620.0_f32.min(ctx.content_rect().width() * 0.90).max(280.0);
            let modal_height = (ctx.content_rect().height() * 0.88).max(260.0);
            ui.set_min_width(modal_width);
            ui.set_max_width(modal_width);
            ui.heading("Catalog Update");
            egui::ScrollArea::vertical()
                .max_height(modal_height)
                .show(ui, |ui| {
            ui.label(
                "Updates are always user initiated. Imported hashes prove integrity, not who supplied the files.",
            );
            ui.separator();

            let (catalog_status, available) = self.model.catalog().status();
            ui.label(format!("Current Live catalog: {catalog_status}"));
            ui.label(format!(
                "Update status: {}",
                live_state_label(self.catalog_live_state)
            ));
            if !available {
                ui.colored_label(ui.visuals().warn_fg_color, "The bundled catalog is unavailable.");
            }

            ui.add_space(6.0);
            ui.strong("Reviewed import candidates");
            if self.catalog_candidates.is_empty() {
                ui.label(
                    "No verified candidates found. Place a complete S073 candidate under the application data catalog/import/live or catalog/import/pts folder, then refresh.",
                );
            }
            for candidate in &self.catalog_candidates {
                if candidate.channel == Channel::Live {
                    let label = format!(
                        "Live {} (API {}, {:.1} MiB)",
                        candidate.game_version,
                        candidate.api_version,
                        candidate.total_bytes as f64 / (1024.0 * 1024.0)
                    );
                    ui.radio_value(
                        &mut self.catalog_selected,
                        Some(candidate.candidate_sha256.clone()),
                        label,
                    );
                } else {
                    ui.label(format!(
                        "PTS preview only: {} (API {})",
                        candidate.game_version, candidate.api_version
                    ));
                }
            }

            let selected_summary = self.catalog_selected.as_ref().and_then(|hash| {
                self.catalog_candidates
                    .iter()
                    .find(|candidate| &candidate.candidate_sha256 == hash)
                    .cloned()
            });
            let compatibility = selected_summary.as_ref().map(|candidate| {
                self.model
                    .catalog()
                    .release()
                    .map_err(|_| "The active catalog metadata could not be read.".to_string())
                    .and_then(|active| {
                        candidate_compatibility(candidate, active.as_ref())
                            .map_err(|error| error.to_string())
                    })
            });

            if let Some(candidate) = &selected_summary {
                ui.group(|ui| {
                    ui.label(format!(
                        "Catalog {} | schema {} | {} sources",
                        candidate.catalog_version, candidate.catalog_schema, candidate.source_count
                    ));
                    ui.label(format!(
                        "Changes: {} added, {} changed, {} removed; icons: {} ready, {} placeholders",
                        candidate.diff.entities_added,
                        candidate.diff.entities_changed,
                        candidate.diff.entities_removed,
                        candidate.ready_icons,
                        candidate.placeholder_icons
                    ));
                    ui.label(format!(
                        "Integrity identity: {}...",
                        &candidate.candidate_sha256[..12]
                    ));
                    let provenance = candidate
                        .acquisition_counts
                        .iter()
                        .map(|(kind, count)| format!("{kind}: {count}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    ui.label(format!(
                        "Source acquisition: {}",
                        if provenance.is_empty() {
                            "none recorded"
                        } else {
                            &provenance
                        }
                    ));
                });
            }
            if let Some(Err(message)) = &compatibility {
                ui.colored_label(
                    ui.visuals().warn_fg_color,
                    format!("Candidate cannot be installed: {message}"),
                );
            }

            let trust_label =
                "I obtained this candidate from a review or release source I trust.";
            if let Some(candidate) = &selected_summary {
                let hash = candidate.candidate_sha256.clone();
                let mut acknowledged =
                    self.catalog_acknowledged_candidate.as_deref() == Some(hash.as_str());
                if ui.checkbox(&mut acknowledged, trust_label).changed() {
                    self.catalog_acknowledged_candidate = acknowledged.then_some(hash);
                }
            } else {
                let mut acknowledged = false;
                ui.add_enabled(false, egui::Checkbox::new(&mut acknowledged, trust_label));
            }
            ui.label(
                "Collector-assisted builds are local-only. ESO saves addon data only after /reloadui, logout, or exit. Captures are parsed as restricted data, never executed or uploaded.",
            );
            ui.label(
                "Collected categories: player skills, crafted abilities, item sets, champion skills, companions, races, and classes. Account and character names are excluded or represented only by a one-way scope key.",
            );
            let collector_status = match self.collector_status {
                Some(crate::collector::lifecycle::CollectorStatus::NotInstalled) => "not installed",
                Some(crate::collector::lifecycle::CollectorStatus::ManagedUpToDate) => "managed and current",
                Some(crate::collector::lifecycle::CollectorStatus::ManagedVersionMismatch) => {
                    "managed update available"
                }
                Some(crate::collector::lifecycle::CollectorStatus::Unmanaged) => {
                    "unmanaged (will not be modified)"
                }
                None => "AddOns directory unavailable",
            };
            ui.label(format!("Local collector: {collector_status}"));
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(!busy, egui::Button::new("Install/update collector"))
                    .clickable()
                    .clicked()
                {
                    match (
                        self.catalog_worker.as_ref(),
                        self.model.catalog_collector_addons_root(),
                    ) {
                        (Some(worker), Some(root)) => {
                            let _ = worker.install_collector(
                                root,
                                self.model.catalog_collector_running_state(),
                                self.model.effective_api_version(),
                            );
                        }
                        _ => {
                            self.catalog_update_message =
                                Some("The ESO AddOns directory could not be resolved.".into());
                        }
                    }
                }
                let wait_label = if self.collector_waiting_fingerprint.is_some() {
                    "Build from flushed capture"
                } else {
                    "Begin capture wait"
                };
                if ui
                    .add_enabled(!busy, egui::Button::new(wait_label))
                    .clickable()
                    .clicked()
                {
                    match (
                        self.catalog_worker.as_ref(),
                        self.model.catalog_collector_capture_path(),
                        self.collector_waiting_fingerprint.clone(),
                    ) {
                        (Some(worker), Some(path), Some(fingerprint)) => {
                            self.catalog_progress = None;
                            self.catalog_update_message = None;
                            let _ = worker.build_collector(path, fingerprint);
                        }
                        (Some(worker), Some(path), None) => {
                            self.catalog_progress = None;
                            self.catalog_update_message = None;
                            let _ = worker.observe_capture(path);
                        }
                        _ => {
                            self.catalog_update_message = Some(
                                "The collector capture location is unavailable until ESO's AddOns directory is configured."
                                    .into(),
                            );
                        }
                    }
                }
                if ui
                    .add_enabled(!busy, egui::Button::new("Delete capture"))
                    .clickable()
                    .clicked()
                {
                    if let (Some(worker), Some(path)) = (
                        self.catalog_worker.as_ref(),
                        self.model.catalog_collector_capture_path(),
                    ) {
                        let _ = worker.delete_capture(path);
                    } else {
                        self.catalog_update_message =
                            Some("The collector capture location could not be resolved.".into());
                    }
                    self.collector_waiting_fingerprint = None;
                }
                if ui
                    .add_enabled(!busy, egui::Button::new("Uninstall collector"))
                    .clickable()
                    .clicked()
                {
                    match (
                        self.catalog_worker.as_ref(),
                        self.model.catalog_collector_addons_root(),
                    ) {
                        (Some(worker), Some(root)) => {
                            let _ = worker.uninstall_collector(
                                root,
                                self.model.catalog_collector_running_state(),
                            );
                        }
                        _ => {
                            self.catalog_update_message =
                                Some("The ESO AddOns directory could not be resolved.".into());
                        }
                    }
                }
            });

            if let Some(progress) = &self.catalog_progress {
                ui.separator();
                ui.strong(stage_label(progress.stage));
                ui.label(&progress.message);
                ui.label(format!(
                    "Elapsed: {:.1} seconds",
                    progress.elapsed_millis as f64 / 1000.0
                ));
                if progress.total_bytes > 0 {
                    let fraction = progress.completed_bytes as f32 / progress.total_bytes as f32;
                    ui.add(
                        egui::ProgressBar::new(fraction.clamp(0.0, 1.0)).text(format!(
                            "{} / {} bytes",
                            progress.completed_bytes, progress.total_bytes
                        )),
                    );
                } else if busy {
                    ui.label("Progress total is not yet known.");
                }
            }
            if let Some(message) = &self.catalog_update_message {
                ui.label(message);
            }

            ui.separator();
            ui.horizontal_wrapped(|ui| {
                let can_install = !busy
                    && compatibility.as_ref().is_some_and(Result::is_ok)
                    && self.catalog_acknowledged_candidate.as_deref()
                        == self.catalog_selected.as_deref();
                if ui
                    .add_enabled(can_install, egui::Button::new("Install selected Live catalog"))
                    .clickable()
                    .clicked()
                {
                    if let (Some(worker), Some(hash)) =
                        (self.catalog_worker.as_ref(), self.catalog_selected.clone())
                    {
                        self.catalog_progress = None;
                        self.catalog_update_message = None;
                        let _ = worker.install(hash, true);
                    }
                }
                if ui
                    .add_enabled(!busy, egui::Button::new("Refresh candidates"))
                    .clickable()
                    .clicked()
                {
                    if let Some(worker) = &self.catalog_worker {
                        let _ = worker.discover();
                    }
                }
                if ui
                    .add_enabled(!busy, egui::Button::new("Roll back"))
                    .clickable()
                    .clicked()
                {
                    if let Some(worker) = &self.catalog_worker {
                        self.catalog_progress = None;
                        self.catalog_update_message = None;
                        let _ = worker.rollback();
                    }
                }
                if busy {
                    let cancellable = self
                        .catalog_progress
                        .as_ref()
                        .is_none_or(|progress| progress.stage != UpdateStage::Committing);
                    if ui
                        .add_enabled(cancellable, egui::Button::new("Cancel"))
                        .clickable()
                        .clicked()
                    {
                        if let Some(worker) = &self.catalog_worker {
                            worker.cancel();
                        }
                    }
                } else if ui.button("Close").clickable().clicked() {
                    self.catalog_update_open = false;
                }
            });
                });
        });
        if modal.should_close() {
            if busy {
                if let Some(worker) = &self.catalog_worker {
                    worker.cancel();
                }
            } else {
                self.catalog_update_open = false;
            }
        }
    }

    fn encounter_history_window(&mut self, ctx: &egui::Context) {
        let mut open = self.encounter_history_open;
        egui::Window::new("Encounter History")
            .id(egui::Id::new("eso_weave_encounter_history"))
            .open(&mut open)
            .default_size(egui::vec2(620.0, 700.0))
            .min_size(egui::vec2(360.0, 320.0))
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.label(
                    "Private local observations. Metrics are versioned observations, not complete encounter or Combat Metrics parity claims.",
                );
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add_enabled(
                            !self.encounter_history_busy
                                && self.encounter_history_worker.is_some(),
                            egui::Button::new("Refresh"),
                        )
                        .clickable()
                        .clicked()
                    {
                        self.request_history_refresh();
                    }
                    if ui
                        .add_enabled(
                            !self.encounter_history_busy
                                && self.encounter_history_worker.is_some(),
                            egui::Button::new("Import Current Capture"),
                        )
                        .on_hover_text(
                            "Import the terminal ESO Weave Encounter capture from the selected Live or PTS environment.",
                        )
                        .clickable()
                        .clicked()
                    {
                        self.request_history_import();
                    }
                    if ui
                        .add_enabled(
                            !self.encounter_history_busy && !self.encounter_history.is_empty(),
                            egui::Button::new("Delete All"),
                        )
                        .clickable()
                        .clicked()
                    {
                        self.encounter_delete_confirmation =
                            Some(EncounterDeleteConfirmation::All);
                    }
                    if self.encounter_history_busy {
                        ui.spinner();
                        ui.label("Working...");
                    }
                });

                if self.encounter_history_worker.is_none() {
                    ui.separator();
                    ui.strong("Store Unavailable");
                    ui.label(
                        "A per-user application data directory could not be resolved. Encounter history is disabled.",
                    );
                    return;
                }
                if let Some(message) = &self.encounter_history_message {
                    ui.label(message);
                }
                if let Some(diagnostic) = &self.encounter_history_diagnostic {
                    ui.separator();
                    ui.strong(history_diagnostic_heading(diagnostic.kind));
                    ui.label(&diagnostic.message);
                }

                ui.separator();
                ui.heading("Local Encounters");
                if self.encounter_history.is_empty() && !self.encounter_history_busy {
                    ui.label("No local encounters. Import a terminal capture to begin.");
                }

                let mut selected = None;
                egui::ScrollArea::vertical()
                    .id_salt("encounter_history_list")
                    .max_height(190.0)
                    .show(ui, |ui| {
                        for summary in self.encounter_history.iter().rev() {
                            let identity = EncounterIdentity::from(summary);
                            let active = self.encounter_history_selected.as_ref() == Some(&identity);
                            ui.group(|ui| {
                                if ui
                                    .selectable_label(active, &summary.encounter_id)
                                    .clickable()
                                    .clicked()
                                {
                                    selected = Some(identity);
                                }
                                ui.label(format!(
                                    "{} | {} capture | {} stored, {} omitted",
                                    summary.channel,
                                    capture_status_label(summary.status),
                                    summary.stored_event_count,
                                    summary.omitted_event_count
                                ));
                                ui.small(format!(
                                    "{} to {} | Session {}",
                                    summary.started_at, summary.finished_at, summary.session_id
                                ));
                            });
                        }
                    });
                if let Some(identity) = selected {
                    self.request_history_detail(identity);
                }

                if self.encounter_history_selected.is_some() {
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        ui.heading("Selected Encounter");
                        if ui
                            .add_enabled(
                                !self.encounter_history_busy,
                                egui::Button::new("Delete Encounter"),
                            )
                            .clickable()
                            .clicked()
                        {
                            self.encounter_delete_confirmation = self
                                .encounter_history_selected
                                .clone()
                                .map(EncounterDeleteConfirmation::One);
                        }
                    });
                    match &self.encounter_history_detail {
                        Some(Ok(projection)) => render_encounter_projection(ui, projection),
                        Some(Err(diagnostic)) => {
                            ui.strong(history_diagnostic_heading(diagnostic.kind));
                            ui.label(&diagnostic.message);
                        }
                        None if self.encounter_history_busy => {
                            ui.label("Loading observed metrics...");
                        }
                        None => {
                            ui.label("Select the encounter again to rebuild observed metrics.");
                        }
                    }
                }
            });
        self.encounter_history_open = open;
        if !open {
            self.encounter_delete_confirmation = None;
        }
    }

    fn request_history_import(&mut self) {
        let Some((source_path, expected_channel)) = self.model.encounter_capture_source() else {
            self.encounter_history_diagnostic = Some(HistoryDiagnostic::source_unavailable());
            self.encounter_history_message = None;
            return;
        };
        match self
            .encounter_history_worker
            .as_ref()
            .map(|worker| worker.import_current(source_path, expected_channel))
        {
            Some(Ok(())) => {
                self.encounter_history_busy = true;
                self.encounter_history_diagnostic = None;
                self.encounter_history_message = None;
            }
            Some(Err(_)) => {
                self.encounter_history_message =
                    Some("Encounter history is already processing another request.".into());
            }
            None => {}
        }
    }

    fn encounter_delete_confirmation(&mut self, ctx: &egui::Context) {
        let Some(confirmation) = self.encounter_delete_confirmation.clone() else {
            return;
        };
        let (title, body, confirm_label) = match &confirmation {
            EncounterDeleteConfirmation::One(identity) => (
                "Delete Encounter?",
                format!(
                    "Delete local encounter {}? This removes its immutable raw record and cannot be undone.",
                    identity.encounter_id
                ),
                "Confirm Delete Encounter",
            ),
            EncounterDeleteConfirmation::All => (
                "Delete All Encounters?",
                "Delete every local encounter raw record? This cannot be undone.".into(),
                "Delete All Encounters",
            ),
        };
        let mut cancel = false;
        let mut confirm = false;
        let modal =
            egui::Modal::new(egui::Id::new("encounter_delete_confirmation")).show(ctx, |ui| {
                ui.heading(title);
                ui.label(body);
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clickable().clicked() {
                        cancel = true;
                    }
                    if ui.button(confirm_label).clickable().clicked() {
                        confirm = true;
                    }
                });
            });
        if modal.should_close() || cancel {
            self.encounter_delete_confirmation = None;
            return;
        }
        if confirm {
            let result = match confirmation {
                EncounterDeleteConfirmation::One(identity) => self
                    .encounter_history_worker
                    .as_ref()
                    .map(|worker| worker.delete_one(identity)),
                EncounterDeleteConfirmation::All => self
                    .encounter_history_worker
                    .as_ref()
                    .map(EncounterHistoryWorker::delete_all),
            };
            match result {
                Some(Ok(())) => {
                    self.encounter_history_busy = true;
                    self.encounter_history_diagnostic = None;
                    self.encounter_history_message = None;
                }
                Some(Err(_)) => {
                    self.encounter_history_message =
                        Some("Encounter history is already processing another request.".into());
                }
                None => {}
            }
            self.encounter_delete_confirmation = None;
        }
    }

    /// Renders settings as a full-frame modal over a dimmed backdrop. Changes are
    /// submitted to each setting's runtime contract and persisted automatically
    /// (coalesced), with no explicit save.
    /// The modal closes on an outside click, on Escape, or on the close control.
    fn settings_modal(&mut self, ctx: &egui::Context, intents: &mut Vec<UiIntent>) {
        let palette = crate::app::theme::palette(self.ui_prefs.theme);
        let layout = self.model.layout_state();
        let runtime_block_px = self.model.runtime_block_px();
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
                        settings_body(ui, &palette, &mut draft, layout, runtime_block_px);
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

        // Auto-submit: each draft change follows its setting's live or staged
        // runtime contract and is persisted through the coalesced save scheduler.
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

fn history_diagnostic_heading(kind: HistoryDiagnosticKind) -> &'static str {
    match kind {
        HistoryDiagnosticKind::SourceUnavailable => "Capture Unavailable",
        HistoryDiagnosticKind::StoreInvalid => "Store Unavailable",
        HistoryDiagnosticKind::CatalogUnavailable => "Catalog Unavailable",
        HistoryDiagnosticKind::CatalogInvalid => "Catalog Invalid",
        HistoryDiagnosticKind::VersionMismatch => "Catalog Version Mismatch",
        HistoryDiagnosticKind::EncounterMissing => "Encounter Missing",
        HistoryDiagnosticKind::OperationFailed => "Operation Failed",
    }
}

fn capture_status_label(status: crate::encounter::CaptureStatus) -> &'static str {
    match status {
        crate::encounter::CaptureStatus::Complete => "Complete",
        crate::encounter::CaptureStatus::Partial => "Partial",
    }
}

fn render_encounter_projection(ui: &mut egui::Ui, projection: &EncounterProjection) {
    ui.label(format!("Encounter ID: {}", projection.encounter_id));
    ui.label(format!("Session ID: {}", projection.session_id));
    ui.label(format!(
        "Duration: {:.2} seconds",
        projection.duration_ms as f64 / 1000.0
    ));

    ui.strong("Observed Data Quality");
    ui.label(quality_label(projection.observed_dps.quality));
    render_virtual_rows(
        ui,
        "encounter_loss_ranges",
        &projection.observed_dps.loss_ranges,
        |ui, range| {
            ui.label(format!(
                "Sequences {}-{} ({})",
                range.missing_sequence_from, range.missing_sequence_to, range.reason
            ));
        },
    );

    ui.separator();
    ui.heading("Observed Metrics");
    render_metric(ui, "Observed DPS", &projection.observed_dps);
    render_metric(ui, "Observed Effective HPS", &projection.effective_hps);

    ui.strong("Observed Ability Damage Share");
    if projection.ability_damage_share.is_empty() {
        ui.label("No observed outgoing ability damage.");
    } else {
        render_virtual_rows(
            ui,
            "encounter_damage_share",
            &projection.ability_damage_share,
            |ui, share| {
                let view = metric_presentation("Ability Damage Share", &share.result);
                ui.label(format!(
                    "Ability {}: {} | Quality: {}",
                    share.ability_id, view.value, view.quality
                ));
            },
        );
    }

    ui.strong("Observed Effect Uptime");
    if projection.effect_uptime.is_empty() {
        ui.label("No observed effect intervals.");
    } else {
        render_virtual_rows(
            ui,
            "encounter_effect_uptime",
            &projection.effect_uptime,
            |ui, uptime| {
                let view = metric_presentation("Effect Uptime", &uptime.result);
                ui.label(format!(
                    "Effect {}: {} | Quality: {}",
                    uptime.ability_id, view.value, view.quality
                ));
            },
        );
    }

    ui.strong("Observed Cast Order");
    if projection.ordered_cast_sequence.ability_ids.is_empty() {
        ui.label("No observed casts.");
    } else {
        ui.label(format!(
            "{} observed casts in sequence order",
            projection.ordered_cast_sequence.ability_ids.len()
        ));
        render_virtual_rows(
            ui,
            "encounter_cast_order",
            &projection.ordered_cast_sequence.ability_ids,
            |ui, ability_id| {
                ui.label(format!("Ability {ability_id}"));
            },
        );
    }
    ui.label(format!(
        "Quality: {}",
        quality_label(projection.ordered_cast_sequence.quality)
    ));

    ui.separator();
    ui.heading("Projection and Catalog Provenance");
    ui.label(format!("Projection Schema: {}", projection.schema_version));
    ui.label(format!(
        "Algorithm Version: {}",
        projection.algorithm_version
    ));
    ui.label(format!(
        "Catalog Schema: {}",
        projection.catalog_join.catalog_schema_version
    ));
    ui.label(format!(
        "Catalog Version: {}",
        projection.catalog_join.catalog_version
    ));
    ui.label(format!("Channel: {}", projection.catalog_join.channel));
    ui.label(format!(
        "API Version: {}",
        projection.catalog_join.api_version
    ));
    ui.label(format!(
        "Known Numeric IDs: {}",
        projection.catalog_join.known_ids.len()
    ));
    ui.label(format!(
        "Unknown Numeric IDs: {}",
        projection.catalog_join.unknown_ids.len()
    ));
    render_virtual_rows(
        ui,
        "encounter_unknown_ids",
        &projection.catalog_join.unknown_ids,
        |ui, value| {
            ui.label(value.to_string());
        },
    );
    ui.label("Catalog Semantic SHA-256:");
    ui.monospace(&projection.catalog_join.catalog_semantic_sha256);
    ui.label("Raw Content SHA-256:");
    ui.monospace(&projection.raw_content_sha256);
}

fn render_metric(ui: &mut egui::Ui, label: &str, result: &MetricResult) {
    let view = metric_presentation(label, result);
    ui.strong(view.label);
    ui.label(&view.value);
    ui.label(format!(
        "Quality: {} | Source Sequences: {}-{}",
        view.quality, result.first_sequence, result.last_sequence
    ));
}

fn render_virtual_rows<T>(
    ui: &mut egui::Ui,
    id_salt: &'static str,
    values: &[T],
    mut render: impl FnMut(&mut egui::Ui, &T),
) {
    if values.is_empty() {
        ui.label("None");
        return;
    }
    let row_height = ui.text_style_height(&egui::TextStyle::Body);
    egui::ScrollArea::vertical()
        .id_salt(id_salt)
        .max_height(120.0)
        .show_rows(ui, row_height, values.len(), |ui, range| {
            for value in &values[range] {
                render(ui, value);
            }
        });
}

/// Renders the clustered settings body into the modal. Each option carries a
/// human-readable label (no underscore) and a short inline help line.
fn settings_body(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    draft: &mut SettingsForm,
    layout: crate::pixelbus::LayoutState,
    runtime_block_px: u32,
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
            ui.add(egui::DragValue::new(&mut draft.fishing.arm_timeout_ms).range(0..=60_000));
        });
        setting(ui, palette, &strings::SET_REEL_DELAY, |ui| {
            ui.add(egui::DragValue::new(&mut draft.fishing.reel_delay_ms).range(0..=60_000));
        });
        setting(ui, palette, &strings::SET_RECAST_DELAY, |ui| {
            ui.add(egui::DragValue::new(&mut draft.fishing.recast_delay_ms).range(0..=60_000));
        });
        labelled_setting(
            ui,
            palette,
            &strings::SET_FISHING_INTERACT_KEY,
            |ui, label_id| {
                combo(
                    "set_fishing_interact_key",
                    draft.fishing.interact_key.display_name(),
                )
                .show_ui(ui, |ui| {
                    for key in Key::ALL {
                        ui.selectable_value(
                            &mut draft.fishing.interact_key,
                            key,
                            key.display_name(),
                        );
                    }
                })
                .response
                .labelled_by(label_id)
                .clickable();
            },
        );
        widgets::muted_help(ui, palette, strings::FISHING_SETTINGS_APPLICATION_HELP);
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
        // Report one coherent live observation. A drafted size is deployed for
        // the next addon reload and reader restart; combining it with columns
        // negotiated by this running reader would describe neither layout.
        widgets::muted_help(
            ui,
            palette,
            &crate::app::grid_footprint_caption(runtime_block_px, layout),
        );
        setting(ui, palette, &strings::SET_TOLERANCE, |ui| {
            ui.add(egui::DragValue::new(&mut draft.reader.tolerance));
        });
        setting(ui, palette, &strings::SET_INTERVAL_FISHING, |ui| {
            ui.add(egui::DragValue::new(&mut draft.reader.interval_fishing_ms).range(1..=60_000));
        });
        setting(ui, palette, &strings::SET_INTERVAL_IDLE, |ui| {
            ui.add(egui::DragValue::new(&mut draft.reader.interval_idle_ms).range(1..=60_000));
        });
        widgets::muted_help(ui, palette, strings::READER_SETTINGS_APPLICATION_HELP);
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

/// Renders a setting and associates its visible label with the interactive
/// control for assistive technology.
fn labelled_setting(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    s: &strings::Setting,
    add: impl FnOnce(&mut egui::Ui, egui::Id),
) {
    ui.horizontal(|ui| {
        let label = ui.label(s.label).on_hover_text(s.help);
        add(ui, label.id);
    });
    widgets::muted_help(ui, palette, s.help);
}

/// A human-readable, underscore-free label for a bindable action.
fn action_label(action: Action) -> &'static str {
    match action {
        Action::Skill1 => strings::ACTION_SKILL_1,
        Action::Skill2 => strings::ACTION_SKILL_2,
        Action::Skill3 => strings::ACTION_SKILL_3,
        Action::Skill4 => strings::ACTION_SKILL_4,
        Action::Skill5 => strings::ACTION_SKILL_5,
        Action::Ultimate => strings::ACTION_ULTIMATE,
        Action::Synergy => strings::ACTION_SYNERGY,
        Action::ToggleSuspend => strings::ACTION_TOGGLE_SUSPEND,
        Action::ToggleFishing => strings::ACTION_TOGGLE_FISHING,
        Action::ToggleAutoPotion => strings::ACTION_TOGGLE_AUTO_POTION,
    }
}

/// The display name for a game environment.
fn env_name(env: crate::beacon::Environment) -> &'static str {
    match env {
        crate::beacon::Environment::Live => "Live",
        crate::beacon::Environment::Pts => "PTS",
    }
}

fn dashboard_status_row(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    line: &StatusLine,
    interaction_width: f32,
    interaction: impl FnOnce(&mut egui::Ui),
) {
    dashboard_metric_row(
        ui,
        palette,
        dashboard_metric(line.title, &line.state_text, line.role, line.tooltip),
        interaction_width,
        interaction,
    );
}

/// Content rendered by one dashboard metric row.
pub struct DashboardMetric<'a> {
    pub title: &'a str,
    pub state: &'a str,
    pub role: crate::app::StatusRole,
    pub tooltip: &'a str,
}

pub fn dashboard_metric<'a>(
    title: &'a str,
    state: &'a str,
    role: crate::app::StatusRole,
    tooltip: &'a str,
) -> DashboardMetric<'a> {
    DashboardMetric {
        title,
        state,
        role,
        tooltip,
    }
}

/// Exact allocations produced by a dashboard metric row.
#[derive(Debug, Clone, Copy)]
pub struct DashboardRowGeometry {
    pub value: egui::Rect,
    pub interaction: Option<egui::Rect>,
}

/// Renders one stable-height dashboard row and returns its exact allocations.
/// The label has a shared leading allocation, the value consumes all remaining
/// width, and an optional control group starts at one fixed trailing origin.
pub fn dashboard_metric_row(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    metric: DashboardMetric<'_>,
    interaction_width: f32,
    interaction: impl FnOnce(&mut egui::Ui),
) -> DashboardRowGeometry {
    let DashboardMetric {
        title,
        state,
        role,
        tooltip,
    } = metric;
    let row_height = ui.spacing().interact_size.y;
    let color = crate::app::theme::status_color(palette, role);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 12.0;
        let (label_rect, _) = ui.allocate_exact_size(
            egui::vec2(DASHBOARD_LABEL_WIDTH, row_height),
            egui::Sense::hover(),
        );
        let mut label_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(label_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        widgets::label_strong(&mut label_ui, palette, title).on_hover_text(tooltip);

        let interaction_reserve = if interaction_width > 0.0 {
            interaction_width + ui.spacing().item_spacing.x
        } else {
            0.0
        };
        let value_width = (ui.available_width() - interaction_reserve).max(0.0);
        let (value_rect, _) =
            ui.allocate_exact_size(egui::vec2(value_width, row_height), egui::Sense::hover());
        let mut value_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(value_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        value_ui.label(egui::RichText::new("●").color(color));
        let response = value_ui
            .add(
                egui::Label::new(egui::RichText::new(state).color(color))
                    .truncate()
                    .sense(egui::Sense::focusable_noninteractive()),
            )
            .on_hover_text(state)
            .on_hover_text(tooltip);
        if response.has_focus() {
            response.show_tooltip_text(state);
        }

        let interaction_rect = if interaction_width > 0.0 {
            let (interaction_rect, _) = ui.allocate_exact_size(
                egui::vec2(interaction_width, row_height),
                egui::Sense::hover(),
            );
            let mut interaction_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(interaction_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
            );
            interaction(&mut interaction_ui);
            Some(interaction_rect)
        } else {
            None
        };
        DashboardRowGeometry {
            value: value_rect,
            interaction: interaction_rect,
        }
    })
    .inner
}

fn game_context_row(
    ui: &mut egui::Ui,
    palette: &crate::app::theme::Palette,
    view: &crate::app::MenuView,
) {
    dashboard_metric_row(
        ui,
        palette,
        dashboard_metric(
            strings::MENU_TITLE,
            view.state,
            view.role,
            strings::MENU_TOOLTIP,
        ),
        0.0,
        |_| {},
    );
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

fn stage_label(stage: UpdateStage) -> &'static str {
    match stage {
        UpdateStage::Checking => "Checking",
        UpdateStage::LocatingSources => "Locating sources",
        UpdateStage::WaitingForCapture => "Waiting for capture",
        UpdateStage::Validating => "Validating",
        UpdateStage::Normalizing => "Normalizing",
        UpdateStage::Building => "Building",
        UpdateStage::ResolvingIcons => "Resolving icons",
        UpdateStage::IntegrityChecking => "Integrity checking",
        UpdateStage::WaitingForLock => "Waiting for another catalog operation",
        UpdateStage::Installing => "Installing",
        UpdateStage::Opening => "Opening",
        UpdateStage::Committing => "Selecting catalog",
        UpdateStage::RollingBack => "Rolling back",
        UpdateStage::Complete => "Complete",
        UpdateStage::Cancelled => "Cancelled",
        UpdateStage::Failed => "Failed",
    }
}

fn live_state_label(state: LiveUpdateState) -> &'static str {
    match state {
        LiveUpdateState::CatalogCurrent => "Catalog current.",
        LiveUpdateState::NewLiveDataAvailable => {
            "New Live game data is available; no reviewed candidate is installed automatically."
        }
        LiveUpdateState::UpdateReadyToImport => {
            "A verified Live catalog candidate is ready for review."
        }
        LiveUpdateState::CollectorCaptureRequired => {
            "A local collector capture is required before building this update."
        }
        LiveUpdateState::UnsupportedSchema => {
            "An imported candidate uses an unsupported catalog schema."
        }
        LiveUpdateState::OfflineStaleCheck => {
            "The Live version check is offline or stale; the accepted catalog remains active."
        }
        LiveUpdateState::CatalogUnavailable => {
            "No usable active catalog is available; review the bundled catalog or imports."
        }
    }
}
