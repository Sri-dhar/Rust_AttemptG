use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

    fn execute_command(command: &str, cwd: &mut PathBuf) -> io::Result<String> {
        let args: Vec<&str> = command.split_whitespace().collect();
        let (cmd, rest) = args.split_at(1);
        let cmd = cmd[0];
        let rest = rest.join(" ");

        if cmd == "cd" {
            if rest.is_empty() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "cd requires a path"));
            }
            
            let new_cwd = if rest.starts_with('/') {
                PathBuf::from(&rest)
            } else if rest.starts_with('.') {
                PathBuf::from(cwd.join(&rest))
            } else {
                cwd.join(&rest)
            };

            match env::set_current_dir(&new_cwd) {
                Ok(_) => {
                    *cwd = env::current_dir()?;
                    Ok("Directory changed".to_string())
                }
                Err(e) => Err(e),
            }
        } else {
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(cwd)
                .output()?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if !stderr.is_empty() {
                Err(io::Error::new(io::ErrorKind::Other, stderr))
            } else {
                Ok(stdout.trim().to_string())
            }
        }
    }

fn main() {
    let stdin = io::stdin();
    let mut cwd = env::current_dir().unwrap(); // Start with the current directory

    loop {
        print!("{}>>): ", cwd.display());
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();

        if command == "exit" {
            break;
        }

        match execute_command(command, &mut cwd) {
            Ok(output) => println!("Output:\n{}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
