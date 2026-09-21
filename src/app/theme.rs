//! Brand theme for the egui GUI (presentation only).
//!
//! Maps the pinned BrandBuilder 2.0 semantic contract (see
//! `docs/src/development/brand-standard.md`) to egui visuals and style for the dark
//! (default) and light modes, and installs the bundled Inter and Geist Mono fonts.
//! This layer carries no correctness-bearing logic; it only styles the view.

use eframe::egui::{self, Color32, Stroke};

use crate::app::StatusRole;
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
    pub background: Color32,
    /// Panels and control fills.
    pub card: Color32,
    /// Popovers and temporary overlays.
    pub overlay: Color32,
    /// Secondary control surface.
    pub secondary: Color32,
    /// Hover surface.
    pub hover: Color32,
    /// Borders and separators.
    pub border: Color32,
    /// Primary action color.
    pub primary: Color32,
    /// Text drawn on a filled primary surface.
    pub on_primary: Color32,
    /// Active-state emphasis and focus.
    pub emphasis: Color32,
    /// Primary text.
    pub text: Color32,
    /// Secondary text.
    pub muted: Color32,
    /// Status: running and healthy.
    pub ok: Color32,
    /// Accessible green used for compact Ready text.
    pub ready: Color32,
    /// Status: warning.
    pub warn: Color32,
    /// Status: error and signal lost.
    pub destructive: Color32,
    /// Text drawn on a destructive fill.
    pub on_destructive: Color32,
    /// Health meter fill.
    pub health: Color32,
    /// Stamina meter fill.
    pub stamina: Color32,
    /// Magicka meter fill.
    pub magicka: Color32,
    /// Ultimate meter fill.
    pub ultimate: Color32,
}

/// Returns the brand palette for a theme.
pub fn palette(theme: Theme) -> Palette {
    match theme {
        Theme::Dark => Palette {
            dark: true,
            background: rgb(0x0E, 0x11, 0x16),
            card: rgb(0x17, 0x1C, 0x24),
            overlay: rgb(0x0A, 0x0D, 0x12),
            secondary: rgb(0x1D, 0x24, 0x30),
            hover: rgb(0x25, 0x2E, 0x3B),
            border: rgb(0x26, 0x26, 0x26),
            primary: rgb(0x2D, 0xD4, 0xBF),
            on_primary: Color32::BLACK,
            emphasis: rgb(0x2D, 0xD4, 0xBF),
            text: Color32::WHITE,
            muted: rgb(0x9A, 0x9A, 0x9A),
            ok: rgb(0x34, 0xD3, 0x99),
            ready: rgb(0x34, 0xD3, 0x99),
            warn: rgb(0xF2, 0xB0, 0x3C),
            destructive: rgb(0xE9, 0x50, 0x5F),
            on_destructive: Color32::BLACK,
            health: rgb(0xE9, 0x50, 0x5F),
            stamina: rgb(0x34, 0xD3, 0x99),
            magicka: rgb(0x60, 0xA5, 0xFA),
            ultimate: rgb(0xA7, 0x8B, 0xFA),
        },
        Theme::Light => Palette {
            dark: false,
            background: rgb(0xF8, 0xF8, 0xF6),
            card: Color32::WHITE,
            overlay: Color32::WHITE,
            secondary: rgb(0xF0, 0xEF, 0xED),
            hover: rgb(0xF0, 0xEF, 0xED),
            border: rgb(0xE5, 0xE5, 0xE5),
            primary: rgb(0x98, 0x60, 0x00),
            on_primary: Color32::WHITE,
            emphasis: rgb(0x98, 0x60, 0x00),
            text: rgb(0x0A, 0x0A, 0x0A),
            muted: rgb(0x6B, 0x6B, 0x6B),
            ok: rgb(0x05, 0x96, 0x69),
            ready: rgb(0x04, 0x78, 0x57),
            warn: rgb(0x98, 0x60, 0x00),
            destructive: rgb(0xC0, 0x29, 0x3A),
            on_destructive: Color32::WHITE,
            health: rgb(0xC0, 0x29, 0x3A),
            stamina: rgb(0x04, 0x78, 0x57),
            magicka: rgb(0x1D, 0x4E, 0xD8),
            ultimate: rgb(0x6D, 0x28, 0xD9),
        },
    }
}

/// Applies the brand visuals and spacing for a theme to the egui context.
pub fn apply(ctx: &egui::Context, theme: Theme) {
    let p = palette(theme);
    ctx.set_theme(if p.dark {
        egui::ThemePreference::Dark
    } else {
        egui::ThemePreference::Light
    });
    let mut v = if p.dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    v.panel_fill = p.background;
    v.window_fill = p.card;
    v.faint_bg_color = p.secondary;
    v.extreme_bg_color = p.overlay;
    v.hyperlink_color = p.emphasis;
    v.warn_fg_color = p.warn;
    v.error_fg_color = p.destructive;
    v.window_corner_radius = egui::CornerRadius::same(12);
    v.window_stroke = Stroke::new(1.0, p.border);
    v.selection.bg_fill = p.primary.gamma_multiply(0.35);
    v.selection.stroke = Stroke::new(2.0, p.emphasis);

    let radius = egui::CornerRadius::same(8);

    // Every state keeps the same size-affecting inputs (zero interaction
    // expansion and a 1.0 border stroke width, which feeds the widget inner
    // margin), so hovering a control changes only its color, never its size, and
    // the layout never reflows on hover. Only appearance (fill and stroke color)
    // differs between states.
    let n = &mut v.widgets.noninteractive;
    n.bg_fill = p.card;
    n.weak_bg_fill = p.card;
    n.bg_stroke = Stroke::new(1.0, p.border);
    n.fg_stroke = Stroke::new(1.0, p.text);
    n.corner_radius = radius;
    n.expansion = 0.0;

    let i = &mut v.widgets.inactive;
    i.bg_fill = p.secondary;
    i.weak_bg_fill = p.secondary;
    i.bg_stroke = Stroke::new(1.0, p.border);
    i.fg_stroke = Stroke::new(1.0, p.text);
    i.corner_radius = radius;
    i.expansion = 0.0;

    let h = &mut v.widgets.hovered;
    h.bg_fill = p.hover;
    h.weak_bg_fill = p.hover;
    h.bg_stroke = Stroke::new(1.0, p.emphasis);
    h.fg_stroke = Stroke::new(1.0, p.text);
    h.corner_radius = radius;
    h.expansion = 0.0;

    let a = &mut v.widgets.active;
    a.bg_fill = p.primary;
    a.weak_bg_fill = p.primary;
    a.bg_stroke = Stroke::new(2.0, p.emphasis);
    a.fg_stroke = Stroke::new(1.0, p.on_primary);
    a.corner_radius = radius;
    a.expansion = 0.0;

    let o = &mut v.widgets.open;
    o.bg_fill = p.hover;
    o.weak_bg_fill = p.hover;
    o.bg_stroke = Stroke::new(2.0, p.emphasis);
    o.fg_stroke = Stroke::new(1.0, p.text);
    o.corner_radius = radius;
    o.expansion = 0.0;

    ctx.set_visuals(v);

    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(12.0, 12.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.interact_size = egui::vec2(44.0, 44.0);
        // Section headings use the bundled SemiBold weight at a larger size, so
        // they read as headings rather than bold body text.
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(17.0, egui::FontFamily::Name(HEADING_FAMILY.into())),
        );
    });
}

/// The named font family used for section headings (Inter SemiBold).
pub const HEADING_FAMILY: &str = "InterSemiBold";
/// The named font family used for identifiers and technical metadata.
pub const MONO_FAMILY: &str = "GeistMono";

/// Installs the bundled Inter font as the proportional family, keeping the
/// framework default fonts as glyph fallback, and registers the Medium and
/// SemiBold weights as named families for headings and emphasis. Call once at
/// startup.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Inter".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/brand/fonts/Inter-Regular.ttf"))
            .into(),
    );
    fonts.font_data.insert(
        "InterMedium".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/brand/fonts/Inter-Medium.ttf"))
            .into(),
    );
    fonts.font_data.insert(
        HEADING_FAMILY.to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/brand/fonts/Inter-SemiBold.ttf"
        ))
        .into(),
    );
    fonts.font_data.insert(
        MONO_FAMILY.to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/brand/fonts/GeistMono-Regular.ttf"
        ))
        .into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Inter".to_owned());
    // Named families keep the regular Inter as glyph fallback behind the weight.
    fonts.families.insert(
        egui::FontFamily::Name(HEADING_FAMILY.into()),
        vec![HEADING_FAMILY.to_owned(), "Inter".to_owned()],
    );
    fonts.families.insert(
        egui::FontFamily::Name("InterMedium".into()),
        vec!["InterMedium".to_owned(), "Inter".to_owned()],
    );
    fonts.families.insert(
        egui::FontFamily::Monospace,
        vec![
            MONO_FAMILY.to_owned(),
            "Inter".to_owned(),
            "Hack".to_owned(),
        ],
    );
    fonts.families.insert(
        egui::FontFamily::Name(MONO_FAMILY.into()),
        vec![
            MONO_FAMILY.to_owned(),
            "Inter".to_owned(),
            "Hack".to_owned(),
        ],
    );
    ctx.set_fonts(fonts);
}

/// The palette color for a status role, so status fields are colorized from one
/// place rather than at each call site.
pub fn status_color(p: &Palette, role: StatusRole) -> Color32 {
    match role {
        StatusRole::Healthy => p.ok,
        StatusRole::Warning => p.warn,
        StatusRole::Active => p.emphasis,
        StatusRole::Muted => p.muted,
        StatusRole::Error => p.destructive,
    }
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
    fn palette_matches_brandbuilder_2_contract() {
        let dark = palette(Theme::Dark);
        assert_eq!(dark.background, rgb(0x0E, 0x11, 0x16));
        assert_eq!(dark.card, rgb(0x17, 0x1C, 0x24));
        assert_eq!(dark.overlay, rgb(0x0A, 0x0D, 0x12));
        assert_eq!(dark.secondary, rgb(0x1D, 0x24, 0x30));
        assert_eq!(dark.hover, rgb(0x25, 0x2E, 0x3B));
        assert_eq!(dark.border, rgb(0x26, 0x26, 0x26));
        assert_eq!(dark.primary, rgb(0x2D, 0xD4, 0xBF));
        assert_eq!(dark.on_primary, Color32::BLACK);
        assert_eq!(dark.emphasis, rgb(0x2D, 0xD4, 0xBF));
        assert_eq!(dark.text, Color32::WHITE);
        assert_eq!(dark.muted, rgb(0x9A, 0x9A, 0x9A));
        assert_eq!(dark.destructive, rgb(0xE9, 0x50, 0x5F));
        assert_eq!(dark.on_destructive, Color32::BLACK);

        let light = palette(Theme::Light);
        assert_eq!(light.background, rgb(0xF8, 0xF8, 0xF6));
        assert_eq!(light.card, Color32::WHITE);
        assert_eq!(light.overlay, Color32::WHITE);
        assert_eq!(light.secondary, rgb(0xF0, 0xEF, 0xED));
        assert_eq!(light.hover, rgb(0xF0, 0xEF, 0xED));
        assert_eq!(light.border, rgb(0xE5, 0xE5, 0xE5));
        assert_eq!(light.primary, rgb(0x98, 0x60, 0x00));
        assert_eq!(light.on_primary, Color32::WHITE);
        assert_eq!(light.emphasis, rgb(0x98, 0x60, 0x00));
        assert_eq!(light.text, rgb(0x0A, 0x0A, 0x0A));
        assert_eq!(light.muted, rgb(0x6B, 0x6B, 0x6B));
        assert_eq!(light.destructive, rgb(0xC0, 0x29, 0x3A));
        assert_eq!(light.on_destructive, Color32::WHITE);
    }

    #[test]
    fn applied_theme_enforces_governed_interaction_target_and_focus_width() {
        for theme in [Theme::Dark, Theme::Light] {
            let ctx = egui::Context::default();
            apply(&ctx, theme);
            let egui_theme = if matches!(theme, Theme::Dark) {
                egui::Theme::Dark
            } else {
                egui::Theme::Light
            };
            let style = ctx.style_of(egui_theme);
            assert!(style.spacing.interact_size.x >= 44.0);
            assert!(style.spacing.interact_size.y >= 44.0);
            assert!(style.visuals.selection.stroke.width >= 2.0);
            assert_eq!(
                style.visuals.window_corner_radius,
                egui::CornerRadius::same(12)
            );
        }
    }

    #[test]
    fn palette_is_legible_in_both_themes() {
        for theme in [Theme::Dark, Theme::Light] {
            let p = palette(theme);
            assert!(
                contrast(p.text, p.background) >= 7.0,
                "primary text on base is not legible for {theme:?}"
            );
            assert!(
                contrast(p.text, p.card) >= 4.5,
                "resource text on panel is not legible for {theme:?}"
            );
            assert!(
                contrast(p.on_primary, p.primary) >= 4.5,
                "text on a primary button is not legible for {theme:?}"
            );
            for (name, fill) in [
                ("health", p.health),
                ("stamina", p.stamina),
                ("magicka", p.magicka),
                ("ultimate", p.ultimate),
            ] {
                assert!(
                    contrast(fill, p.card) >= 3.0,
                    "{name} fill does not meet non-text contrast for {theme:?}"
                );
            }
            assert!(
                contrast(p.muted, p.card) >= 3.0,
                "meter boundary does not meet non-text contrast for {theme:?}"
            );
            assert!(
                contrast(p.ready, p.card) >= 4.5,
                "Ready text does not meet normal-text contrast for {theme:?}"
            );
        }
    }
}
