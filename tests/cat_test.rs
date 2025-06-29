use assert_cmd::Command;
use std::fs;

#[test]
fn test_cat_single_file() {
    let content = "Hello, world!\nThis is a test.\n";
    fs::write("test_cat_single.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("test_cat_single.txt")
        .assert()
        .success()
        .stdout(content);

    fs::remove_file("test_cat_single.txt").unwrap();
}

#[test]
fn test_cat_multiple_files() {
    let content1 = "File 1 content.\n";
    let content2 = "File 2 content.\n";
    fs::write("test_cat_multi1.txt", content1).unwrap();
    fs::write("test_cat_multi2.txt", content2).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("test_cat_multi1.txt")
        .arg("test_cat_multi2.txt")
        .assert()
        .success()
        .stdout(format!("{}{}", content1, content2));

    fs::remove_file("test_cat_multi1.txt").unwrap();
    fs::remove_file("test_cat_multi2.txt").unwrap();
}

#[test]
fn test_cat_stdin() {
    let input = "Hello from stdin!\n";
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.write_stdin(input).assert().success().stdout(input);
}

#[test]
fn test_cat_number_all_lines() {
    let content = "Line 1\n\nLine 3\n";
    let expected = "     1  Line 1\n     2  \n     3  Line 3\n";
    fs::write("test_cat_n.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-n")
        .arg("test_cat_n.txt")
        .assert()
        .success()
        .stdout(expected);

    fs::remove_file("test_cat_n.txt").unwrap();
}

#[test]
fn test_cat_number_nonblank_lines() {
    let content = "Line 1\n\nLine 3\n";
    let expected = "     1  Line 1\n\n     2  Line 3\n";
    fs::write("test_cat_b.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-b")
        .arg("test_cat_b.txt")
        .assert()
        .success()
        .stdout(expected);

    fs::remove_file("test_cat_b.txt").unwrap();
}

#[test]
fn test_cat_show_ends() {
    let content = "Line 1\nLine 2\n";
    let expected = "Line 1$\nLine 2$\n";
    fs::write("test_cat_E.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-E")
        .arg("test_cat_E.txt")
        .assert()
        .success()
        .stdout(expected);

    fs::remove_file("test_cat_E.txt").unwrap();
}

#[test]
fn test_cat_squeeze_blank() {
    let content = "Line 1\n\n\nLine 2\n";
    let expected = "Line 1\n\nLine 2\n";
    fs::write("test_cat_s.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-s")
        .arg("test_cat_s.txt")
        .assert()
        .success()
        .stdout(expected);

    fs::remove_file("test_cat_s.txt").unwrap();
}

#[test]
fn test_cat_show_tabs() {
    let content = "Line\twith\ttabs\n";
    let expected = "Line^Iwith^Itabs\n";
    fs::write("test_cat_T.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-T")
        .arg("test_cat_T.txt")
        .assert()
        .success()
        .stdout(expected);

    fs::remove_file("test_cat_T.txt").unwrap();
}

#[test]
fn test_cat_show_nonprinting() {
    let content = "\x01\x02\x7f\n"; // SOH, STX, DEL
    let expected = "^A^B^?\n";
    fs::write("test_cat_v.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-v")
        .arg("test_cat_v.txt")
        .assert()
        .success()
        .stdout(expected);

    fs::remove_file("test_cat_v.txt").unwrap();
}

#[test]
fn test_cat_show_all() {
    let content = "Line\t1\n\nLine\t3\n";
    let expected = "Line^I1$\n$\nLine^I3$\n";
    fs::write("test_cat_A.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-A")
        .arg("test_cat_A.txt")
        .assert()
        .success()
        .stdout(expected);

    fs::remove_file("test_cat_A.txt").unwrap();
}

#[test]
fn test_cat_e_flag() {
    let content = "Line\n\n";
    let expected = "Line$\n$\n";
    fs::write("test_cat_e.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-e")
        .arg("test_cat_e.txt")
        .assert()
        .success()
        .stdout(expected);

    fs::remove_file("test_cat_e.txt").unwrap();
}

#[test]
fn test_cat_t_flag() {
    let content = "Line\t1\n";
    let expected = "Line^I1\n";
    fs::write("test_cat_t.txt", content).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-t")
        .arg("test_cat_t.txt")
        .assert()
        .success()
        .stdout(expected);

    fs::remove_file("test_cat_t.txt").unwrap();
}