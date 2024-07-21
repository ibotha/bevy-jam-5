use bevy::prelude::*;

use crate::screen::Screen;
use crate::ui::prelude::*;

use super::assets::{FontKey, HandleMap, ImageKey};

#[derive(Event, Debug)]
pub struct SpawnGameUI;

pub fn plugin(app: &mut App) {
    app.observe(spawn_game_ui);

    app.register_type::<GameAction>();
    app.add_systems(Update, handle_game_action.run_if(in_state(Screen::Playing)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum GameAction {
    Bones,
    Choose,
    Menu,
}

fn spawn_game_ui(
    _trigger: Trigger<SpawnGameUI>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    fonts: Res<HandleMap<FontKey>>,
) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Playing))
        .with_children(|children| {
            children
                .button(
                    "",
                    image_handles[&ImageKey::BoneButton].clone_weak(),
                    fonts[&FontKey::PaperCut].clone_weak(),
                )
                .insert(GameAction::Bones);
            children
                .button(
                    "Sail for the Day",
                    image_handles[&ImageKey::Button].clone_weak(),
                    fonts[&FontKey::PaperCut].clone_weak(),
                )
                .insert(GameAction::Choose);
            children
                .button(
                    "Menu",
                    image_handles[&ImageKey::Button].clone_weak(),
                    fonts[&FontKey::PaperCut].clone_weak(),
                )
                .insert(GameAction::Menu);

            // Left panel
            children.spawn((
                Name::new("Left Panel"),
                NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|panel| {
                panel.spawn(ImageBundle {
                    image: UiImage::new(image_handles[&ImageKey::LeftPanel].clone_weak()),
                    ..default()
                });
            });

            // Bottom panel
            children.spawn((
                Name::new("Bottom Panel"),
                NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        position_type: PositionType::Absolute,
                        left: Val::Auto,
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|panel| {
                panel.spawn(ImageBundle {
                    image: UiImage::new(image_handles[&ImageKey::BottomPanel].clone_weak()),
                    ..default()
                });
            });
        });
}

fn handle_game_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&GameAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                GameAction::Menu => next_screen.set(Screen::Title),
                GameAction::Bones => todo!(),
                GameAction::Choose => todo!(),
            }
        }
    }
}
