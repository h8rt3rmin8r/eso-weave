use std::fs;

use eso_weave::catalog::compiler::verify_catalog;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn every_supported_package_carries_the_baseline_catalog() {
    let cargo = read("Cargo.toml");
    let wix = read("wix/main.wxs");
    let release = read(".github/workflows/release.yml");

    assert!(cargo.contains("assets/catalog/catalog.sqlite"));
    assert!(cargo.contains("usr/share/eso-weave/catalog/catalog.sqlite"));
    assert!(wix.contains("assets\\catalog\\catalog.sqlite"));
    assert!(wix.contains("Name=\"catalog.sqlite\""));
    assert!(release.contains("packaging/appimage/AppDir/usr/share/eso-weave/catalog"));
    assert!(release.contains("$dist/catalog/catalog.sqlite"));
}

#[test]
fn baseline_is_verified_and_contains_no_disallowed_payload() {
    let report = verify_catalog("assets/catalog/catalog.sqlite")
        .expect("packaged baseline must be a valid catalog");
    assert_eq!(report.entity_counts.values().sum::<usize>(), 0);
    assert_eq!(report.coverage_counts.get("unknown"), Some(&15));

    let source: serde_json::Value =
        serde_json::from_str(&read("assets/catalog/baseline.json")).unwrap();
    assert!(source["localized_text"].as_array().unwrap().is_empty());
    assert!(source["icon_assets"].as_array().unwrap().is_empty());
    assert!(source["entities"].as_array().unwrap().is_empty());
}

#[test]
fn release_documentation_names_catalog_layout_and_verification() {
    let docs = read("docs/src/development/catalog-compiler.md");
    assert!(docs.contains("catalog/catalog.sqlite"));
    assert!(docs.contains("semantic SHA-256"));
    assert!(docs.contains("rollback"));
    assert!(docs.contains("PTS"));
}
