use bevy::prelude::*;
use rand::Rng;
use ui_palette::{HEADER_TEXT, LABEL_TEXT};

use crate::dialogue;
use crate::screen::{weather_maniac::ToggleWeatherGridEvent, Screen};
use crate::ui::prelude::*;

use super::spawn::journey::{Continue, Journey, Ship};
use super::spawn::predicitons::{Predictions, UpdateDarkMagicUi, UpdateParrotUi, UpdateSpyGlassUi};
use super::spawn::quests::dialogue::{Dialogue, DialogueQueue};
use super::spawn::quests::treasure::Item;
use super::spawn::weather::{AnyWeather, Heat, Moisture, Wind};
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
        Visibility::Inherited
    } else {
        Visibility::Hidden
    }
}

pub fn plugin(app: &mut App) {
    app.observe(spawn_game_ui)
        .observe(update_choices)
        .observe(update_inventory)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum PredictionAction {
    Heat,
    Moisture,
    Wind,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum GameAction {
    Bones,
    Parrot,
    SpyGlass,
    DarkMagic,
    SpyGlassPredictionAction(PredictionAction),
    ParrotPredictionAction(PredictionAction),
    DarkMagicPredictionAction(PredictionAction),
    Continue,
    Choose(String),
    Menu,
}

#[derive(Component)]
struct ChoicePanel;

pub fn inventory_item(
    commands: &mut ChildBuilder,
    images: &HandleMap<ImageKey>,
    image: ImageKey,
    font: &Handle<Font>,
    count: i32,
) {
    commands
        .spawn(ImageBundle {
            image: UiImage {
                texture: images[&ImageKey::InventoryTag].clone_weak(),
                ..default()
            },
            style: Style {
                width: Val::Px(54.0),
                height: Val::Px(10.0),
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::percent(30.0), GridTrack::auto()],
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(ImageBundle {
                image: UiImage {
                    texture: images[&image].clone_weak(),
                    ..default()
                },
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    width: Val::Px(8.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                ..default()
            });
            commands
                .spawn(NodeBundle {
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|commands| {
                    commands.spawn(
                        TextBundle::from_section(
                            format!("{count}"),
                            TextStyle {
                                font_size: 8.0,
                                font: font.clone_weak(),
                                color: LABEL_TEXT,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::default().with_top(Val::Px(2.0)),
                            ..default()
                        }),
                    );
                });
        });
}

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

#[derive(Event, Debug)]
pub struct UpdateInventoryList;

#[derive(Component)]
pub struct ContinueButton;

#[derive(Component)]
struct DialogueContents;

#[derive(Component)]
struct DialogueSpeaker;

#[derive(Component)]
pub struct DarkMagicBox;

#[derive(Component)]
pub struct SpyGlassBox;

#[derive(Component)]
pub struct ParrotBox;

#[derive(Component)]
pub struct InventoryList;

fn update_inventory(
    _trigger: Trigger<UpdateInventoryList>,
    ilist: Query<Entity, With<InventoryList>>,
    fonts: Res<HandleMap<FontKey>>,
    images: Res<HandleMap<ImageKey>>,
    journey: Res<Journey>,
    mut commands: Commands,
) {
    commands
        .entity(ilist.single())
        .despawn_descendants()
        .with_children(|commands| {
            for (key, amount) in &journey.inventory {
                if *amount <= 0 {
                    continue;
                }
                inventory_item(
                    commands,
                    images.as_ref(),
                    match key {
                        super::spawn::quests::treasure::Item::MonkeyPaw => ImageKey::MonkeyPaw,
                        super::spawn::quests::treasure::Item::SirenKiller => ImageKey::SirenKiller,
                        super::spawn::quests::treasure::Item::Journal => ImageKey::Journal,
                        super::spawn::quests::treasure::Item::Cannon => ImageKey::Cannon,
                        super::spawn::quests::treasure::Item::Gold => ImageKey::Gold,
                        super::spawn::quests::treasure::Item::SirensCoveMap => {
                            ImageKey::SirensCoveMap
                        }
                        super::spawn::quests::treasure::Item::SirensScale => ImageKey::SirensScale,
                        super::spawn::quests::treasure::Item::NorthernSeaMap => {
                            ImageKey::NorthernSeaMap
                        }
                        super::spawn::quests::treasure::Item::GreekFire => ImageKey::GreekFire,
                        super::spawn::quests::treasure::Item::SirenChild => ImageKey::SirenChild,
                    },
                    &fonts[&FontKey::LunchDS],
                    *amount,
                );
            }
        });
}

#[derive(Component, Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub enum FocusedDisplay {
    Dialogue,
    Bones,
    Parrot,
    SpyGlass,
    DarkMagic,
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

    // Set everything to false, then just enable what we want
    commands.trigger(ToggleWeatherGridEvent(false));

    if focus == FocusedDisplay::Bones {
        // Handle trigger logic for bones screen here!
        commands.trigger(ToggleWeatherGridEvent(true));
    }

    if focus == FocusedDisplay::Parrot {
        // Handle trigger logic for parrot screen here!
        info!("Parrot Screen");
    }

    if focus == FocusedDisplay::SpyGlass {
        // Handle trigger logic for spy glass screen here!
        info!("Spy Glass Screen");
    }

    if focus == FocusedDisplay::DarkMagic {
        // Handle trigger logic for dark magic screen here!
        info!("Dark Magic Screen");
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
                    // ================= Left Panel ==============
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
                                        height: Val::Percent(100.0),
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        justify_items: JustifyItems::Center,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    // Container for the 2x2 grid of buttons
                                    commands
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Px(43.0),
                                                margin: UiRect::horizontal(Val::Auto)
                                                    .with_top(Val::Px(15.0)),
                                                display: Display::Grid,
                                                grid_template_columns: vec![
                                                    GridTrack::fr(1.0),
                                                    GridTrack::fr(1.0),
                                                ],
                                                grid_template_rows: vec![
                                                    GridTrack::fr(1.0),
                                                    GridTrack::fr(1.0),
                                                ],
                                                column_gap: Val::Px(3.0),
                                                row_gap: Val::Px(3.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|commands| {
                                            // Add 4 buttons
                                            for i in 0..4 {
                                                let (image_key, game_action) = match i {
                                                    0 => (ImageKey::BoneButton, GameAction::Bones),
                                                    1 => {
                                                        (ImageKey::ParrotButton, GameAction::Parrot)
                                                    }
                                                    2 => (
                                                        ImageKey::DarkmagicButton,
                                                        GameAction::DarkMagic,
                                                    ),
                                                    3 => (
                                                        ImageKey::SpyglassButton,
                                                        GameAction::SpyGlass,
                                                    ),
                                                    _ => (ImageKey::MissingImage, GameAction::Menu), // Default case, though it shouldn't occur
                                                };

                                                commands
                                                    .spawn(ButtonBundle {
                                                        image: UiImage::new(
                                                            image_handles[&image_key].clone_weak(),
                                                        ),
                                                        style: Style {
                                                            width: Val::Px(20.0),
                                                            height: Val::Px(20.0),
                                                            margin: UiRect::all(Val::Auto),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    })
                                                    .insert(game_action);
                                            }
                                        });

                                    commands
                                        .spawn(NodeBundle {
                                            style: Style {
                                                display: Display::Flex,
                                                flex_direction: FlexDirection::Column,
                                                justify_items: JustifyItems::Center,
                                                justify_content: JustifyContent::SpaceBetween,
                                                margin: UiRect::all(Val::Px(7.0)),
                                                row_gap: Val::Px(2.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .insert(InventoryList);
                                });
                        });

                    // ================= Bones ==============
                    // NOTICE!!!!
                    // Bones are handled externally, no need to handle it here
                    // commands
                    //     .spawn((
                    //         NodeBundle {
                    //             style: Style {
                    //                 grid_row: GridPlacement::start_span(1, 1),
                    //                 grid_column: GridPlacement::start_span(2, 1),
                    //                 ..default()
                    //             },
                    //             ..default()
                    //         },
                    //         FocusedDisplay::Bones,
                    //     ))
                    //     .with_children(|commands| {
                    //         commands
                    //             .spawn(ImageBundle {
                    //                 image: UiImage::new(
                    //                     image_handles[&ImageKey::DialogueBox].clone_weak(),
                    //                 ),
                    //                 style: Style {
                    //                     width: Val::Px(204.0),
                    //                     height: Val::Px(150.0),
                    //                     display: Display::Flex,
                    //                     flex_direction: FlexDirection::Column,
                    //                     margin: UiRect::all(Val::Auto).with_top(Val::Px(10.0)),
                    //                     ..default()
                    //                 },
                    //                 ..default()
                    //             });
                    //     });

                    // ================= Parrot ==============
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
                            FocusedDisplay::Parrot,
                        ))
                        .with_children(|commands| {
                            commands.spawn((
                                ImageBundle {
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
                                },
                                ParrotBox,
                            ));
                        });

                    // ================= Spy Glass ==============
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
                            FocusedDisplay::SpyGlass,
                        ))
                        .with_children(|commands| {
                            commands.spawn((
                                ImageBundle {
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
                                },
                                SpyGlassBox,
                            ));
                        });

                    // ================= Dark Magic ==============
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
                            FocusedDisplay::DarkMagic,
                        ))
                        .with_children(|commands| {
                            commands.spawn((
                                ImageBundle {
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
                                },
                                DarkMagicBox,
                            ));
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

                                                visibility: Visibility::Inherited,
                                                ..default()
                                            },
                                            ContinueButton,
                                            FocusedDisplay::Dialogue,
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

                    // ================= Bottom Panel ==============
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
    mut predictions: ResMut<Predictions>,
    mut journey: ResMut<Journey>,
    mut dq: ResMut<DialogueQueue>,
    mut ship: ResMut<Ship>,
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

                GameAction::Parrot => {
                    info!("Parrot!!!");
                    commands.trigger(Focus(if *display == FocusedDisplay::Parrot {
                        FocusedDisplay::Dialogue
                    } else {
                        FocusedDisplay::Parrot
                    }));
                }

                GameAction::SpyGlass => {
                    info!("Spy Glass!!!");
                    commands.trigger(Focus(if *display == FocusedDisplay::SpyGlass {
                        FocusedDisplay::Dialogue
                    } else {
                        FocusedDisplay::SpyGlass
                    }));
                }

                GameAction::DarkMagic => {
                    info!("Dark Magic!!!");
                    commands.trigger(Focus(if *display == FocusedDisplay::DarkMagic {
                        FocusedDisplay::Dialogue
                    } else {
                        FocusedDisplay::DarkMagic
                    }));
                }
                GameAction::Continue => commands.trigger(Continue),

                GameAction::SpyGlassPredictionAction(action) => {
                    let scales = *journey.inventory.get(&Item::SirensScale).unwrap_or(&0);
                    let accurate = journey.rng.gen_range(0..(5 - scales).max(1)) < 2;
                    let result = if accurate {
                        match action {
                            PredictionAction::Heat => AnyWeather::Heat(journey.weather.heat),
                            PredictionAction::Moisture => {
                                AnyWeather::Moisture(journey.weather.moisture)
                            }
                            PredictionAction::Wind => AnyWeather::Wind(journey.weather.wind),
                        }
                    } else {
                        match action {
                            PredictionAction::Heat => {
                                let rand = journey.rng.gen_range(0..5);
                                AnyWeather::Heat(match rand {
                                    4 => Heat::Blistering,
                                    3 => Heat::Warm,
                                    2 => Heat::Comfortable,
                                    1 => Heat::Chilly,
                                    _ => Heat::Freezing,
                                })
                            }
                            PredictionAction::Moisture => {
                                let rand = journey.rng.gen_range(0..3);
                                AnyWeather::Moisture(match rand {
                                    2 => Moisture::Humid,
                                    1 => Moisture::Comfortable,
                                    _ => Moisture::Dry,
                                })
                            }
                            PredictionAction::Wind => {
                                let rand = journey.rng.gen_range(0..5);
                                AnyWeather::Wind(match rand {
                                    4 => Wind::GaleForce,
                                    3 => Wind::High,
                                    2 => Wind::Medium,
                                    1 => Wind::Low,
                                    _ => Wind::None,
                                })
                            }
                        }
                    };
                    predictions.spy_glass = Some(result);
                    commands.trigger(UpdateSpyGlassUi)
                }
                GameAction::ParrotPredictionAction(action) => {
                    let result = match action {
                        PredictionAction::Heat => AnyWeather::Heat(journey.weather.heat),
                        PredictionAction::Moisture => {
                            AnyWeather::Moisture(journey.weather.moisture)
                        }
                        PredictionAction::Wind => AnyWeather::Wind(journey.weather.wind),
                    };
                    predictions.parrot = Some(result);
                    commands.trigger(UpdateParrotUi)
                }
                GameAction::DarkMagicPredictionAction(action) => {
                    let mp = *journey.inventory.get(&Item::MonkeyPaw).unwrap_or(&0);
                    if mp > 0 {
                        journey.inventory.insert(Item::MonkeyPaw, mp - 1);
                    } else {
                        match journey.rng.gen_range(0..7) {
                            5 | 4 | 3 => {
                                dq.queue.push_back(
                                    dialogue!("Dark Magic"; "The dark magic takes your treasures."),
                                );
                                let gold = *journey.inventory.get(&Item::Gold).unwrap_or(&0);
                                journey.inventory.insert(Item::Gold, gold - gold.min(20));
                            }
                            2 => {
                                dq.queue.push_back(
                                    dialogue!("Dark Magic"; "The dark magic rusts your iron."),
                                );
                                let cannon = *journey.inventory.get(&Item::Cannon).unwrap_or(&0);
                                journey
                                    .inventory
                                    .insert(Item::Cannon, cannon - cannon.max(1));
                            }
                            1 => {
                                dq.queue.push_back(
                                    dialogue!("Dark Magic"; "The dark magic takes a soul."),
                                );
                                ship.crew -= ship.crew.min(1);
                            }
                            _ => {}
                        }
                    }
                    let result = match action {
                        PredictionAction::Heat => AnyWeather::Heat(journey.weather.heat),
                        PredictionAction::Moisture => {
                            AnyWeather::Moisture(journey.weather.moisture)
                        }
                        PredictionAction::Wind => AnyWeather::Wind(journey.weather.wind),
                    };
                    predictions.dark_magic = Some(result);
                    commands.trigger(UpdateDarkMagicUi)
                }
            }
        }
    }
}
