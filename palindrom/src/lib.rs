#[cfg(test)]
mod tests {
    use crate::palindrom;

    #[test]
    fn palindrom_test() {
        let word = "palindrom";
        let result = "mordnilap";
    }

}

pub fn palindrom(mut word: &str) {
    println!("\n {word} \n ");
}