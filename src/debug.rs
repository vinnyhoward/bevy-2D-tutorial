use bevy::prelude::*;
use bevy_inspector_egui::quick::{WorldInspectorPlugin, ResourceInspectorPlugin};

use crate::player::Player;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
            .init_resource::<Player>() // `ResourceInspectorPlugin` won't initialize the resource
            .register_type::<Player>() // you need to register your type to display it
            .add_plugin(ResourceInspectorPlugin::<Player>::default())
            .add_plugin(ResourceInspectorPlugin::<Time>::default());
        }
    }
}