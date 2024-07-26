use std::collections::VecDeque;

use day_event::DayEvent;
use dialogue::Dialogue;
use rand::RngCore;
use sea_events::select_random_sea_event;

use super::{journey::Ship, weather::DayWeather};

mod constants;
pub mod day_event;
pub mod dialogue;
mod island_events;
mod port_events;
mod sea_events;
pub mod unique_events;

#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Port,
    Island,
    Sea,
}

#[derive(Debug)]
/// Likelyhood of a future event happening
pub enum Certainty {
    /// The event will happen as soon as the distance is covered
    Certain,
    /// The event might happen once the distance is covered, with a given weight
    /// 10 is average,
    Possible(u32),
}

#[derive(Debug)]
pub struct FollowingEvent {
    pub event: DayEvent,
    pub distance: i32,
    pub certainty: Certainty,
    pub environment: Environment,
}

pub struct ChoiceResult {
    pub ship: Ship,
    pub following_events: Vec<FollowingEvent>,
    pub dialogues: VecDeque<Dialogue>,
}

pub type ChoiceFunction = fn(Ship, &DayWeather) -> ChoiceResult;

pub fn select_random_event(rng: &mut impl RngCore, t: Environment) -> DayEvent {
    match t {
        Environment::Port => todo!(),
        Environment::Island => todo!(),
        Environment::Sea => select_random_sea_event(rng),
    }
}
