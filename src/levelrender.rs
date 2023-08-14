use crate::*;
use bevy::{prelude::*, utils::HashSet};

pub struct LevelRenderPlugin;

impl Plugin for LevelRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level)
            .add_systems(Update, render_level);
    }
}

pub fn spawn_level(
    mut commands: Commands,
    background_assets: Res<BackgroundAssets>,
    mut game_data: ResMut<GameWorld>,
) {
    // let level_chunk = LevelChunk::default();

    let visible_chunks = find_visible_chunks(100.);

    for chunk in visible_chunks.chunk_list.iter() {
        let chunk_id = create_chunk(
            &mut commands,
            *chunk,
            &game_data.level.get(0).unwrap().chunks.get(*chunk).unwrap(),
            &background_assets,
        );
        game_data
            .level
            .get_mut(0)
            .unwrap()
            .chunk_map
            .insert(*chunk, chunk_id);
    }

    commands.insert_resource(visible_chunks);
}

pub fn create_chunk(
    commands: &mut Commands,
    id: usize,
    level_chunk: &LevelChunk,
    background_assets: &Res<BackgroundAssets>,
) -> Entity {
    let chunk_entity = commands
        .spawn_empty()
        .insert(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(
                id as f32 * BG_UNIT_WIDTH * CHUNK_WIDTH as f32,
                0.,
                2.,
            )),
            visibility: Visibility::Visible,
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_WIDTH {
                    if let Some(tile) = level_chunk.data[y * CHUNK_HEIGHT + x] {
                        parent.spawn(SpriteSheetBundle {
                            texture_atlas: background_assets.handle.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    (x as f32 * BG_UNIT_WIDTH) + (BG_UNIT_WIDTH / 2.0),
                                    (y as f32 * BG_UNIT_HEIGHT) + (BG_UNIT_HEIGHT / 2.0),
                                    2.0,
                                ),
                                ..Default::default()
                            },
                            sprite: TextureAtlasSprite::new(tile as usize),
                            ..Default::default()
                        });
                    }
                }
            }
        })
        .id();
    chunk_entity
}

pub fn render_level(
    mut commands: Commands,
    visible_chunks: Res<VisibleChunks>,
    player_query: Query<&Transform, With<Player>>,
    background_assets: Res<BackgroundAssets>,
    mut game_data: ResMut<GameWorld>,
) {
    let transform = player_query.get_single().unwrap();
    let new_visible_chunks = find_visible_chunks(transform.translation.x);

    for chunk in visible_chunks.chunk_list.iter() {
        if new_visible_chunks.chunk_list.get(chunk).is_none() {
            info!("Chunk {:?} has disappeared", chunk);
        }
    }

    for chunk in new_visible_chunks.chunk_list.iter() {
        if visible_chunks.chunk_list.get(chunk).is_none() {
            info!("Chunk {:?} has appeared", chunk);
            let new_chunk_id = create_chunk(
                &mut commands,
                *chunk,
                &game_data.level.get(0).unwrap().chunks.get(*chunk).unwrap(),
                &background_assets,
            );
            game_data
                .level
                .get_mut(0)
                .unwrap()
                .chunk_map
                .insert(*chunk, new_chunk_id);
            info!("Created chunk {:?} - Entity {:?}", chunk, new_chunk_id);
        }
    }

    commands.insert_resource(new_visible_chunks);
}

pub fn find_visible_chunks(player_location: f32) -> VisibleChunks {
    // Chunk player is in
    let player_chunk = player_location as usize / (CHUNK_WIDTH * BG_UNIT_WIDTH as usize);

    // If they're in the first chunk don't give back -1
    let mut previous_chunk = None;
    if player_chunk != 0 {
        previous_chunk = Some(player_chunk - 1);
    }

    // If they're in the final chunk we can just generate a blank chunk to display after anyway
    let next_chunk = player_chunk + 1;

    let mut chunk_list = HashSet::new();
    if previous_chunk.is_some() {
        chunk_list.insert(previous_chunk.unwrap());
    }
    chunk_list.insert(player_chunk);
    chunk_list.insert(next_chunk);
    //info!("{:?}", chunk_list);
    VisibleChunks { chunk_list }
}
