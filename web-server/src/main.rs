use std::{
    io::{prelude::*, BufReader},   
    net::{TcpListener, TcpStream}, result,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conection(stream);
    }    
}

fn handle_conection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .collect();

    println!("Reuqest: {:#?}", http_request); 
}
