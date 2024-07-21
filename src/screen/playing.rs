//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::game::{
    assets::{HandleMap, ImageKey},
    spawn::level::SpawnLevel,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>) {
    commands.spawn((
        SpriteBundle {
            texture: image_handles[&ImageKey::BackDrop].clone_weak(),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
    commands.trigger(SpawnLevel);
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
