use crate::gui::proj_editor::main::main;
use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub enum ProjectWindowEvent {
    OpenProject(PathBuf),
    CloseProject,
}

/// Ask the main window to open the project editor for `path`.
///
/// The main window owns the deferred viewport and calls `show` every frame,
/// which keeps the project window alive.
pub fn request_open(ctx: &egui::Context, tx: &Sender<ProjectWindowEvent>, path: PathBuf) {
    let _ = tx.send(ProjectWindowEvent::OpenProject(path));
    request_parent_repaint(ctx);
}

/// Wake the parent window so it can process a message sent by this viewport.
pub fn request_parent_repaint(ctx: &egui::Context) {
    ctx.request_repaint_of(ctx.parent_viewport_id());
}

/// Draw the project editor inside its own eframe viewport.
pub fn show(ctx: &egui::Context, path: PathBuf, tx: Sender<ProjectWindowEvent>) {
    if let Err(error) = main(path, ctx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Could not load project");
            ui.label(error.to_string());
        });
    }

    if ctx.input(|input| input.viewport().close_requested()) {
        let _ = tx.send(ProjectWindowEvent::CloseProject);
        request_parent_repaint(ctx);
    }
}
