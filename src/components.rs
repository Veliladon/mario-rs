use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Player;

#[derive(Component)]
pub struct PrimaryCamera;

#[derive(Component)]
pub struct Chunk;

#[derive(Component, Reflect, Default)]
pub struct Jump(pub f32);
