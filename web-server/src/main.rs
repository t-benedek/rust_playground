use std::{
    fs,
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
        .take_while(|line| !line.is_empty())
        .collect();

    // let status_line = "http/1.1 200 OK\r\n\r\nHELLO";
    let status_line = "http/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let len = contents.len();

    // let response = format!("{status_line}\r\n\r\nHELLO1");

    let response = format!(
        "{status_line}\r\n\
        Content-length: {len}\r\n\r\n\
        {contents}"
    );

    stream.write(response.as_bytes()).unwrap();
}
