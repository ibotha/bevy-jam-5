use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

pub fn the_just_walking_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions).line(captain!("Just more and more jungle."))
}

