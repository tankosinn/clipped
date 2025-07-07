const FIXTURE_PATH: &str = "tests/fixtures/config";
const WARNING: &str = "warning: unneeded `return` statement";

#[test]
fn test_config_default() {
    let mut cmd = assert_cmd::Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH)
        .arg("--level=error")
        .assert()
        .failure()
        .stdout(predicates::str::contains(WARNING));
}

#[test]
fn test_config_from_env() {
    let mut cmd = assert_cmd::Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH)
        .env("CLIPPED_LEVEL", "warning")
        .assert()
        .failure()
        .stdout(predicates::str::contains(WARNING));
}

#[test]
fn test_config_custom_file() {
    let mut cmd = assert_cmd::Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH).arg("--config=custom_config.toml").assert().success();
}

#[test]
fn test_config_env_override() {
    let mut cmd = assert_cmd::Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH)
        .env("CLIPPED_CLIPPY_ARGS", r#"["--", "-A", "clippy::needless_return"]"#)
        .assert()
        .success();
}

#[test]
fn test_config_cli_override() {
    let mut cmd = assert_cmd::Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH)
        .args(["--", "--", "-A", "clippy::needless_return"])
        .assert()
        .success();
}
