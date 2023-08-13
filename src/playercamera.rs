use crate::*;
use bevy::prelude::*;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_camera)
            .add_systems(PostUpdate, follow_camera);
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    // Spawn the main Camera, PrimaryCamera component means the zoom system will never panic
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(
                    window.width() / (SCALE * 2.),
                    window.height() / (SCALE * 2.),
                    999.9,
                ),
                ..Default::default()
            },
            projection: OrthographicProjection {
                scale: (1. / SCALE),
                ..default()
            },
            ..default()
        })
        .insert(PrimaryCamera);
}

pub fn follow_camera(
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let window = window_query.get_single().unwrap();
    let player = player_query.get_single().unwrap();
    let mut camera = camera_query.get_single_mut().unwrap();

    let left_clamp = window.width() / (2.0 * SCALE);

    camera.translation.x = player.translation.x;

    if camera.translation.x <= left_clamp {
        camera.translation.x = left_clamp;
    }
}
