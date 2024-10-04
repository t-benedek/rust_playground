

fn main() {

    use_iterator();

    comsume_iterator();

    adapt_iterator();

}

fn comsume_iterator() {
    let items = vec![11, 22, 33];
    
    // function sum() is "comsuming" the iterator and so taking ownership
    let iter_sum = items.iter();
    let total : u32 = iter_sum.sum();
    println!("Sum of all items in the Vector is {}", total);
}

fn use_iterator() {
    let items = vec![11, 22, 33];
    // we need to make the iterator mutable to be able to call iter.next() as we change the internal state
    // if we only use the for loop, this one takes ownership of the iterator and internally marks it as mutable for us
    let mut iter = items.iter();
    
    // using next() and inspect() with closure for testing purposes
    let first = iter.next();
    first.inspect(|x| println!("Value of 1. element: {} ", x));
    
    // iterating over all the values without the first one that we already printed
    let mut cnt = 1_u8;
    for val in iter {
        cnt += 1;
        println!("Value of {}. element: {} ", cnt, val);
    }
}

fn adapt_iterator() {
    let items = vec![11, 22, 33];
    
    // map() is called an "iterator adapter" and does not comsume the iterator. 
    // It creates a new iterator instead leveraging the closure as parameter
    // collect() is consuming the iterator and collects all values in a colelction data type. 
    let map_vec: Vec<u32> = items.iter().map(|x| x + 13).collect();
    println!("New Vector {:?} with each element incremented by 13 {:?}", items, map_vec);

    // using filter() as another example of iterator adapter
    // we have to use into_iter() because the iterator has to take over ownership of the vector
    let map_vec: Vec<u32> = items.clone().into_iter().filter(|x| *x > 20).collect();
    println!("Filtering the Vector {:?} for values over 20 -> {:?}", items, map_vec);
}
