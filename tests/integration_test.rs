use std::process::Command;
use std::fs;
use std::path::Path;
use std::io::Result;

// Helper function to check if two files are equal
fn files_are_equal(file1: &str, file2: &str) -> Result<bool> {
    let content1 = fs::read(file1)?;
    let content2 = fs::read(file2)?;
    Ok(content1 == content2)
}

#[test]
fn test_bam_re_tagger() -> Result<()> {
    let input_bam            = "tests/data/test.bam";
    let expected_output_bam  = "tests/data/result.bam";
    let actual_output_bam    = "tests/data/output/result.bam";

    let is_release_mode = !cfg!(debug_assertions);

    let command = if is_release_mode {
        "./target/release/bam_re_tagger"
    } else {
        "./target/debug/bam_re_tagger"
    };

    // Ensure the output directory exists
    let output_dir = Path::new("tests/data/output/");
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }

    // Run the bam_re_tagger command with the specified arguments
    let output = Command::new(command)
        .arg("-i")
        .arg(input_bam)
        .arg("-o")
        .arg(actual_output_bam)
        .output()
        .expect("Failed to execute bam_re_tagger");

    // Ensure the command ran successfully
    assert!(
        output.status.success(),
        "bam_re_tagger command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check if the actual output BAM file is the same as the expected output
    assert!(
        files_are_equal(expected_output_bam, actual_output_bam)?,
        "The output BAM file does not match the expected result."
    );

    Ok(())
}
