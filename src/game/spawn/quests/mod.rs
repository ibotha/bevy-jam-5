use day_event::DayEvent;
use island_events::select_random_island_event;
use port_events::select_random_port_event;
use sea_events::select_random_sea_event;

pub mod actions;
pub mod battle;
#[macro_use]
pub mod constants;
pub mod day_event;
pub mod dialogue;
mod island_events;
mod port_events;
pub mod prelude;
mod sea_events;
pub mod treasure;
pub mod unique_events;

pub use actions::StoryActions;

// Import all port_events
pub mod port_stories;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Port,
    Island,
    Sea,
}

#[derive(Debug, PartialEq, Eq)]
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

pub type ChoiceFunction = fn(&mut StoryActions) -> ();

pub fn select_random_event(actions: &mut StoryActions) -> DayEvent {
    match actions.get_environment() {
        Environment::Port => select_random_port_event(actions),
        Environment::Island => select_random_island_event(actions),
        Environment::Sea => select_random_sea_event(actions),
    }
}
