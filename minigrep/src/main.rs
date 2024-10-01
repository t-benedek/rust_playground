use std::env;
use std::fs;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");    
        process::exit(1);
    });

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("Query String <{}>\n", config.query);
    println!("File content: \n{}", contents);
}
