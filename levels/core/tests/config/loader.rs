use std::path::Path;

use levels_core::config::{ConfigLoader, LevelsConfig, MAX_COLUMN_SURFACES, MAX_RADIUS};
use tempfile::TempDir;

fn write_config(text: &str) -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config.json");
    std::fs::write(&path, text).unwrap();
    (dir, path)
}

#[test]
fn missing_file_gives_defaults() {
    let config = ConfigLoader::load(Path::new("definitely/not/here/config.json"));

    assert_eq!(config.keys.toggle, 0x4C);
    assert!(config.keys.ctrl);
    assert_eq!(config.memory_budget_mb, 32);
    assert_eq!(config.scan.radius, 24);
    assert_eq!(config.scan.pass_seconds, 0.25);
    assert_eq!(config.scan.max_labels, 8192);
    assert_eq!(config.light.red_max, 0);
    assert_eq!(config.light.yellow_max, 7);
    assert!(!config.telemetry);
}

#[test]
fn broken_json_gives_defaults() {
    let (_dir, path) = write_config("{not json");

    assert_eq!(ConfigLoader::load(&path), LevelsConfig::default());
}

#[test]
fn partial_json_keeps_other_defaults() {
    let (_dir, path) = write_config(r#"{"scan":{"radius":16}}"#);

    let config = ConfigLoader::load(&path);

    assert_eq!(config.scan.radius, 16);
    assert_eq!(config.scan.max_labels, 8192);
}

#[test]
fn huge_scan_limits_are_capped() {
    let mut huge = LevelsConfig::default();
    huge.scan.max_labels = u32::MAX;
    huge.scan.max_columns_per_frame = u32::MAX;

    let capped = huge.sanitized();

    let side = 2 * MAX_RADIUS + 1;
    assert_eq!(capped.scan.max_labels, side * side * MAX_COLUMN_SURFACES as u32);
    assert_eq!(capped.scan.max_columns_per_frame, side * side);
}

#[test]
fn sanitized_fixes_bad_ranges() {
    let mut low = LevelsConfig::default();
    low.scan.radius = 0;
    low.scan.pass_seconds = 0.0;
    low.scan.max_labels = 0;
    low.scan.max_columns_per_frame = 0;
    low.light.red_max = 5;
    low.light.yellow_max = 3;
    low.label.red = [2.0, -1.0, 0.5, 1.0];
    low.label.scale = 0.0;
    low.label.lift = 0.06;
    low.memory_budget_mb = 0;
    let low = low.sanitized();

    assert_eq!(low.scan.radius, 1);
    assert_eq!(low.scan.pass_seconds, 0.25);
    assert_eq!(low.scan.max_labels, 1);
    assert_eq!(low.scan.max_columns_per_frame, 1);
    assert_eq!(low.light.red_max, 5);
    assert_eq!(low.light.yellow_max, 5);
    assert_eq!(low.label.red, [1.0, 0.0, 0.5, 1.0]);
    assert_eq!(low.label.scale, LevelsConfig::default().label.scale);
    assert_eq!(low.label.lift, LevelsConfig::default().label.lift);
    assert_eq!(low.memory_budget_mb, 1);

    let mut high = LevelsConfig::default();
    high.scan.radius = 100;
    high.light.red_max = 40;
    high.light.yellow_max = 40;
    let high = high.sanitized();

    assert_eq!(high.scan.radius, MAX_RADIUS);
    assert_eq!(high.light.red_max, 15);
    assert_eq!(high.light.yellow_max, 15);
}
