use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn organize_hunting_expedition(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 200 && actions.get_crew() >= 5 {
        actions.delta_items(Item::Gold, -200);
        actions.delta_crew(-1);
        actions.delta_health(-10);
        actions.delta_items(Item::Gold, 500);
        actions.add_dialogue(captain!("We've done it! We slayed the beast and claimed a hefty bounty. Lost a brave soul in the battle, but the crew's morale is through the roof!"));
        // We can potentially add a future event
    } else {
        actions.delta_items(Item::Gold, -50);
        actions.delta_health(-5);
        actions.add_dialogue(captain!("We weren't prepared for such a hunt. We searched in vain and returned with nothing but minor damages and wounded pride."));
    }
}

fn study_the_creature(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -25);
    actions.delta_food(-5);
    actions.add_dialogue(captain!("We spent days observing the creature from a safe distance. The knowledge we've gained is invaluable, and we've charted its territory to avoid future encounters."));
    // We can potentially add a future event
}

fn spread_the_tale(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 100);
    actions.delta_crew(2);
    actions.add_dialogue(captain!("Our tale of the sea monster spread through the port like wildfire. We've earned a reputation as brave explorers, gained some coin from buying rounds, and even attracted new crew members seeking adventure!"));
}

pub fn the_legendary_sea_monster_sighting_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! You won't believe what's got the whole port in an uproar!"))
        .line(captain!("Out with it, then. What's all the commotion about?"))
        .line(crew2!("There's been a sighting of a legendary sea monster just off the coast, Cap'n. A beast so large it could swallow our ship whole!"))
        .line(crew3!("Aye, and the port authorities are offering a massive bounty for anyone brave enough to hunt it down."))
        .line(captain!("A sea monster, eh? This could be a chance for glory... or folly. What are our options?"))
        .conditional_choice("Organize expedition", organize_hunting_expedition, actions.get_item(Item::Gold) >= 50 && actions.get_crew() >= 5)
        .choice("Study", study_the_creature)
        .choice("Spread tales", spread_the_tale)
        .hint("Squawk! The sea holds many secrets, but not all should be disturbed!")
}