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
    println!("\n\n**********************************");
    println!("** First we will revert a tuple **");
    println!("**********************************");
    println!("Reverted Result (127,true) : {:?}", rev);
    println!("Access first part of Tuple rev with rev.0 : {:?}\n", rev.0);
}


fn main() {
    let result = (127, true);
    handle_pair(result);
    
    let xs: [u8; 6] = [1,2,3,4,5,255];
    println!("***************************************");
    println!("** Then lets play around wtih Arrays **");
    println!("***************************************");
    println!("All elements of the Array {:?}", xs);
    println!("Sixth element of Array {}", xs[5]);
    println!("Array occupies {} bytes \n", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("****************************************************");
    println!("** Arrays can be automatically borrowed as slices **");
    println!("****************************************************");
    analyze_slice(&xs[0 .. 6]);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => print!("{}: {}; ", i, xval),
            None => println!("\nSlow down! {} is too far!\n\n ", i),
        }
    }
}
