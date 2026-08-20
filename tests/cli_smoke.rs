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
fn cli_batch_writes_sample_svgs() {
    let out_dir = std::env::temp_dir().join("cangjie-color-cli-batch");
    let _ = std::fs::remove_dir_all(&out_dir);

    let status = Command::new(bin())
        .args([
            "--recipes",
            fixture("testdata/recipes/sample.jsonl").to_str().unwrap(),
            "--graphics",
            fixture("testdata/graphics/sample.jsonl").to_str().unwrap(),
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--mode",
            "full",
        ])
        .status()
        .expect("spawn cj-color");

    assert!(status.success(), "exit {status}");
    let zi = std::fs::read_to_string(out_dir.join("子.svg")).expect("子.svg");
    assert!(zi.contains(r#"xmlns="http://www.w3.org/2000/svg""#), "{zi}");
    assert!(zi.contains("#1a9e3b"), "N green missing: {zi}");
    let entries = std::fs::read_dir(&out_dir)
        .expect("read out_dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "svg"))
        .count();
    assert!(entries >= 6, "expected ≥6 sample svgs, got {entries}");
}

#[test]
fn cli_batch_chars_subset() {
    let out_dir = std::env::temp_dir().join("cangjie-color-cli-batch-subset");
    let _ = std::fs::remove_dir_all(&out_dir);

    let status = Command::new(bin())
        .args([
            "--recipes",
            fixture("testdata/recipes/sample.jsonl").to_str().unwrap(),
            "--graphics",
            fixture("testdata/graphics/sample.jsonl").to_str().unwrap(),
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--chars",
            "子,气",
        ])
        .status()
        .expect("spawn cj-color");

    assert!(status.success(), "exit {status}");
    assert!(out_dir.join("子.svg").is_file());
    assert!(out_dir.join("气.svg").is_file());
    assert!(!out_dir.join("隹.svg").exists());
}

#[test]
fn cli_letter_n_omits_d_red() {
    let out = std::env::temp_dir().join("cangjie-color-cli-zi-letter-n.svg");
    let _ = std::fs::remove_file(&out);

    let status = Command::new(bin())
        .args([
            "--recipes",
            fixture("testdata/recipes/sample.jsonl").to_str().unwrap(),
            "--graphics",
            fixture("testdata/graphics/sample.jsonl").to_str().unwrap(),
            "--char",
            "子",
            "--mode",
            "letter:N",
            "-o",
            out.to_str().unwrap(),
        ])
        .status()
        .expect("spawn cj-color");

    assert!(status.success(), "exit {status}");
    let svg = std::fs::read_to_string(&out).expect("read svg");
    assert!(svg.contains("#1a9e3b"), "N green missing: {svg}");
    assert!(!svg.contains("#e6194b"), "D must not stay red: {svg}");
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
