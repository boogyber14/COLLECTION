use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, attack_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.7, 0.9),
            custom_size: Some(Vec2::new(80.0, 80.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, 0.0, 0.0),
        Player,
    ));

    
    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.2, 0.2),
            custom_size: Some(Vec2::new(80.0, 80.0)),
            ..default()
        },
        Transform::from_xyz(200.0, 0.0, 0.0),
        Enemy,
    ));
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        if keyboard.pressed(KeyCode::KeyA) {
            transform.translation.x -= 4.0;
        }

        if keyboard.pressed(KeyCode::KeyD) {
            transform.translation.x += 4.0;
        }
    }
}

fn attack_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let player_pos = player_query.single().unwrap().translation;

        for mut enemy in &mut enemy_query {
            let distance = player_pos.distance(enemy.translation);

            if distance < 120.0 {
                enemy.translation.x += 50.0;
            }
        }
    }
}
