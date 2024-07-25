/// A credits screen that can be accessed from the title screen.
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
    app.add_systems(OnEnter(Screen::Credits), enter_credits);
    app.add_systems(OnExit(Screen::Credits), exit_credits);

    app.add_systems(
        Update,
        handle_credits_action.run_if(in_state(Screen::Credits)),
    );
    app.register_type::<CreditsAction>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum CreditsAction {
    Back,
}

fn enter_credits(
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    fonts: Res<HandleMap<FontKey>>,
) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.header("Made by",
                    fonts[&FontKey::LunchDS].clone_weak(),
                );
            children.label("Julian",
                    fonts[&FontKey::LunchDS].clone_weak());
            children.label("Justin",
                    fonts[&FontKey::LunchDS].clone_weak());
            children.label("Isard",
                    fonts[&FontKey::LunchDS].clone_weak());

            children.header("Assets",
                    fonts[&FontKey::LunchDS].clone_weak());
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.",
                    fonts[&FontKey::LunchDS].clone_weak());
            children.label("Music - CC BY 3.0 by Kevin MacLeod",
                    fonts[&FontKey::LunchDS].clone_weak());

            children.label("Music - Piratical Cycle by Isard & Eidean Botha",
                    fonts[&FontKey::LunchDS].clone_weak());
            children.label("Art - Justin",
                    fonts[&FontKey::LunchDS].clone_weak());
            children.button("Back", image_handles[&ImageKey::Button].clone_weak(),
                    fonts[&FontKey::LunchDS].clone_weak()).insert(CreditsAction::Back);
        });

    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Credits));
}

fn exit_credits(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Disable);
}

fn handle_credits_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&CreditsAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                CreditsAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
