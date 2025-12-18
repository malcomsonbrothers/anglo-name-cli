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
fn test_male_sex() {
    cmd().args(["-n", "10", "-s", "male"]).assert().success();
}

#[test]
fn test_female_sex() {
    cmd().args(["-n", "10", "-s", "female"]).assert().success();
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
        .args(["-n", "400000", "-s", "male"])
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

// Email and password tests

#[test]
fn test_email_flag() {
    cmd()
        .args(["--email"])
        .assert()
        .success()
        .stdout(predicate::str::contains("@"));
}

#[test]
fn test_email_with_custom_domain() {
    cmd()
        .args(["--email", "--domain", "example.com"])
        .assert()
        .success()
        .stdout(predicate::str::contains("@example.com"));
}

#[test]
fn test_password_flag() {
    let output = cmd().args(["--password"]).output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Output should be name | password (32 chars)
    assert!(stdout.contains("|"), "Expected pipe separator in output");
    // Password should be 32 chars after the pipe
    let parts: Vec<&str> = stdout.trim().split(" | ").collect();
    assert_eq!(parts.len(), 2, "Expected name and password separated by |");
    assert_eq!(parts[1].len(), 32, "Password should be 32 characters");
}

#[test]
fn test_email_and_password_together() {
    cmd()
        .args(["--email", "--password"])
        .assert()
        .success()
        .stdout(predicate::str::contains("@"))
        .stdout(predicate::str::contains("|"));
}

#[test]
fn test_domain_requires_email() {
    cmd().args(["--domain", "example.com"]).assert().failure();
}

#[test]
fn test_json_with_email() {
    cmd()
        .args(["-f", "json", "--email"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"email\":"));
}

#[test]
fn test_json_with_password() {
    cmd()
        .args(["-f", "json", "--password"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"password\":"));
}

#[test]
fn test_csv_with_email_and_password() {
    cmd()
        .args(["-f", "csv", "--email", "--password"])
        .assert()
        .success()
        .stdout(predicate::str::contains("email"))
        .stdout(predicate::str::contains("password"));
}

#[test]
fn test_email_strips_apostrophes() {
    // Generate many names to increase chance of hitting an O'Brien-like name
    // Instead, we can check that no emails contain apostrophes
    let output = cmd()
        .args(["-n", "50", "--email", "--domain", "test.com"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("@") {
            // Extract email part
            let parts: Vec<&str> = line.split(" | ").collect();
            if parts.len() >= 2 {
                let email = parts[1];
                assert!(
                    !email.contains('\''),
                    "Email should not contain apostrophes: {}",
                    email
                );
                assert!(
                    !email.contains('-'),
                    "Email should not contain hyphens: {}",
                    email
                );
                assert!(
                    email.chars().all(|c| c.is_ascii_lowercase()
                        || c.is_ascii_digit()
                        || c == '.'
                        || c == '@'),
                    "Email should only contain lowercase letters, digits, dots, and @: {}",
                    email
                );
            }
        }
    }
}

// Birthday tests

#[test]
fn test_birthday_date_mode() {
    let output = cmd()
        .args([
            "--birthday-from",
            "01-01-1990",
            "--birthday-to",
            "31-12-1999",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Output should contain a date in DD-MM-YYYY format
    assert!(
        stdout.contains("-"),
        "Output should contain birthday with hyphens"
    );
}

#[test]
fn test_birthday_age_single() {
    cmd()
        .args(["--age", "25"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-")); // Birthday contains hyphens
}

#[test]
fn test_birthday_age_range() {
    cmd()
        .args(["--age-from", "25", "--age-to", "30"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-"));
}

#[test]
fn test_birthday_to_requires_from() {
    cmd()
        .args(["--birthday-to", "31-12-1999"])
        .assert()
        .failure();
}

#[test]
fn test_age_to_requires_age_from() {
    cmd().args(["--age-to", "30"]).assert().failure();
}

#[test]
fn test_date_and_age_mutually_exclusive() {
    cmd()
        .args([
            "--birthday-from",
            "01-01-1990",
            "--birthday-to",
            "31-12-1999",
            "--age",
            "25",
        ])
        .assert()
        .failure();
}

#[test]
fn test_invalid_date_error() {
    cmd()
        .args([
            "--birthday-from",
            "31-02-1990",
            "--birthday-to",
            "31-12-1999",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid date"));
}

#[test]
fn test_json_with_birthday() {
    cmd()
        .args(["-f", "json", "--age", "25"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"birthday\":"));
}

#[test]
fn test_csv_with_birthday() {
    cmd()
        .args(["-f", "csv", "--age", "25"])
        .assert()
        .success()
        .stdout(predicate::str::contains("birthday"));
}

#[test]
fn test_birthday_format_dd_mm_yyyy() {
    let output = cmd().args(["--age", "25"]).output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Extract birthday from pipe-separated output
    let parts: Vec<&str> = stdout.trim().split(" | ").collect();
    if parts.len() >= 2 {
        let birthday = parts[1];
        // Should be DD-MM-YYYY format
        assert_eq!(
            birthday.len(),
            10,
            "Birthday should be 10 chars: {}",
            birthday
        );
        let date_parts: Vec<&str> = birthday.split('-').collect();
        assert_eq!(
            date_parts.len(),
            3,
            "Birthday should have 3 parts separated by -"
        );
        assert_eq!(date_parts[0].len(), 2, "Day should be 2 digits");
        assert_eq!(date_parts[1].len(), 2, "Month should be 2 digits");
        assert_eq!(date_parts[2].len(), 4, "Year should be 4 digits");
    }
}

#[test]
fn test_birthday_with_email_and_password() {
    cmd()
        .args(["--age", "25", "--email", "--password"])
        .assert()
        .success()
        .stdout(predicate::str::contains("@"))
        .stdout(predicate::str::contains("|"));
}
