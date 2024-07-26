use bevy::prelude::*;

use crate::game::{assets::{FontKey, HandleMap, ImageKey}, ui::{DarkMagicBox, GameAction, ParrotBox, SpyGlassBox}};
use super::weather::AnyWeather;
use crate::game::ui::PredictionAction;

#[derive(Resource, Debug, PartialEq, Default)]
pub struct Predictions {
    pub dark_magic: Option<AnyWeather>,
    pub parrot: Option<AnyWeather>,
    pub spy_glass: Option<AnyWeather>,
}
use crate::ui::palette::{LABEL_TEXT};

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
    commands.entity(query.single()).despawn_descendants().with_children(|commands|{
        match predictions.dark_magic {
            Some(weather_prediction) => {
                match weather_prediction {
                    AnyWeather::Heat(predicted_heat) => {
                        let predicted_text = format!("Heat: {}", predicted_heat);
                        predicted_selection(commands, &fonts, predicted_text);
                    },
                    AnyWeather::Moisture(predicted_moisture) => {
                        let predicted_text = format!("Moisture: {}", predicted_moisture);
                        predicted_selection(commands, &fonts, predicted_text);
                    },
                    AnyWeather::Wind(predicted_wind) => {
                        let predicted_text = format!("Wind: {}", predicted_wind);
                        predicted_selection(commands, &fonts, predicted_text);
                    },
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
        }
    });
}

#[derive(Event, Debug, PartialEq, Default)]
pub struct UpdateParrotUi;

pub fn parrot_predictor(
    _: Trigger<UpdateParrotUi>,
    predictions: Res<Predictions>,
    mut commands: Commands,
    query: Query<Entity, With<ParrotBox>>,
    image_handles: Res<HandleMap<ImageKey>>,
    fonts: Res<HandleMap<FontKey>>,
) {
    commands.entity(query.single()).despawn_descendants().with_children(|commands| {
        match predictions.parrot {
            Some(weather_prediction) => {
                match weather_prediction {
                    AnyWeather::Heat(predicted_heat) => {
                        // ui element showing the heat, like high, low
                        let predicted_text = format!("Heat: {}", predicted_heat);
                        predicted_selection(commands, &fonts, predicted_text);
                    },
                    AnyWeather::Moisture(predicted_moisture) => {
                        let predicted_text = format!("Moisture: {}", predicted_moisture);
                        predicted_selection(commands, &fonts, predicted_text);
                    },
                    AnyWeather::Wind(predicted_wind) => {
                        let predicted_text = format!("Wind: {}", predicted_wind);
                        predicted_selection(commands, &fonts, predicted_text);
                    },
                }
            },
            None => {
                let actions = (
                    GameAction::ParrotPredictionAction(PredictionAction::Heat),
                    GameAction::ParrotPredictionAction(PredictionAction::Moisture),
                    GameAction::ParrotPredictionAction(PredictionAction::Wind),
                );
                prediction_buttons(commands, &image_handles, actions);
            }
        }
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
    commands.entity(query.single()).despawn_descendants().with_children(|commands| {
        match predictions.spy_glass {
            Some(weather_prediction) => {
                match weather_prediction {
                    AnyWeather::Heat(predicted_heat) => {
                        let predicted_text = format!("Heat: {}", predicted_heat);
                        predicted_selection(commands, &fonts, predicted_text);
                    },
                    AnyWeather::Moisture(predicted_moisture) => {
                        let predicted_text = format!("Moisture: {}", predicted_moisture);
                        predicted_selection(commands, &fonts, predicted_text);
                    },
                    AnyWeather::Wind(predicted_wind) => {
                        let predicted_text = format!("Wind: {}", predicted_wind);
                        predicted_selection(commands, &fonts, predicted_text);
                    },
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
        }
    });
}


pub fn prediction_buttons(
    commands: &mut ChildBuilder,
    image_handles: &Res<HandleMap<ImageKey>>,
    actions: (GameAction, GameAction, GameAction),
) {
    // Show 3 buttons here
    commands
        .spawn(ButtonBundle {
            image: UiImage::new(
                image_handles[&ImageKey::HeatButton].clone_weak(),
            ),
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        })
        .insert(actions.0);

    commands
        .spawn(ButtonBundle {
                image: UiImage::new(
                    image_handles[&ImageKey::MoistureButton].clone_weak(),
                ),
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                ..default()
            })
            .insert(actions.1);

        commands
            .spawn(ButtonBundle {
                    image: UiImage::new(
                        image_handles[&ImageKey::WindButton].clone_weak(),
                    ),
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(80.0),
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    ..default()
                })
                .insert(actions.2);
}

pub fn predicted_selection(
    commands: &mut ChildBuilder,
    // image_handles: &Res<HandleMap<ImageKey>>,
    fonts: &Res<HandleMap<FontKey>>,
    predicted_text: String,
) {
    // ui element showing the heat, like high, low
    commands.spawn(TextBundle {
        text: Text::from_section(
            predicted_text,
            TextStyle {
                font_size: 8.0,
                color: LABEL_TEXT,
                font: fonts[&FontKey::LunchDS].clone_weak(),
            },
        ),
        style: Style { ..default() },
        ..default()
    });
}