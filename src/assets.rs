use crate::*;
use bevy::prelude::*;

pub struct AssetLoadingPlugin;

pub const PLAYER_SPRITE_SHEET: &str = "blue_alien.png";
pub const PLAYER_SPRITE_WIDTH: f32 = 16.;
pub const PLAYER_SPRITE_HEIGHT: f32 = 20.;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_texture_handle = asset_server.load(PLAYER_SPRITE_SHEET);

    let player_texture_atlas = TextureAtlas::from_grid(
        player_texture_handle,
        Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT),
        11,
        1,
        None,
        None,
    );

    let player_atlas = texture_atlases.add(player_texture_atlas);
    let player_assets: PlayerAssets = PlayerAssets {
        handle: player_atlas,
    };
    commands.insert_resource(player_assets);
    info!("Inserted Player Assets!");
}
