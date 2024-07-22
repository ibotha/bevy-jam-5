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
    info!("Generating journey...");
    let journey = Journey::generate(120.0, None, None);
    commands.insert_resource(journey);
    commands.insert_resource(Ship {
        crew: 20,
        food: 50,
        morale: 100,
        ship_condition: ShipCondition::Perfect,
    });
    commands.trigger(NextDay(DayTask::Sail));
}

fn next_day(
    trigger: Trigger<NextDay>,
    mut journey: ResMut<Journey>,
    mut ship: ResMut<Ship>,
    mut commands: Commands,
) {
    let mut hardship: u32 = 0;
    let mut danger: u32 = 0;
    let mut speed: u32 = 0;
    let mut abundance: u32 = 0;

    info!(
        "You chose to {choice:?}: The weather was {weather:?}",
        choice = trigger.event().0,
        weather = journey.weather
    );
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
