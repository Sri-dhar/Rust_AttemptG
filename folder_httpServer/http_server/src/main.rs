use std::env;
use std::fs;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use http_server::ThreadPool;

fn main() {
    let default_port = "7878";
    let default_html_file = "/home/solomons/Rust_AttemptG/folder_httpServer/HTTP-Server/test.html";
    let mut port = String::from(default_port);
    let mut html_file_path = String::from(default_html_file);

    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        if arg.starts_with("--port=") {
            port = arg.trim_start_matches("--port=").to_string();
        } else if arg.ends_with(".html") {
            html_file_path = arg.clone();
        }
    }

    let url = format!("127.0.0.1:{}", port);
    println!("Running on http://{}", url);
    println!("Serving file: {}", html_file_path);

    let listener = TcpListener::bind(url).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let html_file_path_clone = html_file_path.clone();

        pool.execute(move || {
            handle_connection(stream, html_file_path_clone);
        });
    }
}

fn handle_connection(mut stream: TcpStream, html_file_path: String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_root = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK\r\n\r\n", html_file_path)
    } else {
        // Default case if not root
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", String::from("/home/solomons/Rust_AttemptG/folder_httpServer/HTTP-Server/404.html"))
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| String::from("Error loading file."));
    let response = format!("{}{}", status_line, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}