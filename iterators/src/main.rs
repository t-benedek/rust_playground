

fn main() {
    let items = vec![11, 22, 333444];
    print_values(items);
}

fn print_values(items: Vec<u32>) {
    let mut iter = items.iter();
    
    // using next() and inspect() with closure for testing purposes
    let first = iter.next();
    first.inspect(|x| println!("Value of 1. element: {} ", x));
    
    // iterating over all the values without the first one that we already printed above
    let mut cnt = 1_u8;
    for val in iter {
        cnt += 1;
        println!("Value of {}. element: {} ", cnt, val);
    }
}
