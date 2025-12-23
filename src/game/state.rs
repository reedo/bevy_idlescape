use bevy::prelude::*;
use std::collections::VecDeque;
use tracing::info;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FishType {
    #[default]
    None,
    Salmon,
    Trout,
}

impl FishType {
    pub fn name(&self) -> &'static str {
        match self {
            FishType::None => "None",
            FishType::Salmon => "Salmon",
            FishType::Trout => "Trout",
        }
    }
}

#[derive(Resource, Default)]
pub struct FishingState {
    pub selected_fish: FishType,
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct Inventory {
    pub catches: Vec<FishType>,
}

#[derive(Resource, Default)]
pub struct GameLog {
    pub entries: VecDeque<String>,
}

impl GameLog {
    const MAX_ENTRIES: usize = 50;

    pub fn add_entry(&mut self, entry: impl Into<String>) {
        self.entries.push_back(entry.into());
        if self.entries.len() > Self::MAX_ENTRIES {
            self.entries.pop_front();
        }
    }
}

pub fn start_game(mut commands: Commands) {
    commands.set_state(GameState::Starting);
}

pub fn end_game(mut commands: Commands) {
    commands.set_state(GameState::Stopped);
}

pub fn load_game(mut commands: Commands) {
    info!("Loading game");

    commands.insert_resource(FishingState {
        selected_fish: FishType::None,
        timer: Timer::from_seconds(2.0, TimerMode::Repeating),
    });
    commands.init_resource::<Inventory>();
    commands.init_resource::<GameLog>();

    commands.set_state(GameState::Playing);
}

pub fn fishing_system(
    time: Res<Time>,
    mut fishing_state: ResMut<FishingState>,
    mut inventory: ResMut<Inventory>,
    mut game_log: ResMut<GameLog>,
) {
    if fishing_state.selected_fish == FishType::None {
        return;
    }

    if fishing_state.timer.tick(time.delta()).just_finished() {
        inventory.catches.push(fishing_state.selected_fish);
        let message = format!("Caught a {}", fishing_state.selected_fish.name());
        game_log.add_entry(message);
    }
}

pub fn reset_game(mut commands: Commands) {
    info!("Resetting game");

    commands.remove_resource::<FishingState>();
    commands.remove_resource::<Inventory>();
    commands.remove_resource::<GameLog>();

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
