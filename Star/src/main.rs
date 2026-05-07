use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Planet {
    radius: f32,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Atmosphere;

#[derive(Component)]
struct StarLayer {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_on_click,
                movement_system,
                collision_system,
                animate_atmosphere,
                parallax_movement,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {

    commands.spawn(Camera2d);

    let mut rng = rand::thread_rng();


    for _ in 0..100 {
        commands.spawn((
            Sprite::from_color(
                Color::WHITE,
                Vec2::splat(rng.gen_range(1.0..3.0)),
            ),
            Transform::from_translation(Vec3::new(
                rng.gen_range(-1000.0..1000.0),
                rng.gen_range(-600.0..600.0),
                -10.0,
            )),
            StarLayer { speed: 10.0 },
        ));
    }


    for _ in 0..70 {
        commands.spawn((
            Sprite::from_color(
                Color::srgb(0.8, 0.8, 1.0),
                Vec2::splat(rng.gen_range(2.0..4.0)),
            ),
            Transform::from_translation(Vec3::new(
                rng.gen_range(-1000.0..1000.0),
                rng.gen_range(-600.0..600.0),
                -5.0,
            )),
            StarLayer { speed: 25.0 },
        ));
    }


    for _ in 0..40 {
        commands.spawn((
            Sprite::from_color(
                Color::srgb(0.6, 0.7, 1.0),
                Vec2::splat(rng.gen_range(3.0..6.0)),
            ),
            Transform::from_translation(Vec3::new(
                rng.gen_range(-1000.0..1000.0),
                rng.gen_range(-600.0..600.0),
                -1.0,
            )),
            StarLayer { speed: 50.0 },
        ));
    }
}

fn spawn_on_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    asset_server: Res<AssetServer>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok(world_pos) =
        camera.viewport_to_world_2d(camera_transform, cursor_pos)
    else {
        return;
    };

    spawn_planet_at(&mut commands, &asset_server, world_pos);
}

fn spawn_planet_at(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
) {
    let texture = asset_server.load("planet.png");

    let size = 40.0;
    let radius = size / 2.0;

    let mut rng = rand::thread_rng();

    let direction = Vec2::new(
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    )
        .normalize_or_zero();

    let speed = rng.gen_range(50.0..150.0);

    let velocity = direction * speed;

    let planet = commands
        .spawn((
            Sprite::from_image(texture),
            Transform {
                translation: position.extend(0.0),
                scale: Vec3::splat(0.3),
                ..default()
            },
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

fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Planet>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation +=
            (velocity.0 * time.delta_secs()).extend(0.0);
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

            t1.translation -=
                (normal * overlap * 0.5).extend(0.0);

            t2.translation +=
                (normal * overlap * 0.5).extend(0.0);

            let v1n = v1.0.dot(normal);
            let v2n = v2.0.dot(normal);

            let impulse = v2n - v1n;

            v1.0 += normal * impulse;
            v2.0 -= normal * impulse;
        }
    }
}

fn animate_atmosphere(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Atmosphere>>,
) {
    let pulse =
        1.0 + 0.05 * (time.elapsed_secs() * 3.0).sin();

    for mut transform in &mut query {
        transform.scale = Vec3::splat(pulse);
    }
}

fn parallax_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &StarLayer)>,
) {
    for (mut transform, layer) in &mut query {
        transform.translation.x -=
            layer.speed * time.delta_secs();


        if transform.translation.x < -1200.0 {
            transform.translation.x = 1200.0;
        }
    }
}