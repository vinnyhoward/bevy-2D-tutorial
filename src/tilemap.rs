use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE,
};
use bevy::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// components
#[derive(Component)]
pub struct TileCollider;

// system
fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 90.0),
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                tiles.push(tile);
            }
        } 
    }

    commands
    .spawn_empty()
    .insert(Name::new("Map"));
    // .insert(Transform::default())
    // .insert(GlobalTransform::default())
    // .push_children(&tiles);
}

// plugins
pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}
