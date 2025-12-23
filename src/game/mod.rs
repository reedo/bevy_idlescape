//! Top level game module.

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_state::<GameState>();

    app.add_systems(OnEnter(GameState::Starting), load_game);
    app.add_systems(OnEnter(GameState::Playing), simulate_gameplay);
    app.add_systems(OnEnter(GameState::Stopping), reset_game);
}

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
