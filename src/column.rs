pub fn col_letter_to_index(letter: &str) -> usize {
    let mut idx = 0usize;
    for c in letter.chars() {
        let c = c.to_ascii_uppercase();
        if c.is_ascii_uppercase() {
            idx = idx * 26 + ((c as usize) - ('A' as usize) + 1);
        }
    }
    if idx == 0 {
        panic!("Invalid column letter: {}", letter);
    }
    idx - 1
}
