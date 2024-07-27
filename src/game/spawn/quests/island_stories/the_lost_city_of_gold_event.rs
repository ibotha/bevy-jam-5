use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn explore_city(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 2000);
            actions.delta_health(25);
            actions.add_dialogue(captain!("Unbelievable! We've found the legendary city, and it's more magnificent than the tales! Our holds are bursting with gold, and the crew is in high spirits. The ship seems to have been blessed by this discovery!"));
        }
        (H::Warm, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 1000);
            actions.delta_crew(-1);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("We've struck it rich, but the city had some nasty surprises. Lost one of our own to a collapsing temple, and the ship took some damage. Still, the gold we found will set us up for life!"));
        }
        (H::Blistering, W::High, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-3);
            actions.delta_health(-30);
            actions.delta_items(Item::Gold, 250);
            actions.add_dialogue(captain!("Cursed city! The extreme conditions triggered ancient traps. We lost three good men and the ship's barely holding together. We managed to grab some gold as we fled, but was it worth it?"));
        }
        _ => {
            actions.delta_items(Item::Gold, 750);
            actions.delta_food(-25);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We found a good amount of treasure, but the exploration took its toll. Our supplies are running low, and the ship needs some repairs. Still, it's more gold than most see in a lifetime!"));
        }
    }
}

fn decipher_map(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 200 {
        actions.delta_items(Item::Gold, -200);
        actions.delta_items(Item::Gold, 1500);
        actions.delta_items(Item::Cannon, 2);
        actions.add_dialogue(captain!("The investment in proper tools and scholars paid off! We deciphered the map and found a secret vault. It was filled with gold and ancient weapons. These cannons will give us a significant advantage in battles to come!"));
    } else {
        actions.delta_items(Item::Gold, 300);
        actions.delta_health(-15);
        actions.add_dialogue(captain!("Without proper resources, our attempts to decipher the map were only partially successful. We found some gold, but triggered traps that damaged the ship. It's a bittersweet discovery."));
    }
}

fn leave_city(actions: &mut StoryActions) {
    actions.delta_crew(2);
    actions.delta_food(50);
    actions.add_dialogue(captain!("We decided the risk wasn't worth it. As we sailed away, we encountered a ship of fellow explorers. They joined our crew, grateful for the rescue, and shared their ample supplies with us. Sometimes, caution pays off in unexpected ways."));
}

pub fn the_lost_city_of_gold_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! We've found it! The Lost City of Gold!"))
        .line(captain!("Steady on! Are you certain? Many have searched for this legendary place..."))
        .line(crew2!("Aye, Cap'n! The golden spires are unmistakable. But the city looks trapped in time, could be dangerous."))
        .line(crew3!("I've got a partial map here, Cap'n. With some work, we might be able to find a safer way in."))
        .line(captain!("This could change everything, crew. But one wrong move could spell our doom. What's our play?"))
        .choice("Explore City", explore_city)
        .conditional_choice("Decipher Map", decipher_map, actions.get_item(Item::Gold) >= 200)
        .choice("Leave City", leave_city)
        .hint("Squawk! All that glitters is gold, but gold can't buy your life back!")
}