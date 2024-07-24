use bevy::prelude::*;

use crate::screen::{weather_maniac::ToggleWeatherGridEvent, Screen};
use crate::ui::prelude::*;

use super::{
    assets::{FontKey, HandleMap, ImageKey},
    spawn::journey::{DayTask, NextDay},
};

#[derive(Event, Debug)]
pub struct SpawnGameUI;

pub fn plugin(app: &mut App) {
    app.observe(spawn_game_ui)
        .observe(update_choices)
        .add_event::<SpawnGameUI>()
        .add_event::<UpdateChoices>()
        .register_type::<GameAction>()
        .add_systems(Update, handle_game_action.run_if(in_state(Screen::Playing)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum GameAction {
    Bones,
    Choose(DayTask),
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
                    .insert(GameAction::Choose(*choice))
                    .with_children(|parent| {
                        parent.label(
                            match choice {
                                DayTask::Sail => "Sail",
                                DayTask::Fight => "Fight",
                                DayTask::Explore => "Explore",
                                DayTask::Rest => "Rest",
                                DayTask::HunkerDown => "Brace",
                                DayTask::CleanDaDeck => "Clean",
                                DayTask::CookDaFood => "Cook",
                                DayTask::Gamble => "Gamble",
                            },
                            fonts[&FontKey::PaperCut].clone_weak(),
                        );
                    });
            }
        });
}

#[derive(Event, Debug)]
pub struct UpdateChoices(pub Vec<DayTask>);

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
                                                        parent.label(
                                                            text.to_string(),
                                                            fonts[&FontKey::PaperCut].clone_weak(),
                                                        );
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
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                GameAction::Menu => next_screen.set(Screen::Title),
                GameAction::Bones => {
                    info!("Bones!!!");
                    commands.trigger(ToggleWeatherGridEvent);
                }
                GameAction::Choose(task) => commands.trigger(NextDay(*task)),
            }
        }
    }
}
