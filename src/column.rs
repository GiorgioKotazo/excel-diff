/// Convert an Excel column letter (e.g. `"A"`, `"D"`, `"AA"`) to a zero-based index.
///
/// Returns an error if the input is empty or contains non-alphabetic characters.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_letters() {
        assert_eq!(col_letter_to_index("A").unwrap(), 0);
        assert_eq!(col_letter_to_index("Z").unwrap(), 25);
    }

    #[test]
    fn test_multiletter() {
        assert_eq!(col_letter_to_index("AA").unwrap(), 26);
        assert_eq!(col_letter_to_index("AB").unwrap(), 27);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(col_letter_to_index("d").unwrap(), 3);
        assert_eq!(col_letter_to_index("aa").unwrap(), 26);
    }

    #[test]
    fn test_invalid_characters() {
        assert!(col_letter_to_index("123").is_err());
        assert!(col_letter_to_index("").is_err());
        assert!(col_letter_to_index("A1").is_err());
    }
}
