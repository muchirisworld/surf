use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn binary_finds_matches_with_line_numbers() {
    let dir = temp_dir();
    let file = dir.join("sample.txt");
    fs::write(&file, "rust\ncargo\ntrust\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_surf"))
        .args(["-n", "rust", file.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("1:rust"));
    assert!(stdout.contains("3:trust"));
    assert!(!stdout.contains("cargo"));
}

#[test]
fn binary_finds_matches_invert_match() {
    let dir = temp_dir();
    let file = dir.join("sample.txt");
    fs::write(&file, "rust\ncargo\ntrust\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_surf"))
        .args(["-v", "rust", file.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("cargo"));
    assert!(!stdout.contains("rust"));
    assert!(!stdout.contains("trust"));
}

fn temp_dir() -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("surf-test-{unique}"));
    fs::create_dir(&dir).unwrap();
    dir
}
