use core::f32;
use eframe::egui;
use egui::ScrollArea;
use egui::{Key, TopBottomPanel};
use std::env;
use std::io::{self};
use std::path::PathBuf;
use std::process::Command;

pub struct CommandInstace {
    counter: i32,
    command: String,
    output: String,
    status: String,
}

pub struct MyApp {
    show_error: bool,
    command_input: String,
    last_ran_cmd: String,
    send_button_pressed: bool,
    commands: Vec<CommandInstace>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_error: true,
            command_input: String::new(),
            last_ran_cmd: String::new(),
            send_button_pressed: false,
            commands: Vec::new(),
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

        if !stderr.is_empty() {
            Err(io::Error::new(io::ErrorKind::Other, stderr))
        } else {
            Ok(stdout.trim().to_string())
        }
    }
}

impl MyApp {
    fn custom_command_handling(&mut self, command: String) -> bool {
        let args: Vec<&str> = command.split_whitespace().collect();
        let (cmd, rest) = args.split_at(1);
        let cmd = cmd[0];
        let rest = rest.join(" ");

        if cmd == "clear" {
            self.commands.clear();
            self.command_input.clear();
            return true;
        }

        if cmd == "exit" {
            std::process::exit(0);
            return true;
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
            status: "status".to_string(),
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
                    ui.label("Last Command: ");
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
                        .show(ui, |ui| {
                            for command in &self.commands {
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{}", command.counter));
                                        ui.add_space(3.0);
                                        ui.horizontal_wrapped(|ui| {
                                            ui.label(format!("{}", command.command));
                                            ui.add_space(3.0);
                                        });
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{}", command.output));
                                        ui.label(format!("{}", command.status));
                                    });
                                    ui.separator();
                                });
                            }
                        });
                });
            });

            // Bottom panel
            TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
                ui.monospace(format!("{}: ", env::current_dir().unwrap().display()));
                ui.horizontal(|ui| {
                    let text_area = ui.text_edit_singleline(&mut self.command_input);
                    let b1 = ui.button("Send").on_hover_text("Send");

                    if b1.clicked() || self.send_button_pressed {
                        self.handle_send_command();
                    }

                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        self.last_ran_cmd = self.command_input.clone();
                        self.send_button_pressed = true;
                        text_area.request_focus();
                    }
                });
            });
        }
    }
}
