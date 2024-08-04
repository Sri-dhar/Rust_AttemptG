use std::process::Command;
use std::str;

fn main() {
    // let input_string = "how to delete each and every .git file recusively from all my folders in my current directory"; // Replace with actual input
    let mut input_string = String::new();
    println!("Enter your query: ");
    std::io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let output = Command::new("python")
        .arg("/home/solomons/temp/rust-gemini/src/callingGemini.py") // Replace with the path to your Python script
        .arg(input_string)
        .output()
        .expect("Failed to execute command");

    let response_text = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");
    println!("Response from Python script: {}", response_text);
}
