use eframe::egui;
use eframe::App;

pub struct MyApp;

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, World!");
        });
    }

    fn name(&self) -> &str {
        "My App"
    }
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn run() {
    eframe::start_web(Box::new(|_cc| Box::new(MyApp)));
}
