use std::io::{self, Write};
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::process::Command;

struct CustomCommand {
    name: String,
    path: String,
}

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_VECTOR: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref GLOBAL_SavedCommands: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref COMMANDS: Mutex<HashMap<String, CustomCommand>> = Mutex::new(HashMap::new());
}

fn add_function(name: &str, path: &str) {
    let mut commands = COMMANDS.lock().unwrap();
    commands.insert(name.to_string(), CustomCommand {
        name: name.to_string(),
        path: path.to_string(),
    });
}

fn execute_command(name: &str) {
    let commands = COMMANDS.lock().unwrap();
    if let Some(command) = commands.get(name) {
        let output = Command::new(&command.path)
            .output()
            .expect("Failed to execute command");

        let output = String::from_utf8_lossy(&output.stdout);
        println!("{}", output);
    } else {
        println!("Command not found");
    }
}

fn write_to_bashrc(input: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(".bashrc")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", input) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn loop_to_get_input() {
    let mut global_string_vector = GLOBAL_VECTOR.lock().unwrap();
    loop {
        print!("Termie >> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            println!("Invalid command");
            continue;
        }
        let (command, args) = parts.split_at(1);
        match command {
            ["exit"] => break,
            ["add_function"] => {
                if args.len() != 2 {
                    println!("add_function requires exactly two arguments");
                } else {
                    add_function(args[0], args[1]);
                }
            },
            _ => {
                execute_command_with_args(command[0], args);
                global_string_vector.push(input.to_string());
                //write the command to the .bashrc file
                write_to_bashrc(input);
            },
        }
    }
}

fn main() { 
    let now: DateTime<Local> = Local::now();
    write_to_bashrc(now.to_string().as_str());
    loop_to_get_input();
    let mut global_string_vector = GLOBAL_VECTOR.lock().unwrap();
    let mut global_command_vector = GLOBAL_SavedCommands.lock().unwrap();
    //input values into global command vector like ls cd mkdir and exit
    global_command_vector.push("ls".to_string());
    global_command_vector.push("cd".to_string());
    global_command_vector.push("mkdir".to_string());
    global_command_vector.push("exit".to_string());
}
