use crate::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub walking_state: PlayerState,
    pub velocity: Vec2,
}
