use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

pub fn the_plain_sailing_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
    .line(captain!(
        "Nothing on the horizon for today,",
        "Anything I should be aware of sage?"
    ))
}