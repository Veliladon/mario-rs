use crate::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Player {
    pub walking_state: PlayerState,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct PrimaryCamera;

#[derive(Component)]
pub struct Chunk;
