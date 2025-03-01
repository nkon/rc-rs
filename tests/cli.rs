use assert_cmd::Command;
use std::fs;
use std::path::Path;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.arg("1").assert().success().stdout("1\n");
}

#[test]
fn runs2() {
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.arg("1+2*3").assert().success().stdout("7\n");
}

#[test]
fn runs3() {
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.arg("sin(0)").assert().success().stdout("0\n");
}

#[test]
fn runs4() {
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.arg("abs(-2)").assert().success().stdout("2\n");
}

#[test]
fn runs5() {
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.arg("max(1,2,3)").assert().success().stdout("3\n");
}

#[test]
fn runs6() {
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.arg("1.0/4.0").assert().success().stdout("0.25\n");
}

#[test]
fn runs7() {
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.args(["1", "+", "2*3"]).assert().success().stdout("7\n");
}

#[test]
fn runs_test_case() {
    let infile = Path::new("tests/test.case");
    let outfile = Path::new("tests/test.answer");
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.args(["-s", infile.display().to_string().as_str()])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn runs_demo_case() {
    let infile = Path::new("tests/demo.case");
    let outfile = Path::new("tests/demo.answer");
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.args(["-s", infile.display().to_string().as_str()])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn runs_option_test() {
    let outfile = Path::new("tests/cargo_run_test.answer");
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.args(["--test"]).assert().success().stdout(expected);
}
