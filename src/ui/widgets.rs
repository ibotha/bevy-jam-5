//! Helper traits for creating common widgets.

use bevy::{
    ecs::{query, system::EntityCommands},
    log::tracing_subscriber::reload::Handle,
    prelude::*,
    ui::Val::*,
};

use crate::game::assets::{FontKey, HandleMap, ImageKey};

use super::{interaction::InteractionPalette, palette::*};

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(
        &mut self,
        text: impl Into<String>,
        image: bevy::asset::Handle<Image>,
        font: bevy::asset::Handle<Font>,
    ) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(
        &mut self,
        text: impl Into<String>,
        font: bevy::asset::Handle<Font>,
    ) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>, font: bevy::asset::Handle<Font>)
        -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(
        &mut self,
        text: impl Into<String>,
        image: bevy::asset::Handle<Image>,
        font: bevy::asset::Handle<Font>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Px(120.0),
                    height: Px(20.0),
                    padding: UiRect::new(Px(8.0), Px(20.0), Px(8.0), Px(0.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                image: UiImage {
                    texture: image,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
            InteractionPalette {
                none: Color::WHITE,
                hovered: BUTTON_HOVERED,
                pressed: BUTTON_PRESSED,
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Button Text"),
                TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font_size: 10.0,
                            color: BUTTON_TEXT,
                            font,
                        },
                    ),
                    style: Style {
                        margin: UiRect::default().with_top(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
        entity
    }

    fn header(
        &mut self,
        text: impl Into<String>,
        font: bevy::asset::Handle<Font>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Header"),
            NodeBundle {
                style: Style {
                    width: Px(500.0),
                    height: Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Header Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 20.0,
                        color: HEADER_TEXT,
                        font,
                    },
                ),
            ));
        });
        entity
    }

    fn label(
        &mut self,
        text: impl Into<String>,
        font: bevy::asset::Handle<Font>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Label"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Label Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 10.0,
                        color: LABEL_TEXT,
                        font,
                    },
                ),
            ));
        });
        entity
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
