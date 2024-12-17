#![allow(dead_code, unused)]

use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::{loading::PuzzleInputs, puzzle_input_string_asset::PuzzleInputStringAsset, AoCState};
use bevy::prelude::*;
use chumsky::prelude::*;
use itertools::Itertools;
use miette::miette;
use text::newline;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Equations>().add_systems(
        OnEnter(AoCState::Day7),
        (init, process, solve_a, solve_b, vis).chain(),
    );
}

#[derive(Component)]
struct Day7;

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day7"),
            Day7,
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
            StateScoped(AoCState::Day7),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Day 7: Bridge Repair"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
        });
}

#[derive(Clone, Copy, Debug)]
enum Equation {
    Number(i64),
    TestValue(i64),
}

impl Add for Equation {
    type Output = Equation;

    fn add(self, other: Equation) -> Equation {
        match (self, other) {
            (Equation::Number(lhs), Equation::Number(rhs)) => Equation::Number(lhs + rhs),
            _ => panic!("can't handle mismatched Equation variants"),
        }
    }
}

impl AddAssign for Equation {
    fn add_assign(&mut self, other: Equation) {
        *self = match (&*self, other) {
            (Equation::Number(lhs), Equation::Number(rhs)) => Equation::Number(lhs + rhs),
            _ => panic!("can't handle mismatched Equation variants"),
        };
    }
}

impl Mul for Equation {
    type Output = Equation;

    fn mul(self, other: Equation) -> Equation {
        match (self, other) {
            (Equation::Number(lhs), Equation::Number(rhs)) => Equation::Number(lhs * rhs),
            _ => panic!("can't handle mismatched Equation variants"),
        }
    }
}

impl MulAssign for Equation {
    fn mul_assign(&mut self, other: Equation) {
        *self = match (&*self, other) {
            (Equation::Number(lhs), Equation::Number(rhs)) => Equation::Number(lhs * rhs),
            _ => panic!("can't handle mismatched Equation variants"),
        };
    }
}

#[derive(Default, Resource)]
struct Equations {
    candidates: Vec<Vec<Equation>>,
}

fn test_value() -> impl Parser<char, Equation, Error = Simple<char>> {
    text::int(10)
        .map(|s: String| Equation::TestValue(s.parse().unwrap()))
        .then_ignore(just(':'))
        .padded()
}

fn number() -> impl Parser<char, Equation, Error = Simple<char>> {
    text::int(10).map(|s: String| Equation::Number(s.parse().unwrap()))
}

fn equation() -> impl Parser<char, Vec<Equation>, Error = Simple<char>> {
    test_value()
        .then(number().separated_by(just(' ')))
        .map(|(t, n)| {
            let mut eq = vec![t];
            eq.extend(n);
            eq
        })
}

fn parser() -> impl Parser<char, Vec<Vec<Equation>>, Error = Simple<char>> {
    equation().separated_by(newline()).collect()
}

fn process(
    mut equations: ResMut<Equations>,
    puzzle_assets: Res<Assets<PuzzleInputStringAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.seven) else {
        return;
    };

    equations.candidates = parser().parse(puzzle.0.to_owned()).unwrap();
}

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

// NOTE: `multi_cartesian_product` e.g. Chris' solution:
// let ops = operator_count.map(|_| ['*', '+']).multi_cartesian_product();
fn all_operators(length: usize) -> Vec<Vec<Operation>> {
    let total_combinations = 2_u32.pow(length as u32);
    let mut result = Vec::with_capacity(total_combinations as usize);

    for i in 0..total_combinations {
        let mut combination = Vec::with_capacity(length);

        for bit in 0..length {
            let op = if (i & (1 << bit)) != 0 {
                Operation::Add
            } else {
                Operation::Multiply
            };
            combination.push(op);
        }

        result.push(combination);
    }

    result
}

fn eval_if_valid(eq: &[Equation]) -> miette::Result<i64> {
    if let Equation::TestValue(tv) = &eq[0] {
        let remaining_values: Vec<Equation> = eq[1..].into();
        let ops = all_operators(remaining_values.len() - 1);
        for candidate in ops {
            let mut total = 0;
            for i in 0..remaining_values.len() {
                if let Equation::Number(lhs) = remaining_values[i] {
                    if i == 0 {
                        total += lhs;
                        continue;
                    }
                    match candidate[i - 1] {
                        Operation::Add => total += lhs,
                        Operation::Multiply => total *= lhs,
                        _ => (),
                    }
                }
            }

            if total == *tv {
                return Ok(*tv);
            }
        }
    }

    Ok(0)
}

fn eval_if_valid_concat(eq: &[Equation]) -> miette::Result<i64> {
    let Equation::TestValue(tv) = &eq[0] else {
        return Err(miette!("TestValue parse fail"));
    };
    let mut values: Vec<Equation> = eq[1..].into();
    let is_valid = (0..values.len() - 1)
        .map(|_| [Operation::Add, Operation::Multiply, Operation::Concatenate])
        .multi_cartesian_product()
        .any(|op_sequence| {
            let mut op = op_sequence.iter();
            let mut val = values.iter().copied();

            let Some(Equation::Number(result)) = val.reduce(|acc, v| match op.next().unwrap() {
                Operation::Add => acc + v,
                Operation::Multiply => acc * v,
                Operation::Concatenate => {
                    let Equation::Number(lhs) = acc else {
                        panic!("concatenate failed");
                    };
                    let Equation::Number(rhs) = v else {
                        panic!("concatenate failed");
                    };
                    Equation::Number((lhs.to_string() + &rhs.to_string()).parse::<i64>().unwrap())
                }
            }) else {
                return false;
            };
            *tv == result
        });

    if is_valid {
        Ok(*tv)
    } else {
        Ok(0)
    }
}

fn solve_a(equations: Res<Equations>) {
    let mut actual = 0;
    for equation in &equations.candidates {
        actual += eval_if_valid(equation).unwrap();
    }
    dbg!(actual);
}

fn solve_b(equations: Res<Equations>) {
    let mut actual = 0;
    for equation in &equations.candidates {
        actual += eval_if_valid_concat(equation).unwrap();
    }
    dbg!(actual);
}

const fn vis(
    _asset_server: Res<AssetServer>,
    _commands: Commands,
    _day7: Single<Entity, With<Day7>>,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let equations = parser()
            .parse(input)
            .map_err(|_| miette!("puzzle input parse failed!"))?;

        let mut actual = 0;
        for equation in equations {
            actual += eval_if_valid(&equation)?;
        }

        let expected = 3749;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_b() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let equations = parser()
            .parse(input)
            .map_err(|_| miette!("puzzle input parse failed"))?;
        let mut actual = 0;
        for equation in equations {
            actual += eval_if_valid_concat(&equation)?;
        }
        let expected = 11387;
        assert_eq!(expected, actual);
        Ok(())
    }
}
