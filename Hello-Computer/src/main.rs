use std::env;

fn main() {
    // Print a greeting
    println!("Hello from Rust!");

    // Get some basic info about the computer
    let current_dir = env::current_dir().unwrap();
    let username = whoami::username();
    let os = whoami::platform();

    println!("Current directory: {}", current_dir.display());
    println!("Username: {}", username);
    println!("Operating system: {:?}", os);
}
