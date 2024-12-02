use bevy::prelude::*;

mod day1;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((day1::plugin));
}
