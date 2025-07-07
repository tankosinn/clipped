use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;

const FIXTURE_PATH: &str = "tests/fixtures/workspace";
const A_WARNING: &str = "warning: using `clone` on type `bool` which implements the `Copy` trait";
const B_WARNING: &str = "warning: variable does not need to be mutable";

#[test]
fn test_workspace_resolve_single_package() {
    let mut cmd = Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH)
        .arg("--verbose")
        .arg("crates/b/src/lib.rs")
        .assert()
        .failure()
        .stdout(predicates::str::contains(B_WARNING))
        .stderr(predicates::str::contains("-p b"))
        .stderr(predicates::str::contains("-p a").not());
}

#[test]
fn test_workspace_resolve_multiple_packages() {
    let mut cmd = Command::cargo_bin("clipped").unwrap();
    cmd.current_dir(FIXTURE_PATH)
        .arg("--verbose")
        .arg("crates/a/src/lib.rs")
        .arg("crates/b/src/lib.rs")
        .assert()
        .failure()
        .stdout(predicates::str::contains(A_WARNING))
        .stdout(predicates::str::contains(B_WARNING))
        .stderr(predicates::str::contains("-p a -p b"));
}
