//! Shared ESO game version parsing and comparison.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::Channel;

/// Complete identity shared by catalog requests and review manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogVersionTuple {
    pub channel: Channel,
    pub game_version: String,
    pub api_version: u32,
    pub catalog_version: String,
    pub catalog_schema: u32,
    pub locales: Vec<String>,
    pub tool_version: String,
}

/// A parsed, comparable ESO game client version held as four numeric components.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct GameVersion([u16; 4]);

impl GameVersion {
    pub const fn new(parts: [u16; 4]) -> Self {
        Self(parts)
    }
}

impl fmt::Display for GameVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut end = 3;
        while end > 1 && self.0[end] == 0 {
            end -= 1;
        }
        let parts = self.0[..=end]
            .iter()
            .map(u16::to_string)
            .collect::<Vec<_>>();
        formatter.write_str(&parts.join("."))
    }
}

/// Parses the leading dotted numeric token of a source revision message.
pub fn parse_commit_message_version(message: &str) -> Option<GameVersion> {
    let token = message.split_whitespace().next()?;
    let mut parts = [0_u16; 4];
    let mut count = 0;
    for component in token.split('.') {
        if count >= parts.len() {
            break;
        }
        parts[count] = component.parse::<u16>().ok()?;
        count += 1;
    }
    (count > 0).then_some(GameVersion(parts))
}
