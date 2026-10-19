use rust_xlsxwriter::{Format, Workbook};
use std::io::Write;

/// Write results as a formatted `.xlsx` workbook.
pub fn write_xlsx(
    path: &str,
    file1: &str,
    sheet1: &str,
    file2: &str,
    sheet2: &str,
    only_in_1: &[(String, String)],
    only_in_2: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();
    let ws = workbook.add_worksheet();
    ws.set_name("Diff")?;

    let bold = Format::new().set_bold();

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

    let gap_row = (only_in_1.len() + 3) as u32;
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

    workbook.save(path)?;
    Ok(())
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('\"', "\"\""))
    } else {
        s.to_string()
    }
}

/// Write results as a comma-separated values (CSV) file.
pub fn write_csv(
    path: &str,
    only_in_1: &[(String, String)],
    only_in_2: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;
    let mut w = std::io::BufWriter::new(file);

    writeln!(w, "Name,Number")?;
    for (name, num) in only_in_1 {
        writeln!(w, "{},{}", csv_escape(name), csv_escape(num))?;
    }
    writeln!(w)?;
    writeln!(w, "Name,Number")?;
    for (name, num) in only_in_2 {
        writeln!(w, "{},{}", csv_escape(name), csv_escape(num))?;
    }
    Ok(())
}

/// Write results as a tab-separated values (TSV) file.
pub fn write_tsv(
    path: &str,
    only_in_1: &[(String, String)],
    only_in_2: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;
    let mut w = std::io::BufWriter::new(file);

    writeln!(w, "Name\tNumber")?;
    for (name, num) in only_in_1 {
        writeln!(w, "{}\t{}", name, num)?;
    }
    writeln!(w)?;
    writeln!(w, "Name\tNumber")?;
    for (name, num) in only_in_2 {
        writeln!(w, "{}\t{}", name, num)?;
    }
    Ok(())
}

/// Write results as a plain text file.
pub fn write_txt(
    path: &str,
    file1: &str,
    sheet1: &str,
    file2: &str,
    sheet2: &str,
    only_in_1: &[(String, String)],
    only_in_2: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;
    let mut w = std::io::BufWriter::new(file);

    writeln!(
        w,
        "UNIQUE VALUES from file \"{}\" list \"{}\"",
        file1, sheet1
    )?;
    writeln!(w, "Name\tNumber")?;
    for (name, num) in only_in_1 {
        writeln!(w, "{}\t{}", name, num)?;
    }
    writeln!(w)?;
    writeln!(
        w,
        "UNIQUE VALUES from file \"{}\" list \"{}\"",
        file2, sheet2
    )?;
    writeln!(w, "Name\tNumber")?;
    for (name, num) in only_in_2 {
        writeln!(w, "{}\t{}", name, num)?;
    }
    Ok(())
}

/// Write results to `stdout` as plain tab-separated text (for piping).
pub fn write_stdout(
    file1: &str,
    sheet1: &str,
    file2: &str,
    sheet2: &str,
    only_in_1: &[(String, String)],
    only_in_2: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();
    let mut w = stdout.lock();

    writeln!(
        w,
        "UNIQUE VALUES from file \"{}\" list \"{}\"",
        file1, sheet1
    )?;
    writeln!(w, "Name\tNumber")?;
    for (name, num) in only_in_1 {
        writeln!(w, "{}\t{}", name, num)?;
    }
    writeln!(w)?;
    writeln!(
        w,
        "UNIQUE VALUES from file \"{}\" list \"{}\"",
        file2, sheet2
    )?;
    writeln!(w, "Name\tNumber")?;
    for (name, num) in only_in_2 {
        writeln!(w, "{}\t{}", name, num)?;
    }
    Ok(())
}
