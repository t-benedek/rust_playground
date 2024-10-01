use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");    
        process::exit(1);
    });

    // we are only interested if an Error is returned, but not in the Ok case as it only returns the unit type
    // so we do not use "unrwap_or_else" as above
    if let Err(e) = run(config) {
        println!("App error; {e}");
        process::exit(1);
    }    
}