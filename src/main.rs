use bevy::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(ProcessState::default())
        .add_systems(Update, simulate_process)
        .run();
}

#[derive(Resource, Default)]
struct ProcessState {
    counter: usize, }

fn simulate_process(time: Res<Time>, mut state:
ResMut<ProcessState>) {
    state.counter += 1;

    println!(
        "Tick {} - elapsed: {:.2?}",
        state.counter,
        time.elapsed()
    );

    if state.counter > 5 {
        println!("Done processing. Shutting down...");
        std::process::exit(0);
    }
}
