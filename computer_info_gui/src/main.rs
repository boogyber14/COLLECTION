use eframe::egui;
use std::env;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Computer Info",
        options,
        Box::new(|_cc| Ok(Box::<ComputerInfoApp>::default())),
    )
}

#[derive(Default)]
struct ComputerInfoApp {
    username: String,
    os: String,
    current_dir: String,
    unique_id: String,
}

impl eframe::App for ComputerInfoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.username.is_empty() {
            let username = whoami::username();
            let os = format!("{:?}", whoami::platform());
            let current_dir = env::current_dir()
                .unwrap_or_else(|_| "Unknown".into())
                .display()
                .to_string();
            let unique_id = format!(
                "{}-{:x}",
                username,
                md5::compute(username.clone() + &os)
            );

            self.username = username;
            self.os = os;
            self.current_dir = current_dir;
            self.unique_id = unique_id;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🦀 Rust Computer Info App");
            ui.separator();

            ui.label(format!("👤 Username: {}", self.username));
            ui.label(format!("💻 Operating System: {}", self.os));
            ui.label(format!("📂 Current Directory: {}", self.current_dir));
            ui.label(format!("🔑 Unique Computer ID: {}", self.unique_id));

            ui.separator();
            if ui.button("🔄 Refresh Info").clicked() {
                self.username.clear();
            }
        });
    }
}
