use std::{
    fs::{self},
    io::{prelude::*, BufReader},   
    net::{TcpListener, TcpStream}, thread, time::Duration,
};

use web_server::ThreadPool;

const THREAD_POOL_NUMBER : usize = 5;
const THREAD_SLEEP_DURATION : u64 = 5;

// URL calls
const BASE_URL : &str = "127.0.0.1:7878";
const BASE_REQUEST : &str = "GET / HTTP/1.1";
const SLEEP_REQUEST : &str = "GET /sleep HTTP/1.1";
const OK_RESPONSE : &str = "HTTP/1.1 200 OK";
const ERR_RESPONSE : &str = "HTTP/1.1 404 NOT FOUND";
const HELLO_HTML : &str = "hello.html";
const ERR_HTML : &str = "404.html";

// This const is used to demonstrate when the webserver should initiate the shutdown process
const TEST_SHUTDOWN_AFTER_NUMBER_OF_THREADS : usize = 5;

fn main() {
    let listener = TcpListener::bind(BASE_URL).unwrap();
    let pool = ThreadPool::new(THREAD_POOL_NUMBER);

    // limit the for loop to only call 'TEST_SHUTDOWN_AFTER_NUMBER_OF_THREADS' items using 'take(num)'
    for stream in listener.incoming().take(TEST_SHUTDOWN_AFTER_NUMBER_OF_THREADS) {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_conection(stream);
        });
    }    

    println!("Shutting down.");
}

fn handle_conection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
        
    let (status_line, filename) = match &request_line[..] {
        BASE_REQUEST => (OK_RESPONSE, HELLO_HTML),
        SLEEP_REQUEST => {
            thread::sleep(Duration::from_secs(THREAD_SLEEP_DURATION));
            (OK_RESPONSE, HELLO_HTML)
        }
        _ => (ERR_RESPONSE, ERR_HTML),
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