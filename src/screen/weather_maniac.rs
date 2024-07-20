use bevy::prelude::*;
use rand::Rng;

use super::Screen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
enum WeatherConditions {
    Sunny,
    Cloudy,
    Hail,
    Cyclone,
    DoNotUse, // Sorry lord izzard
}

impl From<usize> for WeatherConditions {
    fn from(value: usize) -> Self {
        if value == WeatherConditions::DoNotUse as usize {
            panic!("Do not use the DoNotUse variant");
        }
        match value {
            0 => WeatherConditions::Sunny,
            1 => WeatherConditions::Cloudy,
            2 => WeatherConditions::Hail,
            3 => WeatherConditions::Cyclone,
            _ => panic!("Update the From<uszie> for WeatherConditions enum to include the new variant"),
        }
    }
    
}

const CONDITIONS_COUNT: usize=WeatherConditions::DoNotUse as usize;
const ROW_COUNT: usize=6;
const COLUMN_COUNT: usize=6;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct WeatherControlConditions {
    conditions: WeatherConditions,
    condition_map: [[[u8; ROW_COUNT]; COLUMN_COUNT]; CONDITIONS_COUNT],
    timer: Timer,
}

impl Default for WeatherControlConditions {
    fn default() -> Self {
        let mut rngeezus = rand::thread_rng();
        let mut condition_map = [[[0; ROW_COUNT]; COLUMN_COUNT]; CONDITIONS_COUNT];
        for i in 0..CONDITIONS_COUNT {
            for x in 0..COLUMN_COUNT {
                for y in 0..ROW_COUNT {
                    let random_number = rngeezus.gen_range(0..2);
                    condition_map[i][x][y] = random_number;
                }
            }
        }
        
        Self {
            conditions: WeatherConditions::Sunny,
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
               render_weather_condition,
               toggle_weather_grid,
               handle_toggle_weather_grid,
           )
               .run_if(in_state(Screen::WeatherManiac)),
       );
}

fn enter_weather_maniac(mut commands: Commands) {
    info!("Entering Weather Maniac screen");
    let grid = WeatherControlConditions::default();

    for (offset_x, row) in grid.condition_map[0].iter().enumerate() {
        for (offset_y, cell) in row.iter().enumerate() {
            // spawn a sprite at the cell position
            let color = match cell {
                0 => Color::WHITE,
                1 => Color::BLACK,
                _ => Color::WHITE,
            };
            commands.spawn(
                (
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2 { x: 32.0, y: 32.0}),
                            color,
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(offset_x as f32 * 35.0, offset_y as f32 * 35.0, 0.0)),
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                    IndexX(offset_x as u8), 
                    IndexY(offset_y as u8)
                )
            );
        }
    }
    commands.insert_resource(grid);
}

fn update_weather_condition(mut conditions: ResMut<WeatherControlConditions>, time: Res<Time>) {
    conditions.timer.tick(time.delta());
    if conditions.timer.finished() {
        let mut rng = rand::thread_rng();
        let new_index = rng.gen_range(0..CONDITIONS_COUNT);
        conditions.conditions = new_index.into();
        conditions.timer.reset();
        info!("Weather conditions changed to {:?}", conditions.conditions);
    }
}
#[derive(Component)]
struct IndexX(u8);

#[derive(Component)]
struct IndexY(u8);

fn render_weather_condition(
        conditions: Res<WeatherControlConditions>,
        mut query: Query<(&mut Sprite, &IndexX, &IndexY)>
    ) {
    // Render the weather condition to the screen in a 3x3 grid
    let condition = conditions.condition_map[conditions.conditions as usize];
    for (mut sprite, x, y) in &mut query {
        let pos_x = x.0 as usize;
        let pos_y = y.0 as usize;
        let color = match condition[pos_x][pos_y] {
            0 => Color::WHITE,
            1 => Color::BLACK,
            _ => Color::WHITE,
        };
        sprite.color = color;
    }
}


// VISIBILITY - You can blame Justin for this amazing stuff
#[derive(Event)]
struct ToggleWeatherGridEvent;

fn toggle_weather_grid(input: Res<ButtonInput<KeyCode>>, mut ev_toggle: EventWriter<ToggleWeatherGridEvent>) {
    if input.just_pressed(KeyCode::Space) {
        ev_toggle.send(ToggleWeatherGridEvent);
        info!("Toggle weather grid event sent");
    }
}

fn handle_toggle_weather_grid(
    mut ev_toggle: EventReader<ToggleWeatherGridEvent>,
    mut query: Query<(&mut Visibility, &IndexX, &IndexY)>,
) {
    for _ in ev_toggle.read() {
        for (mut visibility, _, _) in &mut query {
            *visibility = if *visibility == Visibility::Hidden {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            
            info!("Weather grid visibility toggled: {:?}", visibility);
        }

    }
}