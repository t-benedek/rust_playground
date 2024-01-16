use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[u8]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn reverse(pair: (i8, bool)) -> (bool, i8) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
} 

fn handle_pair(pair: (i8, bool)) {
    let rev = reverse(pair);
    let b: bool = rev.0;
    println!("\n -----------------------");
    println!("\nReverted Result (127,true) : {:?}", pair);
    println!("First part of Result : {:?}", b);
}

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
    let result = (127, true);
    handle_pair(result);
    let xs: [u8; 6] = [1,2,3,4,5,255];

    println!("-----------------------");
    println!("\n All elements of the Array {:?}", xs);
    println!("Sixth element of Array {}", xs[5]);
    println!("Array occupies {} bytes \n", mem::size_of_val(&xs));

    println!("-----------------------");
    // Arrays can be automatically borrowed as slices.
    analyze_slice(&xs[0 .. 6]);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => print!("{}: {}; ", i, xval),
            None => println!("\nSlow down! {} is too far!", i),
        }
    }

    let input = "ALERT".to_string();
    println!("Palindrom is {:?}", palindrom(input));
}
