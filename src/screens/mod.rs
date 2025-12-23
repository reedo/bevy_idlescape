//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}
