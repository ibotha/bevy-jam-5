use crate::game::spawn::quests::prelude::*;

fn walk(actions: &mut StoryActions) {
    let pdist = actions.possible_distance();
    actions.travel(pdist)
}

fn leave(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(actions.get_current_sea()));
}

pub fn island_stories_base(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .choice("Walk", walk)
        .choice("Leave", leave)
}

