//! The screen state for the main game loop.

use bevy::{
    input::{common_conditions::input_just_pressed, mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use log::info;

use super::{title::MenuFloating, Screen};
use crate::game::{
    assets::{HandleMap, ImageKey},
    spawn::{journey::Continue, level::SpawnLevel},
    ui::{ContinueButton, FocusedDisplay},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        (
            return_to_title_screen
                .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
            carry_on.run_if(in_state(Screen::Playing).and_then(
                input_just_pressed(KeyCode::Space).or_else(input_just_pressed(KeyCode::Enter)),
            )),
            // check_for_click.run_if(in_state(Screen::Playing)),
        ),
    );
}

#[derive(Component)]
pub struct BackDrop;

fn check_for_click(
    mut mb_events: EventReader<MouseButtonInput>,
    query: Query<&Visibility, With<ContinueButton>>,
    display: Res<FocusedDisplay>,
    mut commands: Commands,
) {
    if *display != FocusedDisplay::Dialogue {
        return;
    }
    for event in mb_events.read() {
        if (event.button == MouseButton::Left) && event.state == ButtonState::Pressed {
            let vis = query.single();
            info!("AAh {vis:?}");
            if vis == Visibility::Inherited || Visibility::Visible == vis {
                commands.trigger(Continue);
            }
        }
    }
}

fn enter_playing(mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>) {
    commands
        .spawn((
            SpriteBundle {
                texture: image_handles[&ImageKey::BackDrop].clone_weak(),
                ..default()
            },
            StateScoped(Screen::Playing),
        ))
        .insert(BackDrop);
    commands.spawn((
        SpriteBundle {
            texture: image_handles[&ImageKey::Ship].clone_weak(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        MenuFloating {
            width: 200.0,
            height_max: 10.0,
            height_min: -20.0,
            speed: 1.0,
            current_x: -100.0,
            direction: false,
            mid_x: 37.0,
        },
        StateScoped(Screen::Playing),
    ));
    commands.trigger(SpawnLevel);
}

fn exit_playing(_commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn carry_on(query: Query<&Visibility, With<ContinueButton>>, mut commands: Commands) {
    if query.single() == Visibility::Visible {
        commands.trigger(Continue);
    }
}
