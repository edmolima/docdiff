use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;

#[test]
fn diff_command_outputs_similarity_and_verdict() {
    let mut cmd = Command::cargo_bin("docdiff").unwrap();
    cmd.args(["diff", "examples/file1.txt", "examples/file2.txt"])
        .assert()
        .success()
        .stdout(
            contains("Result:")
                .and(contains("Document distance:"))
                .and(contains("Similarity:"))
                .and(contains("The files")),
        );
}

#[test]
fn diff_command_identical_files_gives_identical_verdict() {
    let mut cmd = Command::cargo_bin("docdiff").unwrap();
    cmd.args(["diff", "examples/file1.txt", "examples/file1.txt"])
        .assert()
        .success()
        .stdout(contains("almost identical"));
}

#[test]
fn diff_command_different_files_gives_different_verdict() {
    let mut cmd = Command::cargo_bin("docdiff").unwrap();
    cmd.args(["diff", "examples/file3.txt", "examples/file4.txt"])
        .assert()
        .success()
        .stdout(contains("quite different").or(contains("very different")));
}
