use eframe::egui;
use std::sync::mpsc::Sender;
use std::{fs, io::Result, path::PathBuf};

use crate::gui::load_window::{self, ProjectWindowEvent};

pub enum OpenWindowEvent {
    CloseRequested,
}

pub fn open_show(
    ctx: &egui::Context,
    tx_open: Sender<OpenWindowEvent>,
    tx_load: Sender<ProjectWindowEvent>,
) -> Result<()> {
    let mut result = Ok(());

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Select Project to Open:\n");

        let projects: Vec<PathBuf> = match fs::read_dir("./Projects").and_then(|entries| {
            entries
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>>>()
        }) {
            Ok(paths) => paths,
            Err(reason) => {
                ui.label(format!("Could not read files from directory: {}", reason));
                result = Err(reason);
                return;
            }
        };
        for project in projects {
            let project_name = project
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unnamed project");

            if ui.button(project_name).clicked() {
                load_window::request_open(ctx, &tx_load, project.clone());
                let _ = tx_open.send(OpenWindowEvent::CloseRequested);
            }
        }
    });
    if ctx.input(|i| i.viewport().close_requested()) {
        let _ = tx_open.send(OpenWindowEvent::CloseRequested);
        load_window::request_parent_repaint(ctx);
    }

    result
}
