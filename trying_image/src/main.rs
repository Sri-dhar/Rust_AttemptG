use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, apper)
        .run();
}

fn apper(mut contexts: EguiContexts) {

    // Load the image
    let img = image::open("/home/reflore/Documents/pikachu.png").unwrap();

    egui::Window::new("Image insertion")
        .default_size([500.0, 500.0])
        .show(contexts.ctx_mut(),|ui|
            {
                ui.label("Helo how are you");
                ui.separator();
                ui.add(
                    egui::Image::new(egui::include_image!("/home/reflore/Documents/pikachu.png"))

                );
                ui.image("/home/reflore/Documents/pikachu.png");
            });
}