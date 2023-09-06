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
            .add_systems(Update, player_control)
            .add_systems(Update, jump)
            .register_type::<Player>();

        //        .add_systems(Update, walk);
    }
}

pub fn player_control(
    action_query: Query<&ActionState<PlatformAction>>,
    mut physics_query: Query<&mut KinematicCharacterController>,
    time: Res<Time>,
) {
    let action = action_query.single();
    let mut physics = physics_query.single_mut();
    let mut direction = 0.0;

    if action.pressed(PlatformAction::Left) {
        direction = -1.0 * time.delta_seconds() * PLAYER_SPEED;
    }
    if action.pressed(PlatformAction::Right) {
        direction = 1.0 * time.delta_seconds() * PLAYER_SPEED;
    }
    match physics.translation {
        Some(vec) => physics.translation = Some(Vec2::new(direction, vec.y)),
        None => physics.translation = Some(Vec2::new(direction, 0.0)),
    }
}

pub fn jump(
    action_query: Query<&ActionState<PlatformAction>>,
    mut commands: Commands,
    player_query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if player_query.is_empty() {
        return;
    }

    let action = action_query.single();
    let (player, output) = player_query.single();

    if action.pressed(PlatformAction::Jump) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }
}
