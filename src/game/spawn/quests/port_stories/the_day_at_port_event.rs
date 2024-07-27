use super::port_stories_base;
use crate::game::spawn::quests::prelude::*;

fn recruit(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -100);
    actions.delta_crew(1);
}

fn resupply(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -20);
    actions.delta_food(10);
}
fn repair(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -100);
    actions.delta_health(10);
}

pub fn the_day_at_port_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(captain!(
            "We are still at the blasted port!",
            "Is the weather right for us to embark?"
        ))
        .conditional_choice("Recruit", recruit, actions.get_item(Item::Gold) > 100)
        .conditional_choice("Resupply", resupply, actions.get_item(Item::Gold) > 20)
        .conditional_choice("Repair", repair, actions.get_item(Item::Gold) > 100)
}

