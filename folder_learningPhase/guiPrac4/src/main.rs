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
        ui.separator();
        ui.label("Global String");

        ui.collapsing("Click to see what is hidden!", |ui| {
            ui.label("Not much, as it turns out");
            ui.label("But you can put anything here");
        });

        ui.separator();
        ui.text_edit_singleline(&mut *strr);
        if ui.button("print").clicked(){
            println!("{}", *strr);
        }
        if ui.button("Click me")
            .on_hover_ui(|ui|{
            ui.label("Hovering over the button");
            })
            .clicked() {
            println!("Button clicked");
            println!("{}", *strr);
            }

        ui.separator();
        ui.separator();

        // let draggable_button:egui::Button = egui::Button::new("Square").sense(Sense::drag());
        //
        // // Check if button was dragged
        // if draggable_button.drag_started() {
        //     println!("Button dragged");
        // }
        // ui.label("Region 1");
        // let rect = ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        // if rect.clicked(){
        //     println!("rectangular region clicked");
        // }
        // if rect.hovered(){
        //     // ui.separator();
        //     // ui.label("Hovering over the rectangular region");
        //     println!("Hovering over the rectangular region");
        // }
        // ui.separator();
        // ui.label("Region 2");
        // let rect2 = ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        // if rect2.clicked(){
        //     println!("rectangular region 2 clicked");
        // }
        // if rect2.hovered(){
        //     // ui.separator();
        //     // ui.label("Hovering over the rectangular region");
        //     println!("Hovering over the rectangular region 2");
        // }
        //
        ui.separator();
        // let rect3 = ui.allocate_rect(egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(50.0, 50.0)), egui::Sense::hover());


        // PAINTER WINDOW
        // let (response, painter) = ui.allocate_painter(
        //     bevy_egui::egui::Vec2::new(ui.available_width(), 300.0),
        //     Sense::hover(),
        // );


        let (response, painter) = ui.allocate_painter(
            bevy_egui::egui::Vec2::new(ui.available_width(), 300.0),
            Sense::hover(),
        );

        // Draw a rectangle
        let rect = egui::Rect::from_min_size(egui::Pos2::new(50.0, 50.0), egui::Vec2::new(100.0, 100.0));
        // let fill = egui::Color32::RED;
        // painter.rect_filled(rect, 0.0, fill);
        ui.label("Region 1");
        // // Draw a line
        // let line_color = egui::Color32::GREEN;
        // let line_start = egui::Pos2::new(0.0, 0.0);
        // let line_end = egui::Pos2::new(100.0, 100.0);
        // painter.line_segment([line_start, line_end], egui::Stroke::new(2.0, line_color));

        ui.separator();
        // This is the equivalent of `flexDirection: row` in CSS
        ui.horizontal(|ui| {
            ui.label("Same");
            ui.label("row");
            let rect = ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            if rect.hovered(){
                println!("Hovering over the rectangular region");
            }
        });

        // This is the equivalent of `flexDirection: column` in CSS
        ui.vertical(|ui| {
            ui.label("Same");
            ui.label("column");
        });


    });
}