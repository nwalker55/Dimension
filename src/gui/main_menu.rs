use eframe::egui;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::gui::new_proj::new_project;
use crate::gui::open_proj::open_project;

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
    new_proj_vars: Arc<Mutex<new_project::NewProjVars>>,
}

impl DimensionVars {
    fn new() -> Self {
       let (tx_new, rx_new) = channel();
       let (tx_open, rx_open) = channel();

       Self {
           show_new_proj_window: false,
           show_open_proj_window: false,
           tx_new,
           rx_new,
           tx_open,
           rx_open,
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

        egui::CentralPanel::default().show(ctx, |ui| {
           ctx.set_pixels_per_point(1.5);
           ui.heading("Welcome to Dimension Game Engine!");
           ui.label("\n\n");
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
           let proj_vars = Arc::clone(&self.new_proj_vars);

           ctx.show_viewport_deferred(
               viewport_id,
               egui::ViewportBuilder::default()
                   .with_title("New Dimension Project")
                   .with_inner_size([400.0, 300.0]),
               move |ctx, class| {
                   assert!(class == egui::ViewportClass::Deferred);
                   let mut vars = proj_vars.lock().unwrap();
                   let _ = vars.new_show(ctx, tx_clone.clone());
               },
           );
        }

        if self.show_open_proj_window {
           let viewport_id = egui::ViewportId::from_hash_of("open_project_viewport");
           let tx_clone = self.tx_open.clone();
           ctx.show_viewport_deferred(
               viewport_id,
               egui::ViewportBuilder::default()
                   .with_title("Open Dimension Project")
                   .with_inner_size([400.0, 300.0]),
               move |ctx, class| {
                   assert!(class == egui::ViewportClass::Deferred);
                   open_project::open_show(ctx, tx_clone.clone());
               },
           );
        }
    }
}