use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn engage_in_battle(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::Medium, M::Dry | M::Comfortable) => {
            actions.delta_items(Item::Gold, 250);
            actions.delta_health(-15);
            actions.delta_items(Item::Cannon, 2);
            actions.add_dialogue(captain!("Victory! We've defeated the phantom fleet. Their spectral treasure is now ours, and we've even salvaged some of their ethereal cannons!"));
        }
        (W::Medium | W::High, M::Comfortable) => {
            actions.delta_crew(-2);
            actions.delta_items(Item::Gold, 150);
            actions.delta_health(10);
            actions.add_dialogue(captain!("A hard-fought battle! We lost two brave souls, but emerged victorious. The phantom fleet's treasure is ours, and our ship seems more resilient now."));
        }
        (W::High | W::GaleForce, M::Humid) => {
            actions.delta_crew(-3);
            actions.delta_health(-30);
            actions.delta_items(Item::Gold, 50);
            actions.add_dialogue(captain!("A devastating engagement! We were outmatched and barely escaped. We lost three crew members and took heavy damage, but managed to snag a small amount of spectral gold."));
        }
        _ => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_health(-5);
            actions.delta_food(10);
            actions.add_dialogue(captain!("A stalemate! We exchanged fire with the phantom fleet but neither side could gain the upper hand. We've collected some ghostly treasures and provisions, but our ship is slightly damaged."));
        }
    }
}

fn attempt_diplomacy(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(2);
        actions.delta_health(20);
        actions.delta_items(Item::Gold, 200);
        actions.add_dialogue(captain!("The Monkey's Paw glowed, allowing us to communicate with the phantom pirates! They've joined our cause, gifting us two spectral crew members, repairing our ship with ghostly magic, and sharing their ethereal treasure!"));
    } else {
        actions.delta_food(20);
        actions.delta_health(-10);
        actions.delta_items(Item::Gold, 75);
        actions.add_dialogue(captain!("Our diplomatic efforts were partially successful. The phantom pirates were intrigued by our boldness. They've allowed us to take some of their spectral provisions and a small amount of ghost gold, but the supernatural negotiation has left us drained."));
    }
}

fn attempt_to_evade(actions: &mut StoryActions) {
    actions.delta_food(-15);
    actions.delta_health(-5);
    actions.add_dialogue(captain!("We managed to evade the phantom fleet, but it wasn't easy. We've used up extra supplies in our haste, and the crew is exhausted from the effort."));
}

pub fn the_phantom_pirate_fleet_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! There's a fleet of ships approaching... but they're translucent!"))
        .line(captain!("Translucent ships? Are you sure your eyes aren't playing tricks on you?"))
        .line(crew2!("It's true, sir! A whole fleet of ghostly pirate ships, bearing down on us with an eerie green glow!"))
        .line(crew3!("I've heard tales of the Phantom Pirate Fleet, Cap'n. They're said to be cursed souls, forever sailing these waters."))
        .line(captain!("A phantom fleet, eh? This could be our greatest challenge yet... or our greatest opportunity. What's our course of action, crew?"))
        .choice("Engage", engage_in_battle)
        .conditional_choice("Diplomacy", attempt_diplomacy, actions.get_item(Item::MonkeyPaw) > 0)
        //.choice("Evade", attempt_to_evade)
        .hint("Squawk! Even the dead can teach us a thing or two about sailing... or fighting!")
}

