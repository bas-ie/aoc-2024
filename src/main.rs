// #![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use bevy::{dev_tools::states::log_transitions, prelude::*};

mod days;
mod loading;
mod menu;
mod puzzle_input_asset;
mod puzzle_input_lines_asset;
mod puzzle_input_string_asset;

#[derive(States, Debug, Hash, PartialEq, Eq, Copy, Clone, Default)]
pub enum AoCState {
    #[default]
    Loading,
    Menu,
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, loading::plugin, menu::plugin, days::plugin))
        .init_state::<AoCState>()
        .enable_state_scoped_entities::<AoCState>()
        .add_systems(Update, log_transitions::<AoCState>)
        .run();
}
