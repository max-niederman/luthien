use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

#[test]
#[allow(unused_must_use)]
fn integrate() {
    fs::remove_dir_all("tests/out"); // In case the test failed before it could clean up.
    fs::create_dir("tests/out").unwrap();

    // Run
    let mut proc = Command::new(env!("CARGO_BIN_EXE_luthien-templates"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn luthien-templates process.");
    io::copy(
        &mut fs::File::open("tests/input.json").expect("Couldn't find test input."),
        &mut proc.stdin.take().unwrap(),
    )
    .expect("Error copying test input.");

    // Check standard output
    let out = proc
        .wait_with_output()
        .expect("Failed to wait on luthien-templates process.");
    assert_eq!(std::str::from_utf8(&out.stdout).unwrap(), "");
    assert_eq!(
        std::str::from_utf8(&out.stderr).unwrap(),
        "Successfully rendered 1/1 templates.\n",
    );

    // Check template output
    for entry in fs::read_dir("tests/correct").unwrap() {
        let entry = entry.unwrap();

        assert_eq!(
            fs::read(entry.path()).unwrap(),
            fs::read(Path::new("tests/out").join(entry.file_name())).unwrap(),
        )
    }

    // Cleanup
    fs::remove_dir_all("tests/out");
}
