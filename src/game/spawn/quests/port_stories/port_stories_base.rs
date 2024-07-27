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
    DayEvent::new().choice("Embark", embark).conditional_choice(
        "Resupply",
        resupply,
        actions.get_item(Item::Gold) > 20,
    )
}

