fn main() {
    let s1 = String::from("Hello");
    takes_ownership(s1);

    // This does not work as ownership was tranfered
    // println!("{s}");

    let x = 666;
    makes_copy(x);
    println!("{x}");

    let s2 = gives_ownership();
    println!("{s2}");

    let s3 = takes_and_gives_back(s2);
    println!("{s3}");

    // using a reference, we do not have to take ownership to the String back and forth
    // we can borrow the String, but we cannot modify it
    let len = calculate_length(&s3);
    println!("The length of <{s3}> is {len}");

    // we can modify the borrowed String using a mutable reference
    let mut s4 = String::from("change");
    let r1 = &mut s4;
    // this is ok as a first mutable reference. The change call afterwards also works because r1 scope is over after this print statement
    println!("{r1}");

    change(&mut s4);
    println!("Changed {s4}");

    // This will not work as the r1 scope will be extended so that we have two mutable references to s4
    // println!("{r1}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
    // after ending this scope, 'drop' is called for some_string
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn gives_ownership() -> String {
    String::from("World")
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("_me");
}