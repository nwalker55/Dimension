use eframe::egui;

use crate::gui::{
    new_project::n_proj,
    open_project::o_proj
};

// Start main menu
pub fn menu() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Dimension Game Engine",
        native_options,
        Box::new(|cc| Ok(Box::new(DimensionVars::new(cc)))),
    )
}

#[derive(Default)]

// Variables stored here
struct DimensionVars {
}

impl DimensionVars {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

 impl eframe::App for DimensionVars {
     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Scale UI
            ctx.set_pixels_per_point(1.5);


            // Main Menu Options
            ui.heading("Welcome to Dimension Game Engine!");
            ui.label("\n\n");
            if ui.button("New Project").clicked() {
                if let Err(why) = n_proj(ui) {
                    eprintln!("Could not start new project: {why}");
                    ui.label("Could not start new project: {why}");
                }
            }
            if ui.button("Open Project").clicked() {
                if let Err(why) = o_proj(ui) {
                    eprintln!("Could not open existing project: {why}");
                    ui.label("Could not open existing project: {why}");
                }
            }
        });
    }
}