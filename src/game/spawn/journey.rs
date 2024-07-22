use bevy::math;
use bevy::prelude::*;
use log::info;
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;

use crate::game::spawn::weather::AnyWeather;
use crate::game::ui::UpdateChoices;
use crate::screen::weather_maniac::UpdateBoneGrid;

use super::weather::DayWeather;
use super::weather::Heat;
use super::weather::Moisture;
use super::weather::Wind;

#[derive(Event, Debug)]
pub struct NextDay(pub DayTask);

#[derive(Event, Debug)]
pub struct CreateJourney;

#[derive(Debug)]
enum DayEvent {
    Sailing,
    Treasure,
    Island,
    Whale,
}

#[derive(Resource)]
pub struct Journey {
    weather: DayWeather,
    event: DayEvent,
    distance: f32,
    total_distance: f32,
    current_day: u32,
    moisture_cycle_length: u32,
    heat_cycle_length: u32,
    wind_cycle_length: u32,
    rng: StdRng,
    difficulty: f32,
    journey_length: u32, // How many days until max difficulty
    treasure: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Reflect)]
pub enum DayTask {
    Sail,
    Fight,
    Explore,
    Rest,
    HunkerDown,
}

impl Journey {
    pub(super) fn generate(distance: f32, difficulty: Option<f32>, seed: Option<u64>) -> Self {
        let mut rng = StdRng::seed_from_u64(seed.unwrap_or(thread_rng().next_u64()));
        const AVERAGE_DAILY_DISTANCE: f32 = 5.0;
        let difficulty = difficulty.unwrap_or(10.0);
        Self {
            weather: DayWeather::default(),
            event: DayEvent::Sailing,
            distance: 0.0,
            total_distance: distance,
            current_day: 0,
            treasure: 0,
            moisture_cycle_length: rng.gen_range(30..50),
            heat_cycle_length: rng.gen_range(60..120),
            wind_cycle_length: rng.gen_range(15..25),
            rng,
            difficulty,
            journey_length: (distance * ((difficulty - 10.0) * -0.1 + 1.0) / AVERAGE_DAILY_DISTANCE)
                as u32,
        }
    }

    /// Move on the the next day
    fn new_day(&mut self) {
        self.current_day += 1;
        let intensity = math::FloatExt::lerp(
            self.current_day as f32 / self.journey_length as f32,
            0.0,
            self.difficulty,
        );
        self.weather = DayWeather {
            moisture: Moisture::generate_from_cycle(
                &mut self.rng,
                intensity,
                self.moisture_cycle_length,
                self.current_day,
            ),
            heat: Heat::generate_from_cycle(
                &mut self.rng,
                intensity,
                self.heat_cycle_length,
                self.current_day,
            ),
            wind: Wind::generate_from_cycle(
                &mut self.rng,
                intensity,
                self.wind_cycle_length,
                self.current_day,
            ),
        };
        // TODO: Generate a new event for each day;
        let num = self.rng.gen_range(0..100);
        self.event = if num < 1 {
            DayEvent::Treasure
        } else if num < 2 {
            DayEvent::Whale
        } else if num < 5 {
            DayEvent::Island
        } else {
            DayEvent::Sailing
        };
    }

    fn get_options(&self) -> Vec<DayTask> {
        match self.event {
            DayEvent::Sailing => vec![DayTask::Sail, DayTask::HunkerDown, DayTask::Rest],
            DayEvent::Treasure => vec![DayTask::Sail, DayTask::Explore, DayTask::Rest],
            DayEvent::Island => vec![DayTask::Sail, DayTask::Explore, DayTask::Rest],
            DayEvent::Whale => vec![
                DayTask::Sail,
                DayTask::Explore,
                DayTask::Rest,
                DayTask::HunkerDown,
                DayTask::Fight,
            ],
        }
    }
}

#[derive(Resource)]
struct Ship {
    crew: i32,
    food: i32,
    ship_condition: i32,
}

fn create_journey(_trigger: Trigger<CreateJourney>, mut commands: Commands) {
    info!("Generating journey...");
    let journey = Journey::generate(120.0, None, None);
    commands.insert_resource(journey);
    commands.insert_resource(Ship {
        crew: 20,
        food: 50,
        ship_condition: 100,
    });
    commands.trigger(NextDay(DayTask::Sail));
}

fn next_day(
    trigger: Trigger<NextDay>,
    mut journey: ResMut<Journey>,
    mut ship: ResMut<Ship>,
    mut commands: Commands,
) {
    let hardship: f32;
    let danger: f32;
    let speed: f32;
    let abundance: f32;

    let DayWeather {
        wind,
        heat,
        moisture,
    } = journey.weather;
    match (wind, heat, moisture) {
        (wind, Heat::Comfortable, Moisture::Comfortable) => {
            speed = match wind {
                Wind::None => 0.0,
                Wind::Low => 2.0,
                Wind::Medium => 6.0,
                Wind::High => 8.0,
                Wind::GaleForce => 10.0,
            };
            hardship = 0.0;
            danger = 0.0;
            abundance = 10.0;
        }
        _ => {
            speed = 0.0;
            hardship = 10.0;
            danger = 10.0;
            abundance = 0.0;
        }
    }

    ship.food -= (ship.crew as f32 * hardship) as i32;
    if ship.food < 0 {
        ship.crew += ship.food;
        ship.food = 0;
    }
    match trigger.event().0 {
        DayTask::Sail => {
            journey.distance += 10.0 * speed;
            ship.ship_condition -= danger as i32;
        }
        DayTask::Fight => {
            ship.crew -= danger as i32;
            journey.treasure += 10;
        }
        DayTask::Explore => {
            journey.treasure += 10;
        }
        DayTask::Rest => {
            ship.crew += (10.0 * abundance) as i32;
            ship.food += (10.0 * abundance) as i32;
        }
        DayTask::HunkerDown => {
            ship.ship_condition += 10;
        }
    }
    info!(
        "You chose to {choice:?}: The weather was {weather:?}",
        choice = trigger.event().0,
        weather = journey.weather
    );
    if journey.distance > journey.total_distance {
        todo!("We need to handle them completing the journey!");
    }
    journey.new_day();
    info!(
        "Its a new day, the captain wants to {event:?}",
        event = journey.event
    );
    commands.trigger(UpdateChoices(journey.get_options()));

    commands.trigger(UpdateBoneGrid(match journey.rng.gen_range(0..3) {
        0 => AnyWeather::Moisture(journey.weather.moisture),
        1 => AnyWeather::Heat(journey.weather.heat),
        _ => AnyWeather::Wind(journey.weather.wind),
    }));
}

pub fn plugin(app: &mut App) {
    app.observe(create_journey)
        .add_event::<NextDay>()
        .add_event::<CreateJourney>()
        .observe(next_day);
}
