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
}
