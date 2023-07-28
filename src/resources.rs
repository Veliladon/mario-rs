use crate::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAssets {
    pub sheet: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct GameAssets {
    pub background: Handle<Image>,
    pub tiles: Handle<TextureAtlas>,
}
