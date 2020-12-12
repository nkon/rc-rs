use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let commit_hash = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("failed to execute command \"git rev-parse HEAD\"");
    let dest_path = format!("{}/src/git_commit_hash.txt", env!("CARGO_MANIFEST_DIR"));
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(
        format!(
            "\"{}\"",
            String::from_utf8(commit_hash.stdout.to_vec())
                .unwrap()
                .replace("\n", "")
        )
        .as_bytes(),
    )
    .unwrap();
}
