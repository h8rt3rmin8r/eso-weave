//! egui rendering for the main window (thin; validated manually).
//!
//! This layer only reads the [`AppModel`] view and raises intents. It carries no
//! correctness-bearing logic, so it is excluded from the unit-tested surface and
//! validated with the manual checklist in the feature quickstart.
//!
//! Rendering uses a single central panel with inline sections (menu row, main
//! content, an optional log section, and an inline settings view), keeping to a
//! small, stable set of egui widgets.

use std::time::Duration;

use eframe::egui;

use crate::app::log_view::build_log_view;
use crate::app::settings_form::{SettingsForm, UiPrefs};
use crate::app::{AppModel, SkillEdit, UiIntent};
use crate::config::{LevelName, Theme};
use crate::input::{Action, Key};
use crate::weave::WeaveType;

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
        }
    }

    fn apply_prefs(&mut self, ctx: &egui::Context) {
        if self.applied_prefs == Some(self.ui_prefs) {
            return;
        }
        let visuals = match self.ui_prefs.theme {
            Theme::Dark => egui::Visuals::dark(),
            Theme::Light => egui::Visuals::light(),
        };
        ctx.set_visuals(visuals);
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
            // Menu row.
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Settings").clicked() {
                        self.settings_open = true;
                        self.settings_draft = Some(self.model.settings_form());
                    }
                    if ui.button("Exit").clicked() {
                        exit = true;
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.checkbox(&mut self.log_panel_open, "Live Log").changed() {
                        intents.push(UiIntent::ToggleLogPanel(self.log_panel_open));
                    }
                });
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
        if exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}

impl EsoWeaveApp {
    fn main_view(&mut self, ui: &mut egui::Ui, intents: &mut Vec<UiIntent>) {
        let view = self.model.view();

        if self.confirm_uninstall {
            ui.horizontal(|ui| {
                ui.label("Remove the PixelBeacon addon?");
                if ui.button("Uninstall").clicked() {
                    intents.push(UiIntent::UninstallBeacon);
                    self.confirm_uninstall = false;
                }
                if ui.button("Cancel").clicked() {
                    self.confirm_uninstall = false;
                }
            });
            ui.separator();
        }

        // Status region.
        ui.horizontal(|ui| {
            ui.label(format!("State: {}", view.app_state.indicator));
            if ui.button(view.app_state.button).clicked() {
                intents.push(UiIntent::ToggleSuspend);
            }
        });
        ui.horizontal(|ui| {
            ui.label(format!("Fishing: {}", view.fishing.indicator));
            if ui.button(view.fishing.button).clicked() {
                intents.push(UiIntent::SetFishing(view.fishing.button == "Go Fish"));
            }
        });
        ui.horizontal(|ui| {
            let dot = if view.beacon.green {
                egui::Color32::from_rgb(0x40, 0xC0, 0x40)
            } else {
                egui::Color32::from_rgb(0xC0, 0x40, 0x40)
            };
            ui.colored_label(dot, "\u{25CF}")
                .on_hover_text(view.beacon.tooltip);
            ui.label("PixelBeacon");
            if ui.button("Install").clicked() {
                intents.push(UiIntent::InstallBeacon);
            }
            if ui
                .add_enabled(view.uninstall_enabled, egui::Button::new("Uninstall"))
                .clicked()
            {
                self.confirm_uninstall = true;
            }
        });

        ui.separator();
        ui.label("Skills");
        for row in &view.skills {
            ui.horizontal(|ui| {
                ui.label(&row.label);

                let mut active = row.active;
                if ui.checkbox(&mut active, "active").changed() {
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
                    });
                if weave_type != row.weave_type {
                    intents.push(UiIntent::EditSkill(
                        row.index,
                        SkillEdit::WeaveType(weave_type),
                    ));
                }

                let mut has_override = row.override_d_weave.is_some();
                if ui.checkbox(&mut has_override, "override").changed() {
                    let value = if has_override { Some(50) } else { None };
                    intents.push(UiIntent::EditSkill(
                        row.index,
                        SkillEdit::OverrideDWeave(value),
                    ));
                }
                if let Some(current) = row.override_d_weave {
                    let mut value = current;
                    if ui.add(egui::DragValue::new(&mut value)).changed() {
                        intents.push(UiIntent::EditSkill(
                            row.index,
                            SkillEdit::OverrideDWeave(Some(value)),
                        ));
                    }
                }
            });
        }
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
                });
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
            if ui.button("Apply").clicked() {
                apply = true;
            }
            if ui.button("Close").clicked() {
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
                });
            ui.checkbox(&mut draft.ui.always_on_top, "Always on top");

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
            ui.checkbox(&mut draft.latency.enabled, "Enabled");
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
                });
            ui.checkbox(&mut draft.logging.file_enabled, "File logging");

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
                    });
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
