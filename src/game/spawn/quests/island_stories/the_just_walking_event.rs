use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn rest(actions: &mut StoryActions) {
    actions.delta_crew(10);
}

pub fn the_just_walking_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(captain!("Just more and more jungle."))
        .choice("Rest", rest)
}