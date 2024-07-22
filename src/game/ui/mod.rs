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
                                                grid_template_columns: vec![GridTrack::fr(1.0), GridTrack::fr(1.0)],
                                                grid_template_rows: vec![GridTrack::fr(1.0), GridTrack::fr(1.0)],
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
                                                });
                                            }
                                        });
                        
                                    // Existing bone button
                                    commands
                                        .spawn(ButtonBundle {
                                            image: UiImage::new(
                                                image_handles[&ImageKey::BoneButton].clone_weak(),
                                            ),
                                            ..default()
                                        })
                                        .insert(GameAction::Bones);
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
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    // Container for the 2x2 grid of buttons
                                    commands
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(30.0),
                                                height: Val::Percent(25.0),
                                                position_type: PositionType::Absolute,
                                                left: Val::Percent(5.0),
                                                top: Val::Percent(12.5),
                                                display: Display::Grid,
                                                grid_template_columns: vec![GridTrack::fr(1.0), GridTrack::fr(1.0)],
                                                grid_template_rows: vec![GridTrack::fr(1.0), GridTrack::fr(1.0), GridTrack::fr(1.0)],
                                                column_gap: Val::Px(5.0),
                                                row_gap: Val::Px(5.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|commands| {
                                            // Add 6 buttons
                                            for _ in 0..6 {
                                                commands
                                                .spawn(ButtonBundle {
                                                    image: UiImage::new(
                                                        image_handles[&ImageKey::ChoicePanel].clone_weak(),
                                                    ),
                                                    style: Style {
                                                        width: Val::Percent(100.0),
                                                        height: Val::Percent(100.0),
                                                        margin: UiRect::all(Val::Auto),
                                                        ..default()
                                                    },
                                                    ..default()
                                                });
                                            }
                                        });

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
                                                    position_type: PositionType::Absolute,
                                                    right: Val::Percent(5.0),
                                                    top: Val::Percent(12.5),
                                                    width: Val::Percent(27.0),
                                                    aspect_ratio: Some(1.0),
                                                    display: Display::Grid,
                                                    grid_template_columns: vec![GridTrack::px(30.0), GridTrack::auto()],
                                                    grid_template_rows: vec![GridTrack::fr(1.0), GridTrack::fr(1.0), GridTrack::fr(1.0)],
                                                    column_gap: Val::Px(25.0), // Small gap between image and text
                                                    row_gap: Val::Px(-50.0), // Negative row gap to bring rows closer
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                for (image_key, text) in image_text_pairs.iter() {
                                                    // Image column
                                                    parent.spawn(ImageBundle {
                                                        image: UiImage::new(image_handles[image_key].clone_weak()),
                                                        style: Style {
                                                            height: Val::Px(30.0),
                                                            aspect_ratio: Some(1.0),
                                                            left: Val::Px(15.0),
                                                            align_content: AlignContent::Center,
                                                            margin: UiRect::new(Val::Px(0.0), Val::Auto, Val::Px(10.0), Val::Px(0.0)),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    });
                                        
                                                    // Text column
                                                    parent.spawn(TextBundle::from_section(
                                                        text.to_string(),
                                                        TextStyle {
                                                            font: fonts[&FontKey::PaperCut].clone_weak(),
                                                            font_size: 25.0,
                                                            color: Color::srgb(128.0 / 255.0, 98.0 / 255.0, 62.0 / 255.0),
                                                        },
                                                    ).with_style(Style {
                                                        margin: UiRect::new(Val::Px(0.0), Val::Auto, Val::Px(5.0), Val::Auto),
                                                        align_self: AlignSelf::Center,
                                                        ..default()
                                                    }));
                                                }
                                            });
                                });
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
