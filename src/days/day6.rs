#![allow(dead_code)]

use crate::{loading::PuzzleInputs, puzzle_input_string_asset::PuzzleInputStringAsset, AoCState};
use bevy::{prelude::*, utils::HashMap};
use miette::miette;
// use chumsky::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AreaMap>().add_systems(
        OnEnter(AoCState::Day6),
        (init, process, solve_a, solve_b, vis).chain(),
    );
}

#[derive(Component)]
struct Day6;

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
enum GuardFacing {
    #[default]
    North,
    South,
    East,
    West,
}

#[derive(Resource, Default, Debug)]
struct AreaMap {
    pub obstacles: HashMap<IVec2, bool>,
}

#[derive(Component, Clone, Default, Debug)]
struct Guard {
    pub facing: GuardFacing,
    pub pos: IVec2,
    pub visited: Vec<IVec2>,
}

impl Guard {
    fn advance(&mut self, obstacles: &HashMap<IVec2, bool>) {
        let next_pos = match self.facing {
            GuardFacing::East => IVec2 {
                x: self.pos.x + 1,
                y: self.pos.y,
            },
            GuardFacing::South => IVec2 {
                x: self.pos.x,
                y: self.pos.y + 1,
            },
            GuardFacing::West => IVec2 {
                x: self.pos.x - 1,
                y: self.pos.y,
            },
            GuardFacing::North => IVec2 {
                x: self.pos.x,
                y: self.pos.y - 1,
            },
        };
        if obstacles.contains_key(&next_pos) {
            self.turn_right();
            return;
        }

        if !self.visited.contains(&self.pos) {
            self.visited.push(self.pos);
        }

        match self.facing {
            GuardFacing::East => self.pos.x += 1,
            GuardFacing::South => self.pos.y += 1,
            GuardFacing::West => self.pos.x -= 1,
            GuardFacing::North => self.pos.y -= 1,
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            GuardFacing::East => GuardFacing::South,
            GuardFacing::South => GuardFacing::West,
            GuardFacing::West => GuardFacing::North,
            GuardFacing::North => GuardFacing::East,
        }
    }
}

// Every character is EITHER:
//   - empty space ('.')
//   - an obstacle ('#')
//   - a guard ('^')
// Crucially, we also need to know row and column for each character.
#[derive(Debug)]
enum Position {
    Empty,
    Obstacle,
    Guard,
}

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day6"),
            Day6,
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
            StateScoped(AoCState::Day6),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Day 6: Guard Gallivant"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
        });
}

fn find_guard(input: &[&str]) -> miette::Result<Guard> {
    for (y, row) in input.iter().enumerate() {
        if let Some(x) = row.find("^") {
            return Ok(Guard {
                facing: GuardFacing::North,
                pos: IVec2::new(x as i32, y as i32),
                ..default()
            });
        }
    }

    Err(miette!("NO GUARD!"))
}

fn find_obstacles(input: &[&str]) -> miette::Result<HashMap<IVec2, bool>> {
    let mut obstacles: HashMap<IVec2, bool> = [].into();
    for (y, row) in input.iter().enumerate() {
        for (x, _) in row.match_indices("#") {
            obstacles.insert((x as i32, y as i32).into(), true);
        }
    }

    Ok(obstacles)
}

fn process(
    mut area_map: ResMut<AreaMap>,
    mut commands: Commands,
    puzzle_assets: Res<Assets<PuzzleInputStringAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.six) else {
        return;
    };

    let input: Vec<&str> = puzzle.0.lines().collect();
    commands.spawn(find_guard(&input).unwrap());
    area_map.obstacles = find_obstacles(&input).unwrap();
}

fn solve_a(area_map: ResMut<AreaMap>, mut guard: Single<&mut Guard>) {
    let bounds = IRect {
        min: IVec2::ZERO,
        max: IVec2::new(129, 129),
    };
    while bounds.contains(guard.pos) {
        guard.advance(&area_map.obstacles);
    }
    dbg!(&guard.visited.len());
}

fn solve_b(area_map: ResMut<AreaMap>, guard: Single<&Guard>) {
    let circuit_testing_obstacles = area_map.obstacles.clone();
    let circuit_testing_guard = Guard {
        pos: IVec2::new(80, 58),
        facing: GuardFacing::North,
        ..default()
    };

    let bounds = IRect {
        min: IVec2::ZERO,
        max: IVec2::new(129, 129),
    };
    let mut loops = 0;
    for candidate in guard.visited.iter() {
        let mut test_guard = circuit_testing_guard.clone();
        let mut test_obstacles = circuit_testing_obstacles.clone();
        test_obstacles.insert(*candidate, true);
        let mut iterations: i64 = 0;
        while bounds.contains(test_guard.pos) {
            test_guard.advance(&test_obstacles);
            iterations += 1;
            if iterations > 5000 {
                loops += 1;
                break;
            }
        }
    }
    dbg!(loops);
}

const fn vis(
    _asset_server: Res<AssetServer>,
    _commands: Commands,
    _day6: Single<Entity, With<Day6>>,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() -> miette::Result<()> {
        let input: Vec<&str> = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .lines()
            .collect();
        let mut guard = find_guard(&input)?;
        let obstacles = find_obstacles(&input)?;
        let bounds = IRect {
            min: IVec2::ZERO,
            max: IVec2::new(9, 9),
        };
        while bounds.contains(guard.pos) {
            guard.advance(&obstacles);
        }
        let expected = 41;
        assert_eq!(expected, guard.visited.len());
        Ok(())
    }

    #[test]
    fn test_b() -> miette::Result<()> {
        let input: Vec<&str> = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .lines()
            .collect();
        let mut guard = find_guard(&input)?;
        let circuit_testing_guard = guard.clone();

        let obstacles = find_obstacles(&input)?;
        let circuit_testing_obstacles = obstacles.clone();
        let bounds = IRect {
            min: IVec2::ZERO,
            max: IVec2::new(9, 9),
        };
        while bounds.contains(guard.pos) {
            guard.advance(&obstacles);
        }

        let mut actual = 0;
        for candidate in guard.visited[1..].iter() {
            let mut test_guard = circuit_testing_guard.clone();
            let mut test_obstacles = circuit_testing_obstacles.clone();
            test_obstacles.insert(*candidate, true);
            let mut iterations = 0;
            while bounds.contains(test_guard.pos) {
                test_guard.advance(&test_obstacles);
                iterations += 1;
                if iterations > 100 {
                    actual += 1;
                    break;
                }
            }
        }
        let expected = 6;
        assert_eq!(expected, actual);
        Ok(())
    }
}
