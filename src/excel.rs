use calamine::{open_workbook, Data, Reader, Xlsx};
use std::collections::HashSet;

/// Convert a calamine `Data` cell into a plain `String`.
///
/// * Trims surrounding whitespace from strings.
/// * Formats whole-number floats without a decimal point.
pub fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::String(s) => s.trim().to_string(),
        Data::Int(i) => i.to_string(),
        Data::Float(f) => {
            if f.fract() == 0.0 {
                format!("{:.0}", f)
            } else {
                f.to_string()
            }
        }
        Data::Bool(b) => b.to_string(),
        Data::DateTime(d) => d.to_string(),
        Data::DateTimeIso(s) => s.clone(),
        Data::DurationIso(s) => s.clone(),
        Data::Error(e) => e.to_string(),
        Data::Empty => String::new(),
    }
}

/// Read the specified columns from an Excel sheet and return a set of `(name, number)` pairs.
///
/// Rows that do not have values in both requested columns are skipped.
/// The pair is inserted even if only one of the two cells is non-empty.
pub fn read_pairs(
    path: &str,
    sheet: &str,
    name_col: usize,
    num_col: usize,
) -> Result<HashSet<(String, String)>, Box<dyn std::error::Error>> {
    let mut wb: Xlsx<_> = open_workbook(path)?;
    let range = wb
        .worksheet_range(sheet)
        .map_err(|e| format!("Sheet '{}' not found in {}: {}", sheet, path, e))?;

    let mut set = HashSet::new();
    for row in range.rows() {
        if name_col >= row.len() || num_col >= row.len() {
            continue;
        }
        let name = cell_to_string(&row[name_col]);
        let num = cell_to_string(&row[num_col]);
        if !name.is_empty() || !num.is_empty() {
            set.insert((name, num));
        }
    }
    Ok(set)
}
