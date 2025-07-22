use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn diff_command_outputs_distance() {
    let mut cmd = Command::cargo_bin("docdiff").unwrap();
    cmd.args(["diff", "examples/file1.txt", "examples/file2.txt"])
        .assert()
        .success()
        .stdout(contains("Document distance:"));
}

#[test]
fn info_command_outputs_info() {
    let mut cmd = Command::cargo_bin("docdiff").unwrap();
    cmd.arg("info")
        .assert()
        .success()
        .stdout(contains("docdiff - Compare two files using document distance algorithms"));
}
