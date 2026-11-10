use eframe::egui;
use std::{io::Result, path::PathBuf};

pub fn main(path: PathBuf, ctx: &egui::Context) -> Result<()> {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Dimension Project Editor");
        ui.label(format!("Project: {}", path.display()));
        ui.separator();
        ui.label("The project editor is ready for its first tools.");
    });

    Ok(())
}
