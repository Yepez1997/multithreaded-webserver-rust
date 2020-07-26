// single threaded web server in rust 

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

use hello::ThreadPool;
   

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // connect to port 7878 
    let pool = ThreadPool::new(4)
    // shutdown gracefully after 10 requests
    for stream in listener.incoming().take(15) {
        let stream = stream.unwrap(); // returns tcp stream
        // handle_tcp_connection_final(stream);
        pool.execute(|| { 
            handle_tcp_connection_final(stream);
        });
    }

}

// handle tcp connection from client 
fn handle_tcp_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // read stream into a buffer 
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}


// handle tcp connection from client + send response
fn handle_tcp_connection_response(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap(); // read into buffer
    let response = "HTTP/1.1 200 OK\r\n\r\n"; // uri string to return to the client
    stream.write(response.as_bytes());
    stream.flush().unwrap(); // flush output 
}

// handle tcp connection from client + send html response 
fn handle_tcp_connection_html_response(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {

        let contents = fs::read_to_string("../serve.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
           
    } else {
        // other requests
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("../404.html").unwrap();
        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}


// refractored version
fn handle_tcp_connection_final(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (status, file) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "../serve.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "../404.html")
    };
    let contents = fs::read_to_string(file).unwrap();
    let response = format!("{}{}", status, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}



