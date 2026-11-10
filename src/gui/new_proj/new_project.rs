use eframe::egui;
use std::sync::mpsc::Sender;
use std::{fs::File, io::Result, path::PathBuf};

use crate::gui::load_window::{self, ProjectWindowEvent};

// Define custom events to send back to the main window
pub enum NewWindowEvent {
    CloseRequested,
    ProjectCreated(String),
}

#[derive(Default)]
pub struct NewProjVars {
    new_proj_name: String,
    message: String,
}

impl NewProjVars {
    pub fn new() -> Self {
        Self {
            new_proj_name: String::new(),
            message: String::new(),
        }
    }

    pub fn new_show(
        &mut self,
        ctx: &egui::Context,
        tx_new: Sender<NewWindowEvent>,
        tx_load: Sender<ProjectWindowEvent>,
    ) -> Result<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("New Project");
            ui.label("\n\nEnter name of new project:");

            ui.add(
                egui::TextEdit::singleline(&mut self.new_proj_name)
                    .char_limit(20)
                    .desired_width(160.0),
            );

            ui.horizontal(|ui| {
                if ui.button("Create").clicked() && !self.new_proj_name.is_empty() {
                    if let Err(why) = project_create(self.new_proj_name.clone()) {
                        self.message =
                            format!("Could not create {} project: {}", self.new_proj_name, why);
                        println!(
                            "Could not create project {} due to {}",
                            self.new_proj_name, why
                        );
                    } else {
                        self.message =
                            format!("Project name {} successfully created", self.new_proj_name);
                        println!("Project {} successfully created", self.new_proj_name);
                        let _ =
                            tx_new.send(NewWindowEvent::ProjectCreated(self.new_proj_name.clone()));
                        load_window::request_open(
                            ctx,
                            &tx_load,
                            PathBuf::from("Projects").join(&self.new_proj_name),
                        );
                    }
                }

                if ui.button("Cancel").clicked() {
                    close_window(ctx, &tx_new);
                }
            });
            if !self.message.is_empty() {
                ui.label(&self.message);
            }
        });

        // Catch the native OS close button (the 'X' button on the title bar)
        if ctx.input(|i| i.viewport().close_requested()) {
            close_window(ctx, &tx_new);
        }

        Ok(())
    }

    pub fn get_proj_name(&self) -> &str {
        &self.new_proj_name
    }
}

// Later, the project_create will not just be a text file, and will include the project contents in a different format
fn project_create(proj_name: String) -> Result<()> {
    let path_name = PathBuf::from("Projects").join(proj_name);
    let _ = File::create(path_name)?;

    Ok(())
}

fn close_window(ctx: &egui::Context, tx_new: &Sender<NewWindowEvent>) {
    let _ = tx_new.send(NewWindowEvent::CloseRequested);
    load_window::request_parent_repaint(ctx);
}
