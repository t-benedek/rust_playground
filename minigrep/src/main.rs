use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");    
        process::exit(1);
    });

    // we are only interested if an Error is returned, but not in the Ok case as it only returns the unit type
    // so we do not use "unrwap_or_else" as above
    if let Err(e) = run(config) {
        eprintln!("App error; {e}");
        process::exit(1);
    }    
}