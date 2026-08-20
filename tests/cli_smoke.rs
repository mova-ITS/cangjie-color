// tests/cli_smoke.rs
use std::path::PathBuf;
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_cj-color"))
}

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[test]
fn cli_writes_zi_svg() {
    let out = std::env::temp_dir().join("cangjie-color-cli-zi.svg");
    let _ = std::fs::remove_file(&out);

    let status = Command::new(bin())
        .args([
            "--recipes",
            fixture("testdata/recipes/sample.jsonl").to_str().unwrap(),
            "--graphics",
            fixture("testdata/graphics/sample.jsonl").to_str().unwrap(),
            "--char",
            "子",
            "-o",
            out.to_str().unwrap(),
        ])
        .status()
        .expect("spawn cj-color");

    assert!(status.success(), "exit {status}");
    let svg = std::fs::read_to_string(&out).expect("read svg");
    assert!(svg.contains("<svg"));
    assert!(svg.contains("#1a9e3b"));
    assert!(svg.contains("#e6194b"));
}
