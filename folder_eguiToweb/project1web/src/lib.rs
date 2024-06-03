use eframe::egui;
use eframe::App;

pub struct MyApp;

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, World!");
        });
    }
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn run() {
    // The canvas ID should match the ID of the canvas element in your HTML
    eframe::start_web("my_canvas_id", Box::new(|_cc| Box::new(MyApp)));
}
