use bevy::prelude::{Commands, CommandsStatesExt, States};
use tracing::info;

pub fn start_game(mut commands: Commands) {
    commands.set_state(GameState::Starting);
}

pub fn end_game(mut commands: Commands) {
    commands.set_state(GameState::Stopped);
}

pub fn load_game(mut commands: Commands) {
    info!("Loading game");

    commands.set_state(GameState::Playing);
}

pub fn simulate_gameplay() {
    info!("Simulating gameplay");
}

pub fn reset_game(mut commands: Commands) {
    info!("Resetting game");

    commands.set_state(GameState::Stopped);
}

/// The game's top level states.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Stopped,
    Starting,
    Playing,
    Stopping,
}
