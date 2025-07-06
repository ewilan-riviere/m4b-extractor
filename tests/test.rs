use std::fs;
use std::path::Path;
use std::process::Command;

fn test_output_files_exist() {
    let output_dir = "tests/output";

    let expected_files = [
        "1_Chapter 01.mp3",
        "2_Chapter 02.mp3",
        "folder.jpg",
        "metadata.json",
        "tags.yaml",
    ];

    for file in expected_files {
        let path = format!("{}/{}", output_dir, file);
        assert!(
            fs::metadata(&path).is_ok(),
            "Expected file '{}' to exist but it does not.",
            path
        );
    }
}

#[test]
fn test_m4b_extractor_basic() {
    // Path to compiled binary
    // When running tests with `cargo test`, the binary is usually at:
    // ./target/debug/m4b-extractor
    // Use `cargo build` or `cargo build --release` before running tests

    let binary_path = if cfg!(debug_assertions) {
        "./target/debug/m4b-extractor"
    } else {
        "./target/release/m4b-extractor"
    };

    // Define input test file path (put a small test .m4b in tests/data or use a dummy file)
    let input_file = "tests/data/sample.m4b";
    let output_dir = "tests/output";

    // Clean previous test output if exists
    if Path::new(output_dir).exists() {
        fs::remove_dir_all(output_dir).expect("Failed to clean output directory");
    }

    // Run the binary with input and output args
    let output = Command::new(binary_path)
        .args(&[input_file, "--output", output_dir])
        .output()
        .expect("Failed to run m4b-extractor binary");

    // Optional: print stdout/stderr for debug if needed
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Assert the binary exited successfully
    assert!(output.status.success(), "Binary did not exit successfully");

    // Check output directory exists
    assert!(
        Path::new(output_dir).exists(),
        "Output directory not created"
    );

    // Check at least one chapter file was created
    let chapter_files = fs::read_dir(output_dir)
        .expect("Failed to read output directory")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "mp3" || ext == "m4b")
                .unwrap_or(false)
        })
        .count();

    assert!(
        chapter_files > 0,
        "No chapter audio files found in output directory"
    );

    // Check specific expected files exist
    test_output_files_exist();
}
