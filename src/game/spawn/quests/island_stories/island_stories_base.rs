use crate::game::spawn::quests::{
    northern_seas::set_course_northern_sea, prelude::*, sea_stories::sail,
};

fn walk(actions: &mut StoryActions) {
    let pdist = actions.possible_distance();
    actions.travel(pdist)
}

fn leave(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(actions.get_current_sea()));
    sail(actions);
}

fn hunt(actions: &mut StoryActions) {
    actions.delta_food(20);
    actions.delta_crew(actions.danger().max(0) / 2);
}

pub fn island_stories_base(actions: &mut StoryActions) -> DayEvent {
    let e = if actions.get_current_sea() == Sea::Northern && actions.no_course_set() {
        set_course_northern_sea(actions)
    } else {
        DayEvent::new()
    }
    .choice("Hunt", hunt)
    .choice("Walk On", walk);
    if (actions.get_current_sea() == Sea::Intro)
        || (actions.get_current_sea() != Sea::SirensCove && !actions.no_course_set())
    {
        e.choice("Leave Island", leave)
    } else {
        e
    }
}
