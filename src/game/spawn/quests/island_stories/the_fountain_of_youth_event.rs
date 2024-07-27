use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn drink_from_fountain(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Comfortable) => {
            actions.delta_health(50);
            actions.delta_crew(3);
            actions.add_dialogue(captain!("Miraculous! The fountain's waters have rejuvenated us all. The crew feels decades younger, and even the ship seems to have been restored to its prime. We've gained some extra hands as well - seems the fountain brought some of our old injuries back to life!"));
        }
        (H::Warm, W::Medium, M::Humid) => {
            actions.delta_health(25);
            actions.delta_crew(-1);
            actions.delta_items(Item::Gold, 500);
            actions.add_dialogue(captain!("The fountain's effects were... mixed. Most of us feel invigorated, but poor Old Tom vanished entirely - turned back into a young lad and ran off into the jungle! On the bright side, we found a stash of gold near the fountain."));
        }
        (H::Blistering, W::High, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-3);
            actions.delta_health(-20);
            actions.add_dialogue(captain!("Disaster! The fountain's waters turned volatile in this weather. Three of our crew aged rapidly and crumbled to dust! The rest of us barely escaped, and the ship's taken a beating. Some things are best left to legend."));
        }
        _ => {
            actions.delta_health(15);
            actions.delta_food(-20);
            actions.delta_items(Item::Gold, 200);
            actions.add_dialogue(captain!("We all feel a bit better after drinking from the fountain, but not exactly young again. Found some gold artifacts around the site though. The whole experience has left us quite hungry."));
        }
    }
}

fn bottle_water(actions: &mut StoryActions) {
    if actions.get_item(Item::Cannon) >= 1 {
        actions.delta_items(Item::Cannon, -1);
        actions.delta_items(Item::Gold, 2000);
        actions.delta_health(10);
        actions.add_dialogue(captain!("Brilliant idea! We used a cannon to carefully collect the water. Found a merchant ship on our way back and sold most of the bottles for a fortune! Kept a few for ourselves too - the crew's health has improved."));
    } else {
        actions.delta_items(Item::Gold, 300);
        actions.delta_health(-5);
        actions.add_dialogue(captain!("Without proper equipment, bottling the water was tricky. We managed to collect some, but most of it lost its power quickly. Sold what we could in the next port, but it's not the fortune we hoped for."));
    }
}

fn map_location(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 1000);
    actions.delta_food(30);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We carefully mapped the fountain's location without disturbing it. Sold the map to a wealthy explorer for a small fortune! The quest has left us in good spirits, and we traded some of the gold for fresh supplies."));
}

pub fn the_fountain_of_youth_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! You won't believe it... we've found it! The Fountain of Youth!"))
        .line(captain!("Impossible... are you certain, lad?"))
        .line(crew2!("Aye, Cap'n! A glowing spring, hidden in a cave. The plants around it are growin' and witherin' in minutes!"))
        .line(crew3!("We could try to bottle some of the water, Cap'n. Might need somethin' sturdy like a cannon to safely contain it."))
        .line(captain!("This discovery could change everything, crew. But tampering with nature's laws is risky. What shall we do?"))
        .choice("Drink from Fountain", drink_from_fountain)
        .conditional_choice("Bottle Water", bottle_water, actions.get_item(Item::Cannon) >= 1)
        .choice("Map Location", map_location)
        .hint("Squawk! Youth is wasted on the young, but can the old handle it?")
}