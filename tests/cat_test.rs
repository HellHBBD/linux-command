use assert_cmd::Command;
use tempfile::NamedTempFile;
use std::io::Write;

fn normalize_newline(s: &str) -> String {
    s.replace("\r\n", "\n")
}

#[test]
fn test_cat_single_file() {
    let content = "Hello, world!\nThis is a test.\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg(file.path())
        .assert()
        .success()
        .stdout(content);
}

#[test]
fn test_cat_multiple_files() {
    let content1 = "File 1 content.\n";
    let content2 = "File 2 content.\n";
    let mut file1 = NamedTempFile::new().unwrap();
    file1.write_all(content1.as_bytes()).unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    file2.write_all(content2.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg(file1.path())
        .arg(file2.path())
        .assert()
        .success()
        .stdout(normalize_newline(&format!("{}{}", content1, content2)));
}

#[test]
fn test_cat_stdin() {
    let input = "Hello from stdin!\n";
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.write_stdin(input).assert().success().stdout(normalize_newline(input));
}

#[test]
fn test_cat_number_all_lines() {
    let content = "Line 1\n\nLine 3\n";
    let expected = "     1  Line 1\n     2  \n     3  Line 3\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-n")
        .arg(file.path())
        .assert()
        .success()
        .stdout(normalize_newline(expected));
}

#[test]
fn test_cat_number_nonblank_lines() {
    let content = "Line 1\n\nLine 3\n";
    let expected = "     1  Line 1\n\n     2  Line 3\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-b")
        .arg(file.path())
        .assert()
        .success()
        .stdout(normalize_newline(expected));
}

#[test]
fn test_cat_show_ends() {
    let content = "Line 1\nLine 2\n";
    let expected = "Line 1$\nLine 2$\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-E")
        .arg(file.path())
        .assert()
        .success()
        .stdout(normalize_newline(expected));
}

#[test]
fn test_cat_squeeze_blank() {
    let content = "Line 1\n\n\nLine 2\n";
    let expected = "Line 1\n\nLine 2\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-s")
        .arg(file.path())
        .assert()
        .success()
        .stdout(normalize_newline(expected));
}

#[test]
fn test_cat_show_tabs() {
    let content = "Line\twith\ttabs\n";
    let expected = "Line^Iwith^Itabs\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-T")
        .arg(file.path())
        .assert()
        .success()
        .stdout(normalize_newline(expected));
}

#[test]
fn test_cat_show_nonprinting() {
    let content = "\x01\x02\x7f\n"; // SOH, STX, DEL
    let expected = "^A^B^?\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-v")
        .arg(file.path())
        .assert()
        .success()
        .stdout(normalize_newline(expected));
}

#[test]
fn test_cat_show_all() {
    let content = "Line\t1\n\nLine\t3\n";
    let expected = "Line^I1$\n$\nLine^I3$\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-A")
        .arg(file.path())
        .assert()
        .success()
        .stdout(normalize_newline(expected));
}

#[test]
fn test_cat_e_flag() {
    let content = "Line\n\n";
    let expected = "Line$\n$\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-e")
        .arg(file.path())
        .assert()
        .success()
        .stdout(normalize_newline(expected));
}

#[test]
fn test_cat_t_flag() {
    let content = "Line\t1\n";
    let expected = "Line^I1\n";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-t")
        .arg(file.path())
        .assert()
        .success()
        .stdout(normalize_newline(expected));
}