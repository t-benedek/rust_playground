use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("!!! Guess the number !!!\n");

    println!("The secret number is : {secret_number} \n");

    loop {
        let mut input = String::new();
        
        println!("Please input the number: ");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {input}");

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }

    
}
