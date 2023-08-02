mod assets;
mod components;
mod input;
mod resources;
mod states;

pub use crate::assets::*;
pub use crate::components::*;
pub use crate::input::*;
pub use crate::resources::*;
pub use crate::states::*;

use bevy::log::LogPlugin;
use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 400.;

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
        .add_plugins(InputPlugin)
        .run();
}

pub fn game_setup(mut commands: Commands, texture_atlas: Res<PlayerAssets>) {
    // Spawn Camera
    commands.spawn(Camera2dBundle::default());

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
        .insert(Player {
            walking_state: PlayerState::Idle,
        });
    info!("Spawned Player");
}
