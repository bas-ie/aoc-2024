use bevy::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((day1::plugin, day2::plugin, day3::plugin, day4::plugin));
}
