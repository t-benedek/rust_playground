use std::slice;
fn main() {
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = split_at_mut(r, 3);
    println!("\n{:?}, {:?} \n", a , b);
    
    extern_c_call();
    
    raw_pointer();
}

fn raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    let address = 0x01235_usize;
    let _a = address as *const i32;

    unsafe {
        print!("const r1: {} // ", *r1);
        println!("mutable r2: {}\n", *r2);

        // seg fault in unsafe call
        // println!("ref to address: {}", *_a);
    }
}

fn extern_c_call() {
    unsafe {
        println!("Abolute value of -3 acording to C: {}\n", abs(-3));
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn split_at_mut(
    values: &mut [i32],
    mid: usize
) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    
    // get raw pointer to address of values array
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);

    // this does not work because RUST does not know how to handle splitting "values" in two seperate slices
    // compiler is saying "second mutable borrow occurs here"
    // return (&mut values[..mid], &mut values[mid..]);

    unsafe{
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
    
}
