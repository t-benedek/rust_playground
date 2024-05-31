const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let  x = 5;

    // shadow first x. This first one will be shadowed until the end
    let x = x +1;
    {
        //shadow second x in inner block. This will be available again after the inner block ends
        let x = x * 2;
        println!("\nThe value of inner x is {x}");
    }

    // second x visible again
    println!("The value of x is {x}\n");

    // shadowing makes it easy to reuse variable names
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is {spaces}\n");

    // explicitly assign a data type because parsing does not allow the compiler to deduce the data type
    let _guess : u32 = "42".parse().expect("Not a number");

    /* COMPOUND TYPES (tuple & array) */
    
    // 1. TUPLE
    println!("/***************/ ");
    println!("/*** TUPLES ****/ ");
    println!("/***************/ ");
    let tup: (i32, f64, u8) = (500, 6.4, 255);
    let first = tup.0;
    println!("first tuple item is < {first} >");

    let (mut x, _y , _z) = tup;
    println!("first tuple item after deconstructing and before changing < {x} >");

    x = 3;
    println!("first tuple item after changing is < {x} >\n");

    println!("/***************/ ");
    println!("/*** ARRAYS ****/ ");
    println!("/***************/ ");
    let a = [1,2,3,4,5];
    let first = a[0];
    println!("first array item is < {first} >");

}
