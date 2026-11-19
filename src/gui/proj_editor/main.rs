use eframe::egui;
use std::{io::Result, path::PathBuf};

use super::renderer;

pub fn main(
    path: PathBuf,
    ctx: &egui::Context,
    render_target_format: eframe::wgpu::TextureFormat,
) -> Result<()> {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Dimension Project Editor");
        ui.label(format!("Project: {}", path.display()));
        ui.separator();
        ui.label("Scene viewport · rotating cube prototype");

        let viewport_size = ui.available_size();
        let (viewport_rect, _) = ui.allocate_exact_size(viewport_size, egui::Sense::hover());
        if viewport_rect.width() > 0.0 && viewport_rect.height() > 0.0 {
            let time = ctx.input(|input| input.time as f32);
            ui.painter().add(renderer::cube_callback(
                viewport_rect,
                render_target_format,
                time,
            ));
            ctx.request_repaint_after(std::time::Duration::from_secs_f64(1.0 / 60.0));
        }
    });

    Ok(())
}
