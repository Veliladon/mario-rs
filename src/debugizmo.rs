use crate::*;
use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum DebugGizmoState {
    #[default]
    HideGizmos,
    ShowGizmos,
}

pub struct DebugGizmoPlugin;

impl Plugin for DebugGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<DebugGizmoState>()
            .add_systems(Update, debug_control)
            .add_systems(OnEnter(DebugGizmoState::ShowGizmos), add_player_gizmo)
            .add_systems(OnEnter(DebugGizmoState::HideGizmos), remove_player_gizmo);
    }
}

pub fn debug_control(
    keys: Res<Input<KeyCode>>,
    current_state: Res<State<DebugGizmoState>>,
    mut next_state: ResMut<NextState<DebugGizmoState>>,
) {
    if keys.just_pressed(KeyCode::G) {
        next_state.set(DebugGizmoState::HideGizmos);
        if *current_state.get() == DebugGizmoState::HideGizmos {
            next_state.set(DebugGizmoState::ShowGizmos);
        }
        info!("State Changed to: {:?}", next_state);
    }
}

pub fn add_player_gizmo(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let entity = player_query.get_single().unwrap();
    commands.entity(entity).insert(AabbGizmo {
        color: Some(Color::BLACK),
    });
}

pub fn remove_player_gizmo(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let entity = player_query.get_single().unwrap();
    commands.entity(entity).remove::<AabbGizmo>();
}
