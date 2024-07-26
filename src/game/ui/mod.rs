use bevy::prelude::*;
use ui_palette::{HEADER_TEXT, LABEL_TEXT};

use crate::screen::{weather_maniac::ToggleWeatherGridEvent, Screen};
use crate::ui::prelude::*;

use super::spawn::journey::{Continue, Ship};
use super::spawn::quests::dialogue::Dialogue;
use super::{
    assets::{FontKey, HandleMap, ImageKey},
    spawn::journey::ChooseTask,
};

#[derive(Event, Debug)]
pub struct SpawnGameUI;

#[derive(Event, Debug)]
pub struct ShowContinue(pub bool);

fn show_continue(
    trigger: Trigger<ShowContinue>,
    mut query: Query<&mut Visibility, With<ContinueButton>>,
) {
    let mut vis = query.single_mut();
    *vis = if trigger.event().0 {
        Visibility::Visible
    } else {
        Visibility::Hidden
    }
}

pub fn plugin(app: &mut App) {
    app.observe(spawn_game_ui)
        .observe(update_choices)
        .observe(update_ship_stats)
        .observe(focus_display)
        .observe(show_continue)
        .insert_resource(FocusedDisplay::Dialogue)
        .add_event::<SpawnGameUI>()
        .add_event::<UpdateChoices>()
        .register_type::<GameAction>()
        .add_systems(Update, handle_game_action.run_if(in_state(Screen::Playing)))
        .observe(update_dialogue);
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Reflect)]
pub enum GameAction {
    Bones,
    Continue,
    Choose(String),
    Menu,
}

#[derive(Component)]
struct ChoicePanel;

fn update_choices(
    trigger: Trigger<UpdateChoices>,
    query: Query<(Entity, &ChoicePanel)>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut commands: Commands,
    fonts: Res<HandleMap<FontKey>>,
) {
    let (entity, _) = query.single();
    commands
        .entity(entity)
        .despawn_descendants()
        .with_children(|commands| {
            // Add 6 buttons
            for choice in trigger.event().0.iter() {
                commands
                    .spawn(ButtonBundle {
                        image: UiImage::new(image_handles[&ImageKey::ChoicePanel].clone_weak()),
                        style: Style {
                            width: Val::Px(70.0),
                            height: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(GameAction::Choose(choice.to_owned()))
                    .with_children(|parent| {
                        parent.label(choice, fonts[&FontKey::LunchDS].clone_weak());
                    });
            }
        });
}

#[derive(Event, Debug)]
pub struct UpdateShipStatsUI;

fn update_ship_stats(
    _trigger: Trigger<UpdateShipStatsUI>,
    ship: Res<Ship>,
    mut crew: Query<
        &mut Text,
        (
            With<CrewLabel>,
            Without<FoodLabel>,
            Without<ShipHealthLabel>,
        ),
    >,
    mut food: Query<
        &mut Text,
        (
            With<FoodLabel>,
            Without<CrewLabel>,
            Without<ShipHealthLabel>,
        ),
    >,
    mut health: Query<
        &mut Text,
        (
            With<ShipHealthLabel>,
            Without<CrewLabel>,
            Without<FoodLabel>,
        ),
    >,
    fonts: Res<HandleMap<FontKey>>,
) {
    *(crew.single_mut()) = Text::from_section(
        format!("{c}/{m}", c = ship.crew, m = ship.max_crew),
        TextStyle {
            font: fonts[&FontKey::LunchDS].clone_weak(),
            color: LABEL_TEXT,
            font_size: 10.0,
        },
    );
    *(food.single_mut()) = Text::from_section(
        format!("{c}/{m}", c = ship.food, m = ship.max_food),
        TextStyle {
            font: fonts[&FontKey::LunchDS].clone_weak(),
            color: LABEL_TEXT,
            font_size: 10.0,
        },
    );
    *(health.single_mut()) = Text::from_section(
        format!("{c}/{m}", c = ship.health, m = ship.max_health),
        TextStyle {
            font: fonts[&FontKey::LunchDS].clone_weak(),
            color: LABEL_TEXT,
            font_size: 10.0,
        },
    );
}
#[derive(Event, Debug)]
pub struct UpdateChoices(pub Vec<String>);

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct DialogueContents;

#[derive(Component)]
struct DialogueSpeaker;

#[derive(Component, Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub enum FocusedDisplay {
    Dialogue,
    Bones,
}

#[derive(Event, Debug)]
pub struct Focus(pub FocusedDisplay);

fn focus_display(
    trigger: Trigger<Focus>,
    mut query: Query<(&mut Visibility, &FocusedDisplay)>,
    mut commands: Commands,
    mut focused: ResMut<FocusedDisplay>,
) {
    let focus = trigger.event().0;

    *focused = focus;
    if focus == FocusedDisplay::Bones {
        commands.trigger(ToggleWeatherGridEvent(true));
    } else {
        commands.trigger(ToggleWeatherGridEvent(false));
    }

    for (mut vis, display) in &mut query {
        if *display == focus {
            *vis = Visibility::Visible;
        } else {
            *vis = Visibility::Hidden;
        }
    }
}

#[derive(Event, Debug)]
pub struct UpdateDialogBox(pub Dialogue);

fn update_dialogue(
    trigger: Trigger<UpdateDialogBox>,
    contents: Query<Entity, With<DialogueContents>>,
    speaker: Query<Entity, With<DialogueSpeaker>>,
    fonts: Res<HandleMap<FontKey>>,
    mut commands: Commands,
) {
    let contents = contents.single();
    let speaker = speaker.single();

    commands
        .entity(speaker)
        .despawn_descendants()
        .with_children(|commands| {
            commands.spawn(TextBundle {
                text: Text::from_section(
                    format!("{speaker}:", speaker = trigger.event().0.speaker),
                    TextStyle {
                        font_size: 12.0,
                        color: LABEL_TEXT,
                        font: fonts[&FontKey::LunchDS].clone_weak(),
                    },
                ),
                style: Style { ..default() },
                ..default()
            });
        });
    commands
        .entity(contents)
        .despawn_descendants()
        .with_children(|commands| {
            for p in &trigger.event().0.paragraphs {
                commands.spawn(TextBundle {
                    text: Text::from_section(
                        p.to_owned(),
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
        });
}

#[derive(Component, Default)]
struct CrewLabel;

#[derive(Component, Default)]
struct FoodLabel;

#[derive(Component, Default)]
struct ShipHealthLabel;

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
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_rows: (vec![
                            GridTrack::fr((250.0 - 80.0) / 80.0),
                            GridTrack::fr(1.0),
                        ]),
                        grid_template_columns: (vec![
                            GridTrack::fr(1.0),
                            GridTrack::fr(230.0 / 70.0),
                        ]),
                        row_gap: Val::Px(0.0),
                        column_gap: Val::Px(0.0),
                        height: Val::Percent(100.0),
                        aspect_ratio: Some(300.0 / 250.0),
                        margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|commands| {
                    //Left pannel
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                grid_row: GridPlacement::start_span(1, 2),
                                grid_column: GridPlacement::start_span(1, 1),
                                margin: UiRect::new(
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|commands| {
                            commands
                                .spawn(ImageBundle {
                                    image: UiImage::new(
                                        image_handles[&ImageKey::LeftPanel].clone_weak(),
                                    ),
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    // Container for the 2x2 grid of buttons
                                    commands
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(50.0),
                                                aspect_ratio: Some(1.0),
                                                position_type: PositionType::Absolute,
                                                top: Val::Percent(5.0),
                                                left: Val::Percent(10.0),
                                                display: Display::Grid,
                                                grid_template_columns: vec![
                                                    GridTrack::fr(1.0),
                                                    GridTrack::fr(1.0),
                                                ],
                                                grid_template_rows: vec![
                                                    GridTrack::fr(1.0),
                                                    GridTrack::fr(1.0),
                                                ],
                                                column_gap: Val::Percent(2.0),
                                                row_gap: Val::Percent(2.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|commands| {
                                            // Add 4 buttons
                                            for i in 0..4 {
                                                let image_key: ImageKey = match i {
                                                    0 => ImageKey::BoneButton,
                                                    1 => ImageKey::MissingImage,
                                                    2 => ImageKey::MissingImage,
                                                    3 => ImageKey::MissingImage,
                                                    _ => ImageKey::MissingImage,
                                                };

                                                commands
                                                    .spawn(ButtonBundle {
                                                        image: UiImage::new(
                                                            image_handles[&image_key].clone_weak(),
                                                        ),
                                                        style: Style {
                                                            width: Val::Percent(80.0),
                                                            height: Val::Percent(80.0),
                                                            margin: UiRect::all(Val::Auto),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    })
                                                    .insert(GameAction::Bones);
                                            }
                                        });
                                });
                        });
                    // ================= Dialogue Box ==============
                    commands
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    grid_row: GridPlacement::start_span(1, 1),
                                    grid_column: GridPlacement::start_span(2, 1),
                                    ..default()
                                },
                                ..default()
                            },
                            FocusedDisplay::Dialogue,
                        ))
                        .with_children(|commands| {
                            commands
                                .spawn((ImageBundle {
                                    image: UiImage::new(
                                        image_handles[&ImageKey::DialogueBox].clone_weak(),
                                    ),
                                    style: Style {
                                        width: Val::Px(204.0),
                                        height: Val::Px(92.0),
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        margin: UiRect::all(Val::Auto).with_top(Val::Px(10.0)),
                                        ..default()
                                    },
                                    ..default()
                                },))
                                .with_children(|commands| {
                                    commands.spawn((
                                        NodeBundle {
                                            style: Style {
                                                padding: UiRect::horizontal(Val::Px(15.0))
                                                    .with_top(Val::Px(7.0))
                                                    .with_bottom(Val::Px(7.0)),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        DialogueSpeaker,
                                    ));
                                    commands.spawn((
                                        NodeBundle {
                                            style: Style {
                                                margin: UiRect::horizontal(Val::Px(10.0))
                                                    .with_left(Val::Px(22.0)),
                                                display: Display::Flex,
                                                flex_direction: FlexDirection::Column,
                                                row_gap: Val::Px(3.0),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        DialogueContents,
                                    ));
                                    commands
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    margin: UiRect::horizontal(Val::Px(10.0))
                                                        .with_left(Val::Px(22.0)),
                                                    display: Display::Flex,
                                                    flex_direction: FlexDirection::Column,
                                                    row_gap: Val::Px(3.0),
                                                    position_type: PositionType::Absolute,
                                                    right: Val::Px(5.0),
                                                    bottom: Val::Px(10.0),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            ContinueButton,
                                        ))
                                        .with_children(|commands| {
                                            commands.spawn(TextBundle::from_section(
                                                "Continue...",
                                                TextStyle {
                                                    font: fonts[&FontKey::LunchDS].clone_weak(),
                                                    color: HEADER_TEXT,
                                                    font_size: 10.0,
                                                },
                                            ));
                                        })
                                        .insert(GameAction::Continue);
                                });
                        });
                    //Bottom panel
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                grid_row: GridPlacement::start_span(2, 1),
                                grid_column: GridPlacement::start_span(2, 1),
                                margin: UiRect::new(
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|commands| {
                            commands
                                .spawn(ImageBundle {
                                    image: UiImage::new(
                                        image_handles[&ImageKey::BottomPanel].clone_weak(),
                                    ),
                                    style: Style {
                                        width: Val::Px(230.0),
                                        height: Val::Px(80.0),
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::SpaceAround,
                                        column_gap: Val::Px(2.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    // Container for the choices
                                    commands.spawn((
                                        NodeBundle {
                                            style: Style {
                                                display: Display::Grid,
                                                grid_template_columns: vec![
                                                    GridTrack::fr(1.0),
                                                    GridTrack::fr(1.0),
                                                ],
                                                grid_template_rows: vec![
                                                    GridTrack::fr(1.0),
                                                    GridTrack::fr(1.0),
                                                    GridTrack::fr(1.0),
                                                ],
                                                column_gap: Val::Px(2.0),
                                                row_gap: Val::Px(2.0),
                                                justify_items: JustifyItems::Center,
                                                width: Val::Px(150.0),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        ChoicePanel,
                                    ));

                                    // Add the square details panel to the right
                                    let image_text_pairs = [
                                        (ImageKey::CrewImage, "40/40"),
                                        (ImageKey::FoodImage, "50/50"),
                                        (ImageKey::ShipStatsImage, "100/100"),
                                    ];

                                    commands
                                        .spawn(ImageBundle {
                                            image: UiImage::new(
                                                image_handles[&ImageKey::DetailsPanel].clone_weak(),
                                            ),
                                            style: Style {
                                                width: Val::Px(70.0),
                                                aspect_ratio: Some(1.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn(NodeBundle {
                                                    style: Style {
                                                        margin: UiRect::all(Val::Px(6.0)),
                                                        display: Display::Grid,
                                                        grid_template_columns: vec![
                                                            GridTrack::px(8.0),
                                                            GridTrack::auto(),
                                                        ],
                                                        grid_template_rows: vec![
                                                            GridTrack::fr(1.0),
                                                            GridTrack::fr(1.0),
                                                            GridTrack::fr(1.0),
                                                        ],
                                                        column_gap: Val::Px(4.0),
                                                        align_items: AlignItems::Center,
                                                        row_gap: Val::Px(1.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    for (image_key, text) in image_text_pairs.iter()
                                                    {
                                                        // Image column
                                                        parent.spawn(ImageBundle {
                                                            image: UiImage::new(
                                                                image_handles[image_key]
                                                                    .clone_weak(),
                                                            ),
                                                            style: Style {
                                                                height: Val::Px(13.0),
                                                                aspect_ratio: Some(1.0),
                                                                align_content: AlignContent::Center,
                                                                ..default()
                                                            },
                                                            ..default()
                                                        });

                                                        // Text column
                                                        let mut text_section =
                                                            parent.spawn(TextBundle::from_section(
                                                                text.to_string(),
                                                                TextStyle {
                                                                    font: fonts[&FontKey::LunchDS]
                                                                        .clone_weak(),
                                                                    color: LABEL_TEXT,
                                                                    font_size: 10.0,
                                                                },
                                                            ));
                                                        match image_key {
                                                            ImageKey::CrewImage => {
                                                                text_section.insert(CrewLabel);
                                                            }
                                                            ImageKey::FoodImage => {
                                                                text_section.insert(FoodLabel);
                                                            }
                                                            ImageKey::ShipStatsImage => {
                                                                text_section
                                                                    .insert(ShipHealthLabel);
                                                            }
                                                            _ => {}
                                                        }
                                                    }
                                                });
                                        });
                                });
                        });
                });
        });
}

fn handle_game_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&GameAction>,
    mut commands: Commands,
    display: Res<FocusedDisplay>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                GameAction::Menu => next_screen.set(Screen::Title),
                GameAction::Bones => {
                    info!("Bones!!!");
                    commands.trigger(Focus(if *display == FocusedDisplay::Bones {
                        FocusedDisplay::Dialogue
                    } else {
                        FocusedDisplay::Bones
                    }));
                }
                GameAction::Choose(task) => commands.trigger(ChooseTask(task.to_owned())),
                GameAction::Continue => commands.trigger(Continue),
            }
        }
    }
}
