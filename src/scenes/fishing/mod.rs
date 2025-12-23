//! Scene for training the fishing skill.

pub mod level;

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(level::plugin);
}
