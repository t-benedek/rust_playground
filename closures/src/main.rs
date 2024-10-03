use std::vec;

fn main() {
    println!("\n******** Closure definition ********");
    closure_fun();
    println!("**************************************\n");

    println!("\n******** Closure definition ********");
    closure_inference();
    println!("**************************************\n");

    println!("\n******** Closure Borrowing ********");
    cl_capture_immutable_ref();
    println!("**************************************\n");

    use_cl_move_thread();
}

fn closure_inference() {
    let example_closure = |x| x;
    let _s = example_closure(String::from("Hello"));
    let _i = example_closure(32_i8.to_string());
    
    // this won't compile because the compiler inferred that example_closure takes a String as argument
    // let _i = example_closure(32_i8);
}

fn closure_fun() {
    fn  add_one_fn     (x : u8) -> u8 {x + 1}
    let add_one_cl_1 = |x : u8| -> u8 {x + 1};
    let add_one_cl_2 = |x| x + 1_u8;

    println!("adding one to 7 with a function : {}", add_one_fn(7));
    println!("adding one to 7 with verbose  closure : {}", add_one_cl_1(7));
    println!("adding one to 7 with short closure : {}", add_one_cl_2(7));
}

fn cl_capture_immutable_ref() {
    let mut list = vec![1, 2, 3];
    println!("Before defining the closure: {:?}", list);

    let mut borrow_mutably = || list.push(7);
    
    // The println would also borrow "list" which leads to a compile error 
    // println!("Before calling the closure: {:?}", list);
    
    borrow_mutably();

    // This println is allowed because it occures after the mutable borrow in the closure is out of scope
    println!("After calling the closure: {:?}", list);
}

fn use_cl_move_thread() {
    use std::thread; 
    let list = vec![1, 2, 3];
    println!("Before defining the closure: {:?}", list);

    // keyword "move" is needed because thread could outlive the "list" variable
    thread::spawn(move || {
        println!("From thread: {:?}", list)
    }).join().unwrap();

}
