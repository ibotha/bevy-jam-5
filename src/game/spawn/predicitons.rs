use bevy::prelude::*;

use super::{journey::Journey, weather::AnyWeather};
use crate::game::ui::PredictionAction;
use crate::game::{
    assets::{FontKey, HandleMap, ImageKey},
    ui::{DarkMagicBox, GameAction, ParrotBox, SpyGlassBox},
};

#[derive(Resource, Debug, PartialEq, Default)]
pub struct Predictions {
    pub dark_magic: Option<AnyWeather>,
    pub parrot: Option<AnyWeather>,
    pub spy_glass: Option<AnyWeather>,
}
use crate::ui::palette::LABEL_TEXT;

pub fn plugin(app: &mut App) {
    app.init_resource::<Predictions>() // Initiallizes by the defaults
        .observe(dark_magic_predictor)
        .observe(parrot_predictor)
        .observe(spyglass_predictor);
}

#[derive(Event, Debug, PartialEq, Default)]
pub struct UpdateDarkMagicUi;

pub fn dark_magic_predictor(
    _: Trigger<UpdateDarkMagicUi>,
    predictions: Res<Predictions>,
    mut commands: Commands,
    query: Query<Entity, With<DarkMagicBox>>,
    image_handles: Res<HandleMap<ImageKey>>,
    fonts: Res<HandleMap<FontKey>>,
) {
    commands
        .entity(query.single())
        .despawn_descendants()
        .with_children(|commands| match predictions.dark_magic {
            Some(weather_prediction) => match weather_prediction {
                AnyWeather::Heat(predicted_heat) => {
                    let predicted_text = format!("Heat: {}", predicted_heat);
                    predicted_selection(commands, &fonts, predicted_text);
                }
                AnyWeather::Moisture(predicted_moisture) => {
                    let predicted_text = format!("Moisture: {}", predicted_moisture);
                    predicted_selection(commands, &fonts, predicted_text);
                }
                AnyWeather::Wind(predicted_wind) => {
                    let predicted_text = format!("Wind: {}", predicted_wind);
                    predicted_selection(commands, &fonts, predicted_text);
                }
            },
            None => {
                let actions = (
                    GameAction::DarkMagicPredictionAction(PredictionAction::Heat),
                    GameAction::DarkMagicPredictionAction(PredictionAction::Moisture),
                    GameAction::DarkMagicPredictionAction(PredictionAction::Wind),
                );
                prediction_buttons(commands, &image_handles, actions);
            }
        });
}

#[derive(Event, Debug, PartialEq, Default)]
pub struct UpdateParrotUi;

pub fn parrot_predictor(
    _: Trigger<UpdateParrotUi>,
    mut commands: Commands,
    query: Query<Entity, With<ParrotBox>>,
    fonts: Res<HandleMap<FontKey>>,
    journey: Res<Journey>,
) {
    commands
        .entity(query.single())
        .despawn_descendants()
        .with_children(|commands| {
            let hint = journey
                .event
                .as_ref()
                .and_then(|event| event.hint_string.as_ref())
                .map(|hint| hint.to_string())
                .unwrap_or_else(|| "No hint available".to_string());

            predicted_selection(commands, &fonts, hint);
        });
}

#[derive(Event, Debug, PartialEq, Default)]
pub struct UpdateSpyGlassUi;

pub fn spyglass_predictor(
    _: Trigger<UpdateSpyGlassUi>,
    predictions: Res<Predictions>,
    mut commands: Commands,
    query: Query<Entity, With<SpyGlassBox>>,
    image_handles: Res<HandleMap<ImageKey>>,
    fonts: Res<HandleMap<FontKey>>,
) {
    commands
        .entity(query.single())
        .despawn_descendants()
        .with_children(|commands| match predictions.spy_glass {
            Some(weather_prediction) => match weather_prediction {
                AnyWeather::Heat(predicted_heat) => {
                    let predicted_text = format!("Heat: {}", predicted_heat);
                    predicted_selection(commands, &fonts, predicted_text);
                }
                AnyWeather::Moisture(predicted_moisture) => {
                    let predicted_text = format!("Moisture: {}", predicted_moisture);
                    predicted_selection(commands, &fonts, predicted_text);
                }
                AnyWeather::Wind(predicted_wind) => {
                    let predicted_text = format!("Wind: {}", predicted_wind);
                    predicted_selection(commands, &fonts, predicted_text);
                }
            },
            None => {
                let actions = (
                    GameAction::SpyGlassPredictionAction(PredictionAction::Heat),
                    GameAction::SpyGlassPredictionAction(PredictionAction::Moisture),
                    GameAction::SpyGlassPredictionAction(PredictionAction::Wind),
                );
                prediction_buttons(commands, &image_handles, actions);
            }
        });
}

pub fn prediction_buttons(
    commands: &mut ChildBuilder,
    image_handles: &Res<HandleMap<ImageKey>>,
    actions: (GameAction, GameAction, GameAction),
) {
    // Create a parent container for the buttons
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Button 1
            parent
                .spawn(ButtonBundle {
                    image: UiImage::new(image_handles[&ImageKey::HeatButton].clone_weak()),
                    style: Style {
                        height: Val::Px(35.0),
                        aspect_ratio: Some(1.0),
                        margin: UiRect::horizontal(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(actions.0);

            // Button 2
            parent
                .spawn(ButtonBundle {
                    image: UiImage::new(image_handles[&ImageKey::MoistureButton].clone_weak()),
                    style: Style {
                        height: Val::Px(35.0),
                        aspect_ratio: Some(1.0),
                        margin: UiRect::horizontal(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(actions.1);

            // Button 3
            parent
                .spawn(ButtonBundle {
                    image: UiImage::new(image_handles[&ImageKey::WindButton].clone_weak()),
                    style: Style {
                        height: Val::Px(35.0),
                        aspect_ratio: Some(1.0),
                        margin: UiRect::horizontal(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(actions.2);
        });
}

pub fn predicted_selection(
    commands: &mut ChildBuilder,
    fonts: &Res<HandleMap<FontKey>>,
    predicted_text: String,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    predicted_text,
                    TextStyle {
                        font_size: 10.0,
                        color: LABEL_TEXT,
                        font: fonts[&FontKey::LunchDS].clone_weak(),
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Auto), // This helps with centering
                    ..default()
                },
                ..default()
            });
        });
}
