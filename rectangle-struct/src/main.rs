#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    let rect = Rectangle {
        width : 500,
        height : 300
    };

    println!("\n rect has the dimensions {:?}", rect);
}
