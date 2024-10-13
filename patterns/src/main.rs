#![ allow(unused)] 

fn main() {

    println!("\n Showing while loop with stack operation");
    while_loop();

    println!("\n Showing for loop enumarator");
    for_loop();
    
    nested_tuples();

    println!("\n Understanding refrutability in pattern matching");
    refrutability();

    println!("\n Matching literals");
    match_literals();

    println!("\n Matching and shadowing");
    match_shadowing();

    println!("\n Destructuring and matching in one step");
    match_destructure();

    println!("\n Ignoring values");
    match_ignoring();

    println!("\n Using a match guard");
    match_guard();

}

/// Shows how to use a match guard together with pattern matching
/// 
/// # Example
/// ```
/// match num {
///     Some(x) if x %2 == 0 => ...
/// }
/// ```
fn match_guard() {
    let num = Some(5);
    let y = 5;

    // Using a match guard for additional checks
    match num {
        Some(x) if x % 2 == 0 => println!("{x} is even"),
        Some(x) => println!("{x} is odd"),
        None => ()
     }

     // A match guard can also access outer values (in this case "y")
     match num {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("Matched n = {n}"),
        _ => ()
     }
}

fn match_ignoring() {
    struct Point {
        x: i32,
        y: i32,
        z: i32 
    }

    let origin = Point {y: 1, x: 2, z: 3};
    match origin {
        // <..> is handy to have to match all values
        Point {x,  ..} => println!("x is {x}"),
    }
}

fn match_destructure()  {
    struct Point { x: i32, y: i32 }

    let p = Point {x: 0, y: 7};
    match p {
        Point {x, y : 0 } => println!("On the x axis at {x}"),
        Point {x : 0, y} => println!("On the y axis at {y}"),
        Point {x, y} => println!("On neither axis"),
    }
}

fn match_shadowing() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),

        // outer y is shadowed, because new "y" got binding to outer "x" 
        Some(y) => println!("Surprise, matched y = {y}"),

        // irrefutable pattern needed for code to compile
        _ => println!("Default Case, x = {:?}", x),
    }
}

fn match_literals()  {
    let x: u8 = 5;
    match x {
        0 | 1 => println!("matching <one or two>"),
        // no irrefrutable pattern needed as we cover all possible values of u8
        2..=255 => println!("matching three to seven"),
    }
}

fn refrutability() {
    // does not work because using a refrutable pattern in local binding
    let option : Option<u8> = None;
    // let Some(x) = option;

    // the solution is to use an if let statement that can skip if the pattern does not match
    if let Some(x) = option {
        println!("if the option is of type Some, then print value {x}");
    } else {
        println!("Value is not of type Some so ... {:?}", option);
    }
}

fn nested_tuples() {
    let tuple = ((1,2),3);
    println!("\n Nested tuple {:?}", tuple);
    println!("First element of nesten tuple is a tuple {:?}", tuple.0);
    println!("First element of of first element of nesten tuple is a basic value {:?}", tuple.0.0);
    println!("Second element of nesten tuple is a basic valie {:?}", tuple.1);
}

fn while_loop() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("Stack.pop returns -> {top}");
    }
}

fn for_loop()  {
    let vec = vec!['a', 'b', 'c'];
    for (idx, val) in vec.iter().enumerate() {
        println!("Index {idx} -> value {val}");
    }
}

