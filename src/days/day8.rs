#![allow(dead_code, unused)]

use crate::{loading::PuzzleInputs, puzzle_input_string_asset::PuzzleInputStringAsset, AoCState};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use miette::miette;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CityMap>().add_systems(
        OnEnter(AoCState::Day8),
        (init, process, solve_a, solve_b, vis).chain(),
    );
}

#[derive(Component)]
struct Day8;

#[derive(Debug, Clone, Default, Resource)]
struct CityMap {
    pub antennae: HashMap<char, Vec<IVec2>>,
    pub antinodes: Vec<IVec2>,
}

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day8"),
            Day8,
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
            StateScoped(AoCState::Day8),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Day 8: Resonant Collinearity"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
        });
}

fn process(
    mut city_map: ResMut<CityMap>,
    puzzle_assets: Res<Assets<PuzzleInputStringAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.eight) else {
        return;
    };
    let input: Vec<&str> = puzzle.0.lines().collect();
    city_map.antennae = find_antennae(&input);
    city_map.antinodes = find_antinodes(
        &city_map.antennae,
        &IRect {
            min: IVec2::ZERO,
            max: IVec2::new((input[0].len() - 1) as i32, (input.len() - 1) as i32),
        },
    );
}

fn find_antennae(map: &[&str]) -> HashMap<char, Vec<IVec2>> {
    let mut antennae: HashMap<char, Vec<IVec2>> = [].into();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antennae
                .entry(c)
                .or_default()
                .push(IVec2::new(x as i32, y as i32));
        }
    }
    antennae
}

fn find_antinodes(antennae: &HashMap<char, Vec<IVec2>>, bounds: &IRect) -> Vec<IVec2> {
    let mut antinodes: HashSet<IVec2> = [].into();
    for key in antennae.keys() {
        if let Some(val) = antennae.get(key) {
            if val.len() < 2 {
                continue;
            }
            for a in val {
                for b in val {
                    if a == b {
                        continue;
                    }
                    let distance = b - a;
                    let antinode_a = a - distance;
                    let antinode_b = b + distance;
                    if bounds.contains(antinode_a) {
                        antinodes.insert(antinode_a);
                    }
                    if bounds.contains(antinode_b) {
                        antinodes.insert(antinode_b);
                    }
                }
            }
        }
    }
    antinodes.into_iter().collect()
}

fn find_antinodes2(antennae: &HashMap<char, Vec<IVec2>>, bounds: &IRect) -> Vec<IVec2> {
    let mut antinodes: HashSet<IVec2> = [].into();
    for key in antennae.keys() {
        if let Some(val) = antennae.get(key) {
            if val.len() < 2 {
                continue;
            }
            for a in val {
                for b in val {
                    if a == b {
                        continue;
                    }
                    let distance = b - a;
                    let mut antinode_a = a - distance;
                    antinodes.insert(*a);
                    antinodes.insert(*b);
                    while bounds.contains(antinode_a) {
                        antinodes.insert(antinode_a);
                        antinode_a -= distance;
                    }
                    let mut antinode_b = b + distance;
                    while bounds.contains(antinode_b) {
                        antinodes.insert(antinode_b);
                        antinode_b -= distance;
                    }
                }
            }
        }
    }
    antinodes.into_iter().collect()
}

fn solve_a(city_map: Res<CityMap>) {
    dbg!(city_map.antinodes.len());
}

fn solve_b(city_map: Res<CityMap>) {
    dbg!(find_antinodes2(
        &city_map.antennae,
        &IRect {
            min: IVec2::ZERO,
            max: IVec2::new(49, 49),
        },
    )
    .len());
}

const fn vis(
    _asset_server: Res<AssetServer>,
    _commands: Commands,
    _day8: Single<Entity, With<Day8>>,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() -> miette::Result<()> {
        let input: Vec<&str> = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .lines()
            .collect();
        let expected = 14;
        let a = find_antennae(&input);
        let ant = find_antinodes(
            &a,
            &IRect {
                min: IVec2::ZERO,
                max: IVec2::new(11, 11),
            },
        );
        assert_eq!(expected, ant.len());
        Ok(())
    }

    #[test]
    fn test_b() -> miette::Result<()> {
        let input: Vec<&str> = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .lines()
            .collect();
        let expected = 34;
        let a = find_antennae(&input);
        let ant = find_antinodes2(
            &a,
            &IRect {
                min: IVec2::ZERO,
                max: IVec2::new(11, 11),
            },
        );
        assert_eq!(expected, ant.len());
        Ok(())
    }
}
