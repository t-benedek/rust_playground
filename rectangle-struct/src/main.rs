#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width : dbg!(500 * scale),
        height : 300
    };

    println!("\n rect has the dimensions {:#?}", rect);
}
