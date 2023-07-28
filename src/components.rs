use crate::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
}
