use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct NextDay(DayTask);

#[derive(Event, Debug)]
pub struct CreateJourney;

const JOURNEY_LENGTH: usize = 180;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum Moisture {
    Dry,
    Comfortable,
    Humid,
}

impl Moisture {
    fn generate() -> Self {
        let none_ballots = 10;
        let drizzle_ballots = 10;
        let rain_ballots = 10;
        let storm_ballots = 10;
        Moisture::Comfortable
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum Heat {
    Blistering,
    Warm,
    Comfortable,
    Chilly,
    Freezing,
}

impl Heat {
    fn generate() -> Self {
        let none_ballots = 10;
        let drizzle_ballots = 10;
        let rain_ballots = 10;
        let storm_ballots = 10;
        Heat::Comfortable
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum Wind {
    None,
    Low,
    Medium,
    High,
    GaleForce,
}

impl Wind {
    fn generate() -> Self {
        let none_ballots = 10;
        let drizzle_ballots = 10;
        let rain_ballots = 10;
        let storm_ballots = 10;
        Wind::None
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum AnyWeather {
    Heat(Heat),
    Moisture(Moisture),
    Wind(Wind),
}

struct DayWeather {
    wind: Wind,
    heat: Heat,
    moisture: Moisture,
}

impl DayWeather {
    fn generate() -> Self {
        Self {
            wind: Wind::generate(),
            heat: Heat::generate(),
            moisture: Moisture::generate(),
        }
    }
}

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
    distance: u32,
    total_distance: u32,
    current_day: u32,
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
    pub(super) fn generate() -> Self {
        Self {
            weather: DayWeather::generate(),
            event: DayEvent::Sailing,
            distance: 0,
            total_distance: 120,
            current_day: 0,
        }
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
    commands.insert_resource(Journey::generate());
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
                journey.distance += 10;
                ship.food -= ship.crew;
            }
            Wind::Low => {
                journey.distance += 25;
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
    journey.current_day += 1;
    journey.weather = DayWeather::generate();
}

pub fn plugin(app: &mut App) {
    app.observe(create_journey);
    app.observe(next_day);
}
