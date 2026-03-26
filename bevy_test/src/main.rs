use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
struct Sun;

#[derive(Component)]
struct Orbit {
    radius: f32,
    speed: f32,
    angle: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (sun_orbit, rotate_sun, control_speed))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let radius = 300.0;


    commands.spawn((
        ShapeBuilder::new()
            .add(&shapes::Circle {
                radius,
                center: Vec2::ZERO,
            })
            .stroke(Stroke::new(Color::WHITE, 2.0))
            .build(),
        Transform::default(),
    ));

    let sun_texture = asset_server.load("Capture.PNG");


    commands.spawn((
        Sprite {
            image: sun_texture.clone(),
            color: Color::srgba(1.0, 0.7, 0.2, 0.3),
            custom_size: Some(Vec2::new(180.0, 180.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.1),
    ));


    commands.spawn((
        Sprite {
            image: sun_texture,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::default(),
        Sun,
        Orbit {
            radius,
            speed: 1.0,
            angle: 0.0,
        },
    ));
}

fn sun_orbit(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Orbit), With<Sun>>,
) {
    for (mut transform, mut orbit) in &mut query {
        orbit.angle += orbit.speed * time.delta_secs();

        transform.translation.x = orbit.radius * orbit.angle.cos();
        transform.translation.y = orbit.radius * orbit.angle.sin();
    }
}

fn rotate_sun(time: Res<Time>, mut query: Query<&mut Transform, With<Sun>>) {
    for mut transform in &mut query {
        transform.rotate_z(1.5 * time.delta_secs());
    }
}

fn control_speed(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Orbit>,
) {
    for mut orbit in &mut query {
        if keyboard.pressed(KeyCode::ArrowUp) {
            orbit.speed += 0.5 * 0.016;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            orbit.speed -= 0.5 * 0.016;
        }
    }
}