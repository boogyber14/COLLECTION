use enigo::{Enigo, MouseControllable};
use std::{thread, time};

fn main() {
    let mut enigo = Enigo::new();

    println!("Starting auto clicker in 3 seconds...");
    thread::sleep(time::Duration::from_secs(3));


    loop {
        enigo.mouse_click(enigo::MouseButton::Left);
        println!("Clicked!");


        thread::sleep(time::Duration::from_millis(500));
    }
}
