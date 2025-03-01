use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

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

fn normalize_newlines(input: String) -> String {
    if cfg!(target_os = "windows") {
        input.replace("\r\n", "\n")
    } else {
        input.to_string()
    }
}

#[test]
fn runs_test_case() {
    let infile: PathBuf = ["tests", "test.case"].iter().collect();
    let outfile: PathBuf = ["tests", "test.answer"].iter().collect();
    let expected = normalize_newlines(fs::read_to_string(outfile).unwrap());
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.args(["-s", infile.display().to_string().as_str()])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn runs_demo_case() {
    let infile: PathBuf = ["tests", "demo.case"].iter().collect();
    let outfile: PathBuf = ["tests", "demo.answer"].iter().collect();
    let expected = normalize_newlines(fs::read_to_string(outfile).unwrap());
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.args(["-s", infile.display().to_string().as_str()])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn runs_option_test() {
    let outfile: PathBuf = ["tests", "cargo_run_test.answer"].iter().collect();
    let expected = normalize_newlines(fs::read_to_string(outfile).unwrap());
    let mut cmd = Command::cargo_bin("rc").unwrap();
    cmd.args(["--test"]).assert().success().stdout(expected);
}
