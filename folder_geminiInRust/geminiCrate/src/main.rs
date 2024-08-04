use ask_gemini::Gemini;
use std::fs;
use std::process::{Command, Stdio};
use std::io;
use std::io::{BufRead, BufReader, Error};

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


fn do_something(response: Vec<String>) {

//  need to do "export GEMINI_API_KEY=your_api_key" before running the code
    let mut stt : String = String::new();
    print!("Command: ");
    for st in response {
        let st = st.replace("```", "");
        print!("{}", st);
        stt.push_str(st.as_str());
    }
    println!();
    //executing the command in string st
    // let output = Command::new("sh")
    //     .arg("-c")
    //     .arg(stt)
    //     .output()
    //     .expect("failed to execute process");
    // println!("status: {}", output.status);
    if let Err(e) = run_command(stt.as_str()) {
        eprintln!("Error executing command: {}", e);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    //init model
    let gemini = Gemini::new(None, None);

    //read prompt prefix from file, what the model will actually do
    let prompt_prefix = fs::read_to_string("./prompt_context/context1.txt")?.trim().to_string();

    let prompt_content = "How to check my ipv6 address using ipconfig".to_string();
    let prompt = format!("{}{}", prompt_prefix, prompt_content);

    match gemini.ask(prompt.as_str()).await {
        Ok(response) => do_something(response),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
