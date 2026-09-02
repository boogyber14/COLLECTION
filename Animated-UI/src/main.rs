use bevy::diagnostic::{
    DiagnosticsStore,
    FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use bevy::window::PrimaryWindow;

use bevy_egui::{
    egui,
    EguiContexts,
    EguiPlugin,
    EguiPrimaryContextPass,
};

// ============================================================
// APP STATE
// ============================================================

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    MainMenu,

    Settings,
}

// ============================================================
// COMPONENTS
// ============================================================

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct SettingsMenu;

#[derive(Component)]
struct AnimatedButton;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct BackButton;

// ============================================================
// DEBUG OVERLAY RESOURCE
// ============================================================

#[derive(Resource)]
struct DebugOverlay {
    visible: bool,
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self { visible: true }
    }
}

// ============================================================
// MAIN
// ============================================================

fn main() {
    App::new()

        .add_plugins(DefaultPlugins)

        .init_state::<AppState>()

        .insert_resource(DebugOverlay::default())

        .add_plugins(FrameTimeDiagnosticsPlugin::default())

        .add_plugins(EguiPlugin::default())

        .add_systems(Startup, setup)

        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)

        .add_systems(OnEnter(AppState::Settings), setup_settings)

        .add_systems(
            Update,
            (button_system, animate_buttons, toggle_debug_overlay),
        )

        .add_systems(EguiPrimaryContextPass, debug_overlay)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}



fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),

            flex_direction: FlexDirection::Column,

            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,

            row_gap: Val::Px(20.0),

            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
        MainMenu,
        children![

            (
                Text::new("KORBAD OS"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                }
            ),

            (
                Button,
                AnimatedButton,
                PlayButton,
                Node {
                    width: Val::Px(250.0),
                    height: Val::Px(70.0),

                    justify_content: JustifyContent::Center,

                    align_items: AlignItems::Center,

                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.3, 0.8,)),
                children![(
                    Text::new("PLAY"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    }
                )]
            ),

            (
                Button,
                AnimatedButton,
                SettingsButton,
                Node {
                    width: Val::Px(250.0),
                    height: Val::Px(70.0),

                    justify_content: JustifyContent::Center,

                    align_items: AlignItems::Center,

                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.3, 0.8,)),
                children![(
                    Text::new("SETTINGS"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    }
                )]
            )
        ],
    ));
}



fn setup_settings(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),

            flex_direction: FlexDirection::Column,

            justify_content: JustifyContent::Center,

            align_items: AlignItems::Center,

            row_gap: Val::Px(20.0),

            ..default()
        },
        BackgroundColor(Color::srgb(0.08, 0.04, 0.04)),
        SettingsMenu,
        children![

            (
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                }
            ),

            (
                Text::new("Welcome to the settings page!"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                }
            ),

            (
                Button,
                AnimatedButton,
                BackButton,
                Node {
                    width: Val::Px(250.0),
                    height: Val::Px(70.0),

                    justify_content: JustifyContent::Center,

                    align_items: AlignItems::Center,

                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.3, 0.8,)),
                children![(
                    Text::new("BACK"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    }
                )]
            )
        ],
    ));
}



fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayButton>,
            Option<&SettingsButton>,
            Option<&BackButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,

    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color, play_button, settings_button, back_button) in
        &mut interaction_query
    {
        match *interaction {

            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.8, 0.2, 0.2);



                if play_button.is_some() {
                    println!("Play button pressed!");
                }



                if settings_button.is_some() {
                    println!("Settings button pressed!");

                    next_state.set(AppState::Settings);
                }



                if back_button.is_some() {
                    println!("Back button pressed!");

                    next_state.set(AppState::MainMenu);
                }
            }


            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.2, 0.5, 1.0);
            }


            Interaction::None => {
                background_color.0 = Color::srgb(0.1, 0.3, 0.8);
            }
        }
    }
}



fn animate_buttons(
    mut query: Query<(&Interaction, &mut Transform), With<AnimatedButton>>,

    time: Res<Time>,
) {
    for (interaction, mut transform) in &mut query {
        let target_scale = match *interaction {
            Interaction::Hovered => 1.1,

            Interaction::Pressed => 0.95,

            Interaction::None => 1.0,
        };

        let current_scale = transform.scale.x;

        let new_scale = current_scale.lerp(target_scale, time.delta_secs() * 10.0);

        transform.scale = Vec3::splat(new_scale);
    }
}



fn toggle_debug_overlay(keyboard: Res<ButtonInput<KeyCode>>, mut debug: ResMut<DebugOverlay>) {
    if keyboard.just_pressed(KeyCode::F1) {
        debug.visible = !debug.visible;

        println!(
            "Debug overlay: {}",
            if debug.visible { "ON" } else { "OFF" }
        );
    }
}



fn debug_overlay(
    diagnostics: Res<DiagnosticsStore>,
    debug: Res<DebugOverlay>,
    adapter: Res<RenderAdapterInfo>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut contexts: EguiContexts,
) -> Result {


    if !debug.visible {
        return Ok(());
    }

    egui::Window::new("Performance")
        .resizable(false)
        .show(contexts.ctx_mut()?, |ui| {


            ui.heading("KORBAD OS DEBUG");

            ui.separator();



            if let Some(fps) = diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
            {
                let fps_color = if fps >= 120.0 {
                    egui::Color32::GREEN
                } else if fps >= 60.0 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::RED
                };

                ui.colored_label(fps_color, format!("FPS: {:.0}", fps));
            } else {
                ui.label("FPS: Calculating...");
            }



            if let Some(frame_time) = diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                .and_then(|ft| ft.smoothed())
            {
                let frame_color = if frame_time <= 8.33 {
                    egui::Color32::GREEN
                } else if frame_time <= 16.67 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::RED
                };

                ui.colored_label(frame_color, format!("Frame Time: {:.2} ms", frame_time));
            } else {
                ui.label("Frame Time: Calculating...");
            }



            ui.separator();

            ui.heading("Graphics");



            ui.label(format!("GPU: {}", adapter.name));



            ui.label(format!("Backend: {:?}", adapter.backend));



            ui.label(format!("Device: {:?}", adapter.device_type));



            ui.separator();

            ui.label("F1 - Toggle Debug Overlay");

            ui.label("Bevy + egui");

            ui.separator();

            ui.heading("Display");

            if let Ok(window) = windows.single() {
                let width = window.resolution.width();
                let height = window.resolution.height();

                ui.label(format!("Resolution: {:.0} × {:.0}", width, height));
            }
        });

    Ok(())
}
