use std::process::Command;

#[test]
fn test_cli_end_to_end() {
    let output_file = "test_output.xlsx";

    // Clean up any leftover file
    let _ = std::fs::remove_file(output_file);

    let status = Command::new("cargo")
        .args([
            "run",
            "--",
            "examples/example_1.xlsx",
            "list_1",
            "A",
            "B",
            "examples/example_2.xlsx",
            "list_2",
            "A",
            "B",
            output_file,
        ])
        .status()
        .expect("Failed to execute cargo run");

    assert!(status.success(), "CLI exited with a non-zero status");
    assert!(
        std::path::Path::new(output_file).exists(),
        "Output file was not created"
    );

    // Clean up
    std::fs::remove_file(output_file).unwrap();
}
