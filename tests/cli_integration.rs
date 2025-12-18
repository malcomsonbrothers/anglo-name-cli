use std::collections::HashSet;

use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)] // cargo_bin is the standard approach for assert_cmd tests
fn cmd() -> Command {
    Command::cargo_bin("anglo-name-cli").unwrap()
}

#[test]
fn test_generates_single_name() {
    cmd()
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_count_flag() {
    cmd().args(["-n", "5"]).assert().success();
}

#[test]
fn test_generates_correct_count() {
    let output = cmd().args(["-n", "5"]).output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_male_gender() {
    cmd().args(["-n", "10", "-g", "male"]).assert().success();
}

#[test]
fn test_female_gender() {
    cmd().args(["-n", "10", "-g", "female"]).assert().success();
}

#[test]
fn test_with_middle_name() {
    let output = cmd().args(["-n", "1", "-m"]).output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    // With middle name, there should be 3 parts (first middle last)
    let parts: Vec<&str> = stdout.split_whitespace().collect();
    assert_eq!(parts.len(), 3, "Expected 3 name parts, got: {}", stdout);
}

#[test]
fn test_json_output() {
    cmd()
        .args(["-f", "json", "-n", "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("["));
}

#[test]
fn test_json_contains_fields() {
    cmd()
        .args(["-f", "json", "-n", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("first_name"))
        .stdout(predicate::str::contains("last_name"))
        .stdout(predicate::str::contains("full_name"));
}

#[test]
fn test_csv_output() {
    cmd()
        .args(["-f", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "first_name,middle_name,last_name,full_name",
        ));
}

#[test]
fn test_csv_has_correct_rows() {
    let output = cmd().args(["-f", "csv", "-n", "3"]).output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Filter out empty lines (trailing newline creates one)
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), 4, "Expected header + 3 data rows");
}

#[test]
fn test_file_output() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("names.json");

    cmd()
        .args(["-n", "3", "-f", "json", "-o", file_path.to_str().unwrap()])
        .assert()
        .success();

    assert!(file_path.exists());

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("first_name"));
}

#[test]
fn test_unique_names_by_default() {
    let output = cmd().args(["-n", "100"]).output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let names: Vec<&str> = stdout.lines().collect();
    let unique: HashSet<&str> = names.iter().copied().collect();

    assert_eq!(names.len(), unique.len(), "All names should be unique");
}

#[test]
fn test_error_when_requesting_too_many_unique() {
    // Request more unique names than possible (male only, no middle = 741 * 500 = 370,500)
    cmd()
        .args(["-n", "400000", "-g", "male"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot generate"));
}

#[test]
fn test_allow_duplicates_flag() {
    cmd()
        .args(["-n", "10", "--allow-duplicates"])
        .assert()
        .success();
}

#[test]
fn test_help_flag() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate random Anglo names"));
}

#[test]
fn test_version_flag() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("anglo-name"));
}
