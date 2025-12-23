//! Top level game module.

use crate::screens::Screen;
use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    // Nothing requires setup.
}

/// Spawns an instance of the game when entering the gameplay screen.
///
/// Despawns when exiting the gameplay screen.
pub fn spawn_game(mut commands: Commands) {
    commands.spawn((
        Name::new("Game"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
    ));
}
