use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE,
    // tilemap::TileCollider,
};
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// components
#[derive(Component, Reflect, Resource, Default, InspectorOptions, Debug)]
#[reflect(Resource, InspectorOptions)]
pub struct Player {
    name: String,
}

impl Player {
    pub fn new(player_name: String) -> Self {
        Self { name: player_name }
    }
}

// systems
// fn player_movement(
//     mut player_query: Query<(&Player, &mut Transform)>,
//     wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
//     keyboard: Res<Input<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let (player, mut transform) = player_query.single_mut();

//     let mut y_delta = 0.0;
//     if keyboard.pressed(KeyCode::W) {
//         y_delta += player.speed * TILE_SIZE * time.delta_seconds();
//     }
//     if keyboard.pressed(KeyCode::S) {
//         y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
//     }

//     let mut x_delta = 0.0;
//     if keyboard.pressed(KeyCode::A) {
//         x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
//     }
//     if keyboard.pressed(KeyCode::D) {
//         x_delta += player.speed * TILE_SIZE * time.delta_seconds();
//     }

//     let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
//     if wall_collision_check(target, &wall_query) {
//         transform.translation = target;
//     }

//     let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
//     if wall_collision_check(target, &wall_query) {
//         transform.translation = target;
//     }
// }

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));
    
    let player = commands
    .spawn(SpriteSheetBundle {
        sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Player"))
    .insert(Player::new("Fat Boy Slim".to_string()))
    .id();

let mut background_sprite = TextureAtlasSprite::new(0);
background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
background_sprite.custom_size = Some(Vec2::splat(1.0));

let background = commands
.spawn(SpriteSheetBundle {
    sprite: background_sprite,
    texture_atlas: ascii.0.clone(),
    transform: Transform {
        translation: Vec3::new(0.0, 0.0, -1.0),
        ..Default::default()
    },
    ..Default::default()
})
.insert(Name::new("Background"))
.id();

commands.entity(player).push_children(&[background]);
}

// plugin
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}
