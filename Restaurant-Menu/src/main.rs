use eframe::egui;

#[derive(Clone)]
struct MenuItem {
    id: u32,
    name: String,
    description: String,
    price: f32,
    available: bool,
}

struct RestaurantApp {
    menu: Vec<MenuItem>,
}

impl Default for RestaurantApp {
    fn default() -> Self {
        Self {
            menu: vec![
                MenuItem {
                    id: 1,
                    name: "Cheeseburger".to_string(),
                    description: "Beef burger with cheese".to_string(),
                    price: 199.0,
                    available: true,
                },
                MenuItem {
                    id: 2,
                    name: "Pepperoni Pizza".to_string(),
                    description: "Classic pepperoni pizza".to_string(),
                    price: 349.0,
                    available: true,
                },
                MenuItem {
                    id: 3,
                    name: "Iced Coffee".to_string(),
                    description: "Cold brewed coffee".to_string(),
                    price: 99.0,
                    available: true,
                },
            ],
        }
    }
}

impl eframe::App for RestaurantApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🍽 Restaurant Menu");

            ui.separator();

            for item in &self.menu {
                ui.group(|ui| {
                    ui.label(format!("ID: {}", item.id));
                    ui.heading(&item.name);
                    ui.label(&item.description);
                    ui.label(format!("₱{:.2}", item.price));

                    if item.available {
                        ui.label("✅ Available");
                    } else {
                        ui.label("❌ Out of Stock");
                    }
                });

                ui.separator();
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Restaurant Menu",
        options,
        Box::new(|_cc| Ok(Box::new(RestaurantApp::default()))),
    )
}