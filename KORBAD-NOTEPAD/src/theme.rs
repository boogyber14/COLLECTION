use eframe::egui;

pub fn apply_theme(ctx: &egui::Context) {
    ctx.set_visuals(egui::Visuals::dark());
}