use crate::*;
use bevy::prelude::*;

pub struct AssetLoadingPlugin;

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

    let background_texture_handle = asset_server.load(BG_SPRITE_SHEET);

    let background_texture_atlas = TextureAtlas::from_grid(
        background_texture_handle,
        Vec2::new(BG_UNIT_WIDTH, BG_UNIT_HEIGHT),
        20,
        9,
        None,
        None,
    );

    let background_atlas = texture_atlases.add(background_texture_atlas);
    let background_assets: BackgroundAssets = BackgroundAssets {
        handle: background_atlas,
    };
    commands.insert_resource(background_assets);
    info!("Inserted BG Assets!");
}
