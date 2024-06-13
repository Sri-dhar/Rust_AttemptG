# Rust HTTP Server

This project is a basic, multithreaded HTTP server written in Rust. It showcases the power of Rust's concurrency features and its suitability for network programming. This project was developed with the help of the Rust language documentation, ensuring best practices and efficient use of Rust's features. Below are some key points about the project:

## Features

- **Multithreaded Handling**: Utilizes a thread pool to handle incoming HTTP requests concurrently, improving the server's ability to manage multiple connections at once.
- **Customizable Port and HTML File Serving**: Supports command-line arguments to specify the server port and the HTML file to serve, making it flexible for different environments and use cases.
- **Error Handling**: Implements error handling for scenarios such as failing to bind to a port or read the specified HTML file, ensuring the server runs smoothly.

## Technical Details

- **Rust Edition**: Developed using Rust 2021 edition, leveraging the latest features and improvements of the language.
- **Dependencies**: Minimal dependencies, focusing on the standard library to demonstrate Rust's built-in capabilities for network programming and concurrency.
- **ThreadPool Implementation**: Includes a custom `ThreadPool` implementation to manage worker threads efficiently.

## Learning Resources

This project was written with the help of the Rust language documentation.  You can access it here: [The Rust Programming Language Book](https://doc.rust-lang.org/book/).

## Getting Started

To run the server:

1. Ensure Rust is installed on your system.
2. Clone the repository and navigate to the project directory.
3. Build the project using `cargo build --release`. This will compile the project in release mode, optimizing the executable for performance.
4. Run the server with `cargo run --release`. Optionally, specify the port and HTML file to serve using command-line arguments (e.g., `cargo run --release -- --port=8080 --file=index.html`).
5. Alternatively, for avoiding building the project again and again, after building once, you can go to the `target/release` folder and run the executable directly. The command would be `./target/release/http_server`



