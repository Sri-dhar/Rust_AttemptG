use egui::{CtxRef, SidePanel, TopPanel, CentralPanel, Window, ScrollArea, Label, Button, containers::{self, Container}};
use std::sync::Mutex;
use lazy_static::lazy_static;

extern crate lazy_static;

lazy_static! {
    static ref GLOBAL_STRING: Mutex<String> = Mutex::new(String::from("Hello, I am fine"));
    static ref STRR: Mutex<String> = Mutex::new(String::from("Hello, I am fine"));
}

fn main() {
    let mut egui_ctx = CtxRef::default();

    // You would normally run this in your event loop
    egui_ctx.begin_frame(Default::default());

    ui_example_system(&egui_ctx);

    // End the frame and get the output, in a real application you would use this to update your window
    let (_output, _paint_commands) = egui_ctx.end_frame();
    // Normally you would paint the shapes returned by end_frame() here.
}

fn ui_example_system(ctx: &CtxRef) {
    Window::new("Hello").show(ctx, |ui| {
        ui.label("world");
        ui.label("\nhow are you today?");
        ui.separator();
        ui.label("Global String");

        ui.collapsing("Click to see what is hidden!", |ui| {
            ui.label("Not much, as it turns out");
            ui.label("But you can put anything here");
        });

        ui.separator();
        let mut strr = STRR.lock().unwrap();
        ui.text_edit_singleline(&mut *strr);
        if ui.button("print").clicked(){
            println!("{}", *strr);
        }
        if ui.button("Click me")
            .on_hover_text("Hovering over the button")
            .clicked() {
            println!("Button clicked");
            println!("{}", *strr);
        }

        ui.separator();
        ui.separator();

        ui.separator();
    });
}