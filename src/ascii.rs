use bevy::prelude::*;

use crate::TILE_SIZE;

// components
#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

// systems
pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    assert!(index < 256, "Index out of Ascii Range");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

fn load_ascii(
    // Spawns assets, textures, etc
    mut commands: Commands,
    // Helps with loading assets e.g. PNGs
    assets: Res<AssetServer>,
    // Helps with loading texture related assets
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets.load("ascii.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::splat(9.0),
        16,
        16,
        Some(Vec2::splat(2.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(AsciiSheet(texture_atlas_handle));
}

// plugins
pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_ascii.in_base_set(StartupSet::PreStartup));
    }
}
