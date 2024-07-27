use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn harvest_fruit(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Comfortable) => {
            actions.delta_food(40);
            actions.delta_health(10);
            actions.add_dialogue(captain!("Perfect conditions for harvesting! We've gathered a bounty of fresh fruit, and the crew's health has improved from the vitamins."));
        }
        (H::Warm, W::Medium, _) => {
            actions.delta_food(25);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("The heat made the work harder, but we still gathered a good amount of fruit. Some of the crew are a bit sunburnt though."));
        }
        (H::Blistering, _, _) => {
            actions.delta_food(10);
            actions.delta_health(-10);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("The blistering heat was brutal! We got some fruit, but lost a crewman to heatstroke. This wasn't worth it."));
        }
        _ => {
            actions.delta_food(20);
            actions.add_dialogue(captain!("We managed to gather a decent amount of fruit without any trouble. It'll be a nice change from hardtack."));
        }
    }
}

fn search_for_valuables(actions: &mut StoryActions) {
    actions.delta_food(10);
    actions.delta_items(Item::Gold, 100);
    actions.delta_health(-5);
    actions.add_dialogue(captain!("While searching the grove, we found some gold coins buried under an old tree! Must be an old pirate stash. The searching was tiring work though."));
}

fn quick_rest(actions: &mut StoryActions) {
    actions.delta_food(5);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We took a short break in the shade of the fruit trees. The crew's a bit refreshed, and we picked a few fruits for the road."));
}

pub fn the_tropical_fruit_grove_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! There's a grove of fruit trees on this island!"))
        .line(captain!("Aye, I see it. Could be a good chance to restock our supplies."))
        .line(crew2!("The fruit looks ripe and plentiful. Should we gather as much as we can?"))
        .line(crew3!("Or perhaps we could search the area? Old groves like this sometimes hide secrets."))
        .line(captain!("We've got a few options here, crew. What'll it be?"))
        .choice("Harvest", harvest_fruit)
        .choice("Search", search_for_valuables)
        .choice("Rest", quick_rest)
        .hint("Squawk! An orange a day keeps the scurvy away!")
}