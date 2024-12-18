#![allow(dead_code, unused)]

use crate::{loading::PuzzleInputs, puzzle_input_string_asset::PuzzleInputStringAsset, AoCState};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use miette::miette;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<FileMap>().add_systems(
        OnEnter(AoCState::Day9),
        (init, process, solve_a, solve_b, vis).chain(),
    );
}

#[derive(Resource, Default)]
struct FileMap {
    blocks: Vec<Option<u64>>,
}

#[derive(Component)]
struct Day9;

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day9"),
            Day9,
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
            StateScoped(AoCState::Day9),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Day 9: Disk Fragmenter"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
        });
}

fn process(
    mut file_map: ResMut<FileMap>,
    puzzle_assets: Res<Assets<PuzzleInputStringAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.nine) else {
        return;
    };

    let mut blocks: Vec<char> = puzzle.0.chars().collect();
    file_map.blocks = get_file_blocks(&blocks).unwrap();
    compact_blocks(&mut file_map.blocks);
}

fn get_file_blocks(disk_map: &[char]) -> miette::Result<Vec<Option<u64>>> {
    let mut blocks: Vec<Option<u64>> = vec![];
    for (i, c) in disk_map.iter().enumerate() {
        if *c == '\n' {
            continue;
        }
        let val = c
            .to_digit(10)
            .ok_or_else(|| miette!("couldn't parse char to int: {}", c))?;
        let mut block: Option<u64> = if i % 2 == 0 {
            // This is a file length
            let mut id = if i != 0 { i as u64 / 2 } else { 0 };
            Some(id)
        } else {
            None
        };

        for pos in 0..val {
            blocks.push(block);
        }
    }
    Ok(blocks)
}

// fn get_next_block<'a, I>(it: I) -> Option<u64>
// where
//     I: Iterator<Item = &'a Option<u64>>,
// {
//     for block in it {
//         if block.is_none() {
//             continue;
//         }
//         return *block;
//     }
//
//     None
// }

fn compact_blocks(blocks: &mut [Option<u64>]) -> miette::Result<()> {
    let mut i = 0;
    let mut j = blocks.len();

    while i < blocks.len() {
        if blocks[i].is_none() {
            while j > i {
                j -= 1;
                if blocks[j].is_some() {
                    break;
                }
            }
            if j > i {
                blocks[i] = blocks[j];
                blocks[j] = None;
            }
        }
        i += 1;
    }

    Ok(())
}

fn solve_a(file_map: Res<FileMap>) {
    dbg!(file_map
        .blocks
        .iter()
        .filter_map(|x| *x)
        .enumerate()
        .fold(0, |acc, (i, block)| acc + i as u64 * block));
}

const fn solve_b() {}

const fn vis(
    _asset_server: Res<AssetServer>,
    _commands: Commands,
    _day9: Single<Entity, With<Day9>>,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() -> miette::Result<()> {
        let input: Vec<char> = "2333133121414131402".chars().collect();
        let mut file_blocks = get_file_blocks(&input)?;
        compact_blocks(&mut file_blocks)?;
        let expected = 1928;
        let actual = file_blocks
            .iter()
            .filter_map(|x| *x)
            .enumerate()
            .fold(0, |acc, (i, block)| acc + i as u64 * block);
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_b() -> miette::Result<()> {
        let input: Vec<char> = "2333133121414131402".chars().collect();
        Ok(())
    }
}
