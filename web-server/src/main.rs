use std::{
    fs::{self, read_to_string},
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
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
        
    let (status_line, filename) = 
        if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();
    let len = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-length: {len}\r\n\r\n\
        {contents}"
    );

    stream.write(response.as_bytes()).unwrap();
    
}
