//! Automatic data-service ownership, independent requested preferences and root eligibility.
//!
//! Runtime facts never enter settings. The native worker consumes this contract
//! in its separately scoped implementation; reading stored data does not require eligibility.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::beacon::{self, BeaconPrefs, Environment};
use crate::catalog_update::CancellationToken;

/// Application-wide user preferences, independent of selected lifecycle environment.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DataMaintenancePrefs {
    pub catalog_disabled: bool,
    pub native_processing_disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    Absent,
    Unknown,
    Eligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Eligibility {
    pub preferences: DataMaintenancePrefs,
    pub controls_enabled: bool,
    pub catalog: bool,
    pub native_processing: bool,
}

impl Eligibility {
    pub fn new(preferences: DataMaintenancePrefs, presence: Presence) -> Self {
        let controls_enabled = presence == Presence::Eligible;
        Self {
            preferences,
            controls_enabled,
            catalog: controls_enabled && !preferences.catalog_disabled,
            native_processing: controls_enabled && !preferences.native_processing_disabled,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    pub addons_root: PathBuf,
    pub environment: Environment,
}

/// Discover both real roots; selecting Live/PTS only chooses the lifecycle UI target.
pub fn discover_contexts(prefs: &BeaconPrefs) -> Vec<Context> {
    let mut contexts = Vec::new();
    let mut add = |addons_root: PathBuf, environment: Environment| {
        let canonical = std::fs::canonicalize(&addons_root).ok().zip(
            addons_root
                .parent()
                .and_then(|parent| std::fs::canonicalize(parent).ok()),
        );
        if !contexts.iter().any(|context: &Context| {
            context.addons_root == addons_root
                || canonical.as_ref().is_some_and(|canonical| {
                    std::fs::canonicalize(&context.addons_root)
                        .ok()
                        .zip(
                            context
                                .addons_root
                                .parent()
                                .and_then(|parent| std::fs::canonicalize(parent).ok()),
                        )
                        .as_ref()
                        == Some(canonical)
                })
        }) {
            contexts.push(Context {
                addons_root,
                environment,
            });
        }
    };
    if let Some(root) = &prefs.path_override {
        let actual = root
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str());
        let environment = match actual {
            Some(name) if name.eq_ignore_ascii_case("pts") => Environment::Pts,
            Some(name) if name.eq_ignore_ascii_case("live") => Environment::Live,
            _ => prefs.environment,
        };
        add(root.clone(), environment);
        if actual.is_some_and(|name| {
            name.eq_ignore_ascii_case("live") || name.eq_ignore_ascii_case("pts")
        }) {
            if let Some(base) = root.parent().and_then(|p| p.parent()) {
                for (name, environment) in [("live", Environment::Live), ("pts", Environment::Pts)]
                {
                    let sibling = base.join(name).join("AddOns");
                    if sibling.is_dir() {
                        add(sibling, environment);
                    }
                }
            }
        }
    }
    for environment in [Environment::Live, Environment::Pts] {
        let automatic = BeaconPrefs {
            path_override: None,
            environment,
        };
        if let Ok(root) = beacon::resolve_addons_dir(&automatic) {
            add(root, environment);
        }
    }
    contexts.truncate(16);
    contexts
}

struct ControlState {
    preferences: DataMaintenancePrefs,
    persisted: bool,
    catalog: CancellationToken,
    native: CancellationToken,
    roots: BeaconPrefs,
    controls_enabled: bool,
    startup_config_persisted: Option<bool>,
    jobs: Vec<(PathBuf, CancellationToken)>,
}

/// Shared immediate cancellation boundary, independent of UI frame/worker queues.
#[derive(Clone)]
pub struct MaintenanceControl(Arc<Mutex<ControlState>>);

impl MaintenanceControl {
    pub fn new(preferences: DataMaintenancePrefs) -> Self {
        let catalog = CancellationToken::new();
        catalog.cancel();
        let native = CancellationToken::new();
        native.cancel();
        Self(Arc::new(Mutex::new(ControlState {
            preferences,
            persisted: true,
            catalog,
            native,
            roots: BeaconPrefs::default(),
            controls_enabled: false,
            startup_config_persisted: None,
            jobs: Vec::new(),
        })))
    }

    /// Apply requested stop even on save failure. Failed enable retains the
    /// previous stop instead of claiming an unsaved restart succeeded.
    pub fn apply(
        &self,
        mut preferences: DataMaintenancePrefs,
        persisted: bool,
    ) -> DataMaintenancePrefs {
        let mut state = self.0.lock().unwrap_or_else(|e| e.into_inner());
        if !persisted {
            preferences.catalog_disabled |= state.preferences.catalog_disabled;
            preferences.native_processing_disabled |= state.preferences.native_processing_disabled;
        }
        if preferences.catalog_disabled != state.preferences.catalog_disabled {
            state.startup_config_persisted = state.controls_enabled.then_some(false);
            for (_, token) in &state.jobs {
                token.cancel();
            }
            state.catalog.cancel();
            state.catalog = CancellationToken::new();
        }
        if preferences.native_processing_disabled != state.preferences.native_processing_disabled {
            state.native.cancel();
            state.native = CancellationToken::new();
        }
        if !state.controls_enabled || preferences.catalog_disabled {
            state.catalog.cancel();
        }
        if !state.controls_enabled || preferences.native_processing_disabled {
            state.native.cancel();
        }
        state.preferences = preferences;
        state.persisted = persisted;
        preferences
    }

    pub fn preferences(&self) -> DataMaintenancePrefs {
        self.0.lock().unwrap_or_else(|e| e.into_inner()).preferences
    }

    pub fn persisted(&self) -> bool {
        self.0.lock().unwrap_or_else(|e| e.into_inner()).persisted
    }

    pub fn catalog_token(&self) -> CancellationToken {
        self.0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .catalog
            .clone()
    }

    pub fn native_token(&self) -> CancellationToken {
        self.0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .native
            .clone()
    }

    pub fn set_roots(&self, roots: BeaconPrefs) {
        let mut state = self.0.lock().unwrap_or_else(|e| e.into_inner());
        let custom_environment_changed = roots.environment != state.roots.environment
            && roots
                .path_override
                .as_ref()
                .and_then(|root| root.parent())
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    !name.eq_ignore_ascii_case("live") && !name.eq_ignore_ascii_case("pts")
                });
        if state.roots.path_override != roots.path_override || custom_environment_changed {
            for (_, token) in &state.jobs {
                token.cancel();
            }
            state.catalog.cancel();
            state.native.cancel();
            state.catalog = CancellationToken::new();
            state.native = CancellationToken::new();
            state.startup_config_persisted = None;
        }
        if !state.controls_enabled || state.preferences.catalog_disabled {
            state.catalog.cancel();
        }
        if !state.controls_enabled || state.preferences.native_processing_disabled {
            state.native.cancel();
        }
        state.roots = roots;
    }

    pub fn roots(&self) -> BeaconPrefs {
        self.0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .roots
            .clone()
    }

    pub fn controls_enabled(&self) -> bool {
        self.0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .controls_enabled
    }

    pub fn set_presence(&self, available: bool) {
        let mut state = self.0.lock().unwrap_or_else(|e| e.into_inner());
        if !available {
            for (_, token) in &state.jobs {
                token.cancel();
            }
            state.startup_config_persisted = None;
        }
        if available != state.controls_enabled {
            state.catalog.cancel();
            state.native.cancel();
            state.catalog = CancellationToken::new();
            state.native = CancellationToken::new();
        }
        state.controls_enabled = available;
        if !available || state.preferences.catalog_disabled {
            state.catalog.cancel();
        }
        if !available || state.preferences.native_processing_disabled {
            state.native.cancel();
        }
    }

    /// Whether every currently eligible root has the requested literal on disk.
    /// This says nothing about an addon already loaded in the game process.
    pub fn startup_config_persisted(&self) -> Option<bool> {
        self.0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .startup_config_persisted
    }

    pub(crate) fn report_startup_config_persisted(&self, status: Option<bool>) {
        self.0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .startup_config_persisted = status;
    }

    /// One token per actual source root; an absent PTS root cannot cancel Live.
    pub fn register_catalog_job(&self, root: &std::path::Path) -> CancellationToken {
        let mut state = self.0.lock().unwrap_or_else(|e| e.into_inner());
        state.jobs.retain(|(_, token)| !token.is_cancelled());
        if let Some((_, token)) = state.jobs.iter().find(|(path, _)| path == root) {
            return token.clone();
        }
        let token = CancellationToken::new();
        if state.preferences.catalog_disabled || !state.controls_enabled {
            token.cancel();
        }
        state.jobs.push((root.to_path_buf(), token.clone()));
        token
    }

    pub fn cancel_catalog_root(&self, root: &std::path::Path) {
        let state = self.0.lock().unwrap_or_else(|e| e.into_inner());
        for (path, token) in &state.jobs {
            if path == root {
                token.cancel();
            }
        }
    }

    pub fn cancel_all_jobs(&self) {
        let state = self.0.lock().unwrap_or_else(|e| e.into_inner());
        for (_, token) in &state.jobs {
            token.cancel();
        }
        state.catalog.cancel();
        state.native.cancel();
    }
}

mod runtime;
pub use runtime::MaintenanceRuntime;
