use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct NextDay(DayTask);

#[derive(Event, Debug)]
pub struct CreateJourney;

const JOURNEY_LENGTH: usize = 180;

enum Moisture {
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

enum Heat {
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

enum Rain {
    None,
    Drizzle,
    Rain,
    Storm,
}

impl Rain {
    fn generate() -> Self {
        let none_ballots = 10;
        let drizzle_ballots = 10;
        let rain_ballots = 10;
        let storm_ballots = 10;
        Rain::None
    }
}

struct DayWeather {
    rain: Rain,
    heat: Heat,
    moisture: Moisture,
}

impl DayWeather {
    fn generate() -> Self {
        Self {
            rain: Rain::generate(),
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

struct Day {
    event: DayEvent,
    weather: DayWeather,
}

#[derive(Resource)]
pub struct Journey {
    days: Vec<Day>,
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
        let mut days: Vec<Day> = vec![];
        days.reserve(JOURNEY_LENGTH);
        for day in 0..JOURNEY_LENGTH {
            days.push(Day {
                weather: DayWeather {
                    heat: Heat::Comfortable,
                    moisture: Moisture::Comfortable,
                    rain: Rain::None,
                },
                event: DayEvent::Sailing,
            });
        }

        Self {
            days,
            distance: 0,
            total_distance: 120,
            current_day: 0,
        }
    }

    fn get_options(&self) -> Vec<DayTask> {
        let today = &self.days[self.current_day as usize];
        match today.event {
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
    ship_condition: ShipCondition,
}

fn create_journey(trigger: Trigger<CreateJourney>, mut commands: Commands) {
    commands.insert_resource(Journey::generate());
    commands.insert_resource(Ship {
        crew: 20,
        food: 20,
        ship_condition: ShipCondition::Perfect,
    });
}

fn next_day(trigger: Trigger<NextDay>, mut journey: ResMut<Journey>, mut ship: ResMut<Ship>) {
    match trigger.event().0 {
        DayTask::Sail => todo!(),
        DayTask::Fight => todo!(),
        DayTask::Explore => todo!(),
        DayTask::Rest => todo!(),
        DayTask::HunkerDown => todo!(),
    }
    journey.current_day += 1;
}

pub fn plugin(app: &mut App) {
    app.observe(create_journey);
    app.observe(next_day);
}
