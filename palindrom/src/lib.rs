#[cfg(test)]

mod tests {
    use crate::palindrom_recursive;

    #[test]
    fn palindrom_test() {
        let word = String::from("palindrom");
        let result = String::from("mordnilap");
        assert_eq!(palindrom_recursive(word), result);
    }
}

/// Returns the palindrom of a given String
/// 
/// # Panics
/// 
/// This function panics if a word with zero characters is the parameter or String is too long
pub fn palindrom_recursive(word: String) -> String {
    match word.len() {
        0 => {
            panic!("Input must have at least one character");
        },
        1 => word,
        _ => {
            let first = &word[0..1];
            let tail = palindrom_recursive(word[1..].to_string());
            format!("{tail}{first}")
        },
    }
}

/// Returns the palindrom of a given String
/// 
/// # Panics
/// 
/// This function panics if a word with zero characters is the parameter
pub fn palindrom_iterative(mut word : String) -> String {
    match word.len() {
        0 => {
            panic!("Input must have at least one character");
        },
        1 => word,
        _ => {
            for idx in 0..word.len() {
                let c = word.pop().unwrap();
                word.insert(idx, c);
            };
            word
        },
    }

}