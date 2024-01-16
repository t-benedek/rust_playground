fn fibonacci50() -> u64 {
    let mut r1 = 0;
    let mut r2 = 1;
    let mut d;
    for _i in 0..48 {
        d = r2;
        r2 = r2 + r1;
        r1 = d;
    }
    return r2;
}

fn main() {
    let result = fibonacci50();
    println!("{:?}", result);
}
