use crate::game::spawn::quests::northern_seas::set_course_northern_sea;
use crate::game::spawn::quests::prelude::*;
use crate::game::spawn::quests::sea_stories::sail;

fn embark(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(actions.get_current_sea()));
    sail(actions);
}
fn resupply(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -20);
    actions.delta_food(50);
}

pub fn port_stories_base(actions: &mut StoryActions) -> DayEvent {
    let e = if actions.get_current_sea() == Sea::Northern || actions.no_course_set() {
        set_course_northern_sea(actions)
    } else {
        DayEvent::new()
    }
    .conditional_choice("Resupply", resupply, actions.get_item(Item::Gold) > 20);
    if actions.get_current_sea() != Sea::Northern || !actions.no_course_set() {
        e.choice("Embark", embark)
    } else {
        e
    }
}
