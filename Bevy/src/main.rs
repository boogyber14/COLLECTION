use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.4, 0.4),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::default(),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.single_mut() {
        transform.translation.x += 100.0 * time.delta_secs();
    }
}