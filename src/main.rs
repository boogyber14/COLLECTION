use eframe::egui::{self, CentralPanel, Context};

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Rust App with Menu",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    show_about: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                    }
                });
            });
        });

        
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello from the Rust app with a menu!");
        });

        
        let mut open = self.show_about;
        egui::Window::new("About")
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("This is a simple Rust app using egui.");
                if ui.button("Close").clicked() {
                    
                }
            });
        self.show_about = open;

        
    }
}
