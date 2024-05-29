use std::env;

fn palindrom(mut s: String) -> String {    
    if s.len() > 1 {
        let last = s.remove(s.len()-1).to_string();
        let rest: String = palindrom(s);
        let together = format!("{last}{rest}");
        return together;
    }
    return s;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let input = args[1].to_string();
        println!("{}", input);
        println!("Palindrom ist {:?}", palindrom(input));
    } else {
        println!("Please only use exactly one input parameter");
    }
}
