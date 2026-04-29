use bevy::prelude::*;

#[derive(Component)]
struct Planet {
    radius: f32,
}

#[derive(Component)]
struct Orbit {
    radius: f32,
    speed: f32,
    offset: f32,
}

#[derive(Component)]
struct Velocity(Vec2);
#[derive(Component)]
struct Atmosphere;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_system, animate_atmosphere))
        .add_systems(Update, (
            movement_system,
            collision_system,
            animate_atmosphere,
            spawn_on_click
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    for i in 0..4 {
        spawn_planet_at(&mut commands, Vec2::new(i as f32 * 50.0, 0.0));
    }
}

fn spawn_planet_at(commands: &mut Commands, position: Vec2) {
    let size = 20.0 + rand::random::<f32>() * 20.0;
    let radius = size / 2.0;


    let dir = Vec2::new(
        rand::random::<f32>() - 0.5,
        rand::random::<f32>() - 0.5,
    )
        .normalize_or_zero();

    let speed = 50.0 + rand::random::<f32>() * 100.0;

    let velocity = dir * speed;

    let planet = commands
        .spawn((
            Sprite::from_color(
                Color::srgb(rand::random(), rand::random(), rand::random()),
                Vec2::splat(size),
            ),
            Transform::from_translation(position.extend(0.0)),
            Planet { radius },
            Velocity(velocity),
        ))
        .id();


    commands.entity(planet).with_children(|parent| {
        parent.spawn((
            Sprite::from_color(
                Color::srgba(0.5, 0.7, 1.0, 0.2),
                Vec2::splat(size * 1.5),
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

fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}

fn collision_system(
    mut query: Query<(&mut Transform, &mut Velocity, &Planet)>,
) {
    let mut combinations = query.iter_combinations_mut();

    while let Some([(mut t1, mut v1, p1), (mut t2, mut v2, p2)]) =
        combinations.fetch_next()
    {
        let pos1 = t1.translation.truncate();
        let pos2 = t2.translation.truncate();

        let delta = pos2 - pos1;
        let distance = delta.length();
        let min_dist = p1.radius + p2.radius;

        if distance < min_dist && distance > 0.0 {
            let normal = delta.normalize();


            let overlap = min_dist - distance;
            t1.translation -= (normal * overlap * 0.5).extend(0.0);
            t2.translation += (normal * overlap * 0.5).extend(0.0);


            let v1n = v1.0.dot(normal);
            let v2n = v2.0.dot(normal);

            let impulse = v2n - v1n;

            v1.0 += normal * impulse;
            v2.0 -= normal * impulse;
        }
    }
}

fn spawn_on_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera_q.single() else { return };

    let Some(cursor_pos) = window.cursor_position() else { return };

    let Ok(world_pos) =
        camera.viewport_to_world_2d(camera_transform, cursor_pos)
    else {
        return;
    };

    spawn_planet_at(&mut commands, world_pos);
}