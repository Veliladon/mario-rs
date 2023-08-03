use crate::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_physics);
    }
}

pub fn process_physics(mut player_query: Query<(&mut Player, &mut Transform)>, time: Res<Time>) {
    let (mut player, mut transform) = player_query.single_mut();
    transform.translation.x += player.velocity.x * PLAYER_SPEED * time.delta_seconds();
    player.velocity.x = player.velocity.x / 2.0;

    if player.velocity.y != 0.0 && transform.translation.y >= 0.0 {
        transform.translation.y += player.velocity.y * PLAYER_SPEED * time.delta_seconds();
        player.velocity.y -= 9.8 * time.delta_seconds();
        if player.velocity.y.is_sign_negative() {
            player.walking_state = PlayerState::Falling;
        }
    }

    if transform.translation.y < 0.0 {
        transform.translation.y = 0.;
        player.velocity.y = 0.;
        player.walking_state = PlayerState::Idle;
    }
}
