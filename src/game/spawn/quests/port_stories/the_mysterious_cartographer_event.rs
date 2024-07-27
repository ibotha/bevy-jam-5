use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn purchase_map(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 100 {
        actions.delta_items(Item::Gold, -100);
        actions.add_dialogue(captain!("We've acquired the mysterious map. It hints at a valuable location, but deciphering it might take some time."));
        // We can potentially add a future event
    } else {
        actions.delta_items(Item::Gold, -30);
        actions.add_dialogue(captain!("We could only afford a partial copy of the map. It's intriguing, but not very useful on its own."));
    }
}

fn trade_information(actions: &mut StoryActions) {
    actions.delta_food(-5);
    actions.delta_health(10);
    actions.add_dialogue(captain!("We shared our knowledge of the seas in exchange for the cartographer's expertise. Our navigational skills have improved, which should help on future voyages."));
}

fn hire_cartographer(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 75 {
        actions.delta_items(Item::Gold, -75);
        actions.delta_crew(1);
        actions.add_dialogue(captain!("We've hired the cartographer to join our crew. Their skills should prove invaluable in charting new courses."));
    } else {
        actions.delta_items(Item::Gold, -20);
        actions.delta_food(10);
        actions.add_dialogue(captain!("We couldn't afford to hire the cartographer full-time, but they gave us some useful tips and a stock of high-quality paper for our own map-making."));
    }
}

pub fn the_mysterious_cartographer_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! There's an odd fellow at the docks, claiming to be a master cartographer."))
        .line(captain!("A cartographer, you say? What makes this one so special?"))
        .line(crew2!("They've got maps of places I've never even heard of, Cap'n. And they're offering to sell one that supposedly leads to great riches."))
        .line(crew3!("Could be valuable, but also could be a load of hogwash. They seem keen on trading information too."))
        .line(captain!("Interesting... What are our options with this cartographer?"))
        .conditional_choice("Purchase map", purchase_map, actions.get_item(Item::Gold) >= 30)
        .choice("Trade info", trade_information)
        .conditional_choice("Hire", hire_cartographer, actions.get_item(Item::Gold) >= 20)
        .hint("Squawk! A good map is worth its weight in gold, but a bad one can lead you straight to Davy Jones' locker!")
}
