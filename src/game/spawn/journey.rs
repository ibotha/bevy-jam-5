use std::collections::VecDeque;

use bevy::math;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;

use crate::game::spawn::weather::AnyWeather;
use crate::game::ui::Dialogue;
use crate::game::ui::ShowContinue;
use crate::game::ui::UpdateChoices;
use crate::game::ui::UpdateDialogBox;
use crate::game::ui::UpdateShipStatsUI;
use crate::game::ui::{Focus, FocusedDisplay};
use crate::game::weighted_random;
use crate::screen::weather_maniac::UpdateBoneGrid;

use super::weather::DayWeather;
use super::weather::Heat;
use super::weather::Moisture;
use super::weather::Wind;

#[derive(Event, Debug)]
pub struct ChooseTask(pub DayTask);

#[derive(Event, Debug)]
pub struct NextDay;

#[derive(Event, Debug)]
pub struct Continue;

#[derive(Event, Debug)]
pub struct CreateJourney;

#[derive(Debug, PartialEq)]
enum DayEvent {
    Sailing,
    Treasure,
    Island,
    Whale,
}

#[derive(Resource, Debug)]
struct DialogueQueue {
    queue: VecDeque<Dialogue>,
}

#[derive(Resource, Debug, PartialEq)]
pub struct Journey {
    game_over: bool,
    pub weather: DayWeather, // Can use this variable to grab the predicted weather for the coming day
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

    CleanDaDeck,
    CookDaFood,
    Gamble,

    Fight,
    Explore,
    Rest,
    HunkerDown,
}

const CAPTAIN: &str = "Captain";
impl Journey {
    pub(super) fn generate(distance: f32, difficulty: Option<f32>, seed: Option<u64>) -> Self {
        let mut rng = StdRng::seed_from_u64(seed.unwrap_or(thread_rng().next_u64()));
        const AVERAGE_DAILY_DISTANCE: f32 = 5.0;
        let difficulty = difficulty.unwrap_or(10.0);
        Self {
            game_over: false,
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

        // TODO: Select specific days from longer quests
        // Select from the pool of random daily events
        self.event = weighted_random(
            Some(&mut self.rng),
            vec![
                (DayEvent::Sailing, 14),
                (DayEvent::Treasure, 2),
                (DayEvent::Island, 8),
                (DayEvent::Whale, 1),
            ],
        );
    }

    fn get_options(&self) -> (Dialogue, Vec<DayTask>) {
        match self.event {
            DayEvent::Sailing => (
                Dialogue {
                    speaker: CAPTAIN.to_string(),
                    paragraphs: vec![
                        "Looks like plain sailing today, anything we might want to look out for?"
                            .to_string(),
                    ],
                },
                vec![
                    DayTask::Sail,
                    DayTask::CleanDaDeck,
                    DayTask::CookDaFood,
                    DayTask::HunkerDown,
                    DayTask::Rest,
                ],
            ),
            DayEvent::Treasure => (
                Dialogue {
                    speaker: CAPTAIN.to_string(),
                    paragraphs: vec![
                        "Looks like there's a sinkin' ship! I smell gold...".to_string()
                    ],
                },
                vec![
                    DayTask::Sail,
                    DayTask::Gamble,
                    DayTask::Explore,
                    DayTask::Rest,
                ],
            ),
            DayEvent::Island => (
                Dialogue {
                    speaker: CAPTAIN.to_string(),
                    paragraphs: vec![
                        "Land Ho! Should we go see what's there and re-supply?".to_string()
                    ],
                },
                vec![DayTask::Sail, DayTask::Explore, DayTask::Rest],
            ),
            DayEvent::Whale => (
                Dialogue {
                    speaker: CAPTAIN.to_string(),
                    paragraphs: vec!["Shiver me timbers...Is that a whale!?!".to_string()],
                },
                vec![
                    DayTask::Sail,
                    DayTask::CookDaFood,
                    DayTask::Explore,
                    DayTask::Rest,
                    DayTask::HunkerDown,
                    DayTask::Fight,
                ],
            ),
        }
    }
}

#[derive(Resource)]
pub struct Ship {
    pub crew: i32,
    pub max_crew: i32,
    pub food: i32,
    pub max_food: i32,
    pub health: i32,
    pub max_health: i32,
}

fn create_journey(
    _trigger: Trigger<CreateJourney>,
    mut commands: Commands,
    mut dialog_queue: ResMut<DialogueQueue>,
) {
    info!("Generating journey...");
    let journey = Journey::generate(120.0, None, None);
    commands.insert_resource(journey);
    commands.insert_resource(Ship {
        crew: 5,
        max_crew: 5,
        food: 100,
        max_food: 100,
        health: 100,
        max_health: 100,
    });
    dialog_queue.queue.push_back(Dialogue {
        speaker: CAPTAIN.to_string(),
        paragraphs: vec!["You are about to embark on a magical journey!".to_string()],
    });
    commands.trigger(Continue);
}

const MONSOON: DayWeather = DayWeather {
    wind: Wind::GaleForce,
    heat: Heat::Freezing,
    moisture: Moisture::Humid,
};
const BLISTERING_DEATH: DayWeather = DayWeather {
    wind: Wind::None,
    heat: Heat::Blistering,
    moisture: Moisture::Dry,
};
const FREEZING_HELL: DayWeather = DayWeather {
    wind: Wind::None,
    heat: Heat::Freezing,
    moisture: Moisture::Dry,
};
const FREEZING_DEATH: DayWeather = DayWeather {
    wind: Wind::GaleForce,
    heat: Heat::Freezing,
    moisture: Moisture::Dry,
};
const SCORCHING_DESERT: DayWeather = DayWeather {
    wind: Wind::Low,
    heat: Heat::Blistering,
    moisture: Moisture::Dry,
};
const HUMID_SWAMP: DayWeather = DayWeather {
    wind: Wind::Low,
    heat: Heat::Blistering,
    moisture: Moisture::Humid,
};
const MILD_SUMMER: DayWeather = DayWeather {
    wind: Wind::Medium,
    heat: Heat::Warm,
    moisture: Moisture::Dry,
};
const COLD_FRONT: DayWeather = DayWeather {
    wind: Wind::High,
    heat: Heat::Chilly,
    moisture: Moisture::Dry,
};
const TROPICAL_STORM: DayWeather = DayWeather {
    wind: Wind::GaleForce,
    heat: Heat::Warm,
    moisture: Moisture::Humid,
};
const AUTUMN_BREEZE: DayWeather = DayWeather {
    wind: Wind::Medium,
    heat: Heat::Chilly,
    moisture: Moisture::Dry,
};
const SPRING_SHOWER: DayWeather = DayWeather {
    wind: Wind::Low,
    heat: Heat::Warm,
    moisture: Moisture::Humid,
};
const COOL_DRIZZLE: DayWeather = DayWeather {
    wind: Wind::Medium,
    heat: Heat::Chilly,
    moisture: Moisture::Humid,
};
fn choose_task(
    trigger: Trigger<ChooseTask>,
    mut commands: Commands,
    mut journey: ResMut<Journey>,
    mut ship: ResMut<Ship>,
) {
    commands.trigger(UpdateChoices(vec![]));
    commands.trigger(Focus(FocusedDisplay::Dialogue));
    if journey.game_over {
        return;
    }
    let mut hardship: i32 = 1;
    let mut danger: i32 = 0;
    let mut speed = match journey.weather.wind {
        Wind::None => 0,
        Wind::Low => 1,
        Wind::Medium => 2,
        Wind::High => 3,
        Wind::GaleForce => 4,
    };

    let mut abundance = match journey.weather.heat {
        Heat::Blistering => 1,
        Heat::Warm => 2,
        Heat::Comfortable => 3,
        Heat::Chilly => 2,
        Heat::Freezing => 1,
    };
    match journey.weather {
        BLISTERING_DEATH => {
            hardship = 3;
            danger = 3;
        }
        MONSOON => {
            hardship += 4;
            danger += 3;
        }
        FREEZING_HELL => {
            hardship += 1;
            danger += 1;
        }
        FREEZING_DEATH => {
            hardship += 4;
            danger += 3;
        }
        SCORCHING_DESERT => {
            hardship += 1;
            danger += 1;
        }
        HUMID_SWAMP => {
            hardship += 2;
            danger += 1;
        }
        MILD_SUMMER => {
            hardship -= 1;
            danger -= 1;
        }
        COLD_FRONT => {
            hardship += 1;
            danger += 1;
        }
        TROPICAL_STORM => {
            if rand::thread_rng().gen_range(0..100) < 10 {
                speed = -speed;
            }
            hardship += 2;
            danger += 1;
        }
        AUTUMN_BREEZE => {
            hardship -= 1;
            danger -= 1;
        }
        SPRING_SHOWER => {
            hardship -= 1;
            danger -= 1;
        }
        COOL_DRIZZLE => {
            hardship -= 1;
            danger -= 1;
        }
        _ => {}
    }

    match trigger.event().0 {
        DayTask::Sail => {
            journey.distance += 2.0 * (speed as f32);
            ship.health -= danger;
        }
        DayTask::Fight => {
            ship.crew -= danger;
            journey.treasure += 10;
        }
        DayTask::CookDaFood => {
            ship.food += ship.crew * abundance;
        }
        DayTask::CleanDaDeck => {}
        DayTask::Gamble => {
            journey.treasure += 10;
        }
        DayTask::Explore => {
            journey.treasure += 10;
            ship.crew -= danger;
        }
        DayTask::Rest => {
            ship.crew += 1 * abundance;
        }
        DayTask::HunkerDown => {}
    }

    ship.crew = ship.crew.min(ship.max_crew);
    ship.food = ship.food.min(ship.max_food);
    ship.health = ship.health.min(ship.max_health);

    ship.food -= ship.crew * hardship;
    if ship.food < 0 {
        ship.crew += ship.food;
        ship.food = 0;
    }
    if ship.crew <= 0 {
        info!("GAME OVER!");
        journey.game_over = true;
        commands.trigger(UpdateDialogBox(Dialogue {
            speaker: "Game Over".to_string(),
            paragraphs: vec![
                "Mutiny!".to_string(),
                "I followed your advice to the letter sage...".to_string(),
                "You're a fraud!".to_string(),
            ],
        }));
        return;
    }
    if ship.health <= 0 {
        info!("GAME OVER!");
        journey.game_over = true;
        commands.trigger(UpdateDialogBox(Dialogue {
            speaker: "Game Over".to_string(),
            paragraphs: vec![
                "A captain always goes down with his ship...".to_string(),
                "To hell with that! I'm getting in the last rowboat!".to_string(),
            ],
        }));
        return;
    }
    if journey.distance >= journey.total_distance {
        info!("You Win!");
        journey.game_over = true;
        commands.trigger(UpdateDialogBox(Dialogue {
            speaker: "Tortuga".to_string(),
            paragraphs: vec![
                "Looks like we are back home, with a ship full of loot at that!".to_string(),
                format!("You have {treasure} treasure!", treasure = journey.treasure),
            ],
        }));
        return;
    }
    commands.trigger(UpdateDialogBox(Dialogue {
        speaker: "Night Fall".to_string(),
        paragraphs: vec![
            "You'll look at this then carry on!".to_string(),
            format!("You have {treasure} treasure!", treasure = journey.treasure),
            format!(
                "You have {distance} leagues left to go!",
                distance = journey.total_distance - journey.distance
            ),
        ],
    }));
    commands.trigger(ShowContinue(true));
    commands.trigger(UpdateShipStatsUI);
}

fn continue_journey(
    _trigger: Trigger<Continue>,
    mut commands: Commands,
    mut dialoges: ResMut<DialogueQueue>,
    mut ship: ResMut<Ship>,
) {
    match dialoges.queue.pop_front() {
        Some(dialogue) => {
            commands.trigger(UpdateDialogBox(dialogue));
        }
        None => {
            commands.trigger(NextDay);
            commands.trigger(ShowContinue(false));
        }
    }
}

fn next_day(_trigger: Trigger<NextDay>, mut commands: Commands, mut journey: ResMut<Journey>) {
    journey.new_day();
    let (dialog, choices) = journey.get_options();
    commands.trigger(UpdateChoices(choices));

    commands.trigger(UpdateDialogBox(dialog));
    commands.trigger(UpdateBoneGrid(match journey.rng.gen_range(0..3) {
        0 => AnyWeather::Moisture(journey.weather.moisture),
        1 => AnyWeather::Heat(journey.weather.heat),
        _ => AnyWeather::Wind(journey.weather.wind),
    }));
    commands.trigger(UpdateShipStatsUI);
}

pub fn plugin(app: &mut App) {
    app.observe(create_journey)
        .add_event::<ChooseTask>()
        .add_event::<CreateJourney>()
        .insert_resource(DialogueQueue {
            queue: VecDeque::new(),
        })
        .observe(choose_task)
        .observe(continue_journey)
        .observe(next_day);
}
