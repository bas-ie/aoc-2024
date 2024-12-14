use std::str::FromStr;

use crate::{loading::PuzzleInputs, puzzle_input_lines_asset::PuzzleInputLinesAsset, AoCState};
use bevy::prelude::*;
use miette::miette;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PrintQueue>().add_systems(
        OnEnter(AoCState::Day5),
        (init, process, solve_a, solve_b, vis).chain(),
    );
}

#[derive(Component)]
struct Day5;

#[derive(Resource, Debug, Default)]
struct PrintQueue {
    pub rules: Vec<Rule>,
    pub updates: Vec<Vec<i32>>,
}

fn init(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Day5"),
            Day5,
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
            StateScoped(AoCState::Day5),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Day 5: Print Queue"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
        });
}

#[derive(Copy, Clone, Default, Debug)]
struct Rule {
    pub before: i32,
    pub after: i32,
}

impl FromStr for Rule {
    type Err = miette::Error;

    fn from_str(s: &str) -> miette::Result<Self> {
        let (before, after) = s
            .split_once('|')
            .ok_or_else(|| miette!("missing field on rule"))?;
        Ok(Self {
            before: before
                .parse()
                .map_err(|e| miette!("rule parse i32 failed: {}", e))?,
            after: after
                .parse()
                .map_err(|e| miette!("rule parse i32 failed: {}", e))?,
        })
    }
}

fn get_rules(input: &[&str]) -> miette::Result<Vec<Rule>> {
    input.iter().map(|&row| row.parse()).collect()
}

fn process(
    mut queue: ResMut<PrintQueue>,
    puzzle_assets: Res<Assets<PuzzleInputLinesAsset>>,
    puzzle_inputs: Res<PuzzleInputs>,
) {
    let Some(puzzle) = puzzle_assets.get(&puzzle_inputs.five) else {
        return;
    };

    let Some(divider) = puzzle.rows.iter().position(|x| x.is_empty()) else {
        return;
    };

    let (rule_section, update_section) = puzzle.rows.split_at(divider);

    let r: Vec<&str> = rule_section.iter().map(String::as_ref).collect();
    queue.rules = get_rules(&r).unwrap();
    queue.updates = update_section[1..]
        .iter()
        .map(|l| l.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();
}

fn update_is_valid(rules: &[Rule], update: &[i32]) -> bool {
    for rule in rules {
        let Some(a) = update.iter().position(|x| *x == rule.before) else {
            continue;
        };
        let Some(b) = update.iter().position(|x| *x == rule.after) else {
            continue;
        };
        if b < a {
            return false;
        }
    }

    true
}

fn fix_update(rules: &[Rule], update: &[i32]) -> miette::Result<Vec<i32>> {
    let mut fixed: Vec<i32> = update.to_vec();
    let mut idx = 0;
    while idx < rules.len() {
        let Some(a) = fixed.iter().position(|x| *x == rules[idx].before) else {
            idx += 1;
            continue;
        };
        let Some(b) = fixed.iter().position(|x| *x == rules[idx].after) else {
            idx += 1;
            continue;
        };
        if b < a {
            fixed.insert(a + 1, fixed[b]);
            fixed.remove(b);
            idx = 0;
            continue;
        }
        idx += 1;
    }
    Ok(fixed)
}

fn solve_a(queue: Res<PrintQueue>) {
    let mut total = 0;
    for update in &queue.updates {
        if update_is_valid(&queue.rules, update) {
            total += update[(update.len() as f32 / 2.).floor() as usize];
        }
    }
    dbg!(total);
}

fn solve_b(queue: Res<PrintQueue>) {
    let mut total = 0;
    let mut fixed_updates: Vec<Vec<i32>> = vec![];
    for update in &queue.updates {
        if !update_is_valid(&queue.rules, update) {
            let fixed = fix_update(&queue.rules, update).unwrap();
            fixed_updates.push(fixed.clone());
            total += fixed[(fixed.len() as f32 / 2.).floor() as usize];
        }
    }
    for update in fixed_updates {
        if !update_is_valid(&queue.rules, &update) {
            info!("INVALID: {:?}", update);
        }
    }
    dbg!(total);
}

const fn vis(
    _asset_server: Res<AssetServer>,
    _commands: Commands,
    _day5: Single<Entity, With<Day5>>,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let r = get_rules(&rules.lines().collect::<Vec<&str>>())?;
        let u: Vec<Vec<i32>> = updates
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|n| {
                        n.parse::<i32>()
                            .map_err(|e| miette!("i32 parsing failed: {}", e))
                    })
                    .collect()
            })
            .collect::<miette::Result<_>>()?;
        let expected = 143;
        let mut actual = 0;
        for update in u {
            if update_is_valid(&r, &update) {
                actual += update[(update.len() as f32 / 2.).floor() as usize];
            }
        }
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_b() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let r = get_rules(&rules.lines().collect::<Vec<&str>>())?;
        let u: Vec<Vec<i32>> = updates
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|n| {
                        n.parse::<i32>()
                            .map_err(|e| miette!("i32 parsing failed: {}", e))
                    })
                    .collect()
            })
            .collect::<miette::Result<_>>()?;

        let mut fixed: Vec<Vec<i32>> = vec![];
        for update in u {
            if !update_is_valid(&r, &update) {
                let f = fix_update(&r, &update)?;
                fixed.push(f);
            }
        }
        let mut actual = 0;
        for update in fixed.iter() {
            dbg!(&update);
            actual += update[(update.len() as f32 / 2.).floor() as usize];
        }

        let expected = 123;
        assert_eq!(expected, actual);
        Ok(())
    }
}
