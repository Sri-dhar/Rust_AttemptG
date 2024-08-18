use ask_gemini::Gemini;
use std::fs;
use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader};

fn run_command(command_string: &str) -> io::Result<()> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command_string)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.as_mut().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to open stdout"))?;
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let exit_status = child.wait()?;
    println!("\nCommand exited with status: {}", exit_status);

    Ok(())
}

fn do_something(response: Vec<String>) -> Result<(), io::Error> {
    let mut stt: String = String::new();
    print!("Command: ");
    for st in response {
        let st = st.replace("```", "");
        print!("{}", st);
        stt.push_str(st.as_str());
    }
    println!();
    
    if let Err(e) = run_command(&stt) {
        eprintln!("Error executing command: {}", e);
        return Err(e);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize model
    let gemini = Gemini::new(None, None);

    // Read prompt prefix from file
    let prompt_prefix = fs::read_to_string("./prompt_context/context1.txt")?.trim().to_string();

    let prompt_content = "how to see all the python environments in my pc using venv".to_string();
    let prompt = format!("{}{}", prompt_prefix, prompt_content);

    match gemini.ask(&prompt).await {
        Ok(response) => do_something(response)?,
        Err(e) => eprintln!("Error asking Gemini: {}", e),
    }

    Ok(())
}
