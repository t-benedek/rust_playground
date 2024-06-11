use std::collections::HashMap;

// CONST values cannot be String, only &str
const TEAM_BLUE: &str = "team-blue";
const TEAM_YELLOW: &str = "team-yellow";

fn main() {

    let mut scores = HashMap::new();
    
    scores.insert(TEAM_BLUE, 10);
    scores.insert(TEAM_YELLOW, 20);

    // get returns Option(&i32)
    // "copied()"" changes from Option(&i32) to Option(i32)
    // unwrap_or(val) on Option returns the entry of the Hash_map or a default velue if the Option is NONE 
    let blue_score = scores.get(TEAM_BLUE).copied().unwrap_or(0);
    
    println!("The score of {TEAM_BLUE} is {blue_score}");

    // Iterating over the hashmap as with a vector, but with a tuple
    for (key, val) in scores {
        println!("{key}: {val}");
    }

    let s = String::from("Score Value");
    let default = String::from("DEFAULT");
    let mut scores2 = HashMap::new();
    scores2.insert(TEAM_BLUE, s);

    // hashmaps with Strings take over the ownership
    // this line will lead to a compiler error as in this scope s is not available
    // println!("String Score value {s}");
    
    // but we can borrow the value from the hashmap when we use "get()""
    let score_value = scores2.get(TEAM_BLUE).unwrap_or(&default);
    println!("String Score value is <{score_value}>");
}
