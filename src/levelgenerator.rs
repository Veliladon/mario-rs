use crate::*;
use bevy::prelude::*;

pub struct LevelGeneratorPlugin;

#[derive(Resource, Clone, Debug)]
pub struct GameWorld {
    pub level: Vec<LevelData>,
}
#[derive(Clone, Debug)]
pub struct LevelData {
    pub chunks: Vec<LevelChunk>,
    // pub chunk_map: HashMap<usize, Entity>,
}

#[derive(Clone, Debug)]
pub struct LevelChunk {
    pub data: Vec<Option<Tile>>,
}

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Ground = 22,
    GroudRightEdge = 23,
    GroundLeftEdge = 21,
    RightEdge = 123,
    LeftEdge = 121,
    Filler = 122,
}

impl Plugin for LevelGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, generate_world);
    }
}

pub fn generate_world(mut commands: Commands) {
    let game_world = GameWorld::default();
    commands.insert_resource(game_world);
}

impl Default for GameWorld {
    fn default() -> Self {
        let mut level = Vec::new();
        let first_level = LevelData::default();
        level.push(first_level);
        GameWorld { level }
    }
}

impl Default for LevelData {
    fn default() -> Self {
        let mut chunks = Vec::new();

        // First chunk is flat without any obstacles
        let first_chunk = construct_flat_level_chunk();
        chunks.push(first_chunk);

        chunks.push(construct_gap(3, 3, construct_flat_level_chunk()));

        // Intermediate chunks can be created however we want, for now just flat ground
        for _ in 2..(CHUNK_PER_LEVEL - 1) {
            chunks.push(construct_flat_level_chunk());
        }

        // Last chunk has goal tape or whatever
        let last_chunk = construct_flat_level_chunk();
        chunks.push(last_chunk);
        info!("{} chunks generated", chunks.len());
        LevelData { chunks }
    }
}

impl LevelChunk {
    pub fn new() -> Self {
        LevelChunk { data: Vec::new() }
    }
}

impl Default for LevelChunk {
    fn default() -> Self {
        construct_flat_level_chunk()
    }
}

pub fn construct_flat_level_chunk() -> LevelChunk {
    let mut data: Vec<Option<Tile>> = vec![None; CHUNK_HEIGHT * CHUNK_WIDTH];
    for x in 0..CHUNK_WIDTH {
        data[GROUND_HEIGHT * CHUNK_WIDTH + x] = Some(Tile::Ground);
    }
    for y in (0..GROUND_HEIGHT).rev() {
        for x in 0..CHUNK_WIDTH {
            data[y * CHUNK_WIDTH + x] = Some(Tile::Filler);
        }
    }

    LevelChunk { data }
}

pub fn construct_mushroom(x: usize, y: usize, width: usize, chunk: LevelChunk) {}

pub fn construct_gap(x: usize, width: usize, mut chunk: LevelChunk) -> LevelChunk {
    for y in 0..CHUNK_HEIGHT {
        for x in x..(x + width) {
            chunk.data[y * CHUNK_WIDTH + x] = None;
        }
    }
    chunk
}
