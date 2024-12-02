use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    puzzle_input_asset::{PuzzleInputAsset, PuzzleInputAssetLoader},
    AoCState,
};

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<PuzzleInputAsset>()
        .init_asset_loader::<PuzzleInputAssetLoader>()
        .add_loading_state(
            LoadingState::new(AoCState::Loading)
                .continue_to_state(AoCState::Menu)
                .load_collection::<PuzzleInputs>(),
        );
}

#[derive(AssetCollection, Resource)]
pub(crate) struct PuzzleInputs {
    #[asset(path = "input/1.aoc")]
    pub one: Handle<PuzzleInputAsset>,
}
