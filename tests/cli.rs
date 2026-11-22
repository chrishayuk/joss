//! The command line: `-i` types a file in, then carries on with what is typed.

use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn a_file_then_the_console() {
    let dir = std::env::temp_dir().join(format!("joss-cli-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("steps.joss");
    std::fs::write(&file, "1.1 Type 1/a.\n").unwrap();
    let mut joss = Command::new(env!("CARGO_BIN_EXE_joss"))
        .arg("-i")
        .arg(&file)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    joss.stdin
        .take()
        .unwrap()
        .write_all(b"Do part 1.\nSet a = 4.\nGo.\n")
        .unwrap();
    let page = String::from_utf8(joss.wait_with_output().unwrap().stdout).unwrap();
    assert_eq!(
        page,
        "1.1 Type 1/a.\nDo part 1.\nError at step 1.1:  a is undefined.\nSet a = 4.\nGo.\n     1/a =        .25\n"
    );
    std::fs::remove_dir_all(&dir).ok();
}
