//! CLI utility that compares two Excel sheets and writes the differences to a file or stdout.

use std::env;
use std::path::Path;

mod column;
mod excel;
mod output;

use column::col_letter_to_index;
use excel::read_pairs;
use output::{write_csv, write_stdout, write_tsv, write_txt, write_xlsx};

/// Entry point. Expects 9 positional CLI arguments (10th for output is optional).
///
/// Arguments: `<file1> <sheet1> <name_col1> <num_col1> <file2> <sheet2> <name_col2> <num_col2> [output]`
///
/// Output format is inferred from the file extension:
///   `.xlsx` — Excel workbook (default)
///   `.csv`  — Comma-separated values
///   `.tsv`  — Tab-separated values
///   `.txt`  — Plain text
///   `-`     — Print plain text to stdout (useful for piping)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 9 || args.len() > 10 {
        eprintln!(
            "Usage: {} <file1> <sheet1> <name_col1> <num_col1> <file2> <sheet2> <name_col2> <num_col2> [output]",
            args[0]
        );
        eprintln!(
            "Example: {} 'example_1.xlsx' 'list_1' D E 'example_2.xlsx' 'list_2' D E result.csv",
            args[0]
        );
        std::process::exit(1);
    }

    let file1 = &args[1];
    let sheet1 = &args[2];
    let name_col1 = col_letter_to_index(&args[3])?;
    let num_col1 = col_letter_to_index(&args[4])?;

    let file2 = &args[5];
    let sheet2 = &args[6];
    let name_col2 = col_letter_to_index(&args[7])?;
    let num_col2 = col_letter_to_index(&args[8])?;

    let output_path = if args.len() == 10 {
        args[9].clone()
    } else {
        "diff_result.xlsx".to_string()
    };
    let stdout_mode = output_path == "-";

    if !Path::new(file1).exists() {
        return Err(format!("File not found: {}", file1).into());
    }
    if !Path::new(file2).exists() {
        return Err(format!("File not found: {}", file2).into());
    }

    let set1 = read_pairs(file1, sheet1, name_col1, num_col1)?;
    let set2 = read_pairs(file2, sheet2, name_col2, num_col2)?;

    let only_in_1: Vec<(String, String)> = set1.difference(&set2).cloned().collect();
    let only_in_2: Vec<(String, String)> = set2.difference(&set1).cloned().collect();

    if stdout_mode {
        write_stdout(file1, sheet1, file2, sheet2, &only_in_1, &only_in_2)?;
        eprintln!("File 1 total rows: {}", set1.len());
        eprintln!("File 2 total rows: {}", set2.len());
        eprintln!("New: {} rows", only_in_1.len());
        eprintln!("Deleted: {} rows", only_in_2.len());
    } else {
        let ext = Path::new(&output_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("xlsx")
            .to_lowercase();

        match ext.as_str() {
            "csv" => write_csv(&output_path, &only_in_1, &only_in_2)?,
            "tsv" => write_tsv(&output_path, &only_in_1, &only_in_2)?,
            "txt" => write_txt(
                &output_path,
                file1,
                sheet1,
                file2,
                sheet2,
                &only_in_1,
                &only_in_2,
            )?,
            _ => write_xlsx(
                &output_path,
                file1,
                sheet1,
                file2,
                sheet2,
                &only_in_1,
                &only_in_2,
            )?,
        }

        println!("Result saved to: {}", output_path);
        println!("  File 1 total rows: {}", set1.len());
        println!("  File 2 total rows: {}", set2.len());
        println!("  New: {} rows", only_in_1.len());
        println!("  Deleted: {} rows", only_in_2.len());
    }

    Ok(())
}
