//! The binding table: action-to-key mappings with conflict rejection and
//! settings persistence.

use std::collections::BTreeMap;

use crate::config::{Notice, NoticeKind};
use crate::input::action::Action;
use crate::input::key::Key;

/// A rejected rebind because the key is already used by another action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Conflict {
    /// The key that is already in use.
    pub key: Key,
    /// The action that already uses it.
    pub existing: Action,
}

/// The full set of action-to-key bindings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingTable {
    map: BTreeMap<Action, Key>,
}

impl Default for BindingTable {
    fn default() -> Self {
        let map = Action::ALL
            .into_iter()
            .map(|action| (action, action.default_key()))
            .collect();
        BindingTable { map }
    }
}

impl BindingTable {
    /// The key currently bound to `action`.
    pub fn key_for(&self, action: Action) -> Key {
        self.map
            .get(&action)
            .copied()
            .unwrap_or_else(|| action.default_key())
    }

    /// The action bound to `key`, with its suspend-exempt flag, if any.
    pub fn lookup(&self, key: Key) -> Option<(Action, bool)> {
        self.map
            .iter()
            .find(|(_, bound)| **bound == key)
            .map(|(action, _)| (*action, action.suspend_exempt()))
    }

    /// Rebinds `action` to `key`, rejecting the change if another action already
    /// uses that key and leaving the table unchanged in that case.
    pub fn rebind(&mut self, action: Action, key: Key) -> Result<(), Conflict> {
        if let Some(existing) = self
            .map
            .iter()
            .find(|(bound_action, bound_key)| **bound_action != action && **bound_key == key)
            .map(|(bound_action, _)| *bound_action)
        {
            return Err(Conflict { key, existing });
        }
        self.map.insert(action, key);
        Ok(())
    }

    /// Serializes the table to the settings string map (action name to key name).
    pub fn to_settings_map(&self) -> BTreeMap<String, String> {
        self.map
            .iter()
            .map(|(action, key)| (action.as_str().to_string(), key.as_str().to_string()))
            .collect()
    }

    /// Builds a table from the settings string map. Unknown actions or keys and
    /// conflicting entries fall back to defaults for the affected actions, each
    /// reported as a notice.
    pub fn from_settings_map(raw: &BTreeMap<String, String>) -> (BindingTable, Vec<Notice>) {
        let mut notices = Vec::new();
        let mut custom: BTreeMap<Action, Key> = BTreeMap::new();

        for (action_name, key_name) in raw {
            let Some(action) = Action::parse(action_name) else {
                notices.push(Notice {
                    kind: NoticeKind::UnknownKeys,
                    message: format!("ignoring unknown binding action: {action_name}"),
                });
                continue;
            };
            let Some(key) = Key::parse(key_name) else {
                notices.push(Notice {
                    kind: NoticeKind::InvalidValue,
                    message: format!(
                        "binding for {action_name} names unknown key {key_name}; using default"
                    ),
                });
                continue;
            };
            custom.insert(action, key);
        }

        // Remove any customized entries that collide on a key, reverting them to
        // their defaults, until no collision remains. Defaults are unique, so this
        // terminates.
        loop {
            let mut table = BindingTable::default();
            for (action, key) in &custom {
                table.map.insert(*action, *key);
            }

            let mut by_key: BTreeMap<Key, Vec<Action>> = BTreeMap::new();
            for (action, key) in &table.map {
                by_key.entry(*key).or_default().push(*action);
            }

            let colliding: Vec<Action> = by_key
                .into_iter()
                .filter(|(_, actions)| actions.len() > 1)
                .flat_map(|(_, actions)| actions)
                .filter(|action| custom.contains_key(action))
                .collect();

            if colliding.is_empty() {
                return (table, notices);
            }

            for action in colliding {
                custom.remove(&action);
                notices.push(Notice {
                    kind: NoticeKind::InvalidValue,
                    message: format!("conflicting binding for {}; using default", action.as_str()),
                });
            }
        }
    }
}
