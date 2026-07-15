mod app;
mod file;
mod theme;

use app::NotepadApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("RustPad"),
        ..Default::default()
    };

    eframe::run_native(
        "Korbad_Pad",
        options,
        Box::new(|cc| Ok(Box::new(NotepadApp::new(cc)))),
    )
}