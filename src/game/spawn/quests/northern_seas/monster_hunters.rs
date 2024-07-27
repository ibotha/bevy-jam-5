use crate::game::spawn::quests::sea_events::sail;

use super::super::{prelude::*, sirens_cove::to_sirens_cove};

fn investigate(actions: &mut StoryActions) {}

fn navy_base(actions: &mut StoryActions) {}

fn mysterious_island(actions: &mut StoryActions) {}

pub fn monster_hunters(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("A ship Captain!"))
        .line(captain!("Do you reckognise the colours?"))
        .line(crew1!("No sir, looks like a kraken on their flag."))
        .choice("Investigate", investigate)
        .choice("Ignore", sail)
}
