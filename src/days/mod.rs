use bevy::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        day1::plugin,
        day2::plugin,
        day3::plugin,
        day4::plugin,
        day5::plugin,
        day6::plugin,
        day7::plugin,
    ));
}
