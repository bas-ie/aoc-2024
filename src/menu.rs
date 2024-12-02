use bevy::{
    color::palettes::css::{FIRE_BRICK, GREEN},
    prelude::*,
};

use crate::AoCState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AoCState::Menu), init);
}

#[derive(Component)]
struct Menu;

fn init(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));

    commands.spawn((
        Name::new("Menu"),
        Menu,
        Node {
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            justify_self: JustifySelf::Center,
            padding: UiRect::all(Val::Px(10.)),
            ..default()
        },
        StateScoped(AoCState::Menu),
    ));

    commands.run_system_cached_with(spawn_puzzle_link, AoCState::Day1);
}

fn spawn_puzzle_link(
    state: In<AoCState>,
    mut commands: Commands,
    menu: Single<Entity, With<Menu>>,
) {
    commands.entity(*menu).with_children(|p| {
        p.spawn((
            Button,
            BackgroundColor(FIRE_BRICK.into()),
            BorderColor(GREEN.into()),
            Node {
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.)),
                height: Val::Px(50.),
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Px(5.)),
                padding: UiRect::all(Val::Px(20.)),
                width: Val::Px(50.),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(Text::new("One"));
        })
        .observe(
            move |_ev: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<AoCState>>| {
                next_state.set(*state);
            },
        );
    });
}
