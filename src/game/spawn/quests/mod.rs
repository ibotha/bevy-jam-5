use day_event::DayEvent;
use island_events::select_random_island_event;
use port_events::select_random_port_event;
use sea_events::select_random_sea_event;

pub mod actions;
pub mod battle;
pub mod constants;
pub mod day_event;
pub mod dialogue;
mod island_events;
pub mod northern_seas;
mod port_events;
pub mod prelude;
mod sea_events;
pub mod sirens_cove;
pub mod treasure;
pub mod unique_events;

pub use actions::StoryActions;

// Import all port_events
pub mod port_stories;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Port {
    Random,
    Any,
    Tortuga,
    ShadyCove,
    EdgeOfTheWorld,
    RoyalNavyBase,
}

impl Port {
    fn compare(&self, other: &Self) -> bool {
        *self == Self::Any || *other == Self::Any || self == other
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Island {
    Random,
    Any,
    MysteriousIsland,
    SirensCove,
    TrinketSeller,
}

impl Island {
    fn compare(&self, other: &Self) -> bool {
        *self == Self::Any || *other == Self::Any || self == other
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sea {
    Intro,
    Any,
    Northern,
    SirensCove,
}

impl Sea {
    fn compare(&self, other: &Self) -> bool {
        *self == Self::Any || *other == Self::Any || self == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Port(Port),
    Island(Island),
    Sea(Sea),
}

impl Environment {
    pub fn compare(&self, other: &Self) -> bool {
        match (self, other) {
            (Environment::Sea(s), Environment::Sea(o)) => s.compare(o),
            (Environment::Island(s), Environment::Island(o)) => s.compare(o),
            (Environment::Port(s), Environment::Port(o)) => s.compare(o),
            _ => false,
        }
    }
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

#[derive(Debug, PartialEq, Eq)]
pub enum Delay {
    None,
    Days(i32),
    Distance(i32),
}
impl Delay {
    pub(crate) fn is_over(&self) -> bool {
        match self {
            crate::game::spawn::quests::Delay::None => true,
            crate::game::spawn::quests::Delay::Days(days) => *days <= 0,
            crate::game::spawn::quests::Delay::Distance(dis) => *dis <= 0,
        }
    }
}

#[derive(Debug)]
pub struct FollowingEvent {
    pub event: EventBuilder,
    pub delay: Delay,
    pub certainty: Certainty,
    pub environment: Environment,
}

pub type ChoiceFunction = fn(&mut StoryActions) -> ();
pub type EventBuilder = fn(&mut StoryActions) -> DayEvent;

pub fn select_random_event(actions: &mut StoryActions) -> EventBuilder {
    match actions.get_environment() {
        Environment::Port(p) => select_random_port_event(actions, p),
        Environment::Island(i) => select_random_island_event(actions, i),
        Environment::Sea(s) => select_random_sea_event(actions, s),
    }
}
