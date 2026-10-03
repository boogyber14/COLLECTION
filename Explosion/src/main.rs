use bevy::prelude::*;

#[derive(Component)]
struct Particle {
    velocity: Vec2,
    lifetime: f32,
    max_lifetime: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Particle Explosion".into(),
                resolution: (1000, 600).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                mouse_explosion,
                keyboard_explosion,
                update_particles,
                clear_particles,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn mouse_explosion(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let window_size = Vec2::new(window.width(), window.height());

    let world_position = cursor_position - window_size / 2.0;

    spawn_explosion(&mut commands, world_position);
}

fn keyboard_explosion(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::Space) {
        spawn_explosion(&mut commands, Vec2::ZERO);
    }
}

fn spawn_explosion(commands: &mut Commands, position: Vec2) {
    let particle_count = 100;

    for i in 0..particle_count {
        let angle = (i as f32 / particle_count as f32) * std::f32::consts::TAU;

        let variation = ((i * 37) % 100) as f32 / 100.0;

        let speed = 100.0 + variation * 350.0;

        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);

        let lifetime = 0.5 + variation * 1.0;

        let size = 3.0 + variation * 6.0;

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.25 + variation * 0.5, 0.02),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
            Particle {
                velocity,
                lifetime,
                max_lifetime: lifetime,
            },
        ));
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
) {
    let delta = time.delta_secs();

    for (entity, mut transform, mut sprite, mut particle) in particles.iter_mut() {
        transform.translation.x += particle.velocity.x * delta;

        transform.translation.y += particle.velocity.y * delta;

        particle.velocity.y -= 250.0 * delta;

        particle.velocity *= 0.97_f32.powf(delta * 60.0);

        particle.lifetime -= delta;

        let alpha = (particle.lifetime / particle.max_lifetime).clamp(0.0, 1.0);

        sprite.color = sprite.color.with_alpha(alpha);

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn clear_particles(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    particles: Query<Entity, With<Particle>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }

    for entity in particles.iter() {
        commands.entity(entity).despawn();
    }
}
