use std::collections::HashMap;

// CONST values cannot be String, only &str
const TEAM_BLUE: &str = "team-blue";
const TEAM_YELLOW: &str = "team-yellow";
const TEAM_GREEN: &str = "team-green";

fn main() {

    let mut scores = HashMap::new();
    
    scores.insert(TEAM_BLUE, 10);
    scores.insert(TEAM_YELLOW, 20);

    // get returns Option(&i32)
    // "copied()" changes from Option(&i32) to Option(i32)
    // unwrap_or(val) on Option returns the entry of the Hash_map or a default value if the Option is NONE 
    let blue_score = scores.get(TEAM_BLUE).copied().unwrap_or(0);
    let green_score = scores.get(TEAM_GREEN).copied().unwrap_or(0);
    
    println!("The score of {TEAM_BLUE} is {blue_score}");
    println!("The score of {TEAM_GREEN} is {green_score}\n");

    // we can use "or_inesrt" to only insert if the key is not already existing or to do nothing if it exists
    // after these calls blue wont be changed, but green is added to the Hashmap
    scores.entry(TEAM_BLUE).or_insert(50);
    scores.entry(TEAM_GREEN).or_insert(50);

    // Iterating over the hashmap as with a vector, but with a tuple
    for (key, val) in scores {
        println!("{key}: {val}");
    }

    // demonstrates that hashmaps take over the ownership of strings
    hashmap_takes_ownership();

    // how to use a hashmap to count the different words in a text easily
    parse_test_using_hashmap();

}

fn parse_test_using_hashmap() {
    let text = "hello world wunderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for w in text.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }

    println!("Counted words {:#?}", map);
}

fn hashmap_takes_ownership() {
    let s = String::from("Score Value");
    let default = String::from("DEFAULT");
    let mut scores2 = HashMap::new();
    scores2.insert(TEAM_BLUE, s);

    // hashmaps with Strings take over the ownership
    // this line will lead to a compiler error as in this scope s is not available any more
    // println!("String Score value {s}");
    
    // but we can borrow the value from the hashmap when we use "get()""
    let score_value = scores2.get(TEAM_BLUE).unwrap_or(&default);
    println!("\n\nString Score value is <{score_value}>\n");

}
