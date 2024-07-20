use bevy::{prelude::*, utils::hashbrown::HashMap};
use rand::Rng;

use crate::game::{
    assets::{HandleMap, ImageKey},
    spawn::journey::{AnyWeather, Heat, Moisture, Rain},
};

use super::Screen;

const ROW_COUNT: usize = 4;
const COLUMN_COUNT: usize = 4;
const GRID_BLOCK_SIZE: usize = 16;
const GRID_BLOCK_PADDING: usize = 1;

type BonePattern = [[u8; ROW_COUNT]; COLUMN_COUNT];
type ConditionMap = HashMap<AnyWeather, BonePattern>;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct WeatherControlConditions {
    condition_map: ConditionMap,
    timer: Timer,
}

fn generate_random_bone_pattern() -> BonePattern {
    let mut ret: BonePattern = BonePattern::default();
    let mut rngeezus = rand::thread_rng();
    for x in 0..COLUMN_COUNT {
        for y in 0..ROW_COUNT {
            let random_number = rngeezus.gen_range(0..2);
            ret[x][y] = random_number;
        }
    }
    ret
}

impl Default for WeatherControlConditions {
    fn default() -> Self {
        let mut condition_map: ConditionMap = HashMap::new();
        for condition in [
            AnyWeather::Heat(Heat::Blistering),
            AnyWeather::Heat(Heat::Warm),
            AnyWeather::Heat(Heat::Comfortable),
            AnyWeather::Heat(Heat::Chilly),
            AnyWeather::Heat(Heat::Freezing),
            AnyWeather::Moisture(Moisture::Dry),
            AnyWeather::Moisture(Moisture::Comfortable),
            AnyWeather::Moisture(Moisture::Humid),
            AnyWeather::Rain(Rain::None),
            AnyWeather::Rain(Rain::Drizzle),
            AnyWeather::Rain(Rain::Rain),
            AnyWeather::Rain(Rain::Storm),
        ] {
            condition_map.insert(condition, generate_random_bone_pattern());
        }
        Self {
            condition_map,
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WeatherControlConditions>()
        .add_event::<ToggleWeatherGridEvent>()
        .add_systems(OnEnter(Screen::WeatherManiac), enter_weather_maniac)
        .add_systems(
            Update,
            (
                update_weather_condition,
                toggle_weather_grid,
                handle_toggle_weather_grid,
            )
                .run_if(in_state(Screen::WeatherManiac)),
        )
        .observe(render_weather_condition);
}

fn enter_weather_maniac(mut commands: Commands) {
    info!("Entering Weather Maniac screen");
    let grid = WeatherControlConditions::default();

    for (offset_x, row) in grid.condition_map[&AnyWeather::Heat(Heat::Comfortable)]
        .iter()
        .enumerate()
    {
        for (offset_y, cell) in row.iter().enumerate() {
            // spawn a sprite at the cell position
            let color = match cell {
                0 => Color::WHITE,
                1 => Color::BLACK,
                _ => Color::WHITE,
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 {
                            x: GRID_BLOCK_SIZE as f32,
                            y: GRID_BLOCK_SIZE as f32,
                        }),
                        color,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        Vec3::new(offset_x as f32, offset_y as f32, 0.0)
                            * (GRID_BLOCK_SIZE + GRID_BLOCK_PADDING) as f32,
                    ),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                DisplayPosition {
                    x: offset_x as u8,
                    y: offset_y as u8,
                },
            ));
        }
    }
    commands.insert_resource(grid);
    commands.trigger(UpdateGrid(AnyWeather::Heat(Heat::Comfortable)));
}

fn update_weather_condition(
    mut conditions: ResMut<WeatherControlConditions>,
    time: Res<Time>,
    mut commands: Commands,
) {
    conditions.timer.tick(time.delta());
    if conditions.timer.finished() {
        conditions.timer.reset();
        commands.trigger(UpdateGrid(AnyWeather::Moisture(Moisture::Dry)));
    }
}
#[derive(Component)]
struct DisplayPosition {
    pub x: u8,
    pub y: u8,
}

#[derive(Event, Debug)]
struct UpdateGrid(AnyWeather);

fn render_weather_condition(
    _trigger: Trigger<UpdateGrid>,
    conditions: Res<WeatherControlConditions>,
    mut query: Query<(
        &mut Sprite,
        &mut Handle<Image>,
        &mut Transform,
        &DisplayPosition,
    )>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    // Render the weather condition to the screen in a 3x3 grid
    let condition = conditions.condition_map[&_trigger.event().0];
    for (mut sprite, mut texture, mut transform, position) in &mut query {
        let pos_x = position.x as usize;
        let pos_y = position.y as usize;
        sprite.color = match condition[pos_x][pos_y] {
            0 => Color::srgba(0.0, 0.0, 0.0, 0.0),
            1 => Color::WHITE,
            _ => Color::WHITE,
        };
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..4);
        *texture = match index {
            0 => image_handles[&ImageKey::Bone1].clone_weak(),
            1 => image_handles[&ImageKey::Bone2].clone_weak(),
            2 => image_handles[&ImageKey::Bone3].clone_weak(),
            _ => image_handles[&ImageKey::Bone4].clone_weak(),
        };
        transform.rotate_z(rng.gen_range(0f32..3.14f32));
    }
}

// VISIBILITY - You can blame Justin for this amazing stuff
#[derive(Event)]
struct ToggleWeatherGridEvent;

fn toggle_weather_grid(
    input: Res<ButtonInput<KeyCode>>,
    mut ev_toggle: EventWriter<ToggleWeatherGridEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        ev_toggle.send(ToggleWeatherGridEvent);
        info!("Toggle weather grid event sent");
    }
}

fn handle_toggle_weather_grid(
    mut ev_toggle: EventReader<ToggleWeatherGridEvent>,
    mut query: Query<&mut Visibility, With<DisplayPosition>>,
) {
    for _ in ev_toggle.read() {
        for mut visibility in &mut query {
            *visibility = if *visibility == Visibility::Hidden {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };

            info!("Weather grid visibility toggled: {:?}", visibility);
        }
    }
}
