fn main() {
    let s = String::from("hello wolrd there");
    let first: &str = first_word(&s);
    let second: &str = second_word(&s);
    println!("{first}");
    println!("{second}");

    fn first_word(some_string: &str) -> &str {
        let bytes = some_string.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &some_string[0..i];
            }
        }
        &some_string[..]
    }

    fn second_word(some_string: &str) -> &str {
        const DELIMITER_LENGTH: usize = 1;
        let first_word = first_word(some_string);
        let begin_idx = first_word.len() + DELIMITER_LENGTH;
        let rest_string = &some_string[begin_idx..];
        let bytes = rest_string.as_bytes();

        println!("{rest_string}");

        for (i, &item) in bytes.iter().enumerate() {
            println!("item {item}  idx {i}");

            if item == b' ' {
                let end_idx = begin_idx + i;
                return &some_string[begin_idx..end_idx];
            }
        }

        &some_string[begin_idx..]
    }
}
