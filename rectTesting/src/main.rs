use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::sync::Mutex;
use lazy_static::lazy_static;

//for draggable button
use crate::egui::Sense;


extern crate lazy_static;

lazy_static! {
    static ref GLOBAL_STRING: Mutex<String> = Mutex::new(String::from("Hello, I am fine"));
    static ref SSS: Mutex<f32> = Mutex::new(0.0);
    static ref STRR: Mutex<String> = Mutex::new(String::from("Hello, I am fine"));
}


fn main() {

    let mut s1 : String = GLOBAL_STRING.lock().unwrap().to_string();
    println!("{}", s1);
    let mut string_local = String::from("Hello hwo are you: ");
    s1 = string_local.clone();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}


fn ui_example_system(mut contexts: EguiContexts) {
    let mut strr = STRR.lock().unwrap();

    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
        ui.label("\nhow are you today?");

        let rect = ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        ui.separator();

        
    });
}

