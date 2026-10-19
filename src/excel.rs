use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::collections::HashSet;

pub fn cell_to_string(cell: &DataType) -> String {
    match cell {
        DataType::String(s) => s.trim().to_string(),
        DataType::Int(i) => i.to_string(),
        DataType::Float(f) => {
            if f.fract() == 0.0 {
                format!("{:.0}", f)
            } else {
                f.to_string()
            }
        }
        DataType::Bool(b) => b.to_string(),
        DataType::DateTime(d) => d.to_string(),
        DataType::Duration(d) => d.to_string(),
        DataType::Empty => String::new(),
        _ => String::new(),
    }
}

pub fn read_pairs(
    path: &str,
    sheet: &str,
    name_col: usize,
    num_col: usize,
) -> Result<HashSet<(String, String)>, Box<dyn std::error::Error>> {
    let mut wb: Xlsx<_> = open_workbook(path)?;
    let range = wb
        .worksheet_range(sheet)
        .ok_or(format!("Sheet '{}' not found in {}", sheet, path))??;

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
