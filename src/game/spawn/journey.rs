use std::collections::VecDeque;

use bevy::math;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;

use crate::game::spawn::quests::ChoiceResult;
use crate::{
    game::{
        spawn::{
            quests::{dialogue::Dialogue, unique_events::embark_event},
            weather::AnyWeather,
        },
        ui::{
            Focus, FocusedDisplay, ShowContinue, UpdateChoices, UpdateDialogBox, UpdateShipStatsUI,
        },
    },
    screen::weather_maniac::UpdateBoneGrid,
};

use super::quests::FollowingEvent;
use super::{
    quests::{day_event::DayEvent, dialogue::DialogueQueue, select_random_event, Environment},
    weather::{DayWeather, Heat, Moisture, Wind},
};

#[derive(Event, Debug)]
pub struct ChooseTask(pub String);

#[derive(Event, Debug)]
pub struct NextDay;

#[derive(Event, Debug)]
pub struct Continue;

#[derive(Event, Debug)]
pub struct CreateJourney;

#[derive(Resource, Debug)]
pub struct Journey {
    game_over: bool,
    weather: DayWeather,
    event: Option<DayEvent>,
    events: Vec<FollowingEvent>,
    distance: i32,
    current_day: u32,
    moisture_cycle_length: u32,
    heat_cycle_length: u32,
    wind_cycle_length: u32,
    rng: StdRng,
    difficulty: f32,
    journey_length: u32, // How many days until max difficulty
    environment: Environment,
}

impl Journey {
    pub(super) fn generate(distance: f32, difficulty: Option<f32>, seed: Option<u64>) -> Self {
        let mut rng = StdRng::seed_from_u64(seed.unwrap_or(thread_rng().next_u64()));
        const AVERAGE_DAILY_DISTANCE: f32 = 5.0;
        let difficulty = difficulty.unwrap_or(10.0);
        Self {
            game_over: false,
            weather: DayWeather::default(),
            event: None,
            distance: 0,
            current_day: 0,
            moisture_cycle_length: rng.gen_range(30..50),
            heat_cycle_length: rng.gen_range(60..120),
            wind_cycle_length: rng.gen_range(15..25),
            rng,
            difficulty,
            events: vec![],
            journey_length: (distance * ((difficulty - 10.0) * -0.1 + 1.0) / AVERAGE_DAILY_DISTANCE)
                as u32,
            environment: Environment::Sea,
        }
    }

    /// Move on the the next day
    fn generate_new_weather(&mut self) {
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
    }
}

#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Ship {
    pub crew: i32,
    pub max_crew: i32,
    pub food: i32,
    pub max_food: i32,
    pub health: i32,
    pub max_health: i32,
    pub distance_travelled: i32,
}

fn create_journey(_trigger: Trigger<CreateJourney>, mut commands: Commands) {
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
        distance_travelled: 0,
    });
    commands.trigger(SetJouneyEvent(embark_event()));
    commands.trigger(Continue);
}

#[derive(Event, Debug)]
struct SetJouneyEvent(DayEvent);

fn set_journey_event(
    trigger: Trigger<SetJouneyEvent>,
    mut dialog_queue: ResMut<DialogueQueue>,
    mut journey: ResMut<Journey>,
) {
    journey.event = Some(trigger.event().0.clone());
    for d in &trigger.event().0.dialog {
        dialog_queue.queue.push_back(d.clone());
    }
}

fn choose_task(
    trigger: Trigger<ChooseTask>,
    mut commands: Commands,
    mut journey: ResMut<Journey>,
    mut dialog_queue: ResMut<DialogueQueue>,
    mut ship: ResMut<Ship>,
) {
    commands.trigger(UpdateChoices(vec![]));
    commands.trigger(Focus(FocusedDisplay::Dialogue));
    if journey.game_over {
        return;
    }

    let ChoiceResult {
        ship: mut updated_ship,
        mut following_events,
        mut dialogues,
    } = journey.event.as_ref().expect("choice is valid").choices[&trigger.event().0](
        ship.clone(),
        &journey.weather,
    );
    journey.event = None;

    journey.events.append(&mut following_events);

    updated_ship.crew = updated_ship.crew.min(updated_ship.max_crew);
    updated_ship.food = updated_ship.food.min(updated_ship.max_food);
    updated_ship.health = updated_ship.health.min(updated_ship.max_health);

    updated_ship.food -= updated_ship.crew;
    if updated_ship.food < 0 {
        updated_ship.crew += updated_ship.food;
        updated_ship.food = 0;
    }

    dialog_queue.queue.append(&mut dialogues);

    let mut updates: Vec<String> = vec![];

    if updated_ship.crew != ship.crew {
        updates.push(diff_readout(
            ship.crew,
            updated_ship.crew,
            "crew member",
            true,
        ));
    }
    if updated_ship.max_crew != ship.max_crew {
        updates.push(diff_readout(
            ship.max_crew,
            updated_ship.max_crew,
            "crew member capacity",
            false,
        ));
    }
    if updated_ship.health != ship.health {
        updates.push(damage_diff_readout(ship.health, updated_ship.health));
    }
    if updated_ship.max_health != ship.max_health {
        updates.push(diff_readout(
            ship.max_health,
            updated_ship.max_health,
            "max ship health",
            false,
        ));
    }
    if updated_ship.distance_travelled != 0 {
        updates.push(format!(
            "You covered {leagues} leagues.",
            leagues = updated_ship.distance_travelled
        ));
    }
    if updated_ship.food != ship.food {
        updates.push(diff_readout(ship.food, updated_ship.food, "food", false));
    }
    if updated_ship.max_food != ship.max_food {
        updates.push(diff_readout(
            ship.max_food,
            updated_ship.max_food,
            "food capacity",
            false,
        ));
    }

    for strings in updates.as_slice().chunks(5) {
        dialog_queue.queue.push_back(Dialogue::new_from_strings(
            "Updates",
            strings.iter().map(|s| s.to_owned()),
        ));
    }

    for event in &mut journey.events {
        event.distance -= updated_ship.distance_travelled;
    }

    journey.distance += updated_ship.distance_travelled;
    updated_ship.distance_travelled = 0;
    *ship = updated_ship;

    if ship.crew <= 0 {
        info!("GAME OVER!");
        journey.game_over = true;
        dialog_queue.queue.push_back(Dialogue::new(
            "Game Over",
            &[
                "I have no crew left!",
                "I followed your advice to the letter sage...",
                "You're a fraud!",
            ],
        ));
    } else if ship.health <= 0 {
        info!("GAME OVER!");
        journey.game_over = true;
        dialog_queue.queue.push_back(Dialogue::new(
            "Game Over",
            &[
                "A captain always goes down with his ship...",
                "To hell with that! I'm getting in the last rowboat!",
            ],
        ));
    }

    commands.trigger(UpdateShipStatsUI);
    commands.trigger(Continue);
}

fn diff_readout(before: i32, after: i32, unit: &str, pluralize: bool) -> String {
    base_diff_readout(before, after, ("gained", "lossed"), unit, pluralize)
}

fn damage_diff_readout(before: i32, after: i32) -> String {
    base_diff_readout(before, after, ("restored", "took"), "damage", false)
}

fn base_diff_readout(
    before: i32,
    after: i32,
    verbs: (&str, &str),
    unit: &str,
    pluralize: bool,
) -> String {
    let diff = (before - after).abs();
    format!(
        "You {verb} {diff} {unit}{pluralize}",
        verb = if before < after { verbs.0 } else { verbs.1 },
        pluralize = if pluralize && (diff != 1) { "s" } else { "" }
    )
}

fn continue_journey(
    _trigger: Trigger<Continue>,
    mut commands: Commands,
    mut dialoges: ResMut<DialogueQueue>,
    journey: Res<Journey>,
) {
    info!("Continuing: {dialoges:?}");
    match dialoges.queue.pop_front() {
        Some(dialogue) => {
            commands.trigger(UpdateDialogBox(dialogue));
        }
        None => {}
    }

    if dialoges.queue.len() == 0 {
        match journey.event.as_ref() {
            Some(e) => {
                commands.trigger(UpdateChoices(e.choices.keys().map(|s| s.clone()).collect()));
                commands.trigger(ShowContinue(false));
            }
            None => {
                commands.trigger(ShowContinue(true));
                commands.trigger(NextDay);
            }
        }
    } else {
        commands.trigger(ShowContinue(true));
    }
}

fn next_day(_trigger: Trigger<NextDay>, mut commands: Commands, mut journey: ResMut<Journey>) {
    journey.generate_new_weather();

    commands.trigger(UpdateBoneGrid(match journey.rng.gen_range(0..3) {
        0 => AnyWeather::Moisture(journey.weather.moisture),
        1 => AnyWeather::Heat(journey.weather.heat),
        _ => AnyWeather::Wind(journey.weather.wind),
    }));
    commands.trigger(UpdateShipStatsUI);

    let env = journey.environment;
    commands.trigger(SetJouneyEvent(select_random_event(&mut journey.rng, env)));
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
        .observe(set_journey_event)
        .observe(next_day);
}
