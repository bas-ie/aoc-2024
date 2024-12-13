use bevy::prelude::*;

use crate::{loading::PuzzleInputs, puzzle_input_asset::PuzzleInputAsset, AoCState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AoCState::Day2),
        (init, process, solve_a, solve_b).chain(),
    );
}

#[derive(Component)]
struct Day2;

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day2"),
            Day2,
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
            StateScoped(AoCState::Day2),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Day 2: Red-Nosed Reports"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
        });
}

// Currently no processing required.
fn process() {}

fn is_safe(report: &[i32]) -> bool {
    let mut diffs: Vec<i32> = vec![];
    let mut col_iter = report.iter().peekable();
    while let Some(lhs) = col_iter.next() {
        if let Some(rhs) = col_iter.peek() {
            diffs.push(*lhs - *rhs);
        }
    }

    if !diffs.iter().all(|x| *x > 0) && !diffs.iter().all(|x| *x < 0) {
        return false;
    }

    if diffs.iter().any(|x| x.abs() > 3) {
        return false;
    }

    true
}

fn solve_a(puzzle_assets: Res<Assets<PuzzleInputAsset>>, puzzle_inputs: Res<PuzzleInputs>) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.two) else {
        return;
    };
    let mut safe_reports = 0;

    for row in &puzzle.rows {
        if is_safe(row) {
            safe_reports += 1;
        }
    }
    dbg!(safe_reports);
}

fn solve_b(puzzle_assets: Res<Assets<PuzzleInputAsset>>, puzzle_inputs: Res<PuzzleInputs>) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.two) else {
        return;
    };
    let mut safe_reports = 0;

    for row in &puzzle.rows {
        if is_safe(row) {
            safe_reports += 1;
            continue;
        }

        for i in 0..row.len() {
            let test = [&row[..i], &row[i + 1..]].concat();
            if is_safe(&test) {
                safe_reports += 1;
                break;
            }
        }
    }
    dbg!(safe_reports);
}
