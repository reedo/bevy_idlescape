//! Top level game module.

mod state;
mod ui;

use bevy::prelude::*;

pub use state::{GameState, end_game, start_game};

pub(crate) fn plugin(app: &mut App) {
    app.init_state::<GameState>();

    app.add_systems(OnEnter(GameState::Starting), state::load_game);
    app.add_systems(OnEnter(GameState::Playing), state::simulate_gameplay);
    app.add_systems(OnEnter(GameState::Stopping), state::reset_game);
}
