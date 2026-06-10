use eframe::egui;
use rfd::FileDialog;
use rodio::{
    Decoder,
    OutputStream,
    Sink,
};
use std::fs::File;
use std::io::BufReader;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "MP3 Player Korbad",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    selected_file: Option<String>,
    sink: Option<Sink>,
    stream: Option<OutputStream>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_file: None,
            sink: None,
            stream: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 My MP3 Player");

            if ui.button("📂 Open MP3").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("MP3 Files", &["mp3"])
                    .pick_file()

                {
                    self.selected_file =
                        Some(path.display().to_string());
                }
            }

            ui.separator();
            if ui.button("▶ Play").clicked() {
                if let Some(path) = &self.selected_file {
                    if let Ok(file) = File::open(path) {
                        let reader = BufReader::new(file);

                        if let Ok(source) = Decoder::new(reader) {
                            let stream =
                                rodio::OutputStreamBuilder::open_default_stream()
                                    .unwrap();

                            let sink =
                                Sink::connect_new(stream.mixer());

                            sink.append(source);

                            self.stream = Some(stream);
                            self.sink = Some(sink);
                        }
                    }
                }
            }

            if ui.button("⏸ Pause").clicked() {
                if let Some(sink) = &self.sink {
                    sink.pause();
                }
            }

            if ui.button("▶ Resume").clicked() {
                if let Some(sink) = &self.sink {
                    sink.play();
                }
            }

            match &self.selected_file {
                Some(file) => {
                    ui.label(format!(
                        "Selected File:\n{}",
                        file
                    ));
                }
                None => {
                    ui.label("No file selected");
                }
            }
        });
    }
}

