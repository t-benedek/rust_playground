use palindrom::{palindrom_iterative, palindrom_recursive};
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("middle.txt").unwrap();
    let word = String::from("This is");

    println!("Recursive {}", palindrom_recursive(word));
    println!("Iterative {}", palindrom_iterative(file));
}