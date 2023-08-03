use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Jumping,
    Falling,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GroundedState {
    #[default]
    Grounded,
    Jumping,
    Falling,
}
