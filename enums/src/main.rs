#[derive(Debug)]
enum Message {
    Quit, 
    Move {_x: i32, _y: i32}, 
    Write(String),
    ChangeColor(i32, i32, i32)    
}

impl Message {
    fn call(&self) {
        println!("This it the message : {:?}", self);
    }
}

fn main() {
    // How to construct these enum variants
    let quit = Message::Quit;
    let move_it = Message::Move { _x: 10, _y: 20 };
    let write = Message::Write(String::from("Hello"));
    let change = Message::ChangeColor(255, 255, 0);

    quit.call();
    write.call();
    change.call();
    move_it.call();

}


