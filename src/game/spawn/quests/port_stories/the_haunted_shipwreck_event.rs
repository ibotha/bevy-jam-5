use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn explore_shipwreck(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_items(Item::MonkeyPaw, 1);
            actions.add_dialogue(captain!("Perfect conditions for exploration! We found a treasure trove and a mysterious monkey's paw. The crew's in high spirits!"));
        }
        (H::Chilly, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_crew(1);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We encountered a friendly ghost who joined our crew! However, the cold took its toll on the ship."));
        }
        (H::Blistering | H::Freezing, _, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-2);
            actions.delta_health(-15);
            actions.add_dialogue(captain!("The wreck was truly haunted! We barely escaped with our lives, losing two crew members and sustaining heavy damage."));
        }
        _ => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_food(-10);
            actions.add_dialogue(captain!("We found some gold, but strange occurrences spooked the crew. We used up extra provisions calming their nerves."));
        }
    }
}

fn hire_medium(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 100 {
        actions.delta_items(Item::Gold, -100);
        actions.delta_health(10);
        actions.delta_crew(1);
        actions.add_dialogue(captain!("The medium communed with the spirits, appeasing them. Our ship feels rejuvenated, and a ghostly sailor joined our crew!"));
    } else {
        actions.delta_items(Item::Gold, -50);
        actions.delta_health(-5);
        actions.add_dialogue(captain!("We couldn't afford the full séance. The medium partially calmed the spirits, but our ship took some spectral damage."));
    }
}

fn report_to_authorities(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 75);
    actions.delta_food(5);
    actions.add_dialogue(captain!("The port authorities thanked us for the information. They rewarded us with gold and supplies for steering clear of danger."));
}

pub fn the_haunted_shipwreck_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! There's talk of a ghostly shipwreck just off the coast. Fishermen report strange lights and eerie sounds!"))
        .line(captain!("A haunted shipwreck, you say? That's not something you hear every day. What else do we know?"))
        .line(crew2!("They say it only appears on foggy nights, Cap'n. Some claim it's cursed, others think it might hold treasure."))
        .line(crew3!("A few locals are considering hiring a medium to investigate, but they're short on coin."))
        .line(captain!("Intriguing indeed. What options do we have?"))
        .choice("Explore", explore_shipwreck)
        .conditional_choice("Hire medium", hire_medium, actions.get_item(Item::Gold) >= 50)
        .choice("Report", report_to_authorities)
        .hint("Squawk! Not all treasure glitters, and not all that glitters is treasure!")
}