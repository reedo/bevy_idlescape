use crate::{screens::Screen, theme::widget};
use bevy::{ecs::children, prelude::*};

pub(crate) fn plugin(_app: &mut App) {
    // No setup to do yet.
}

/// A system that spawns the level.
pub fn spawn_level(mut commands: Commands) {
    info!("Spawning fishing level");

    commands.spawn((
        Name::new("Fishing Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![(
            widget::ui_root("UI Root - Fishing Level"),
            children![
                (
                    Name::new("Left Panel"),
                    Node {
                        width: px(250.0),
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
                ),
                (
                    // Name::new("Viewport"),
                    widget::header("Game Window"),
                    Node {
                        flex_grow: 1.0,
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
                ),
                (
                    // Name::new("Right Panel"),
                    widget::header("Right Panel"),
                    Node {
                        width: px(250.0),
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                ),
            ],
        )],
    ));
}
