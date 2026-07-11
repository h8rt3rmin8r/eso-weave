//! PixelBeacon status-light derivation (pure).

use crate::beacon::BeaconStatus;

/// The condition the status light reflects, derived from the Beacon Manager
/// status and the AddOns discovery result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeaconCondition {
    /// Installed and current.
    InstalledCurrent,
    /// Installed but outdated or unmanaged (a folder is present but not current).
    InstalledOutdated,
    /// Not installed.
    NotInstalled,
    /// The AddOns directory could not be resolved.
    AddonsNotFound,
}

impl BeaconCondition {
    /// Derives the condition from a Beacon Manager status.
    pub fn from_status(status: BeaconStatus) -> Self {
        match status {
            BeaconStatus::ManagedUpToDate => BeaconCondition::InstalledCurrent,
            BeaconStatus::ManagedVersionMismatch | BeaconStatus::Unmanaged => {
                BeaconCondition::InstalledOutdated
            }
            BeaconStatus::NotInstalled => BeaconCondition::NotInstalled,
        }
    }
}

/// The derived status light: a color and an exact-condition tooltip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeaconLight {
    /// True renders green (installed and current); false renders red.
    pub green: bool,
    /// The exact condition text for the tooltip.
    pub tooltip: &'static str,
}

/// Maps a condition to the status light.
pub fn beacon_light(condition: BeaconCondition) -> BeaconLight {
    match condition {
        BeaconCondition::InstalledCurrent => BeaconLight {
            green: true,
            tooltip: "installed and current",
        },
        BeaconCondition::InstalledOutdated => BeaconLight {
            green: false,
            tooltip: "installed but outdated",
        },
        BeaconCondition::NotInstalled => BeaconLight {
            green: false,
            tooltip: "not installed",
        },
        BeaconCondition::AddonsNotFound => BeaconLight {
            green: false,
            tooltip: "AddOns directory not found",
        },
    }
}

/// Whether the Uninstall control is enabled: only when a folder is present to
/// remove (installed conditions).
pub fn uninstall_enabled(condition: BeaconCondition) -> bool {
    matches!(
        condition,
        BeaconCondition::InstalledCurrent | BeaconCondition::InstalledOutdated
    )
}
