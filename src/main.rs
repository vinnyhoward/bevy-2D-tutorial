use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
    render::camera::ScalingMode
};

mod player;
mod ascii;


use player::PlayerPlugin;
use ascii::AsciiPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "I am a window!".into(),
                        resolution: (1600., 900.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .run();
}

#[derive(Component, Debug, Default)]
struct Player {
    name: String,
}

impl Player {
    pub fn new(player_name: String) -> Self {
        Self { name: player_name }
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(500.0),
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn(camera);
}
