//! Brand theme for the egui GUI (presentation only).
//!
//! Maps the "Arcane gold on ink" brand tokens (see
//! `docs/brand/ESO-Weave-Brand-v1.md`) to egui visuals and style for the dark
//! (default) and light modes, and installs the bundled Inter font. This layer
//! carries no correctness-bearing logic; it only styles the view.

use eframe::egui::{self, Color32, Stroke};

use crate::config::Theme;

const fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

/// The resolved brand color roles for a theme. Status and accent colors are
/// sourced from here rather than hard-coded at the call sites.
pub struct Palette {
    /// Whether this is the dark theme.
    pub dark: bool,
    /// Window and base surface.
    pub base: Color32,
    /// Panels and control fills.
    pub panel: Color32,
    /// Hover and active fills.
    pub elevated: Color32,
    /// Borders and separators.
    pub stroke: Color32,
    /// Primary action color.
    pub gold: Color32,
    /// Gold hover and active.
    pub gold_hover: Color32,
    /// Text drawn on a filled gold surface.
    pub gold_text: Color32,
    /// Supporting accent.
    pub teal: Color32,
    /// Primary text.
    pub text: Color32,
    /// Secondary text.
    pub muted: Color32,
    /// Status: running and healthy.
    pub ok: Color32,
    /// Status: warning.
    pub warn: Color32,
    /// Status: error and signal lost.
    pub err: Color32,
}

/// Returns the brand palette for a theme.
pub fn palette(theme: Theme) -> Palette {
    match theme {
        Theme::Dark => Palette {
            dark: true,
            base: rgb(0x0E, 0x11, 0x16),
            panel: rgb(0x15, 0x1B, 0x23),
            elevated: rgb(0x1C, 0x25, 0x30),
            stroke: rgb(0x2A, 0x33, 0x40),
            gold: rgb(0xF2, 0xB0, 0x3C),
            gold_hover: rgb(0xFB, 0xCB, 0x6B),
            gold_text: rgb(0x24, 0x17, 0x04),
            teal: rgb(0x2D, 0xD4, 0xBF),
            text: rgb(0xE6, 0xED, 0xF3),
            muted: rgb(0x8B, 0x97, 0xA7),
            ok: rgb(0x34, 0xD3, 0x99),
            warn: rgb(0xFB, 0x9E, 0x3C),
            err: rgb(0xF8, 0x71, 0x71),
        },
        Theme::Light => Palette {
            dark: false,
            base: rgb(0xF7, 0xF5, 0xF0),
            panel: rgb(0xFF, 0xFF, 0xFF),
            elevated: rgb(0xEC, 0xE8, 0xDE),
            stroke: rgb(0xDC, 0xD9, 0xD0),
            gold: rgb(0xE7, 0xA4, 0x2C),
            gold_hover: rgb(0xC6, 0x87, 0x1F),
            gold_text: rgb(0x24, 0x17, 0x04),
            teal: rgb(0x0D, 0x94, 0x88),
            text: rgb(0x14, 0x11, 0x0B),
            muted: rgb(0x6B, 0x64, 0x55),
            ok: rgb(0x05, 0x96, 0x69),
            warn: rgb(0xB4, 0x53, 0x09),
            err: rgb(0xDC, 0x26, 0x26),
        },
    }
}

/// Applies the brand visuals and spacing for a theme to the egui context.
pub fn apply(ctx: &egui::Context, theme: Theme) {
    let p = palette(theme);
    let mut v = if p.dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    v.panel_fill = p.base;
    v.window_fill = p.panel;
    v.faint_bg_color = p.elevated;
    v.extreme_bg_color = if p.dark {
        rgb(0x0A, 0x0D, 0x11)
    } else {
        p.panel
    };
    v.hyperlink_color = p.teal;
    v.warn_fg_color = p.warn;
    v.error_fg_color = p.err;
    v.window_stroke = Stroke::new(1.0, p.stroke);
    v.selection.bg_fill = p.gold.gamma_multiply(0.35);
    v.selection.stroke = Stroke::new(1.0, p.gold);

    let radius = egui::CornerRadius::same(6);

    let n = &mut v.widgets.noninteractive;
    n.bg_fill = p.panel;
    n.weak_bg_fill = p.panel;
    n.bg_stroke = Stroke::new(1.0, p.stroke);
    n.fg_stroke = Stroke::new(1.0, p.text);
    n.corner_radius = radius;

    let i = &mut v.widgets.inactive;
    i.bg_fill = p.elevated;
    i.weak_bg_fill = p.elevated;
    i.bg_stroke = Stroke::new(1.0, p.stroke);
    i.fg_stroke = Stroke::new(1.0, p.text);
    i.corner_radius = radius;

    let h = &mut v.widgets.hovered;
    h.bg_fill = p.elevated;
    h.weak_bg_fill = p.elevated;
    h.bg_stroke = Stroke::new(1.2, p.gold);
    h.fg_stroke = Stroke::new(1.0, p.text);
    h.corner_radius = radius;

    let a = &mut v.widgets.active;
    a.bg_fill = p.gold;
    a.weak_bg_fill = p.gold;
    a.bg_stroke = Stroke::new(1.0, p.gold_hover);
    a.fg_stroke = Stroke::new(1.0, p.gold_text);
    a.corner_radius = radius;

    let o = &mut v.widgets.open;
    o.bg_fill = p.elevated;
    o.weak_bg_fill = p.elevated;
    o.bg_stroke = Stroke::new(1.0, p.gold);
    o.fg_stroke = Stroke::new(1.0, p.text);
    o.corner_radius = radius;

    ctx.set_visuals(v);

    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(10.0, 5.0);
        style.spacing.interact_size.y = 22.0;
    });
}

/// Installs the bundled Inter font as the proportional family, keeping the
/// framework default fonts as glyph fallback. Call once at startup.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Inter".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/brand/fonts/Inter-Regular.ttf"))
            .into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Inter".to_owned());
    ctx.set_fonts(fonts);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Relative luminance per WCAG, used to check legibility of the palette.
    fn luminance(c: Color32) -> f32 {
        let channel = |x: u8| {
            let s = x as f32 / 255.0;
            if s <= 0.03928 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(c.r()) + 0.7152 * channel(c.g()) + 0.0722 * channel(c.b())
    }

    fn contrast(a: Color32, b: Color32) -> f32 {
        let (la, lb) = (luminance(a), luminance(b));
        let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
        (hi + 0.05) / (lo + 0.05)
    }

    #[test]
    fn dark_flag_matches_theme() {
        assert!(palette(Theme::Dark).dark);
        assert!(!palette(Theme::Light).dark);
    }

    #[test]
    fn palette_is_legible_in_both_themes() {
        for theme in [Theme::Dark, Theme::Light] {
            let p = palette(theme);
            assert!(
                contrast(p.text, p.base) >= 7.0,
                "primary text on base is not legible for {theme:?}"
            );
            assert!(
                contrast(p.gold_text, p.gold) >= 4.0,
                "text on a gold button is not legible for {theme:?}"
            );
        }
    }
}
