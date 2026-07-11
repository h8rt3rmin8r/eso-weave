//! egui rendering for the main window (thin; validated manually).
//!
//! This layer only reads the [`AppModel`] view and raises intents. It carries no
//! correctness-bearing logic, so it is excluded from the unit-tested surface and
//! validated with the manual checklist in the feature quickstart.
//!
//! Rendering uses a single central panel with inline sections (menu row, main
//! content, an optional log section, and an inline settings view), keeping to a
//! small, stable set of egui widgets.

use std::time::{Duration, Instant};

use eframe::egui;

use crate::app::log_view::build_log_view;
use crate::app::settings_form::{SettingsForm, UiPrefs};
use crate::app::{override_edit_for, strings, widgets, AppModel, SkillEdit, StatusLine, UiIntent};
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

const KEYS: [Key; 11] = [
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
];

const LEVELS: [LevelName; 6] = [
    LevelName::Off,
    LevelName::Error,
    LevelName::Warn,
    LevelName::Info,
    LevelName::Debug,
    LevelName::Trace,
];

/// The eframe application: renders the main window from the [`AppModel`].
pub struct EsoWeaveApp {
    model: AppModel,
    ui_prefs: UiPrefs,
    applied_prefs: Option<UiPrefs>,
    log_panel_open: bool,
    settings_open: bool,
    settings_draft: Option<SettingsForm>,
    confirm_uninstall: bool,
    toast: Option<widgets::Toast>,
}

impl EsoWeaveApp {
    /// Creates the app over the view-model.
    pub fn new(model: AppModel) -> Self {
        let ui_prefs = model.ui_prefs();
        Self {
            model,
            ui_prefs,
            applied_prefs: None,
            log_panel_open: false,
            settings_open: false,
            settings_draft: None,
            confirm_uninstall: false,
            toast: None,
        }
    }

    fn apply_prefs(&mut self, ctx: &egui::Context) {
        if self.applied_prefs == Some(self.ui_prefs) {
            return;
        }
        crate::app::theme::apply(ctx, self.ui_prefs.theme);
        let level = if self.ui_prefs.always_on_top {
            egui::WindowLevel::AlwaysOnTop
        } else {
            egui::WindowLevel::Normal
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(level));
        self.applied_prefs = Some(self.ui_prefs);
    }
}

impl eframe::App for EsoWeaveApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.apply_prefs(&ctx);

        let mut intents: Vec<UiIntent> = Vec::new();
        let mut exit = false;

        egui::CentralPanel::default().show(ui, |ui| {
            // Menu bar.
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button(strings::MENU_FILE, |ui| {
                    if ui
                        .button(strings::MENU_SETTINGS)
                        .on_hover_text(strings::MENU_SETTINGS_TOOLTIP)
                        .clickable()
                        .clicked()
                    {
                        self.settings_open = true;
                        self.settings_draft = Some(self.model.settings_form());
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
                    }
                })
                .response
                .clickable();
            });
            ui.separator();

            if self.settings_open {
                self.settings_view(ui, &mut intents);
            } else {
                self.main_view(ui, &mut intents);
                if self.log_panel_open {
                    ui.separator();
                    self.log_view(ui, &mut intents);
                }
            }
        });

        for intent in intents {
            self.model.apply_intent(intent);
        }

        // Coalesced auto-save: flush any settled changes and confirm with a toast.
        let now = Instant::now();
        if self.model.maybe_flush(now) {
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

        if exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}

impl EsoWeaveApp {
    fn main_view(&mut self, ui: &mut egui::Ui, intents: &mut Vec<UiIntent>) {
        let view = self.model.view();

        let palette = crate::app::theme::palette(self.ui_prefs.theme);

        if self.confirm_uninstall {
            ui.horizontal(|ui| {
                ui.label("Remove the PixelBeacon addon?");
                if ui.button("Uninstall").clickable().clicked() {
                    intents.push(UiIntent::UninstallBeacon);
                    self.confirm_uninstall = false;
                }
                if ui.button("Cancel").clickable().clicked() {
                    self.confirm_uninstall = false;
                }
            });
            ui.separator();
        }

        // Status region: title, colorized state, and control, aligned in a grid
        // that spans the same width as the Skills grid below.
        egui::Grid::new("status")
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
                        .add_enabled(view.uninstall_enabled, egui::Button::new("Uninstall"))
                        .on_hover_text(strings::BEACON_UNINSTALL_TOOLTIP)
                        .clickable()
                        .clicked()
                    {
                        self.confirm_uninstall = true;
                    }
                });
                ui.end_row();
            });

        ui.separator();
        widgets::heading(ui, strings::SKILLS_TITLE);
        // A single grid so the label, enabled toggle, weave selector, override
        // toggle, and delay align in labeled columns across every row. When a row
        // has no override, the Delay cell shows the inherited default (muted) so a
        // row never displays a meaningless zero.
        egui::Grid::new("skills")
            .num_columns(5)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                for (header, tip) in strings::SKILL_COLUMNS {
                    ui.label(egui::RichText::new(header).strong())
                        .on_hover_text(tip);
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
                    egui::ComboBox::from_id_salt(("weave", row.index))
                        .selected_text(weave_type_name(weave_type))
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

                    if row.is_override {
                        let mut value = row.effective_delay;
                        if ui
                            .add(egui::DragValue::new(&mut value))
                            .on_hover_text(strings::SKILL_COLUMNS[4].1)
                            .changed()
                        {
                            intents.push(UiIntent::EditSkill(
                                row.index,
                                override_edit_for(row.weave_type, Some(value)),
                            ));
                        }
                    } else {
                        ui.add(egui::Label::new(
                            egui::RichText::new(row.effective_delay.to_string())
                                .color(palette.muted),
                        ))
                        .on_hover_text(strings::SKILL_COLUMNS[4].1);
                    }
                    ui.end_row();
                }
            });
    }

    fn log_view(&mut self, ui: &mut egui::Ui, intents: &mut Vec<UiIntent>) {
        let filter = self.model.view().log_filter;
        ui.horizontal(|ui| {
            ui.label("Live Log");
            let mut selected = filter;
            egui::ComboBox::from_id_salt("log_filter")
                .selected_text(level_name(selected))
                .show_ui(ui, |ui| {
                    for level in LEVELS {
                        ui.selectable_value(&mut selected, level, level_name(level));
                    }
                })
                .response
                .clickable();
            if selected != filter {
                intents.push(UiIntent::SetLogFilter(selected));
            }
        });
        let events = self.model.log_handle().recent(1000);
        let rows = build_log_view(&events, filter);
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for row in rows {
                    let color = egui::Color32::from_rgb(row.color.r, row.color.g, row.color.b);
                    ui.colored_label(color, row.text);
                }
            });
    }

    fn settings_view(&mut self, ui: &mut egui::Ui, intents: &mut Vec<UiIntent>) {
        let Some(draft) = self.settings_draft.as_mut() else {
            self.settings_open = false;
            return;
        };
        let mut apply = false;
        let mut close = false;
        ui.horizontal(|ui| {
            ui.heading("Settings");
            if ui.button("Apply").clickable().clicked() {
                apply = true;
            }
            if ui.button("Close").clickable().clicked() {
                close = true;
            }
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Theme and window");
            egui::ComboBox::from_label("Theme")
                .selected_text(theme_name(draft.ui.theme))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut draft.ui.theme, Theme::Dark, "Dark");
                    ui.selectable_value(&mut draft.ui.theme, Theme::Light, "Light");
                })
                .response
                .clickable();
            ui.checkbox(&mut draft.ui.always_on_top, "Always on top")
                .clickable();

            ui.separator();
            ui.heading("Global timing (ms)");
            timing_row(
                ui,
                "global_cooldown",
                &mut draft.weave.timing.global_cooldown,
            );
            timing_row(ui, "d_weave", &mut draft.weave.timing.d_weave);
            timing_row(ui, "d_heavy", &mut draft.weave.timing.d_heavy);
            timing_row(ui, "d_bash", &mut draft.weave.timing.d_bash);

            ui.separator();
            ui.heading("Latency adaptation");
            ui.checkbox(&mut draft.latency.enabled, "Enabled")
                .clickable();
            ui.horizontal(|ui| {
                ui.label("k");
                ui.add(egui::DragValue::new(&mut draft.latency.k).speed(0.05));
            });

            ui.separator();
            ui.heading("Fishing (ms)");
            timing_row(ui, "arm_timeout_ms", &mut draft.fishing.arm_timeout_ms);
            timing_row(ui, "reel_delay_ms", &mut draft.fishing.reel_delay_ms);
            timing_row(ui, "recast_delay_ms", &mut draft.fishing.recast_delay_ms);

            ui.separator();
            ui.heading("Pixel bus");
            u8_row(ui, "tolerance", &mut draft.reader.tolerance);
            u64_row(
                ui,
                "interval_fishing_ms",
                &mut draft.reader.interval_fishing_ms,
            );
            u64_row(ui, "interval_idle_ms", &mut draft.reader.interval_idle_ms);

            ui.separator();
            ui.heading("Logging");
            egui::ComboBox::from_label("Log level")
                .selected_text(level_name(draft.logging.level))
                .show_ui(ui, |ui| {
                    for level in LEVELS {
                        ui.selectable_value(&mut draft.logging.level, level, level_name(level));
                    }
                })
                .response
                .clickable();
            ui.checkbox(&mut draft.logging.file_enabled, "File logging")
                .clickable();

            ui.separator();
            ui.heading("Keybindings");
            for action in Action::ALL {
                let current = draft.bindings.key_for(action);
                let mut selected = current;
                egui::ComboBox::from_label(action.as_str())
                    .selected_text(selected.as_str())
                    .show_ui(ui, |ui| {
                        for key in KEYS {
                            ui.selectable_value(&mut selected, key, key.as_str());
                        }
                    })
                    .response
                    .clickable();
                if selected != current {
                    let _ = draft.bindings.rebind(action, selected);
                }
            }
        });

        if apply {
            intents.push(UiIntent::ApplySettings(Box::new(draft.clone())));
            self.ui_prefs = draft.ui;
            close = true;
        }
        if close {
            self.settings_open = false;
            self.settings_draft = None;
        }
    }
}

/// Renders the first two cells of a status grid row: the section title, then the
/// colorized, normalized state field. The caller adds the third (control) cell.
fn status_cells(ui: &mut egui::Ui, palette: &crate::app::theme::Palette, line: &StatusLine) {
    ui.label(egui::RichText::new(line.title).strong())
        .on_hover_text(line.tooltip);
    let color = crate::app::theme::status_color(palette, line.role);
    ui.label(egui::RichText::new(&line.state_text).color(color))
        .on_hover_text(line.tooltip);
}

fn timing_row(ui: &mut egui::Ui, label: &str, value: &mut u32) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value));
    });
}

fn u8_row(ui: &mut egui::Ui, label: &str, value: &mut u8) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value));
    });
}

fn u64_row(ui: &mut egui::Ui, label: &str, value: &mut u64) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value));
    });
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
