use bevy::prelude::*;

use crate::{loading::PuzzleInputs, puzzle_input_asset::PuzzleInputAsset, AoCState};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<OrderedLocationLists>()
        .add_systems(OnEnter(AoCState::Day1), (init, process).chain());
}

#[derive(Default, Resource)]
struct OrderedLocationLists {
    pub left: Vec<i32>,
    pub right: Vec<i32>,
}

#[derive(Component)]
struct Day1;

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day1"),
            Day1,
            Node {
                align_items: AlignItems::Start,
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                justify_self: JustifySelf::Center,
                padding: UiRect::all(Val::Px(10.)),
                width: Val::Percent(100.),
                ..default()
            },
            StateScoped(AoCState::Day1),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Day 1: Historian Hysteria"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
        });
}

fn process(
    processed: ResMut<OrderedLocationLists>,
    puzzle_assets: Res<Assets<PuzzleInputAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
) {
    if let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.one) {
        let mut left_list: Vec<i32> = vec![];
        let mut right_list: Vec<i32> = vec![];
        for (left, right) in &puzzle.rows {
            left_list.push(*left);
            right_list.push(*right);
        }
    }
}

fn solve() {}
