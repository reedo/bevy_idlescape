use crate::game::state::{FishType, FishingState, GameLog, Inventory};
use crate::screens::Screen;
use crate::theme::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_fishing_ui);
    app.add_systems(
        Update,
        (update_catch_list, update_game_log).run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
struct CatchListRoot;

#[derive(Component)]
struct LogRoot;

fn spawn_fishing_ui(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Fishing UI Root"),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(px(20.0)),
                ..default()
            },
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|parent| {
            // Main Top Area (Three columns)
            parent
                .spawn((
                    Name::new("Main Area"),
                    Node {
                        width: percent(100.0),
                        height: percent(70.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                ))
                .with_children(|main| {
                    // Left Column: Fish Selection
                    main.spawn((
                        Name::new("Left Column"),
                        Node {
                            flex_direction: FlexDirection::Column,
                            width: percent(25.0),
                            row_gap: px(10.0),
                            ..default()
                        },
                    ))
                    .with_children(|left| {
                        left.spawn(widget::header("Select Fish"));
                        left.spawn(widget::button("Salmon", select_salmon));
                        left.spawn(widget::button("Trout", select_trout));
                    });

                    // Middle Column: Visual Scene Placeholder
                    main.spawn((
                        Name::new("Middle Column"),
                        Node {
                            flex_direction: FlexDirection::Column,
                            width: percent(45.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                    ))
                    .with_children(|middle| {
                        middle.spawn((
                            Name::new("Visual Placeholder"),
                            Node {
                                width: px(300.0),
                                height: px(300.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.4, 0.6)), // Blue for water
                        ));
                        middle.spawn(widget::label("The River"));
                    });

                    // Right Column: Catch List
                    main.spawn((
                        Name::new("Right Column"),
                        Node {
                            flex_direction: FlexDirection::Column,
                            width: percent(25.0),
                            row_gap: px(10.0),
                            ..default()
                        },
                    ))
                    .with_children(|right| {
                        right.spawn(widget::header("Catches"));
                        right.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            CatchListRoot,
                        ));
                    });
                });

            // Bottom Panel: Game Log
            parent
                .spawn((
                    Name::new("Bottom Panel"),
                    Node {
                        width: percent(100.0),
                        height: percent(30.0),
                        flex_direction: FlexDirection::Column,
                        border: UiRect::top(px(2.0)),
                        padding: UiRect::top(px(10.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                ))
                .with_children(|bottom| {
                    bottom.spawn(widget::header("Log"));
                    bottom.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        LogRoot,
                    ));
                });
        });
}

fn select_salmon(_: On<Pointer<Click>>, mut fishing_state: ResMut<FishingState>) {
    fishing_state.selected_fish = FishType::Salmon;
}

fn select_trout(_: On<Pointer<Click>>, mut fishing_state: ResMut<FishingState>) {
    fishing_state.selected_fish = FishType::Trout;
}

fn update_catch_list(
    mut commands: Commands,
    inventory: Option<Res<Inventory>>,
    query: Query<Entity, With<CatchListRoot>>,
) {
    let Some(inventory) = inventory else {
        return;
    };

    if !inventory.is_changed() {
        return;
    }

    if let Ok(root_entity) = query.single() {
        commands.entity(root_entity).despawn_children();
        commands.entity(root_entity).with_children(|parent| {
            // Show last 10 catches
            for fish in inventory.catches.iter().rev().take(10) {
                parent.spawn(widget::label(fish.name()));
            }
        });
    }
}

fn update_game_log(
    mut commands: Commands,
    game_log: Option<Res<GameLog>>,
    query: Query<Entity, With<LogRoot>>,
) {
    let Some(game_log) = game_log else {
        return;
    };

    if !game_log.is_changed() {
        return;
    }

    if let Ok(root_entity) = query.single() {
        commands.entity(root_entity).despawn_children();
        commands.entity(root_entity).with_children(|parent| {
            for entry in game_log.entries.iter().rev() {
                parent.spawn(widget::label(entry.clone()));
            }
        });
    }
}
