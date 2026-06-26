use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Rust Video Player",
        options,
        Box::new(|_cc| Ok(Box::new(PlayerApp::default()))),
    )
}

struct PlayerApp {
    playing: bool,
    progress: f32,
}

impl Default for PlayerApp {
    fn default() -> Self {
        Self {
            playing: false,
            progress: 0.0,
        }
    }
}

impl eframe::App for PlayerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎬 My Rust Video Player");

            if ui.button("Play / Pause").clicked() {
                self.playing = !self.playing;
            }

            ui.add(
                egui::Slider::new(&mut self.progress, 0.0..=1.0)
                    .text("Progress"),
            );
        });
    }
}