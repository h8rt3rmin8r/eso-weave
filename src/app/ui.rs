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
    settings_applied: Option<SettingsForm>,
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
            settings_applied: None,
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
                    }
                })
                .response
                .clickable();
            });
            ui.separator();

            self.main_view(ui, &mut intents);
            if self.log_panel_open {
                ui.separator();
                self.log_view(ui, &mut intents);
            }
        });

        if self.settings_open {
            self.settings_modal(&ctx, &mut intents);
        }

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

        let modal = egui::Modal::new(egui::Id::new("eso_weave_settings")).show(ctx, |ui| {
            ui.set_width((screen.width() * 0.9).min(720.0));
            ui.horizontal(|ui| {
                widgets::heading(ui, strings::MENU_SETTINGS);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Close").clickable().clicked() {
                        close = true;
                    }
                });
            });
            ui.separator();
            egui::ScrollArea::vertical()
                .max_height(screen.height() * 0.78)
                .show(ui, |ui| {
                    settings_body(ui, &palette, &mut draft);
                });
        });
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
            egui::ComboBox::from_id_salt("set_theme")
                .selected_text(theme_name(draft.ui.theme))
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
            egui::ComboBox::from_id_salt("set_env")
                .selected_text(env_name(draft.beacon.environment))
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

    widgets::heading(ui, strings::CLUSTER_LOGGING);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        setting(ui, palette, &strings::SET_LOG_LEVEL, |ui| {
            egui::ComboBox::from_id_salt("set_log_level")
                .selected_text(level_name(draft.logging.level))
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
                egui::ComboBox::from_id_salt(("bind", action.as_str()))
                    .selected_text(selected.as_str())
                    .show_ui(ui, |ui| {
                        for key in KEYS {
                            ui.selectable_value(&mut selected, key, key.as_str());
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
    ui.label(egui::RichText::new(line.title).strong())
        .on_hover_text(line.tooltip);
    let color = crate::app::theme::status_color(palette, line.role);
    ui.label(egui::RichText::new(&line.state_text).color(color))
        .on_hover_text(line.tooltip);
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
