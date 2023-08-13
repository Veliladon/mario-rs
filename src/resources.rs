use bevy::{prelude::*, utils::HashSet};

#[derive(Resource)]
pub struct PlayerAssets {
    pub handle: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct BackgroundAssets {
    pub handle: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct GameAssets {
    pub background: Handle<Image>,
    pub tiles: Handle<TextureAtlas>,
}
#[derive(Resource)]
pub struct VisibleChunks {
    pub chunk_list: HashSet<usize>,
}
