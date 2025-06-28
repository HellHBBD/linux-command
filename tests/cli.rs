use assert_cmd::Command;

use std::fs;

#[test]
fn test_touch_creates_file() {
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("test_file").assert().success();
    assert!(fs::metadata("test_file").is_ok());
    fs::remove_file("test_file").unwrap();
}

#[test]
fn test_touch_no_create() {
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("-c")
        .arg("non_existent_file")
        .assert()
        .success();
    assert!(fs::metadata("non_existent_file").is_err());
}

#[test]
fn test_touch_access_time() {
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("test_file_a").assert().success();
    let initial_meta = fs::metadata("test_file_a").unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("-a").arg("test_file_a").assert().success();
    let updated_meta = fs::metadata("test_file_a").unwrap();
    assert_ne!(
        initial_meta.accessed().unwrap(),
        updated_meta.accessed().unwrap()
    );
    assert_eq!(
        initial_meta.modified().unwrap(),
        updated_meta.modified().unwrap()
    );
    fs::remove_file("test_file_a").unwrap();
}

#[test]
fn test_touch_modify_time() {
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("test_file_m").assert().success();
    let initial_meta = fs::metadata("test_file_m").unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("-m").arg("test_file_m").assert().success();
    let updated_meta = fs::metadata("test_file_m").unwrap();
    assert_eq!(
        initial_meta.accessed().unwrap(),
        updated_meta.accessed().unwrap()
    );
    assert_ne!(
        initial_meta.modified().unwrap(),
        updated_meta.modified().unwrap()
    );
    fs::remove_file("test_file_m").unwrap();
}

#[test]
fn test_touch_reference_file() {
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("ref_file").assert().success();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("test_file_r")
        .arg("-r")
        .arg("ref_file")
        .assert()
        .success();
    let ref_meta = fs::metadata("ref_file").unwrap();
    let test_meta = fs::metadata("test_file_r").unwrap();
    assert_eq!(ref_meta.modified().unwrap(), test_meta.modified().unwrap());
    assert_eq!(ref_meta.accessed().unwrap(), test_meta.accessed().unwrap());
    fs::remove_file("ref_file").unwrap();
    fs::remove_file("test_file_r").unwrap();
}

#[test]
fn test_touch_date_string() {
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("-d")
        .arg("2023-01-01 12:00:00")
        .arg("test_file_d")
        .assert()
        .success();
    let meta = fs::metadata("test_file_d").unwrap();
    let mtime = meta.modified().unwrap();
    let dt: chrono::DateTime<chrono::Local> = mtime.into();
    assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2023-01-01 12:00:00");
    fs::remove_file("test_file_d").unwrap();
}

#[test]
fn test_touch_time_format() {
    let mut cmd = Command::cargo_bin("touch").unwrap();
    cmd.arg("-t")
        .arg("202301011200.00")
        .arg("test_file_t")
        .assert()
        .success();
    let meta = fs::metadata("test_file_t").unwrap();
    let mtime = meta.modified().unwrap();
    let dt: chrono::DateTime<chrono::Local> = mtime.into();
    assert_eq!(dt.format("%Y%m%d%H%M.%S").to_string(), "202301011200.00");
    fs::remove_file("test_file_t").unwrap();
}
