//! CLI utility that compares two Excel sheets and writes the differences to a new `.xlsx` file.

use rust_xlsxwriter::{Format, Workbook};
use std::env;
use std::path::Path;

mod column;
mod excel;

use column::col_letter_to_index;
use excel::read_pairs;

/// Entry point. Expects 9 positional CLI arguments (10th for output is optional).
///
/// Arguments: `<file1> <sheet1> <name_col1> <num_col1> <file2> <sheet2> <name_col2> <num_col2> [output]`
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 9 || args.len() > 10 {
        eprintln!(
            "Usage: {} <file1> <sheet1> <name_col1> <num_col1> <file2> <sheet2> <name_col2> <num_col2> [output.xlsx]",
            args[0]
        );
        eprintln!(
            "Example: {} 'example_1.xlsx' 'list_1' D E 'example_2.xlsx' 'list_2' D E result.xlsx",
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

    let mut workbook = Workbook::new();
    let ws = workbook.add_worksheet();
    ws.set_name("Diff")?;

    let bold = Format::new().set_bold();

    // Table 1: only in file 1
    ws.write_string_with_format(
        0,
        0,
        format!("UNIQUE VALUES from file \"{}\" list \"{}\"", file1, sheet1),
        &bold,
    )?;
    ws.write_string_with_format(1, 0, "Name", &bold)?;
    ws.write_string_with_format(1, 1, "Number", &bold)?;
    for (i, (name, num)) in only_in_1.iter().enumerate() {
        let row = (i + 2) as u32;
        ws.write_string(row, 0, name)?;
        ws.write_string(row, 1, num)?;
    }

    // empty gap
    let gap_row = (only_in_1.len() + 3) as u32;

    // Table 2: only in file 2
    ws.write_string_with_format(
        gap_row,
        0,
        format!("UNIQUE VALUES from file \"{}\" list \"{}\"", file2, sheet2),
        &bold,
    )?;
    let header2 = gap_row + 1;
    ws.write_string_with_format(header2, 0, "Name", &bold)?;
    ws.write_string_with_format(header2, 1, "Number", &bold)?;
    for (i, (name, num)) in only_in_2.iter().enumerate() {
        let row = header2 + 1 + (i as u32);
        ws.write_string(row, 0, name)?;
        ws.write_string(row, 1, num)?;
    }

    workbook.save(&output_path)?;

    println!("Result saved to: {}", output_path);
    println!("  File 1 total rows: {}", set1.len());
    println!("  File 2 total rows: {}", set2.len());
    println!("  New: {} rows", only_in_1.len());
    println!("  Deleted: {} rows", only_in_2.len());

    Ok(())
}
