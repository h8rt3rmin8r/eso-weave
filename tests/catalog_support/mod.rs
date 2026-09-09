#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use tempfile::TempDir;

pub const LIVE_FIXTURE: &str = "specs/070-catalog-compiler/fixtures/minimal-live.json";
pub const CHANGED_LIVE_FIXTURE: &str =
    "specs/070-catalog-compiler/fixtures/minimal-live-changed.json";
pub const PTS_FIXTURE: &str = "specs/070-catalog-compiler/fixtures/minimal-pts.json";

pub struct CatalogSandbox {
    pub root: TempDir,
}

impl CatalogSandbox {
    pub fn new() -> Self {
        Self {
            root: tempfile::tempdir().expect("catalog sandbox"),
        }
    }

    pub fn path(&self, relative: &str) -> PathBuf {
        self.root.path().join(relative)
    }

    pub fn mutated_fixture(&self, name: &str, edit: impl FnOnce(&mut Value)) -> PathBuf {
        let mut value: Value = serde_json::from_slice(
            &fs::read(LIVE_FIXTURE).expect("read live fixture for mutation"),
        )
        .expect("parse live fixture for mutation");
        edit(&mut value);
        let path = self.path(name);
        fs::write(
            &path,
            serde_json::to_vec_pretty(&value).expect("serialize mutated fixture"),
        )
        .expect("write mutated fixture");
        path
    }
}

pub fn bytes(path: impl AsRef<Path>) -> Vec<u8> {
    fs::read(path).expect("read catalog bytes")
}
