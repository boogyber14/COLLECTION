use bevy::prelude::*;

#[derive(Component)]
struct Planet;

#[derive(Component)]
struct Orbit {
    radius: f32,
    speed: f32,
    offset: f32,
}

#[derive(Component)]
struct Atmosphere;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_system, animate_atmosphere))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    for i in 0..4 {
        spawn_planet(
            &mut commands,
            120.0 + i as f32 * 80.0,
            0.5 + i as f32 * 0.2,
            30.0 + i as f32 * 10.0,
            i as f32,
        );
    }
}

fn spawn_planet(
    commands: &mut Commands,
    radius: f32,
    speed: f32,
    size: f32,
    offset: f32,
) {
    let planet = commands
        .spawn((
            Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(size)),
            Transform::default(),
            Planet,
            Orbit { radius, speed, offset },
        ))
        .id();


    commands.entity(planet).with_children(|parent| {
        parent.spawn((
            Sprite::from_color(
                Color::srgba(0.3, 0.6, 1.0, 0.2),
                Vec2::splat(size * 1.6),
            ),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
            Atmosphere,
        ));
    });
}

fn orbit_system(time: Res<Time>, mut query: Query<(&mut Transform, &Orbit), With<Planet>>) {
    let t = time.elapsed_secs();

    for (mut transform, orbit) in &mut query {
        let angle = t * orbit.speed + orbit.offset;

        transform.translation.x = orbit.radius * angle.cos();
        transform.translation.y = orbit.radius * angle.sin();
    }
}

fn animate_atmosphere(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Atmosphere>>,
) {
    let pulse = 1.0 + 0.05 * (time.elapsed_secs() * 3.0).sin();

    for mut transform in &mut query {
        transform.scale = Vec3::splat(pulse);
    }
}