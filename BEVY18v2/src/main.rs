use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sun_movement)
        .run();
}

fn setup(mut commands: Commands) {

    commands.spawn(Camera2d);


    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.9, 0.2), // sun color
            custom_size: Some(Vec2::new(80.0, 80.0)),
            ..default()
        },
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
}

fn sun_movement(time: Res<Time>, mut query: Query<&mut Transform, With<Sprite>>) {
    let t = time.elapsed_secs();

    for mut transform in &mut query {
        let radius = 200.0;

        transform.translation.x = radius * t.cos();
        transform.translation.y = radius * t.sin();
    }
}