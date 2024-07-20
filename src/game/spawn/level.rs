//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Event, Debug)]
pub struct NextDay(DayTask);

const JOURNEY_LENGTH: usize = 180;

enum Moisture {
    Dry,
    Comfortable,
    Humid,
}

enum Heat {
    Blistering,
    Warm,
    Comfortable,
    Chilly,
    Freezing,
}

enum Rain {
    None,
    Drizzle,
    Rain,
    Storm,
}

struct DayWeather {
    rain: Rain,
    heat: Heat,
    moisture: Moisture,
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
struct Journey {
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

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.insert_resource(Journey::generate());
    commands.trigger(SpawnPlayer);
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
