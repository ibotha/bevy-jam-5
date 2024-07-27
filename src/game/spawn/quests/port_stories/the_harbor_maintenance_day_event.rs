use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn repair_hull(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 50 {
        actions.delta_items(Item::Gold, -50);
        actions.delta_health(15);
        actions.add_dialogue(captain!("We've patched up the hull nicely. The ship's in much better shape now, ready for our next voyage."));
    } else {
        actions.delta_items(Item::Gold, -20);
        actions.delta_health(5);
        actions.add_dialogue(captain!("We could only afford minor repairs, but it's better than nothing. We'll need to be careful out there."));
    }
}

fn restock_supplies(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 40 {
        actions.delta_items(Item::Gold, -40);
        actions.delta_food(20);
        actions.add_dialogue(captain!("Our hold is well-stocked now. We shouldn't go hungry on our next journey."));
    } else {
        actions.delta_items(Item::Gold, -15);
        actions.delta_food(8);
        actions.add_dialogue(captain!("We picked up what supplies we could afford. It's not much, but it'll have to do."));
    }
}

fn train_crew(actions: &mut StoryActions) {
    actions.delta_food(-5);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We spent the day running drills and sharing knowledge. The crew's more prepared now, and we even discovered a few ways to improve the ship's efficiency."));
}

pub fn the_harbor_maintenance_day_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n, we've got some time in port. The ship could use some attention."))
        .line(captain!("Indeed, a well-maintained ship is crucial. What are our priorities?"))
        .line(crew2!("The hull's taken a beating on our last voyage, and our supplies are running low."))
        .line(crew3!("We could also use this time to train and improve our skills, Cap'n."))
        .line(captain!("All important tasks. Let's decide what to focus on."))
        .conditional_choice("Repair hull", repair_hull, actions.get_item(Item::Gold) >= 20)
        .conditional_choice("Restock supplies", restock_supplies, actions.get_item(Item::Gold) >= 15)
        .choice("Train", train_crew)
        .hint("Squawk! A stitch in time saves nine, and a well-maintained ship sails twice as far!")
}