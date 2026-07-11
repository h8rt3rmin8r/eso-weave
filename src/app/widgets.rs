//! Reusable presentation helpers for the egui layer: a colorized toggle switch,
//! section headings, small inline help text, and a transient save toast.
//!
//! These carry no correctness-bearing logic; they only render. Behavior (the
//! boolean values, the save timing) lives in the view-model.

use std::time::{Duration, Instant};

use eframe::egui;

use crate::app::theme::Palette;

/// A colorized physical toggle switch. Renders a pill track (gold when on, muted
/// when off) with a sliding knob. Returns the response so the caller can detect
/// changes and attach a hover tooltip.
pub fn toggle_switch(ui: &mut egui::Ui, on: &mut bool, palette: &Palette) -> egui::Response {
    let height = ui.spacing().interact_size.y;
    let desired_size = egui::vec2(1.9 * height, height);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
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

/// Renders a section heading (SemiBold, larger) from the heading text style.
pub fn heading(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add(egui::Label::new(egui::RichText::new(text).heading()))
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

    /// Paints the toast anchored to the bottom-right, fading out near the end.
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
                egui::Frame::popup(ui.style())
                    .fill(palette.elevated)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(&self.message).color(palette.text));
                    });
            });
    }
}
