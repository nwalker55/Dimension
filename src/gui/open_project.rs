use eframe::egui;
use std::sync::mpsc::Sender;

// Define custom events to send back to the main window
pub enum OpenWindowEvent {
    CloseRequested,
    // Add other events here, e.g., DataUpdated(String)
}

/// Renders the viewport content. This function is called every frame inside the closure.
pub fn open_show(ctx: &egui::Context, tx_open: Sender<OpenWindowEvent>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Open Project");
        ui.label("This interface logic lives entirely in a separate file.");

        ui.add_space(20.0);

        
    });
    if ctx.input(|i| i.viewport().close_requested()) {
        let _ = tx_open.send(OpenWindowEvent::CloseRequested);
    }

}
