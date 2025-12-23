use crate::screens::Screen;
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
            Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
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
                    Name::new("Viewport"),
                    Node {
                        flex_grow: 1.0,
                        height: percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
                ),
                (
                    Name::new("Right Panel"),
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
