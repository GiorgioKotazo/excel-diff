pub fn col_letter_to_index(letter: &str) -> Result<usize, Box<dyn std::error::Error>> {
    if letter.is_empty() {
        return Err("Column letter cannot be empty".into());
    }
    let mut idx = 0usize;
    for c in letter.chars() {
        let c = c.to_ascii_uppercase();
        if !c.is_ascii_uppercase() {
            return Err(format!(
                "Invalid column letter '{}': must contain only A-Z characters",
                letter
            )
            .into());
        }
        idx = idx * 26 + ((c as usize) - ('A' as usize) + 1);
    }
    if idx == 0 {
        return Err(format!("Invalid column letter: {}", letter).into());
    }
    Ok(idx - 1)
}
