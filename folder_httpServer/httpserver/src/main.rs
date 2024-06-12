use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure we have the correct number of arguments
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <filename> [port]", args[0]);
        std::process::exit(1);
    }

    // Get the filename and port from the arguments
    let filename = &args[1];
    let port = if args.len() == 3 {
        &args[2]
    } else {
        "7878"
    };

    // Bind the TCP listener to the specified port
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).unwrap();
    println!("Server running on port {}", port);
    println!("URL : http://localhost:{}", port);
    println!("Press Ctrl+C to stop the server");

    // Loop to handle incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, filename);
    }
}

fn handle_connection(mut stream: TcpStream, filename: &str) {
    // Buffer to store the request
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Convert the buffer to a string for easier processing
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);

    // Craft a response
    let (status_line, contents) = if request.starts_with("GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", fs::read_to_string(filename).unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("404.html").unwrap())
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // Write the response to the stream
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
