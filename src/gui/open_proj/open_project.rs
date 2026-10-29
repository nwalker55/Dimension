use eframe::egui;
use std::sync::mpsc::Sender;
use std::{fs, io};
use std::path::{PathBuf, Path};
use std::io::Result;

// Define custom events to send back to the main window
pub enum OpenWindowEvent {
    CloseRequested,
    // Add other events here, e.g., DataUpdated(String)
}

pub struct OpenProjVars {
    selected: String,
}

impl OpenProjVars {
    pub fn new() -> Self {
        Self {
            selected: String::new(),
        }
    }

    pub fn set_selected(&mut self, selected: String) {
        self.selected = selected;
    }
}


/// Renders the viewport content. This function is called every frame inside the closure.
pub fn open_show(ctx: &egui::Context, tx_open: Sender<OpenWindowEvent>) -> Result<()> {
    let mut open_vars = OpenProjVars::new();
    let mut result = Ok(());

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Open Project");


        let projects: Vec<PathBuf> = match fs::read_dir("./Projects")
            .and_then(|entries| {
                entries
                    .map(|res| res.map(|e| e.path()))
                    .collect::<io::Result<Vec<_>>>()
            })
        {
            Ok(paths) => paths,
            Err(reason) => {
                ui.label(format!("Could not read files from directory: {}", reason));
                result = Err(reason);
                return;
            }
        };
        let mut project: &PathBuf = &PathBuf::new();

        let mut path_vars: Vec<egui::Response> = Vec::new();
        for (i, proj) in projects.iter().enumerate() {
            path_vars.push(ui.button(format!("{}", proj.display().to_string().replace("./Projects/", ""))));

            if let Some(index) = path_vars.iter().position(|r| r.clicked()) {
                project = &projects[index];


                open_vars.set_selected(String::from(format!("Selected project: {}", project.display())));
            }
        }
        if !project.display().to_string().is_empty() {
            println!("{}", open_vars.selected);
        }



    });
    if ctx.input(|i| i.viewport().close_requested()) {
        let _ = tx_open.send(OpenWindowEvent::CloseRequested);
    }

    result

}
