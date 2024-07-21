//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::{
        assets::{FontKey, HandleMap, ImageKey, SoundtrackKey},
        audio::soundtrack::PlaySoundtrack,
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    WeatherManiac,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    fonts: Res<HandleMap<FontKey>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: image_handles[&ImageKey::BackDrop].clone_weak(),
            ..default()
        },
        StateScoped(Screen::Title),
    ));
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children
                .button(
                    "Play",
                    image_handles[&ImageKey::Button].clone_weak(),
                    fonts[&FontKey::PaperCut].clone_weak(),
                )
                .insert(TitleAction::Play);
            children
                .button(
                    "Credits",
                    image_handles[&ImageKey::Button].clone_weak(),
                    fonts[&FontKey::PaperCut].clone_weak(),
                )
                .insert(TitleAction::Credits);
            children
                .button(
                    "Weather Maniac",
                    image_handles[&ImageKey::Button].clone_weak(),
                    fonts[&FontKey::PaperCut].clone_weak(),
                )
                .insert(TitleAction::WeatherManiac);

            #[cfg(not(target_family = "wasm"))]
            children
                .button(
                    "Exit",
                    image_handles[&ImageKey::Button].clone_weak(),
                    fonts[&FontKey::PaperCut].clone_weak(),
                )
                .insert(TitleAction::Exit);
        });
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Playing),
                TitleAction::Credits => next_screen.set(Screen::Credits),
                TitleAction::WeatherManiac => next_screen.set(Screen::WeatherManiac),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
