mod assets;
mod components;
mod input;
mod physics;
mod resources;
mod states;

pub use crate::assets::*;
pub use crate::components::*;
pub use crate::input::*;
pub use crate::physics::*;
pub use crate::resources::*;
pub use crate::states::*;
pub use leafwing_input_manager::prelude::*;

pub use bevy::log::LogPlugin;
pub use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 400.;
pub const JUMP_HEIGHT: f32 = 2.;
pub const SCALE: f32 = 2.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.5)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                }),
        )
        .add_plugins(AssetLoadingPlugin)
        .add_systems(Startup, game_setup)
        .add_systems(PostStartup, zoom_2d)
        .add_plugins(InputPlugin)
        .add_plugins(PhysicsPlugin)
        .run();
}

pub fn game_setup(mut commands: Commands, texture_atlas: Res<PlayerAssets>) {
    // Spawn the main Camera, PrimaryCamera component means the zoom system will never panic
    commands
        .spawn(Camera2dBundle::default())
        .insert(PrimaryCamera);

    // Spawn Player Sprite
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas.handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
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
        });
    info!("Spawned Player");
}

fn zoom_2d(mut q: Query<&mut OrthographicProjection, With<PrimaryCamera>>) {
    let mut projection = q.single_mut();

    projection.scale *= 1. / SCALE;
}
