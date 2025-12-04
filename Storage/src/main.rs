use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct AppData {
    username: String,
    score: u32,
}

fn save_data(data: &AppData) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    fs::write("storage.json", json)?;
    Ok(())
}

fn load_data() -> std::io::Result<AppData> {
    let data = fs::read_to_string("storage.json")?;
    let parsed: AppData = serde_json::from_str(&data)?;
    Ok(parsed)
}

fn main() {
    let data = AppData {
        username: "Alice".to_string(),
        score: 42,
    };

    
    save_data(&data).unwrap();
    println!("Data saved!");


    let loaded = load_data().unwrap();
    println!("Loaded: {:?}", loaded);
}
