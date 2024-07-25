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
    app.add_systems(
        Update,
        (handle_title_action, move_ship).run_if(in_state(Screen::Title)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

#[derive(Component)]
struct MenuFloating {
    /// The width of area the ship should float in
    width: f32,
    /// The minimum x value for the ship
    height_min: f32,
    /// The maximum x value for the ship
    height_max: f32,
    /// How fast in pixels per second the ship will go
    speed: f32,
    /// The current direction of the ship (true is left)
    direction: bool,
    current_x: f32,
    mid_x: f32,
}

fn move_ship(mut query: Query<(&mut Transform, &mut MenuFloating, &mut Sprite)>, time: Res<Time>) {
    let (mut transform, mut floating, mut sprite) = query.single_mut();
    let octave_base = 1.9;
    let wave_speed = 0.05;
    floating.current_x += (if floating.direction {
        -floating.speed
    } else {
        floating.speed
    }) * time.delta_seconds();
    transform.translation.x = (floating.current_x).floor();
    transform.translation.y = (((floating.height_min + floating.height_max) / 2.0)
        + (floating.height_max - floating.height_min)
            * 0.5
            * (((time.elapsed().as_secs_f32() * octave_base * wave_speed).sin() * 0.5)
                + ((time.elapsed().as_secs_f32() * octave_base.powi(2) * wave_speed).sin()
                    * 0.25)
                + ((time.elapsed().as_secs_f32() * octave_base.powi(3) * wave_speed).sin()
                    * 0.125)
                + ((time.elapsed().as_secs_f32() * octave_base.powi(4) * wave_speed).sin()
                    * 0.075)
                + ((time.elapsed().as_secs_f32() * octave_base.powi(5) * wave_speed).sin()
                    * 0.075)))
        .floor();

    if floating.current_x > (floating.width / 2.0 + floating.mid_x) {
        floating.direction = true;
    } else if floating.current_x < (floating.width / -2.0 + floating.mid_x) {
        floating.direction = false;
    }
    sprite.flip_x = floating.direction;
}

fn enter_title(
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    fonts: Res<HandleMap<FontKey>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: image_handles[&ImageKey::BackDrop].clone_weak(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        StateScoped(Screen::Title),
    ));
    commands.spawn((
        SpriteBundle {
            texture: image_handles[&ImageKey::Logo].clone_weak(),
            transform: Transform::from_xyz(-150.0, 120.0, 0.5),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Title),
    ));
    commands.spawn((
        SpriteBundle {
            texture: image_handles[&ImageKey::Ship].clone_weak(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        MenuFloating {
            width: 250.0,
            height_max: 10.0,
            height_min: -10.0,
            speed: 1.0,
            current_x: -125.0,
            direction: false,
            mid_x: 20.0,
        },
        StateScoped(Screen::Title),
    ));
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children
                .spawn(NodeBundle {
                    style: Style {
                        aspect_ratio: Some(300.0 / 250.0),
                        height: Val::Vh(100.0),
                        display: Display::Grid,
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            GridTrack::px(20.0),
                            GridTrack::px(20.0),
                        ],
                        padding: UiRect::all(Val::Px(10.0)),
                        row_gap: Val::Px(10.0),
                        border: UiRect::all(Val::Px(2.0)),
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    ..default()
                })
                .with_children(|children| {
                    children
                        .button(
                            "Play",
                            image_handles[&ImageKey::Button].clone_weak(),
                            fonts[&FontKey::LunchDS].clone_weak(),
                        )
                        .insert(Style {
                            grid_row: GridPlacement::start_span(2, 1),
                            grid_column: GridPlacement::start_span(1, 1),
                            width: Val::Px(92.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(0.0)).with_bottom(Val::Percent(5.0)),
                            margin: UiRect::all(Val::Auto),
                            ..default()
                        })
                        .insert(TitleAction::Play);
                    children
                        .button(
                            "Credits",
                            image_handles[&ImageKey::Button].clone_weak(),
                            fonts[&FontKey::LunchDS].clone_weak(),
                        )
                        .insert(Style {
                            grid_row: GridPlacement::start_span(2, 1),
                            grid_column: GridPlacement::start_span(2, 1),
                            width: Val::Px(92.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(0.0)).with_bottom(Val::Percent(5.0)),
                            margin: UiRect::all(Val::Auto),
                            ..default()
                        })
                        .insert(TitleAction::Credits);

                    #[cfg(not(target_family = "wasm"))]
                    children
                        .button(
                            "Exit",
                            image_handles[&ImageKey::Button].clone_weak(),
                            fonts[&FontKey::LunchDS].clone_weak(),
                        )
                        .insert(Style {
                            grid_row: GridPlacement::start_span(3, 1),
                            grid_column: GridPlacement::start_span(2, 1),
                            width: Val::Px(92.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(0.0)).with_bottom(Val::Percent(5.0)),
                            margin: UiRect::all(Val::Auto),
                            ..default()
                        })
                        .insert(TitleAction::Exit);
                });
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

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
