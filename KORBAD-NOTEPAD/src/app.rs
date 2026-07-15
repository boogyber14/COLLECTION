use eframe::egui;

pub struct NotepadApp {
    pub text: String,
}

impl NotepadApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            text: String::new(),
        }
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Menu Bar
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {

            egui::MenuBar::new().ui(ui, |ui| {

                ui.menu_button("File", |ui| {

                    if ui.button("New").clicked() {
                        self.text.clear();
                        ui.close();
                    }

                    ui.button("Open");
                    ui.button("Save");
                    ui.button("Save As");
                });

                ui.menu_button("Edit", |ui| {
                    ui.label("Coming Soon...");
                });

            });

        });


        egui::CentralPanel::default().show(ctx, |ui| {

            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .desired_rows(40)
                    .desired_width(f32::INFINITY)
            );

        });


        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {

            let chars = self.text.chars().count();
            let lines = self.text.lines().count();

            ui.horizontal(|ui| {
                ui.label(format!("Characters: {}", chars));
                ui.separator();
                ui.label(format!("Lines: {}", lines));
            });

        });
    }
}