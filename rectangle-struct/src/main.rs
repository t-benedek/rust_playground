#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width : dbg!(500 * scale),
        height : 300
    };

    println!("\n rect has the dimensions {:#?}", rect);

    println!("\n The area of the rect is {} square pixels", rect.area());
}
