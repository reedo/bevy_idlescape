use bevy::prelude::*;
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

    commands.set_state(GameState::Playing);
}

pub fn fishing_system(
    time: Res<Time>,
    mut fishing_state: ResMut<FishingState>,
    mut inventory: ResMut<Inventory>,
) {
    if fishing_state.selected_fish == FishType::None {
        return;
    }

    if fishing_state.timer.tick(time.delta()).just_finished() {
        inventory.catches.push(fishing_state.selected_fish);
        info!("Caught a {:?}", fishing_state.selected_fish);
    }
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
