use crate::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlatformAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Jump,
    Ability2,
    Ability3,
    Ability4,
    Ultimate,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlatformAction>::default())
            .add_systems(Update, player_control);
        //        .add_systems(Update, walk);
    }
}

pub fn player_control(mut player_query: Query<(&ActionState<PlatformAction>, &mut Player)>) {
    let (action_state, mut player) = player_query.single_mut();

    let mut direction = player.velocity;
    let mut next_state = PlayerState::Idle;

    if action_state.pressed(PlatformAction::Left) {
        direction.x = -1.0;
        next_state = PlayerState::Walking;
    }
    if action_state.pressed(PlatformAction::Right) {
        direction.x = 1.0;
        next_state = PlayerState::Walking;
    }

    // Player must be on the ground to jump.

    if action_state.just_pressed(PlatformAction::Jump)
        && (player.walking_state == PlayerState::Idle
            || player.walking_state == PlayerState::Walking)
    {
        direction.y = JUMP_HEIGHT;
        next_state = PlayerState::Jumping;
        info!("Jump!");
    }

    // Physics module takes the player state back to idle after finishing the jump

    if player.walking_state == PlayerState::Jumping || player.walking_state == PlayerState::Falling
    {
        next_state = player.walking_state.clone();
    }

    player.velocity = direction;
    player.walking_state = next_state;
}
