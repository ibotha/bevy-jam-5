use std::collections::VecDeque;

use bevy::math;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use log::info;
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;

use crate::game::ui::UpdateInventoryList;
use crate::game::weighted_random;
use crate::screen::Screen;
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

use super::predicitons::Predictions;
use super::predicitons::UpdateParrotUi;
use super::quests::treasure::Item;
use super::quests::Certainty;
use super::quests::FollowingEvent;
use super::quests::StoryActions;
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
    pub events: Vec<FollowingEvent>,
    pub distance: i32,
    pub event: Option<DayEvent>,
    pub weather: DayWeather, // Can use this variable to grab the predicted weather for the coming day
    current_day: u32,
    moisture_cycle_length: u32,
    heat_cycle_length: u32,
    wind_cycle_length: u32,
    pub rng: StdRng,
    difficulty: f32,
    journey_length: u32, // How many days until max difficulty
    pub inventory: HashMap<Item, i32>,
    pub environment: Environment,
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
            inventory: Default::default(),
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
    mut commands: Commands,
    mut dialog_queue: ResMut<DialogueQueue>,
    mut journey: ResMut<Journey>,
) {
    journey.event = Some(trigger.event().0.clone());
    for d in &trigger.event().0.dialog {
        dialog_queue.queue.push_back(d.clone());
    }
    commands.trigger(UpdateParrotUi);
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

    let mut updates: Vec<String> = vec![];
    journey.event.as_ref().expect("choice is valid").choices[&trigger.event().0](
        &mut StoryActions::new(&mut ship, &mut journey, &mut dialog_queue, &mut updates),
    );
    journey.event = None;

    let food_consumption = ship.crew / 2 + 1;
    updates.push(format!(
        "Your crew ate {count} food.",
        count = ship.food.min(food_consumption)
    ));
    ship.food -= food_consumption;
    if ship.food < 0 {
        updates.push(format!(
            "You lost {crew} crew to hunger!",
            crew = -ship.food
        ));
        ship.crew += ship.food;
        ship.food = 0;
    }

    for strings in updates.as_slice().chunks(4) {
        dialog_queue
            .queue
            .push_back(Dialogue::new("Updates").paras(strings));
    }

    if ship.crew <= 0 {
        info!("GAME OVER!");
        journey.game_over = true;
        dialog_queue.queue.push_back(
            Dialogue::new("Game Over")
                .para("I have no crew left!")
                .para("I followed your advice to the letter sage...")
                .para("You're a fraud!"),
        );
    } else if ship.health <= 0 {
        info!("GAME OVER!");
        journey.game_over = true;
        dialog_queue.queue.push_back(
            Dialogue::new("Game Over")
                .para("A captain always goes down with his ship...")
                .para("To hell with that! I'm getting in the last rowboat!"),
        );
    }

    commands.trigger(UpdateShipStatsUI);
    commands.trigger(UpdateInventoryList);
    commands.trigger(Continue);
}

fn continue_journey(
    _trigger: Trigger<Continue>,
    mut commands: Commands,
    mut dialoges: ResMut<DialogueQueue>,
    mut next_screen: ResMut<NextState<Screen>>,
    journey: Res<Journey>,
) {
    match dialoges.queue.pop_front() {
        Some(dialogue) => {
            commands.trigger(UpdateDialogBox(dialogue));
        }
        None => {
            if journey.game_over {
                next_screen.set(Screen::Credits)
            }
        }
    }

    if journey.game_over {
        return;
    }

    if dialoges.queue.is_empty() {
        match journey.event.as_ref() {
            Some(e) => {
                commands.trigger(UpdateChoices(e.choices.keys().cloned().collect()));
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

fn next_day(
    _trigger: Trigger<NextDay>,
    mut commands: Commands,
    mut journey: ResMut<Journey>,
    mut ship: ResMut<Ship>,
    mut dialoges: ResMut<DialogueQueue>,
    mut predictions: ResMut<Predictions>,
) {
    *predictions = Predictions {
        ..Default::default()
    };
    journey.generate_new_weather();

    commands.trigger(UpdateBoneGrid(match journey.rng.gen_range(0..3) {
        0 => AnyWeather::Moisture(journey.weather.moisture),
        1 => AnyWeather::Heat(journey.weather.heat),
        _ => AnyWeather::Wind(journey.weather.wind),
    }));
    commands.trigger(UpdateShipStatsUI);

    let env = journey.environment;

    let certain_events: Vec<DayEvent> = journey
        .events
        .iter()
        .filter(|e| e.environment == env && e.distance < 0 && e.certainty == Certainty::Certain)
        .map(|e| e.event.clone())
        .collect();

    let event = if !certain_events.is_empty() {
        let event = certain_events[journey.rng.gen_range(0..certain_events.len())].clone();
        let index = journey
            .events
            .iter()
            .position(|e| e.event == event)
            .expect("Event is present");
        journey.events.remove(index);
        info!("{events:?}", events = journey.events);
        event
    } else {
        let mut potential_events: Vec<(DayEvent, u32)> = journey
            .events
            .iter()
            .filter(|e| e.environment == env && e.distance < 0)
            .map(|e| {
                (
                    e.event.clone(),
                    match e.certainty {
                        Certainty::Possible(p) => p,
                        _ => 10,
                    },
                )
            })
            .collect();

        let mut updates: Vec<String> = vec![];
        potential_events.push((
            select_random_event(&mut StoryActions::new(
                ship.as_mut(),
                journey.as_mut(),
                dialoges.as_mut(),
                &mut updates,
            )),
            10,
        ));
        info!("selecting from random events! {potential_events:?}");
        weighted_random(Some(&mut journey.rng), &potential_events).clone()
    };

    commands.trigger(SetJouneyEvent(event));
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
