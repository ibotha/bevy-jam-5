use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn enter_time_storm(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 1000);
            actions.delta_items(Item::Cannon, 2);
            actions.delta_health(10);
            actions.delta_crew(10);
            actions.add_dialogue(captain!("Inconceivable! We've navigated the time storm perfectly! We've plundered treasures from every era, acquired futuristic cannons, and even recruited legendary pirates to our crew. Our ship now seems to exist in multiple time periods at once, making it nigh invulnerable!"));
        }
        (H::Warm, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_items(Item::Cannon, 1);
            actions.delta_crew(-5);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We've weathered the time storm, but at a great cost. Five of our crew are lost in time, perhaps never born or long dead. But we've returned with riches from the future and cannons that won't be invented for centuries. The ship feels... unstuck in time."));
        }
        (H::Blistering, W::High, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-15);
            actions.delta_health(-10);
            actions.delta_items(Item::Gold, -200);
            actions.add_dialogue(captain!("Catastrophe! The time storm tore our ship apart across eons! Most of our crew never existed, we've lost gold to time paradoxes, and the ship is a temporal wreck. We're lucky the universe still remembers we exist!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 300);
            actions.delta_items(Item::Cannon, 2);
            actions.delta_crew(2);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("We've emerged from the time storm changed. We've gained treasures from other eras and advanced cannons, even picked up two crew members from the past. But the temporal shifts have taken a toll on our ship's integrity. It's a miracle we're all still sane."));
        }
    }
}

fn use_cannons(actions: &mut StoryActions) {
    if actions.get_item(Item::Cannon) >= 3 {
        actions.delta_items(Item::Cannon, -3);
        actions.delta_items(Item::Gold, 500);
        actions.delta_health(20);
        actions.delta_crew(5);
        actions.add_dialogue(captain!("Brilliant! Firing our cannons into the time storm stabilized it! We've created a safe temporal bubble around our ship. We've harvested incredible treasures from multiple timelines, and even rescued some of history's greatest pirates to join our crew!"));
    } else {
        actions.delta_health(-20);
        actions.delta_crew(-5);
        actions.delta_items(Item::Gold, -10);
        actions.add_dialogue(captain!("Disaster! Without enough cannons to stabilize the time storm, we were tossed through history like a leaf in a hurricane. We've lost crew members to different eras, our gold has been erased from existence, and the ship is barely holding together across multiple timestreams."));
    }
}

fn avoid_time_storm(actions: &mut StoryActions) {
    actions.delta_food(-10);
    actions.delta_health(-10);
    actions.delta_items(Item::Gold, 100);
    actions.delta_crew(1);
    actions.add_dialogue(captain!("We managed to skirt the edge of the time storm. It was a grueling journey, but as we passed, a ship from the future crashed into us and then vanished! We've salvaged futuristic gold from the wreckage, and one of their crew members was left behind, joining us with tales of times yet to come."));
}

pub fn the_time_storm_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("CAP'N! The sea ahead... it's changing! I see past, present, and future all at once!"))
        .line(captain!("Great Neptune's ghost... it's a time storm! I thought they were just legends!"))
        .line(crew2!("I see ships from every era, Cap'n! Galleons, steamships, even... flying ships?"))
        .line(crew3!("Our cannons might be able to stabilize the time distortions, Cap'n, if we have enough of them!"))
        .line(captain!("This is beyond anything we've faced, lads. Our next decision will echo through eternity itself!"))
        .choice("Enter Storm", enter_time_storm)
        .conditional_choice("Use Cannons", use_cannons, actions.get_item(Item::Cannon) >= 3)
        .choice("Avoid Storm", avoid_time_storm)
        .hint("Squawk! Past, present, or future - a pirate's life is timeless!")
}