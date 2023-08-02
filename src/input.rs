use crate::*;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_input);
    }
}

pub fn process_input(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    let mut direction = Vec2::new(0.0, 0.0);
    let mut next_state = PlayerState::Idle;

    if keyboard.pressed(KeyCode::Left) {
        direction.x = -1.0;
        next_state = PlayerState::Walking;
    }
    if keyboard.pressed(KeyCode::Right) {
        direction.x = 1.0;
        next_state = PlayerState::Walking;
    }

    transform.translation.x =
        transform.translation.x + (direction.x * PLAYER_SPEED * time.delta_seconds());
    player.walking_state = next_state;
    info!("Player State: {:?}", player.walking_state);
}
