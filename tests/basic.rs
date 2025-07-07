use assert_cmd::Command;
use predicates::str::contains;

const FIXTURE_PATH: &str = "tests/fixtures/basic";
const WARNING: &str = "warning: unused variable: `x`";

#[test]
fn test_basic_empty_files() {
    let mut cmd = Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH).assert().failure().stdout(contains(WARNING));
}

#[test]
fn test_basic_valid_files_with_warnings() {
    let mut cmd = Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH).arg("src/lib.rs").assert().failure().stdout(contains(WARNING));
}

#[test]
fn test_basic_valid_files_without_warnings() {
    let mut cmd = Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH).arg("src/a.rs").assert().success();
}

#[test]
fn test_basic_invalid_files() {
    let mut cmd = Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH).arg("src/invalid.rs").assert().failure();
}

#[test]
fn test_basic_level_warnings() {
    let mut cmd = Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH)
        .arg("--level=warning")
        .assert()
        .failure()
        .stdout(contains(WARNING));
}

#[test]
fn test_basic_level_errors() {
    let mut cmd = Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH).arg("--level=error").assert().success();
}
