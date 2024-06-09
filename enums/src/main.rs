#[derive(Debug)]
enum Message {
    Quit, 
    Move {_x: i32, _y: i32}, 
    Write(String),
    ChangeColor(i32, i32, i32)    
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("This it the Quit  message"),
            Message::Write(msg) => println!("This it the Write  message with message : {msg}"),
            Message::ChangeColor(r,g,b) => println!("This it the ChangeColor message with color values: {r} {g} {b}"),
            Message::Move{_x, _y} => println!("This it the Move message with values {_x} {_y}"),

            // you can put this as a default value for message types that do not match any other match arm
            // other => println!("Message not handled {:?}", other),
        }

        if let Message::Write(msg) = self {
            println!("<if let ...> can be used as a shortcut for the match statement if you only use one value: Write({msg})")
        }
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


