use crate::game::spawn::quests::{northern_seas::set_course_northern_sea, prelude::*};

fn walk(actions: &mut StoryActions) {
    let pdist = actions.possible_distance();
    actions.travel(pdist)
}

fn leave(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(actions.get_current_sea()));
}

pub fn island_stories_base(actions: &mut StoryActions) -> DayEvent {
    let e = if actions.get_current_sea() == Sea::Northern {
        set_course_northern_sea(actions)
    } else {
        DayEvent::new()
    }
    .choice("Walk", walk);
    if actions.get_current_sea() != Sea::Northern {
        e.choice("Leave", leave)
    } else {
        e
    }
}
