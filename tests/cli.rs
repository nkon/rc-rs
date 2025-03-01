use assert_cmd::Command;

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
