use eframe::egui;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::gui::load_window::{self, ProjectWindowEvent};
use crate::gui::new_proj::new_project;
use crate::gui::open_proj::open_project;
use std::path::PathBuf;

// Start main menu
pub fn menu() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Dimension Game Engine",
        native_options,
        Box::new(|_cc| Ok(Box::new(DimensionVars::new()))),
    )
}

struct DimensionVars {
    show_new_proj_window: bool,
    show_open_proj_window: bool,
    tx_new: Sender<new_project::NewWindowEvent>,
    rx_new: Receiver<new_project::NewWindowEvent>,
    tx_open: Sender<open_project::OpenWindowEvent>,
    rx_open: Receiver<open_project::OpenWindowEvent>,
    tx_load: Sender<ProjectWindowEvent>,
    rx_load: Receiver<ProjectWindowEvent>,
    loaded_project: Option<PathBuf>,
    new_proj_vars: Arc<Mutex<new_project::NewProjVars>>,
}

impl DimensionVars {
    fn new() -> Self {
        let (tx_new, rx_new) = channel();
        let (tx_open, rx_open) = channel();
        let (tx_load, rx_load) = channel();

        Self {
            show_new_proj_window: false,
            show_open_proj_window: false,
            tx_new,
            rx_new,
            tx_open,
            rx_open,
            tx_load,
            rx_load,
            loaded_project: None,
            new_proj_vars: Arc::new(Mutex::new(new_project::NewProjVars::new())),
        }
    }
}

impl eframe::App for DimensionVars {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(new_event) = self.rx_new.try_recv() {
            match new_event {
                new_project::NewWindowEvent::CloseRequested => {
                    self.show_new_proj_window = false;
                }
                new_project::NewWindowEvent::ProjectCreated(name) => {
                    println!("Project created: {}", name);

                    self.show_new_proj_window = false;
                }
            }
        }
        while let Ok(open_event) = self.rx_open.try_recv() {
            match open_event {
                open_project::OpenWindowEvent::CloseRequested => {
                    self.show_open_proj_window = false;
                }
            }
        }
        while let Ok(load_event) = self.rx_load.try_recv() {
            match load_event {
                ProjectWindowEvent::OpenProject(path) => {
                    self.loaded_project = Some(path);
                    self.show_new_proj_window = false;
                    self.show_open_proj_window = false;
                }
                ProjectWindowEvent::CloseProject => {
                    self.loaded_project = None;
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(1.5);
            ui.heading("Welcome to Dimension Game Engine!\n");
            if ui.button("New Project").clicked() {
                self.show_new_proj_window = true;
            }
            if ui.button("Open Project").clicked() {
                self.show_open_proj_window = true;
            }
        });

        if self.show_new_proj_window {
            let viewport_id = egui::ViewportId::from_hash_of("new_project_viewport");
            let tx_clone = self.tx_new.clone();
            let tx_load = self.tx_load.clone();
            let proj_vars = Arc::clone(&self.new_proj_vars);

            ctx.show_viewport_deferred(
                viewport_id,
                egui::ViewportBuilder::default()
                    .with_title("New Dimension Project")
                    .with_inner_size([400.0, 300.0]),
                move |ctx, class| {
                    assert!(class == egui::ViewportClass::Deferred);
                    let mut vars = proj_vars.lock().unwrap();
                    let _ = vars.new_show(ctx, tx_clone.clone(), tx_load.clone());
                },
            );
        }

        if self.show_open_proj_window {
            let viewport_id = egui::ViewportId::from_hash_of("open_project_viewport");
            let tx_clone = self.tx_open.clone();
            let tx_load = self.tx_load.clone();
            ctx.show_viewport_deferred(
                viewport_id,
                egui::ViewportBuilder::default()
                    .with_title("Open Dimension Project")
                    .with_inner_size([400.0, 300.0]),
                move |ctx, class| {
                    assert!(class == egui::ViewportClass::Deferred);
                    let _ = open_project::open_show(ctx, tx_clone.clone(), tx_load.clone());
                },
            );
        }

        if let Some(path) = self.loaded_project.clone() {
            let tx_load = self.tx_load.clone();
            let project_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Project");

            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("project_editor_viewport"),
                egui::ViewportBuilder::default()
                    .with_title(format!("Dimension - {project_name}"))
                    .with_inner_size([1000.0, 700.0]),
                move |ctx, class| {
                    assert!(class == egui::ViewportClass::Deferred);
                    load_window::show(ctx, path.clone(), tx_load.clone());
                },
            );
        }
    }
}
