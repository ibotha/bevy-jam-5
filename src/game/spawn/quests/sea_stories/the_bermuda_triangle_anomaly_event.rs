use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn navigate_through(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture: _,
    } = actions.weather();

    match (heat, wind) {
        (H::Comfortable | H::Warm, W::Low | W::Medium) => {
            actions.delta_items(Item::Gold, 250);
            actions.delta_health(25);
            actions.add_dialogue(captain!("Incredible! We've emerged with a hull full of gold from a long-lost shipwreck, and our ship seems stronger than ever!"));
        }
        (H::Chilly | H::Freezing, W::Medium | W::High) => {
            actions.delta_crew(-1);
            actions.delta_food(40);
            actions.delta_items(Item::Cannon, 2);
            actions.add_dialogue(captain!("We've lost a crew member to the anomaly, but our food stores are overflowing and we've acquired advanced weaponry from what seemed to be the future!"));
        }
        (H::Blistering, W::High | W::GaleForce) => {
            actions.delta_crew(-2);
            actions.delta_health(-20);
            actions.delta_items(Item::Gold, 100);
            actions.add_dialogue(captain!("The anomaly nearly tore us apart! We lost two crew members and took heavy damage, but managed to snag some gold in the chaos."));
        }
        _ => {
            actions.delta_food(20);
            actions.delta_health(10);
            actions.add_dialogue(captain!("We've safely navigated through the anomaly. Our supplies have mysteriously replenished, and the ship feels somewhat rejuvenated."));
        }
    }
}

fn study_anomaly(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(2);
        actions.delta_health(15);
        actions.delta_items(Item::Gold, 150);
        actions.add_dialogue(captain!("The Monkey's Paw seemed to resonate with the anomaly! We've gained incredible insights, two new crew members from another time, and a cache of gold!"));
    } else {
        actions.delta_food(-15);
        actions.delta_health(-10);
        actions.delta_items(Item::Gold, 50);
        actions.add_dialogue(captain!("Our attempts to study the anomaly were mostly fruitless. We've lost time and resources, but did manage to salvage some gold from a spatial rift."));
    }
}

fn retreat_immediately(actions: &mut StoryActions) {
    actions.delta_food(-10);
    actions.delta_health(-5);
    actions.add_dialogue(captain!("We quickly turned the ship around and escaped the anomaly. We're safe, but the effort has drained our supplies somewhat."));
}

pub fn the_bermuda_triangle_anomaly_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! The sea's gone mad! The compass is spinning and I swear I just saw a ship vanish into thin air!"))
        .line(captain!("Calm down! What in Neptune's name is happening?"))
        .line(crew2!("It's like we've sailed into the Bermuda Triangle, sir! Strange lights in the water, and the air feels... wrong."))
        .line(crew3!("I've heard tales of ships finding great treasures in places like this, Cap'n. But also of entire crews disappearing without a trace."))
        .line(captain!("This could be our greatest opportunity... or our doom. What's our course of action, crew?"))
        .choice("Navigate Through", navigate_through)
        .conditional_choice("Study Anomaly", study_anomaly, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Retreat", retreat_immediately)
        .hint("Squawk! Sometimes the greatest discoveries lie beyond the edge of the map!")
}