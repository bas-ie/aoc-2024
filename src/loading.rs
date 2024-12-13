use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    puzzle_input_asset::{PuzzleInputAsset, PuzzleInputAssetLoader},
    puzzle_input_lines_asset::{PuzzleInputLinesAsset, PuzzleInputLinesAssetLoader},
    AoCState,
};

pub fn plugin(app: &mut App) {
    app.init_asset::<PuzzleInputAsset>()
        .init_asset::<PuzzleInputLinesAsset>()
        .init_asset_loader::<PuzzleInputAssetLoader>()
        .init_asset_loader::<PuzzleInputLinesAssetLoader>()
        .add_loading_state(
            LoadingState::new(AoCState::Loading)
                .continue_to_state(AoCState::Menu)
                .load_collection::<PuzzleInputs>(),
        );
}

#[derive(AssetCollection, Resource)]
pub struct PuzzleInputs {
    #[asset(path = "input/1.aoc")]
    pub one: Handle<PuzzleInputAsset>,
    #[asset(path = "input/2.aoc")]
    pub two: Handle<PuzzleInputAsset>,
    #[asset(path = "input/3.aoc")]
    pub three: Handle<PuzzleInputLinesAsset>,
    #[asset(path = "input/4.aoc")]
    pub four: Handle<PuzzleInputLinesAsset>,
    #[asset(path = "input/5.aoc")]
    pub five: Handle<PuzzleInputLinesAsset>,
}
