use crate::{loading::PuzzleInputs, puzzle_input_lines_asset::PuzzleInputLinesAsset, AoCState};
use bevy::prelude::*;
use miette::miette;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WordSearch>()
        .init_resource::<WordSearchGrid>()
        .add_systems(
            OnEnter(AoCState::Day4),
            (init, process, process2, solve_a, solve_b, vis).chain(),
        );
}

#[derive(Component)]
struct Day4;

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day4"),
            Day4,
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
            StateScoped(AoCState::Day4),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Day 4: Ceres Search"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
        });
}

#[derive(Resource, Default, Debug)]
struct WordSearch {
    pub diagonal: Vec<String>,
    pub horizontal: Vec<String>,
    pub vertical: Vec<String>,
}

#[derive(Copy, Clone, Debug)]
struct Character {
    pub letter: char,
    pub is_match: bool,
}

#[derive(Resource, Default, Debug)]
struct WordSearchGrid {
    pub characters: Vec<Vec<Character>>,
}

fn get_diagonals(rows: &Vec<String>) -> Vec<String> {
    let mut diagonals = vec![];
    for i in 0..rows.len() {
        let mut diag = String::new();
        for (col, row) in rows[i..].iter().enumerate() {
            let chars: Vec<char> = row.chars().collect();
            if col < row.len() {
                diag.push(chars[col]);
            }
        }
        diagonals.push(diag);
    }

    // Get the rest of the diagonals
    for i in 1..rows.len() {
        let mut diag = String::new();
        let mut col = i;
        for row in rows {
            let chars: Vec<char> = row.chars().collect();
            if col < row.len() {
                diag.push(chars[col]);
            }
            col += 1;
        }
        diagonals.push(diag);
    }

    diagonals
}

fn get_verticals(rows: &Vec<String>) -> Vec<String> {
    let mut verticals: Vec<String> = vec![];
    for (col, _) in rows.iter().enumerate() {
        let mut vert = String::new();

        for row in rows {
            let chars: Vec<char> = row.chars().collect();
            vert.push(chars[col]);
        }

        verticals.push(vert);
    }

    verticals
}

// 1. process all forwards
// 2. process all backwards
// 3. create diagonals, process backwards and forwards
// 3a. create other diagonal, process back/forward
// 4. create verticals, process backwards and forwards
fn process(
    puzzle_assets: Res<Assets<PuzzleInputLinesAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
    mut word_search: ResMut<WordSearch>,
) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.four) else {
        return;
    };

    for row in &puzzle.rows {
        word_search.horizontal.push(row.into());
    }

    word_search
        .vertical
        .append(&mut get_verticals(&puzzle.rows));

    let mut diagonals = get_diagonals(&puzzle.rows);
    let mut reversed = puzzle.rows.clone();
    reversed.reverse();
    let mut r_diags = get_diagonals(&reversed);

    word_search.diagonal.append(&mut diagonals);
    word_search.diagonal.append(&mut r_diags);
}

fn make_grid(input: &Vec<String>) -> Vec<Vec<Character>> {
    let mut grid: Vec<Vec<Character>> = vec![];

    for row in input {
        let mut r: Vec<Character> = vec![];
        for character in row.chars() {
            r.push(Character {
                letter: character,
                is_match: false,
            });
        }
        grid.push(r);
    }

    grid
}

// M S       M M      S S      S M
//  A    or   A   or   A   or   A
// M S       S S      M M      S M
fn is_x(grid: &[Vec<Character>]) -> bool {
    let lhs = [grid[0][0].letter, grid[1][1].letter, grid[2][2].letter];
    let rhs = [grid[0][2].letter, grid[1][1].letter, grid[2][0].letter];
    (lhs == ['M', 'A', 'S'] || lhs == ['S', 'A', 'M'])
        && (rhs == ['M', 'A', 'S'] || rhs == ['S', 'A', 'M'])
}

fn grid_slice(grid: &[Vec<Character>], index: (usize, usize)) -> Vec<Vec<Character>> {
    vec![
        grid[index.0][index.1..index.1 + 3].to_vec(),
        grid[index.0 + 1][index.1..index.1 + 3].to_vec(),
        grid[index.0 + 2][index.1..index.1 + 3].to_vec(),
    ]
}

fn process2(
    puzzle_assets: Res<Assets<PuzzleInputLinesAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
    mut grid: ResMut<WordSearchGrid>,
) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.four) else {
        return;
    };

    grid.characters = make_grid(&puzzle.rows);
}

fn count_xmas(input: Vec<String>) -> miette::Result<usize> {
    let mut count = 0;
    for row in input {
        count += row.match_indices("XMAS").count();
        count += row
            .chars()
            .rev()
            .collect::<String>()
            .match_indices("XMAS")
            .count();
    }
    Ok(count)
}

fn count_x(input: &mut [Vec<Character>]) -> miette::Result<usize> {
    let mut count = 0;
    for i in 0..input.len() - 2 {
        for j in 0..input[i].len() - 2 {
            let check_grid = grid_slice(input, (i, j));
            if is_x(&check_grid) {
                input[i][j].is_match = true;
                input[i][j + 2].is_match = true;
                input[i + 1][j + 1].is_match = true;
                input[i + 2][j].is_match = true;
                input[i + 2][j + 2].is_match = true;
                count += 1;
            }
        }
    }
    Ok(count)
}

fn solve_a(word_search: Res<WordSearch>) {
    let Ok(horizontal) = count_xmas(word_search.horizontal.clone()) else {
        return;
    };
    let Ok(vertical) = count_xmas(word_search.vertical.clone()) else {
        return;
    };
    let Ok(diagonal) = count_xmas(word_search.diagonal.clone()) else {
        return;
    };

    dbg!(horizontal + vertical + diagonal);
}

fn solve_b(mut grid: ResMut<WordSearchGrid>) {
    let Ok(x) = count_x(&mut grid.characters) else {
        return;
    };
    // let matched_chars = grid.characters.iter().filter(|x| x.is_match).count();

    dbg!(x);
    // dbg!(matched_chars);
}

#[derive(Component)]
struct GridNode;

fn vis(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    day4: Single<Entity, With<Day4>>,
    grid: Res<WordSearchGrid>,
) {
    for row in &grid.characters {
        let ui_row = commands
            .spawn(Node {
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                ..default()
            })
            .id();
        commands.entity(*day4).add_child(ui_row);
        for col in row {
            commands.entity(ui_row).with_children(|p| {
                // let mut background_color = BackgroundColor::default();
                // if col.is_match {
                //     background_color = BackgroundColor(FIRE_BRICK.into());
                // }
                p.spawn((
                    // background_color,
                    GridNode,
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(3.)),
                        ..default()
                    },
                    StateScoped(AoCState::Day4),
                ))
                .with_children(|p| {
                    let letter = if col.is_match { "." } else { "🎄" };
                    p.spawn((
                        Text::new(letter),
                        TextFont {
                            font: asset_server.load("NotoColorEmoji.ttf"),
                            font_size: 10.,
                            ..default()
                        },
                    ));
                });
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_all(input: Vec<&str>) -> miette::Result<usize> {
        let strings: Vec<String> = input.clone().into_iter().map(String::from).collect();
        let mut reversed = strings.clone();
        reversed.reverse();
        let verticals = get_verticals(&strings);
        let mut diagonals = get_diagonals(&strings);
        diagonals.append(&mut get_diagonals(&reversed));

        let horizontals = count_xmas(strings)?;
        let verticals = count_xmas(verticals)?;
        let diagonals = count_xmas(diagonals)?;

        Ok(horizontals + verticals + diagonals)
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        let expected = 18;
        let actual = find_all(input).map_err(|e| miette!("badness happened: {}", e))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        let mut grid = make_grid(&input.into_iter().map(String::from).collect());
        let expected = 9;
        let actual = count_x(&mut grid)?;
        let mut vis = vec![];
        for row in grid {
            let mut r = String::new();
            for col in row {
                if col.is_match {
                    r.push(col.letter);
                } else {
                    r.push('.');
                }
            }
            vis.push(r);
        }
        dbg!(vis);
        assert_eq!(expected, actual);
        Ok(())
    }
}
