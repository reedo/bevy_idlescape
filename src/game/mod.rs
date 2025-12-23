//! Top level game module.

mod state;
mod ui;

use bevy::prelude::*;

pub use state::{GameState, end_game, start_game};

pub(crate) fn plugin(app: &mut App) {
    app.init_state::<GameState>();

    app.add_plugins(ui::plugin);

    app.add_systems(OnEnter(GameState::Starting), state::load_game);
    app.add_systems(
        Update,
        state::fishing_system.run_if(in_state(GameState::Playing)),
    );
    app.add_systems(OnEnter(GameState::Stopping), state::reset_game);
}
