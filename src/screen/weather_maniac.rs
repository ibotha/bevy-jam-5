use std::f32::consts::PI;

use bevy::{prelude::*, utils::hashbrown::HashMap};
use rand::Rng;

use crate::game::{
    assets::{HandleMap, ImageKey},
    spawn::weather::{AnyWeather, Heat},
};

use super::Screen;

const ROW_COUNT: usize = 4;
const COLUMN_COUNT: usize = 4;
const GRID_BLOCK_SIZE: usize = 16;
const GRID_BLOCK_PADDING: usize = 0;

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
    for column in ret.iter_mut() {
        for item in column.iter_mut() {
            let random_number = rngeezus.gen_range(0..2);
            *item = random_number;
        }
    }
    ret
}

impl Default for WeatherControlConditions {
    fn default() -> Self {
        Self {
            condition_map: HashMap::new(),
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<WeatherControlConditions>()
        .add_event::<ToggleWeatherGridEvent>()
        .add_systems(OnEnter(Screen::Playing), enter_weather_maniac)
        .observe(render_weather_condition)
        .observe(handle_toggle_weather_grid);
}

fn enter_weather_maniac(mut commands: Commands, images: Res<HandleMap<ImageKey>>) {
    info!("Entering Weather Maniac screen");
    let grid = WeatherControlConditions::default();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..Default::default()
            },
            texture: images[&ImageKey::DetailsPanel].clone_weak(),
            transform: Transform::from_translation(Vec3::new(-11.0, -11.0, 1.9)),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        ToggleWithBones,
    ));
    for offset_x in 0..COLUMN_COUNT {
        for offset_y in 0..ROW_COUNT {
            // spawn a sprite at the cell position
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 {
                            x: GRID_BLOCK_SIZE as f32,
                            y: GRID_BLOCK_SIZE as f32,
                        }),
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        Vec3::new(offset_x as f32, offset_y as f32, 2.0)
                            * (GRID_BLOCK_SIZE + GRID_BLOCK_PADDING) as f32,
                    ),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                ToggleWithBones,
                DisplayPosition {
                    x: offset_x as u8,
                    y: offset_y as u8,
                },
            ));
        }
    }
    commands.insert_resource(grid);
    commands.trigger(UpdateBoneGrid(AnyWeather::Heat(Heat::Comfortable)));
}

#[derive(Component)]
struct DisplayPosition {
    pub x: u8,
    pub y: u8,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct UpdateBoneGrid(pub AnyWeather);

fn render_weather_condition(
    trigger: Trigger<UpdateBoneGrid>,
    mut conditions: ResMut<WeatherControlConditions>,
    mut query: Query<(
        &mut Sprite,
        &mut Handle<Image>,
        &mut Transform,
        &DisplayPosition,
    )>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let condition = trigger.event().0;
    if !conditions.condition_map.contains_key(&condition) {
        conditions
            .condition_map
            .insert(condition, generate_random_bone_pattern());
    }
    let bone_pattern = conditions.condition_map[&condition];
    for (mut sprite, mut texture, mut transform, position) in &mut query {
        let pos_x = position.x as usize;
        let pos_y = position.y as usize;
        sprite.color = match bone_pattern[pos_x][pos_y] {
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
        transform.rotate_z(rng.gen_range(0f32..PI));
    }
}

// VISIBILITY - You can blame Justin for this amazing stuff
#[derive(Event)]
pub struct ToggleWeatherGridEvent(pub bool);

#[derive(Component)]
pub struct ToggleWithBones;

fn handle_toggle_weather_grid(
    trigger: Trigger<ToggleWeatherGridEvent>,
    mut query: Query<&mut Visibility, With<ToggleWithBones>>,
) {
    info!("Toggling the bones!");
    for mut visibility in &mut query {
        *visibility = if trigger.event().0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
