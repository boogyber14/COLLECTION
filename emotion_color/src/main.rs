use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;


fn emotion_to_color(emotion: &str) -> (u8, u8, u8) {
    let mut hasher = DefaultHasher::new();
    emotion.to_lowercase().hash(&mut hasher);
    let hash = hasher.finish();

    let r = (hash & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = ((hash >> 16) & 0xFF) as u8;

    (r, g, b)
}


fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn main() {
    println!("🎨 Emotion to Color Generator 🎨");
    println!("Type an emotion (like 'happy', 'angry', 'calm'):\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let emotion = input.trim();

    if emotion.is_empty() {
        println!("❗ Please enter an emotion next time!");
        return;
    }

    let (r, g, b) = emotion_to_color(emotion);
    let hex = rgb_to_hex(r, g, b);

    println!("\nEmotion: {}", emotion);
    println!("RGB: ({}, {}, {})", r, g, b);
    println!("Hex Color: {}", hex);

    println!(
        "\n🧠 Meaning: Your emotion '{}' translates to this unique color!",
        emotion
    );
}
