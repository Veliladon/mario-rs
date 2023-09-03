use crate::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_physics.after(player_control))
            .add_systems(PostUpdate, resolve_state_and_clamp); //.after(process_physics));

        //.add_systems(Update, process_physics);
    }
}

pub fn process_physics(
    player_query: Query<&mut Player>,
    mut physics_query: Query<&mut KinematicCharacterController>,
    time: Res<Time>,
) {
    let player = player_query.single();
    let mut physics = physics_query.single_mut();
    let mut translation = Vec2::new(0., 0.);

    translation.x += player.velocity.x * PLAYER_SPEED * time.delta_seconds();

    if player.velocity.y != 0.0 {
        translation.y += player.velocity.y * PLAYER_SPEED * time.delta_seconds();
    }

    physics.translation = Some(translation);
}

pub fn resolve_state_and_clamp(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    physics_query: Query<&KinematicCharacterControllerOutput>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    if let Ok(physics) = physics_query.get_single() {
        if physics.grounded {
            player.velocity.y = 0.;
            player.walking_state = PlayerState::Idle;
        }

        if physics.grounded && player.velocity.x != 0. {
            player.walking_state = PlayerState::Walking;
        }

        if physics.grounded == false {
            player.velocity.y -= 9.8 * time.delta_seconds();
        }

    /*   if physics.grounded == false && player.walking_state != PlayerState::Jumping {
        player.walking_state = PlayerState::Falling;
    } */

    /*         if physics.grounded == false
        && player.walking_state == PlayerState::Jumping
        && player.velocity.y <= 0.
    {
        player.walking_state = PlayerState::Falling;
    } */
    /*if physics.grounded
        && (player.walking_state != PlayerState::Jumping
            || player.walking_state != PlayerState::Falling)
    {} */
    } else {
        return;
    }

    if transform.translation.x < 0. + (PLAYER_SPRITE_WIDTH / 2.) {
        transform.translation.x = 0. + (PLAYER_SPRITE_WIDTH / 2.);
    }

    player.velocity.x = player.velocity.x / 2.0;
}
