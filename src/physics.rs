use crate::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rise).add_systems(Update, fall);
    }
}

pub fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut physics, mut jump) = query.single_mut();

    let mut movement = time.delta_seconds() * (PLAYER_SPEED * JUMP_IMPULSE);

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match physics.translation {
        Some(vec) => physics.translation = Some(Vec2::new(vec.x, movement)),
        None => physics.translation = Some(Vec2::new(0.0, movement)),
    }
}

pub fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if query.is_empty() {
        return;
    }

    let mut physics = query.single_mut();

    let movement = time.delta_seconds() * (PLAYER_SPEED * JUMP_IMPULSE) / 1.5 * -1.0;

    match physics.translation {
        Some(vec) => physics.translation = Some(Vec2::new(vec.x, movement)),
        None => physics.translation = Some(Vec2::new(0.0, movement)),
    }
}
