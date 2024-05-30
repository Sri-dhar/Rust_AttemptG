mod functions;
mod semdata;
use std::sync::Mutex;
use eframe::egui;
use egui::TopBottomPanel;
// use egui_extras;
// use egui::{Button, ViewportCommand};

extern crate lazy_static;

lazy_static::lazy_static! {
    static ref SCALE: Mutex<f32> = Mutex::new(1.5);
}

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CPI/SPI Calculator",
        options,
        Box::new(|cc| {
            Box::new(MyApp::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    show_error: bool,
    var1: i32,
    var2: i32,
    show_scale_window: bool,
    scale_input: String,
    option_cpi_spi: i32,
    sem_no: i32,
    sem_no_f32 : f32,
    sem_no_str : String,
    sem_option : i32,
}

// impl MyApp {
//     fn add(&self) -> i32 {
//         self.var1 + self.var2
//     }
// }

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            show_error: true,
            var1: 10,
            var2: 20,
            show_scale_window: false,
            scale_input: "1.2".to_string(),
            option_cpi_spi: -1,
            sem_no: 0,
            sem_no_f32: 0.0,
            sem_no_str: "0".to_string(),
            sem_option: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let scale = *SCALE.lock().unwrap();
        let scale = functions::read_data();
        *SCALE.lock().unwrap() = scale;
        ctx.set_pixels_per_point(scale);

        TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Menu", |ui| {
                    if ui.button("Change Scale").clicked() {
                        self.show_scale_window = true;
                    }
                });
            });
        });


        if self.show_scale_window {
            egui::Window::new("Change Scale").show(ctx, |ui| {
                ui.label("Enter the scale between 0.5 and 2.5");

                if ui.text_edit_singleline(&mut self.scale_input).lost_focus() {
                    if let Ok(new_scale) = self.scale_input.parse::<f32>() {
                        if new_scale >= 0.5 && new_scale <= 2.5 {
                            *SCALE.lock().unwrap() = new_scale;
                            functions::write_data(new_scale);

                        }
                    }
                }

                if ui.button("Close").clicked() {
                    self.show_scale_window = false;
                }
            });
        }


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CPI/SPI Calculator");
            ui.separator();

            egui::ComboBox::from_label("Select Calculation")
                .selected_text(match self.option_cpi_spi {
                    0 => "Calculate SPI",
                    1 => "Calculate CPI",
                    _ => "Select",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.option_cpi_spi, 0, "Calculate SPI");
                    ui.selectable_value(&mut self.option_cpi_spi, 1, "Calculate CPI");
                });
            
            if self.option_cpi_spi == 0 || self.option_cpi_spi == 1{
                ui.horizontal(
                    |ui| {
                        ui.label("Select the semester :");
                        ui.add(egui::Slider::new(&mut self.sem_no, 1..=8).text("Semester"));                         
                    }
                );
                ui.horizontal(
                    |ui| {
                        if self.sem_no == 7 {
                            ui.label("Select an option");
                            egui::ComboBox::from_label("")
                                .selected_text(match self.sem_option {
                                    1 => "Option 1",
                                    2 => "Option 2",
                                    _ => "Select",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.sem_option, 1, "Option 1");
                                    ui.selectable_value(&mut self.sem_option, 2, "Option 2");
                                });
                        }
                        else if self.sem_no == 8 {
                            ui.label("Select an option");
                            egui::ComboBox::from_label("")
                                .selected_text(match self.sem_option {
                                    1 => "Option 1",
                                    2 => "Option 2",
                                    3 => "Option 3",
                                    _ => "Select",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.sem_option, 1, "Option 1");
                                    ui.selectable_value(&mut self.sem_option, 2, "Option 2");
                                    ui.selectable_value(&mut self.sem_option, 3, "Option 3");
                                });
                        }
                    }
                ); 
                
                ui.horizontal(|ui| {
                    self.sem_no_f32 = self.sem_no as f32 ;
                    ui.label(format!("Semester : {}", self.sem_no).to_string());
                    if self.sem_no == 7 || self.sem_no == 8
                    {
                        self.sem_no_f32 = self.sem_no_f32 + 0.1 * self.sem_option as f32;
                        ui.label(format!("Option {}", self.sem_option).to_string());
                    }
                });
            }

        });

        egui::TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(5.0);
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                };
                ui.add_space(2.0);
            });
        });

    }
}
