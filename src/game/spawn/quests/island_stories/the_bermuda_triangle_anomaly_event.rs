use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn navigate_anomaly(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 1500);
            actions.delta_items(Item::Cannon, 3);
            actions.delta_health(30);
            actions.add_dialogue(captain!("Incredible! We've navigated through the anomaly and emerged in a pocket dimension! We've discovered untold riches, advanced cannons, and the ship seems stronger than ever. The crew swears we were gone for months, but only hours have passed outside!"));
        }
        (H::Warm, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 800);
            actions.delta_crew(-2);
            actions.delta_health(-15);
            actions.add_dialogue(captain!("We've returned from the anomaly, but at a cost. Two crew members vanished into thin air, and the ship's taken a beating from spatial distortions. However, we've brought back a fortune in gold from a parallel world!"));
        }
        (H::Blistering, W::High, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-4);
            actions.delta_health(-40);
            actions.add_dialogue(captain!("Disaster! The anomaly nearly tore us apart! We've lost four crew to the void, and the ship's barely holding together. We're lucky to have escaped with our lives, but the horrors we've seen will haunt us forever."));
        }
        _ => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_food(-30);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("We've navigated the anomaly and returned with some interdimensional treasures. The journey took a toll on our supplies and the ship, but the gold we've brought back makes it worthwhile... I think."));
        }
    }
}

fn study_anomaly(actions: &mut StoryActions) {
    if actions.get_item(Item::Cannon) >= 1 {
        actions.delta_items(Item::Cannon, -1);
        actions.delta_items(Item::Gold, 1000);
        actions.delta_health(20);
        actions.add_dialogue(captain!("Eureka! By firing our cannon into the anomaly, we created a stable portal. We were able to reach through and grab a chest of gold from another dimension! The ship seems to have absorbed some of the anomaly's energy, making it stronger."));
    } else {
        actions.delta_items(Item::Gold, 200);
        actions.delta_crew(-1);
        actions.delta_health(-10);
        actions.add_dialogue(captain!("Without a cannon to stabilize the anomaly, our studies were dangerous. We gathered some strange gold coins, but lost a crewman to a sudden spatial rift. The ship's taken some damage from the volatile energies."));
    }
}

fn avoid_anomaly(actions: &mut StoryActions) {
    actions.delta_food(40);
    actions.delta_health(10);
    actions.delta_crew(1);
    actions.add_dialogue(captain!("We steered clear of that unnatural phenomenon. As we sailed away, we rescued a castaway who claims to have escaped the anomaly. He's joined our crew and shared his ample supplies with us. Sometimes, discretion is the better part of valor."));
}

pub fn the_bermuda_triangle_anomaly_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! The sea's actin' strange... the compass is spinnin' and I swear I saw a ship vanish into thin air!"))
        .line(captain!("By Neptune's beard... we've sailed into the legendary Bermuda Triangle!"))
        .line(crew2!("I've heard tales of ships entering another world in these waters, Cap'n. Comin' back with untold riches... or never returnin' at all."))
        .line(crew3!("We could try to study it from a distance, Cap'n. Might be safer, but we'd need a cannon to probe it properly."))
        .line(captain!("This could be our greatest adventure or our last, lads. What's our course of action?"))
        .choice("Navigate Anomaly", navigate_anomaly)
        .conditional_choice("Study Anomaly", study_anomaly, actions.get_item(Item::Cannon) >= 1)
        .choice("Avoid Anomaly", avoid_anomaly)
        .hint("Squawk! The unknown can lead to fortune or folly, choose wisely!")
}