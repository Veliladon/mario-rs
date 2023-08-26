use bevy::prelude::*;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, player_controller);
    }
}

pub fn player_controller() {
    todo!();
}
