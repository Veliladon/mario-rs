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

pub fn player_control(
    mut player_query: Query<(&ActionState<PlatformAction>, &mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    let mut direction = Vec2::new(0.0, 0.0);
    let mut next_state = PlayerState::Idle;

    let (action_state, mut player, mut transform) = player_query.single_mut();
    //info!("{:?}", action_state);
    if action_state.pressed(PlatformAction::Left) {
        direction.x = -1.0;
        next_state = PlayerState::Walking;
    }
    if action_state.pressed(PlatformAction::Right) {
        direction.x = 1.0;
        next_state = PlayerState::Walking;
    }
    if action_state.just_pressed(PlatformAction::Jump) {
        info!("Jump!");
    }

    transform.translation.x =
        transform.translation.x + (direction.x * PLAYER_SPEED * time.delta_seconds());
    player.walking_state = next_state;
}
