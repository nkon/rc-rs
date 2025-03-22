use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn runs() -> TestResult {
    Command::cargo_bin("rc")?
        .arg("1")
        .assert()
        .success()
        .stdout("1\n");
    Ok(())
}

#[test]
fn runs2() -> TestResult {
    Command::cargo_bin("rc")?
        .arg("1+2*3")
        .assert()
        .success()
        .stdout("7\n");
    Ok(())
}

#[test]
fn runs3() -> TestResult {
    Command::cargo_bin("rc")?
        .arg("sin(0)")
        .assert()
        .success()
        .stdout("0\n");
    Ok(())
}

#[test]
fn runs4() -> TestResult {
    Command::cargo_bin("rc")?
        .arg("abs(-2)")
        .assert()
        .success()
        .stdout("2\n");
    Ok(())
}

#[test]
fn runs5() -> TestResult {
    Command::cargo_bin("rc")?
        .arg("max(1,2,3)")
        .assert()
        .success()
        .stdout("3\n");
    Ok(())
}

#[test]
fn runs6() -> TestResult {
    Command::cargo_bin("rc")?
        .arg("1.0/4.0")
        .assert()
        .success()
        .stdout("0.25\n");
    Ok(())
}

#[test]
fn runs7() -> TestResult {
    Command::cargo_bin("rc")?
        .args(["1", "+", "2*3"])
        .assert()
        .success()
        .stdout("7\n");
    Ok(())
}

fn normalize_newlines(input: String) -> String {
    if cfg!(target_os = "windows") {
        input.replace("\r\n", "\n")
    } else {
        input.to_string()
    }
}

#[test]
fn runs_test_case() -> TestResult {
    let infile: PathBuf = ["tests", "test.case"].iter().collect();
    let outfile: PathBuf = ["tests", "test.answer"].iter().collect();
    let expected = normalize_newlines(fs::read_to_string(outfile)?);
    Command::cargo_bin("rc")?
        .args(["-s", infile.display().to_string().as_str()])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn runs_demo_case() -> TestResult {
    let infile: PathBuf = ["tests", "demo.case"].iter().collect();
    let outfile: PathBuf = ["tests", "demo.answer"].iter().collect();
    let expected = normalize_newlines(fs::read_to_string(outfile)?);
    Command::cargo_bin("rc")?
        .args(["-s", infile.display().to_string().as_str()])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn runs_option_test() -> TestResult {
    let outfile: PathBuf = ["tests", "cargo_run_test.answer"].iter().collect();
    let expected = normalize_newlines(fs::read_to_string(outfile)?);
    Command::cargo_bin("rc")?
        .args(["--test"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
