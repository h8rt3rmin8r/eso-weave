//! Reusable presentation helpers for the egui layer: a colorized toggle switch,
//! section headings, small inline help text, and a transient save toast.
//!
//! These carry no correctness-bearing logic; they only render. Behavior (the
//! boolean values, the save timing) lives in the view-model.

use std::time::{Duration, Instant};

use eframe::egui;

use crate::app::theme::Palette;
use crate::app::{
    ResourcePresentation, ResourceTheme, ResourceView, UltimatePresentation, UltimateView,
};

const RESOURCE_LABEL_WIDTH: f32 = 70.0;
const RESOURCE_STATE_WIDTH: f32 = 118.0;
const RESOURCE_READY_WIDTH: f32 = 48.0;
const RESOURCE_MIN_TRACK_WIDTH: f32 = 96.0;
const RESOURCE_GAP: f32 = 8.0;

/// Stable geometry shared by ordinary and Ultimate resource meters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResourceMeterGeometry {
    pub row: egui::Rect,
    pub label: egui::Rect,
    pub track: egui::Rect,
    pub numeric: egui::Rect,
    pub ready: egui::Rect,
    pub quarters: [[egui::Pos2; 2]; 3],
    pub threshold: Option<[egui::Pos2; 2]>,
}

/// Computes meter adornments without consulting dynamic readiness.
pub fn resource_meter_geometry(
    rect: egui::Rect,
    threshold_fraction: Option<f32>,
) -> ResourceMeterGeometry {
    let state_left = rect.right() - RESOURCE_STATE_WIDTH;
    let label = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2(rect.left() + RESOURCE_LABEL_WIDTH, rect.bottom()),
    );
    let track = egui::Rect::from_min_max(
        egui::pos2(
            rect.left() + RESOURCE_LABEL_WIDTH + RESOURCE_GAP,
            rect.top() + 3.0,
        ),
        egui::pos2(state_left - RESOURCE_GAP, rect.bottom() - 3.0),
    );
    let ready = egui::Rect::from_min_max(
        egui::pos2(rect.right() - RESOURCE_READY_WIDTH, rect.top()),
        rect.right_bottom(),
    );
    let numeric = egui::Rect::from_min_max(
        egui::pos2(state_left, rect.top()),
        egui::pos2(ready.left(), rect.bottom()),
    );
    let quarter = |fraction: f32| {
        let x = track.left() + track.width() * fraction;
        [
            egui::pos2(x, track.top() + 2.0),
            egui::pos2(x, track.bottom() - 2.0),
        ]
    };
    let threshold = threshold_fraction.map(|fraction| {
        let x = track.left() + track.width() * fraction.clamp(0.0, 1.0);
        [
            egui::pos2(x, track.bottom() - 4.0),
            egui::pos2(x, rect.bottom()),
        ]
    });
    ResourceMeterGeometry {
        row: rect,
        label,
        track,
        numeric,
        ready,
        quarters: [quarter(0.25), quarter(0.5), quarter(0.75)],
        threshold,
    }
}

fn meter_size(ui: &egui::Ui) -> egui::Vec2 {
    let height = ui.spacing().interact_size.y.max(20.0);
    let width = ui.available_width().max(
        RESOURCE_LABEL_WIDTH + RESOURCE_STATE_WIDTH + RESOURCE_MIN_TRACK_WIDTH + 2.0 * RESOURCE_GAP,
    );
    egui::vec2(width, height)
}

fn paint_meter_track(
    ui: &egui::Ui,
    palette: &Palette,
    geometry: ResourceMeterGeometry,
    fill: egui::Color32,
    fraction: Option<f32>,
    stroke: egui::Stroke,
) {
    let radius = egui::CornerRadius::same(4);
    ui.painter()
        .rect_filled(geometry.track, radius, palette.panel);
    if let Some(fraction) = fraction {
        let filled = egui::Rect::from_min_max(
            geometry.track.min,
            egui::pos2(
                geometry.track.left() + geometry.track.width() * fraction.clamp(0.0, 1.0),
                geometry.track.bottom(),
            ),
        );
        if filled.width() > 0.0 {
            ui.painter().rect_filled(filled, radius, fill);
        }
    }
    for segment in geometry.quarters {
        ui.painter()
            .line_segment(segment, egui::Stroke::new(1.0, palette.stroke));
    }
    ui.painter()
        .rect_stroke(geometry.track, radius, stroke, egui::StrokeKind::Inside);
    if let Some(segment) = geometry.threshold {
        ui.painter()
            .line_segment(segment, egui::Stroke::new(2.0, palette.text));
    }
}

/// Renders one stable, accessible resource meter.
///
/// The resource name and exact value or state sit outside the fill, so text
/// contrast never depends on how much of the bar is filled. The complete row is
/// one progress indicator for assistive technology; non-numeric states omit the
/// numeric value instead of presenting an invented zero.
pub fn resource_meter(
    ui: &mut egui::Ui,
    palette: &Palette,
    name: &str,
    view: &ResourceView,
    theme: ResourceTheme,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(meter_size(ui), egui::Sense::hover());

    response.widget_info(|| {
        let mut info = egui::WidgetInfo::labeled(
            egui::WidgetType::ProgressIndicator,
            ui.is_enabled(),
            format!("{name}: {}", view.text),
        );
        info.value = view.percent().map(f64::from);
        info
    });

    if ui.is_rect_visible(rect) {
        let fill = match theme {
            ResourceTheme::Health => palette.health,
            ResourceTheme::Stamina => palette.stamina,
            ResourceTheme::Magicka => palette.magicka,
            ResourceTheme::Ultimate => palette.ultimate,
        };
        let geometry = resource_meter_geometry(rect, None);
        let stroke = match view.presentation {
            ResourcePresentation::Low(_) | ResourcePresentation::Unavailable => {
                egui::Stroke::new(2.0, palette.warn)
            }
            ResourcePresentation::Observed(_) | ResourcePresentation::Dormant => {
                egui::Stroke::new(1.0, palette.muted)
            }
        };
        paint_meter_track(ui, palette, geometry, fill, view.fraction(), stroke);
        let font = egui::FontId::proportional(ui.style().text_styles[&egui::TextStyle::Body].size);
        ui.painter().text(
            rect.left_center(),
            egui::Align2::LEFT_CENTER,
            name,
            font.clone(),
            palette.text,
        );
        ui.painter().text(
            geometry.numeric.left_center(),
            egui::Align2::LEFT_CENTER,
            &view.text,
            font,
            palette.text,
        );
    }

    response.on_hover_text(crate::app::strings::RESOURCE_TOOLTIP)
}

/// Renders the exact Ultimate value, active-bar threshold, and fixed Ready slot.
pub fn ultimate_meter(
    ui: &mut egui::Ui,
    palette: &Palette,
    name: &str,
    view: &UltimateView,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(meter_size(ui), egui::Sense::hover());
    response.widget_info(|| {
        let mut info = egui::WidgetInfo::labeled(
            egui::WidgetType::ProgressIndicator,
            ui.is_enabled(),
            &view.accessibility,
        );
        info.value = view
            .current
            .zip(view.maximum)
            .map(|(current, maximum)| f64::from(current) / f64::from(maximum) * 100.0);
        info
    });
    if ui.is_rect_visible(rect) {
        let geometry = resource_meter_geometry(rect, view.threshold_fraction());
        let stroke = match view.presentation {
            UltimatePresentation::Unavailable => egui::Stroke::new(2.0, palette.warn),
            _ => egui::Stroke::new(1.0, palette.muted),
        };
        paint_meter_track(
            ui,
            palette,
            geometry,
            palette.ultimate,
            view.fraction(),
            stroke,
        );
        let font = egui::FontId::proportional(ui.style().text_styles[&egui::TextStyle::Body].size);
        ui.painter().text(
            rect.left_center(),
            egui::Align2::LEFT_CENTER,
            name,
            font.clone(),
            palette.text,
        );
        ui.painter().text(
            geometry.numeric.left_center(),
            egui::Align2::LEFT_CENTER,
            &view.text,
            font.clone(),
            palette.text,
        );
        if view.ready == Some(true) {
            ui.painter().text(
                geometry.ready.left_center(),
                egui::Align2::LEFT_CENTER,
                "Ready",
                font,
                palette.ready,
            );
        }
    }
    response.on_hover_text(crate::app::strings::ULTIMATE_TOOLTIP)
}

/// A colorized physical toggle switch. Renders a pill track (gold when on, muted
/// when off) with a sliding knob. Returns the response so the caller can detect
/// changes and attach a hover tooltip.
pub fn toggle_switch(ui: &mut egui::Ui, on: &mut bool, palette: &Palette) -> egui::Response {
    toggle_switch_named(ui, on, palette, "")
}

/// A toggle switch with a meaningful accessibility name.
pub fn toggle_switch_named(
    ui: &mut egui::Ui,
    on: &mut bool,
    palette: &Palette,
    label: &str,
) -> egui::Response {
    let height = ui.spacing().interact_size.y;
    let desired_size = egui::vec2(1.9 * height, height);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, label)
    });

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let radius = 0.5 * rect.height();
        let track = if *on { palette.gold } else { palette.elevated };
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(radius as u8), track);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        let knob = if *on {
            palette.gold_text
        } else {
            palette.muted
        };
        ui.painter().circle_filled(center, 0.72 * radius, knob);
    }
    response
}

/// Renders a complete resource group followed by its single boundary gap.
///
/// The gap belongs to the group rather than today's final resource, so adding a
/// fourth descriptor cannot leave spacing stranded after Magicka.
pub fn resource_group(
    ui: &mut egui::Ui,
    palette: &Palette,
    resources: &[(&str, &crate::app::ResourceView, crate::app::ResourceTheme)],
    trailing_gap: f32,
) {
    for &(title, resource, theme) in resources {
        resource_meter(ui, palette, title, resource, theme);
    }
    ui.add_space(trailing_gap);
}

/// Renders a section heading (SemiBold, larger) from the heading text style.
pub fn heading(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add(egui::Label::new(egui::RichText::new(text).heading()))
}

/// Renders an emphasized label (SemiBold at body size, primary text color) for
/// section titles and column headers.
///
/// This intentionally avoids egui's `RichText::strong()`: its color comes from
/// `strong_text_color()`, which is `widgets.active.text_color()`, and the brand
/// theme sets that to `gold_text` (the dark ink used for text on a gold button).
/// On the dark base that reads as an almost invisible brown, so emphasis labels
/// are colored from the palette here instead.
pub fn label_strong(ui: &mut egui::Ui, palette: &Palette, text: &str) -> egui::Response {
    let size = ui.style().text_styles[&egui::TextStyle::Body].size;
    let font = egui::FontId::new(
        size,
        egui::FontFamily::Name(super::theme::HEADING_FAMILY.into()),
    );
    ui.add(egui::Label::new(
        egui::RichText::new(text).font(font).color(palette.text),
    ))
}

/// Renders a small muted inline help line beneath a control.
pub fn muted_help(ui: &mut egui::Ui, palette: &Palette, text: &str) {
    ui.label(egui::RichText::new(text).small().color(palette.muted));
}

/// A transient bottom-right notification that fades out and auto-dismisses.
pub struct Toast {
    message: String,
    shown_at: Instant,
    ttl: Duration,
}

impl Toast {
    /// Creates a toast shown at `now`, living for a short interval.
    pub fn new(message: impl Into<String>, now: Instant) -> Self {
        Self {
            message: message.into(),
            shown_at: now,
            ttl: Duration::from_millis(2200),
        }
    }

    /// Whether the toast has outlived its interval and should be dropped.
    pub fn expired(&self, now: Instant) -> bool {
        now.duration_since(self.shown_at) >= self.ttl
    }

    /// Paints the toast anchored to the bottom-right, fading out near the end. It
    /// is a success confirmation, so it fills with the brand success (green) color
    /// and draws its text in the contrasting base color at a heavier weight, which
    /// stays legible in both light and dark themes.
    pub fn show(&self, ctx: &egui::Context, palette: &Palette, now: Instant) {
        let elapsed = now.duration_since(self.shown_at).as_secs_f32();
        let total = self.ttl.as_secs_f32();
        let fade = 0.4;
        let alpha = if elapsed > total - fade {
            ((total - elapsed) / fade).clamp(0.0, 1.0)
        } else {
            1.0
        };
        egui::Area::new(egui::Id::new("eso_weave_save_toast"))
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-16.0, -16.0))
            .order(egui::Order::Foreground)
            .interactable(false)
            .show(ctx, |ui| {
                ui.set_opacity(alpha);
                let font = egui::FontId::new(
                    ui.style().text_styles[&egui::TextStyle::Body].size,
                    egui::FontFamily::Name(super::theme::HEADING_FAMILY.into()),
                );
                egui::Frame::popup(ui.style())
                    .fill(palette.ok)
                    .stroke(egui::Stroke::new(1.0, palette.ok))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&self.message)
                                .font(font)
                                .color(palette.base),
                        );
                    });
            });
    }
}
