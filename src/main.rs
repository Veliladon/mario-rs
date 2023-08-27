mod assets;
mod components;
mod debugizmo;
mod input;
mod levelgenerator;
mod levelrender;
mod physics;
mod playercamera;
mod playercontroller;
mod resources;
mod states;

pub use crate::assets::*;
pub use crate::components::*;
pub use crate::debugizmo::*;
pub use crate::input::*;
pub use crate::levelgenerator::*;
pub use crate::levelrender::*;
pub use crate::physics::*;
pub use crate::playercamera::*;
pub use crate::resources::*;
pub use crate::states::*;
pub use playercontroller::*;

use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub use leafwing_input_manager::prelude::*;

pub use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PLAYER_SPEED: f32 = 400.;
pub const JUMP_HEIGHT: f32 = 2.;
pub const SCALE: f32 = 2.;

pub const PLAYER_SPRITE_SHEET: &str = "blue_alien.png";
pub const PLAYER_SPRITE_WIDTH: f32 = 16.;
pub const PLAYER_SPRITE_HEIGHT: f32 = 20.;

pub const BG_SPRITE_SHEET: &str = "tiles_packed.png";
pub const BG_UNIT_HEIGHT: f32 = 18.;
pub const BG_UNIT_WIDTH: f32 = 18.;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 32;
pub const GROUND_HEIGHT: usize = 2;
pub const CHUNK_PER_LEVEL: usize = 16;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::TURQUOISE))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                }),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AssetLoadingPlugin)
        .add_plugins(PlayerCameraPlugin)
        .add_systems(Startup, spawn_player)
        .add_plugins(InputPlugin)
        .add_plugins(LevelGeneratorPlugin)
        .add_plugins(LevelRenderPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(DebugGizmoPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}

pub fn spawn_player(mut commands: Commands, texture_atlas: Res<PlayerAssets>) {
    // Spawn Player Sprite
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas.handle.clone(),
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 5.0),
                ..default()
            },
            sprite: TextureAtlasSprite::new(0),
            ..default()
        })
        .insert(InputManagerBundle::<PlatformAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::Left, PlatformAction::Left),
                (KeyCode::Right, PlatformAction::Right),
                (KeyCode::Space, PlatformAction::Jump),
            ]),
        })
        .insert(Player {
            walking_state: PlayerState::Idle,
            velocity: Vec2::new(0., 0.),
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            PLAYER_SPRITE_WIDTH / 2.,
            PLAYER_SPRITE_HEIGHT / 2.,
        ))
        .insert(KinematicCharacterController::default());
    info!("Spawned Player");
}
