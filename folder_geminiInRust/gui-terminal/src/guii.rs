use core::f32;
use eframe::egui;
use egui::{Key, ScrollArea, TopBottomPanel};
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use ask_gemini::Gemini;
use std::io::{self,Error};


pub struct CommandInstace {
    counter: i32,
    command: String,
    output: String,
    // status: String,
}

pub struct MyApp {
    show_error: bool,
    command_input: String,
    last_ran_cmd: String,
    send_button_pressed: bool,
    commands: Vec<CommandInstace>,
    text_area_id: Option<egui::Id>,
    request_focus_next_frame: bool,
    gemini_response : String,
    gemini_input : String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_error: true,
            command_input: String::new(),
            last_ran_cmd: String::new(),
            send_button_pressed: false,
            commands: Vec::new(),
            text_area_id: None,
            request_focus_next_frame: false,
            gemini_response: String::new(),
            gemini_input: String::new(),
        }
    }
}

fn execute_command(command: &str, cwd: &mut PathBuf) -> io::Result<String> {
    let args: Vec<&str> = command.split_whitespace().collect();
    let (cmd, rest) = args.split_at(1);
    let cmd = cmd[0];
    let rest = rest.join(" ");

    if cmd == "cd" {
        if rest.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cd requires a path",
            ));
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

        // println!("stdout: {}", stdout);
        // println!("stderr: {}", stderr);

        if !stderr.is_empty() {
            Err(io::Error::new(io::ErrorKind::Other, stderr))
        } else {
            Ok(stdout.trim().to_string())
        }
    }
}

// #[tokio::main]
// async fn callGemini(input: String) -> Result<(), Error> {
//     //need to export your api key as
//     //export GEMINI_API_KEY = your_api_key
//     let gemini = Gemini::new(None, None);
//     let prompt_prefix = fs::read_to_string("./../prompt_context/context1.txt")?.trim().to_string();

//     Ok()
// }

fn process(input :Vec<String>) -> String{
    let mut output = String::new();
    for i in input{
        output.push_str(i.as_str());
    }
    output
}

async fn callGemini(prompt_content: String) -> Result<String, Error> {
    let gemini = Gemini::new(None, None);
    let prompt_prefix = fs::read_to_string("./../prompt_context/context1.txt")?.trim().to_string();
    let prompt = format!("{}{}", prompt_prefix, prompt_content);

    match gemini.ask(prompt.as_str()).await {
        Ok(response) => Ok(process(response)),
        Err(_e) => Err(io::Error::new(io::ErrorKind::InvalidData, "Response could not be fetched")),
    }
}

impl MyApp {
    fn custom_command_handling(&mut self, command: String) -> bool {
        let args: Vec<&str> = command.split_whitespace().collect();
        let (cmd, _rest) = args.split_at(1);
        let cmd = cmd[0];    // status: String,

        let _rest = _rest.join(" ");

        if cmd == "clear" {
            self.commands.clear();
            self.command_input.clear();
            return true;
        }

        if cmd == "exit" {
            std::process::exit(0);
        }

        false
    }

    fn handle_send_command(&mut self) {
        println!("Command: {}", self.command_input);
        if self.command_input.is_empty() {
            self.send_button_pressed = false;
            return;
        }
        if self.custom_command_handling(self.command_input.clone()) {
            self.send_button_pressed = false;
            return;
        }

        let mut cwd = env::current_dir().unwrap();

        let output = match execute_command(&self.command_input, &mut cwd) {
            Ok(output) => output,
            Err(e) => e.to_string(),
        };
        self.commands.push(CommandInstace {
            counter: self.commands.len() as i32,
            command: self.command_input.clone(),
            output: output,
            // status: "status".to_string(),
        });

        self.command_input.clear();
        self.send_button_pressed = false;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut _show_error = self.show_error;
        if _show_error {
            ctx.set_pixels_per_point(1.5);

            // Top panel
            TopBottomPanel::top("Top Panel").show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(">> ");
                    ui.monospace(self.last_ran_cmd.clone());
                    ui.add_space(10.0);
                });
            });

            // Central panel with scroll area
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical(|ui| {
                    ScrollArea::vertical()
                        .max_width(f32::INFINITY)
                        .animated(true)
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            for command in &self.commands {
                                ui.separator();
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{}", command.counter));
                                        ui.add_space(3.0);
                                        ui.horizontal_wrapped(|ui| {
                                            ui.label(format!("{}", command.command));
                                            ui.add_space(3.0);
                                        });
                                    });
                                    // ui.horizontal(|ui| {
                                    ui.label(format!("{}", command.output));
                                    // ui.label(format!("{}", command.status));
                                    // });
                                });
                            }
                            ui.add_space(40.0);
                        });
                });
            });

            // Bottom panel
            TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
                // showing current directory path
                ui.monospace(format!("{}: ", env::current_dir().unwrap().display()));
                ui.horizontal(|ui| {
                    let text_edit = egui::TextEdit::singleline(&mut self.command_input)
                        .desired_width(ui.available_width() - 129.2);
                    let text_area = ui.add(text_edit);

                    self.text_area_id = Some(text_area.id);

                    let b1 = ui.button("Send").on_hover_text("Send");
                    let b2 = ui.button("Ask Gemini").on_hover_text("Ask Gemini");

                    if b1.clicked() || self.send_button_pressed {
                        self.handle_send_command();
                    }

                    if b2.clicked() {
                        // callGemini(self.command_input.clone());
                    }

                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        self.last_ran_cmd = self.command_input.clone();
                        self.send_button_pressed = true;
                        // Set a flag to request focus in the next frame
                        self.request_focus_next_frame = true;
                    }
                });
            });

            if self.request_focus_next_frame {
                if let Some(id) = self.text_area_id {
                    ctx.memory_mut(|memory| memory.request_focus(id));
                }
                self.request_focus_next_frame = false;
            }
        }
    }
}
