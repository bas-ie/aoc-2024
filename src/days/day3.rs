use regex::Regex;

use bevy::prelude::*;
use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::{recognize, rest},
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{loading::PuzzleInputs, puzzle_input_lines_asset::PuzzleInputLinesAsset, AoCState};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ParsedStatements>().add_systems(
        OnEnter(AoCState::Day3),
        (init, process2, solve_a, solve_b).chain(),
    );
}

#[derive(Component)]
struct Day3;

#[derive(Resource, Default)]
struct ParsedStatements {
    mul: Vec<(i32, i32)>,
}

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day3"),
            Day3,
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
            StateScoped(AoCState::Day3),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Day 3: Mull It Over"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
        });
}

// fn process(
//     mut parsed_statements: ResMut<ParsedStatements>,
//     puzzle_assets: Res<Assets<PuzzleInputLinesAsset>>,
//     puzzle_inputs: Res<PuzzleInputs>,
// ) {
//     let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.three) else {
//         return;
//     };
//     let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
//     for line in &puzzle.rows {
//         let Ok((_remaining, parsed)) = many0(mul)(line) else {
//             info!("Failed to parse input line.");
//             return;
//         };
//
//         parsed_statements.mul.append(
//             &mut parsed
//                 .iter()
//                 .filter(|x| **x != "mul(")
//                 .filter_map(|x| {
//                     if let Some(caps) = re.captures(x) {
//                         let lhs: i32 = caps[1].parse().unwrap();
//                         let rhs: i32 = caps[2].parse().unwrap();
//                         Some((lhs, rhs))
//                     } else {
//                         None
//                     }
//                 })
//                 .collect::<Vec<(i32, i32)>>(),
//         );
//     }
// }
//
// &str
// "and struh"
// "stir"
// "string-ref"
// "and stir"
// "ref stir"
// french: "éh struh"
// STIR
fn process_line(line: &str) -> miette::Result<Vec<(i32, i32)>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut dos: Vec<&str> = vec![];
    let (remainder, parsed) =
        many0(dont)(line).map_err(|e| miette!("don't parsing fail: {}", e))?;
    dbg!(&parsed[0]);

    // The beginning of the string is considered a do() block because this is the default.
    // After this, we must check to see if mul() statements are "re-enabled" with a do()
    // statement.
    dos.push(parsed[0]);

    for chunk in &parsed[1..] {
        let Ok((r, _)) = do_(chunk) else {
            continue;
        };
        dos.push(r);
    }
    if let Ok((r, _)) = do_(remainder) {
        dos.push(r);
    }

    // At this point, we should have all statements following `do()` AND the initial statements
    // before the first `don't()`.

    let mut parsed_statements: Vec<(i32, i32)> = vec![];
    for chunk in &dos {
        let (_, p) = many0(mul)(chunk).map_err(|e| miette!("do parsing fail: {}", e))?;

        parsed_statements.append(
            &mut p
                .iter()
                .filter(|x| **x != "mul(")
                .filter_map(|x| {
                    re.captures(x).map(|caps| {
                        let lhs: i32 = caps[1].parse().unwrap();
                        let rhs: i32 = caps[2].parse().unwrap();
                        (lhs, rhs)
                    })
                })
                .collect::<Vec<(i32, i32)>>(),
        );
    }

    Ok(parsed_statements)
}

fn process2(
    mut parsed_statements: ResMut<ParsedStatements>,
    puzzle_assets: Res<Assets<PuzzleInputLinesAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.three) else {
        return;
    };
    let Ok(mut statements) = process_line(&puzzle.rows.concat()) else {
        dbg!("EARLY EXIT");
        return;
    };
    parsed_statements.mul.append(&mut statements);
}

fn dont(input: &str) -> IResult<&str, &str> {
    let (r, p) = take_until("don't()")(input)?;
    let (r, _) = alt((tag("don't()"), rest))(r)?;
    Ok((r, p))
}

fn do_(input: &str) -> IResult<&str, &str> {
    take_until("do()")(input)
}

fn mul(input: &str) -> IResult<&str, &str> {
    let (remainder, _) = take_until("mul(")(input)?;
    alt((
        recognize(tuple((tag("mul("), digit1, char(','), digit1, char(')')))),
        // If we get a malformed mul, consume the start of the pattern so we don't try to match it
        // again. Is there a better way to accomplish this? And can we avoid it ended up in the
        // parsed input?
        tag("mul("),
    ))(remainder)
}

fn solve_a(parsed_statements: Res<ParsedStatements>) {
    let mut total = 0;
    for (lhs, rhs) in &parsed_statements.mul {
        total += lhs * rhs;
    }
    dbg!(total);
}

fn solve_b(parsed_statements: Res<ParsedStatements>) {
    let mut total = 0;
    for (lhs, rhs) in &parsed_statements.mul {
        total += lhs * rhs;
    }
    dbg!(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = process_line(input)?
            .iter()
            .fold(0, |acc, (a, b)| acc + a * b);
        assert_eq!(expected, 48);
        Ok(())
    }
}
