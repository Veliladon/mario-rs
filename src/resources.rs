use bevy::{prelude::*, utils::HashMap, utils::HashSet};

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

impl Default for VisibleChunks {
    fn default() -> VisibleChunks {
        VisibleChunks {
            chunk_list: HashSet::new(),
        }
    }
}

#[derive(Resource)]
pub struct VisibleChunksMap {
    pub chunk_list: HashMap<usize, Entity>,
}
