use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::sync::Mutex;
use lazy_static::lazy_static;


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
    // let mut s1 : String = GLOBAL_STRING.lock().unwrap().to_string();
    //declare a empty string
    let mut s1 = STRR.lock().unwrap();
    let mut booll = true;
    let mut my_f32 = 0.0;
    // let mut vvv = 0.0;
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
        ui.label("\nhow are you today?");
        ui.hyperlink("https://bevyengine.org");
        //add image of pikachu.jpg in the project file
        // let image = ui.image(egui::TextureId::User(0), [100.0, 100.0]);
        ui.label("Pikachu");
        ui.label("This is a Pikachu");
        ui.separator();

        ui.text_edit_singleline(&mut *s1);
        if ui.button("print").clicked(){
            println!("{}", s1);
        }


        ui.checkbox(&mut booll, "check me");

        if ui.button("Click me").clicked() {
            println!("Button clicked");
            println!("{}", s1);
            println!("{}", booll);
        }

        ui.separator();

        ui.label("Global String");

        ui.collapsing("Click to see what is hidden!", |ui| {
            ui.label("Not much, as it turns out");
            ui.label("But you can put anything here");
        });

        // ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0));
        // ui.add(egui::DragValue::new(&mut my_f32));
        //
        // if ui.button("PassingSliderVal").on_hover_text("Passing the value of the slider to the global string").clicked() {
        //     println!("Button clicked");
        //     println!("The Value is : {}", my_f32);
        //     ui.label("The Value is :");
        //     ui.label(my_f32.to_string());
        // }

        ui.separator();

        //using a  global varible makes the slider hold the value of the slider
        //another way is to use structures "Struct" which may cause errors like trait not implemented, etc
        let mut sss = SSS.lock().unwrap();
        ui.add(egui::Slider::new(&mut *sss, 0.0..=100.9));
        ui.add(egui::DragValue::new(&mut *sss));

        if ui.button("PassingSliderVal").on_hover_text("Passing the value of the slider to the global string").clicked() {
            println!("Button clicked");
            println!("The Value is : {}", *sss);
            ui.label("The Value is :");
            ui.label(sss.to_string());
        }
        
        ui.separator();
        //draggable button
        // ui.label("Draggable Button");
        // let b1 = ui.button("Square").sense(Sense::drag());
        //
        // // Check if button was dragged
        // if b1.drag_started() {
        //     println!("Button dragged");
        // }


        
    });
}