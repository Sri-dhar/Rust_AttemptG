use eframe::egui;
use egui::TopBottomPanel;
use egui_extras;
use egui::{
    ViewportCommand,
    Button,
};

use std::process::Command;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    show_error: bool,
    var1 :i32,
    var2 :i32,
}

impl MyApp {
    fn add(&self) -> i32 {
        self.var1 + self.var2
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            show_error: true,
            var1: 10,
            var2: 20,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut _show_error = self.show_error;

        if _show_error{
            ctx.set_pixels_per_point(1.8);

            //top panel
            TopBottomPanel::top("Top Panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("CPI Calculator");
                    ui.label("Top Panel");
                    ui.add_space(10.0);
                });
            });

            egui::CentralPanel::default().show(ctx, |ui| {

                //bottom panel
                TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Bottom Panel");
                        ui.add_space(10.0);
                        ui.label("after space");
                    });
                });
                ui.horizontal(|ui| {
                    let name_label = ui.label("Your name: ");
                    ui.text_edit_singleline(&mut self.name)
                        .labelled_by(name_label.id);
                });
                ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                if ui.button("Increment").clicked() {
                    self.age += 1;
                }
                ui.label(format!("Hello '{}', age {}", self.name, self.age));
    
                //adding quit button
                if ui.button( "Quit App").clicked() {
                    self.show_error = false;
                }
                ui.add_space(3.0);
                ui.separator();
                ui.add_space(3.0);

                if ui.add_sized([100.0,100.0],Button::new("Quit and Rerun")).clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                    //send a terminal command to rerun this project using cargo run
                    Command::new("cargo")
                        .arg("run")
                        .spawn()
                        .expect("Failed to execute command");
                }

                ui.add_space(3.0);
                ui.separator();
                ui.add_space(3.0);

                ui.horizontal(
                    |ui|{
                    ui.label("Var1: ");
                    ui.add(egui::Slider::new(&mut self.var1, 0..=100));
                });

                ui.horizontal(
                    |ui|{
                        ui.label("Var2: ");
                        ui.add(egui::Slider::new(&mut self.var2, 0..=10));
                    }
                );
                
                ui.horizontal(
                    |ui|{
                        ui.label("Result: ");
                        ui.label(format!("{}",self.add()));
                    }
                );

                ui.add_space(5.0);
                let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
                ui.collapsing("Theme", |ui| {
                    theme.ui(ui);
                });

                ui.separator();

                //simply apply theme for light and dark mode in my app
                
            });
        }
        
    }
}


