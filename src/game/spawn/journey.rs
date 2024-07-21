use bevy::math;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;

use super::weather::DayWeather;
use super::weather::Heat;
use super::weather::Moisture;
use super::weather::Wind;

#[derive(Event, Debug)]
pub struct NextDay(DayTask);

#[derive(Event, Debug)]
pub struct CreateJourney;

const JOURNEY_LENGTH: usize = 180;

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
}

#[derive(Debug)]
enum DayTask {
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
        self.event = DayEvent::Sailing;
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

enum ShipCondition {
    Perfect,
    Cosmetic,
    Battered,
    Tattered,
    Immobilised,
    Sunk,
}

#[derive(Resource)]
struct Ship {
    crew: u32,
    food: u32,
    morale: u32,
    ship_condition: ShipCondition,
}

fn create_journey(_trigger: Trigger<CreateJourney>, mut commands: Commands) {
    commands.insert_resource(Journey::generate(120.0, None, None));
    commands.insert_resource(Ship {
        crew: 20,
        food: 50,
        morale: 100,
        ship_condition: ShipCondition::Perfect,
    });
}

fn next_day(trigger: Trigger<NextDay>, mut journey: ResMut<Journey>, mut ship: ResMut<Ship>) {
    let mut hardship: u32 = 0;
    let mut danger: u32 = 0;
    let mut speed: u32 = 0;
    let mut abundance: u32 = 0;

    match trigger.event().0 {
        DayTask::Sail => match journey.weather.wind {
            Wind::None => {
                journey.distance += 10.0;
                ship.food -= ship.crew;
            }
            Wind::Low => {
                journey.distance += 25.0;
                ship.food -= (ship.crew as f32 * 1.2).floor() as u32;
            }
            Wind::Medium => todo!(),
            Wind::High => todo!(),
            Wind::GaleForce => todo!(),
        },
        DayTask::Fight => todo!(),
        DayTask::Explore => todo!(),
        DayTask::Rest => todo!(),
        DayTask::HunkerDown => todo!(),
    }
    journey.new_day();
}

pub fn plugin(app: &mut App) {
    app.observe(create_journey);
    app.observe(next_day);
}
