use crate::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_physics.after(player_control))
            .add_systems(Update, resolve_state.after(process_physics));

        //.add_systems(Update, process_physics);
    }
}

pub fn process_physics(
    mut player_query: Query<&mut Player>,
    mut physics_query: Query<&mut KinematicCharacterController>,
    time: Res<Time>,
) {
    let mut player = player_query.single_mut();
    let mut physics = physics_query.single_mut();
    let mut translation = Vec2::new(0., 0.);

    translation.x += player.velocity.x * PLAYER_SPEED * time.delta_seconds();
    player.velocity.x = player.velocity.x / 2.0;

    translation.y += player.velocity.y * PLAYER_SPEED * time.delta_seconds();
    player.velocity.y -= 9.8 * time.delta_seconds();

    physics.translation = Some(translation);

    /*if transform.translation.y < 0.0 {
        transform.translation.y = 0.;
        player.velocity.y = 0.;
        player.walking_state = PlayerState::Idle;
    } */
}

pub fn resolve_state(
    mut player_query: Query<&mut Player>,
    physics_query: Query<&KinematicCharacterControllerOutput>,
) {
    let mut player = player_query.single_mut();

    if let Ok(physics) = physics_query.get_single() {
        if physics.grounded {
            player.walking_state = PlayerState::Idle;
        }

        if physics.grounded && player.velocity.x != 0. {
            player.walking_state = PlayerState::Walking;
        }

        if !physics.grounded && player.velocity.y >= 0. {
            player.walking_state = PlayerState::Jumping;
        }

        if !physics.grounded && player.velocity.y < 0. {
            player.walking_state = PlayerState::Falling;
        }
    } else {
        return;
    }
}
