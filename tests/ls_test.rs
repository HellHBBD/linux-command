use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_ls_current_directory() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("src"));
}

#[test]
fn test_ls_all_flag() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg("-a")
        .assert()
        .success()
        .stdout(predicate::str::contains(".git"));
}

#[test]
fn test_ls_long_flag() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg("-l")
        .assert()
        .success()
        .stdout(predicate::str::contains("hellhbbd"));
}

#[test]
fn test_ls_with_path() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file");
    fs::write(&file_path, "content").unwrap();

    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("test_file"));
}
