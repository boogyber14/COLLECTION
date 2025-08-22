use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Resource, Serialize, Deserialize, Debug)]
struct GameConfig {
    player_name: String,
    difficulty: u8,
    fullscreen: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player_name: "Player".to_string(),
            difficulty: 1,
            fullscreen: false,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(load_config("config.json"))
        .add_systems(Startup, save_config_system)
        .add_systems(Update, print_config_system)
        .run();
}

fn load_config(path: &str) -> GameConfig {
    if let Ok(contents) = fs::read_to_string(path) {
        if let Ok(config) = serde_json::from_str::<GameConfig>(&contents) {
            println!("Loaded config from {}", path);
            return config;
        } else {
            eprintln!("Failed to parse {}, using default config", path);
        }
    } else {
        eprintln!("Config file not found, using default config");
    }
    GameConfig::default()
}

fn save_config_system(config: Res<GameConfig>) {
    let json = serde_json::to_string_pretty(&*config).unwrap();
    if let Err(err) = fs::write("config.json", json) {
        eprintln!("Failed to save config: {}", err);
    } else {
        println!("Config saved to config.json");
    }
}

fn print_config_system(config: Res<GameConfig>) {
    println!(
        "Player: {}, Difficulty: {}, Fullscreen: {}",
        config.player_name, config.difficulty, config.fullscreen
    );
}
