use crate::*;
use bevy::prelude::*;

pub struct LevelRenderPlugin;

impl Plugin for LevelRenderPlugin {
    fn build(&self, app: &mut App) {
        app //.add_systems(Startup, spawn_level)
            .insert_resource(VisibleChunks::default())
            .add_systems(PreUpdate, calculate_chunk_visibility)
            .add_systems(Update, render_level);
    }
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
                        parent
                            .spawn(SpriteSheetBundle {
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
                            })
                            .insert(RigidBody::Fixed)
                            .insert(Collider::cuboid(BG_UNIT_WIDTH / 2., BG_UNIT_HEIGHT / 2.));
                    }
                }
            }
        })
        .id();
    chunk_entity
}

pub fn render_level(
    mut commands: Commands,
    new_visible_chunks: Res<VisibleChunks>,
    background_assets: Res<BackgroundAssets>,
    game_data: Res<GameWorld>,
    mut visible_chunks: Local<VisibleChunksMap>,
) {
    let mut chunks_to_be_removed = Vec::new();

    for (chunk, _) in visible_chunks.chunk_list.iter() {
        if new_visible_chunks.chunk_list.get(chunk).is_none() {
            info!("Chunk {:?} has disappeared", chunk);
            chunks_to_be_removed.push(*chunk);
        }
    }

    for chunk in chunks_to_be_removed.iter() {
        let removed_chunk = visible_chunks.chunk_list.remove(chunk).unwrap();
        info!(
            "Chunk {:?} removed, entity {:?} deleted",
            chunk, removed_chunk
        );
        commands.entity(removed_chunk).despawn_recursive();
    }

    for chunk in new_visible_chunks.chunk_list.iter() {
        if visible_chunks.chunk_list.get(chunk).is_none() {
            info!("Chunk {:?} has appeared", chunk);
            let new_chunk_id = create_chunk(
                &mut commands,
                *chunk,
                game_data
                    .level
                    .get(0)
                    .unwrap()
                    .chunks
                    .get(*chunk)
                    .unwrap_or(&construct_flat_level_chunk()),
                &background_assets,
            );
            visible_chunks.chunk_list.insert(*chunk, new_chunk_id);
            info!("Created chunk {:?} - Entity {:?}", chunk, new_chunk_id);
        }
    }
}

pub fn calculate_chunk_visibility(
    camera_query: Query<&Camera, With<PrimaryCamera>>,
    transform_query: Query<&Transform, With<PrimaryCamera>>,
    mut visible_chunks: ResMut<VisibleChunks>,
) {
    visible_chunks.chunk_list.clear();

    let camera = camera_query.single();
    let transform = transform_query.single();

    let camera_width = camera.physical_target_size().unwrap().x as f32;
    let chunk_width = CHUNK_WIDTH as f32 * BG_UNIT_WIDTH;

    // what chunk is the middle of the camera?
    let middle_chunk = (transform.translation.x / chunk_width) as usize;
    // f32 exact number of chunks
    let exact_chunks_per_viewport = camera_width / chunk_width;

    // add one for the round up to usize, add two buffer each side
    let chunks_per_viewport = exact_chunks_per_viewport as usize + 5;

    let starting_chunk = middle_chunk.saturating_sub(chunks_per_viewport / 2);
    let ending_chunk = middle_chunk + chunks_per_viewport / 2;

    for chunk_index in starting_chunk..ending_chunk {
        visible_chunks.chunk_list.insert(chunk_index);
    }
}
