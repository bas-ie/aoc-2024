use bevy::prelude::*;

use crate::{loading::PuzzleInputs, puzzle_input_asset::PuzzleInputAsset, AoCState};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<LocationDistances>()
        .init_resource::<OrderedLocationLists>()
        .add_systems(
            OnEnter(AoCState::Day1),
            (init, process, solve_a, solve_b).chain(),
        );
    // .add_systems(Update, visualise);
}

#[derive(Component)]
struct CurrentRow;

#[derive(Default, Resource)]
struct OrderedLocationLists {
    pub left: Vec<i32>,
    pub right: Vec<i32>,
}

#[derive(Default, Resource)]
struct LocationDistances {
    pub all: Vec<i32>,
    pub total: i32,
}

#[derive(Component)]
struct Day1;

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day1"),
            Day1,
            Node {
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.),
                justify_content: JustifyContent::Start,
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
    mut processed: ResMut<OrderedLocationLists>,
    puzzle_assets: Res<Assets<PuzzleInputAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
) {
    if let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.one) {
        let mut left_list: Vec<i32> = vec![];
        let mut right_list: Vec<i32> = vec![];
        for row in &puzzle.rows {
            left_list.push(row[0]);
            right_list.push(row[1]);
        }
        left_list.sort();
        right_list.sort();
        processed.left = left_list;
        processed.right = right_list;
    }
}

fn solve_a(
    mut commands: Commands,
    day1: Single<Entity, With<Day1>>,
    locations: Res<OrderedLocationLists>,
    mut distances: ResMut<LocationDistances>,
) {
    for (left, right) in locations.left.iter().zip(locations.right.iter()) {
        distances.all.push(left.max(right) - left.min(right));
    }
    distances.total = distances.all.iter().sum();
    dbg!(distances.total);
    commands.entity(*day1).with_children(|p| {
        p.spawn(Text::new(format!(
            "Total distance between lists: {}",
            distances.total
        )));
    });
}

fn solve_b(
    mut commands: Commands,
    day1: Single<Entity, With<Day1>>,
    locations: Res<OrderedLocationLists>,
    mut similarity: Local<i32>,
) {
    for loc in &locations.left {
        *similarity += loc * locations.right.iter().filter(|&x| x == loc).count() as i32;
    }
    dbg!(&similarity);
    commands.entity(*day1).with_children(|p| {
        p.spawn(Text::new(format!("List similarity: {}", *similarity)));
    });
}

// fn visualise(
//     mut commands: Commands,
//     mut current_row: Local<usize>,
//     locations: Res<OrderedLocationLists>,
//     distances: ResMut<LocationDistances>,
//     row_visualisation: Query<Entity, With<CurrentRow>>,
// ) {
//     if *current_row < distances.all.len() {
//         if let Ok(vis) = row_visualisation.get_single() {
//             commands.entity(vis).despawn_recursive();
//         }
//
//         let (left, right) =
//         let vis = commands.spawn(CurrentRow).id();
//         for n in 0..
//     }
// }
