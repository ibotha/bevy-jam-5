use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn enter_tomb(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 3000);
            actions.delta_items(Item::Cannon, 3);
            actions.delta_health(25);
            actions.add_dialogue(captain!("Unbelievable! We've navigated the Pirate King's traps and riddles. His treasure hoard is ours, including three legendary cannons! The crew's morale is soaring, and the ship seems to have absorbed some of the tomb's ancient power!"));
        }
        (H::Warm, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 1500);
            actions.delta_crew(-2);
            actions.delta_health(-15);
            actions.add_dialogue(captain!("We've claimed the Pirate King's treasure, but at a steep cost. Two of our bravest fell to the tomb's traps. The gold we've found is substantial, but the air is heavy with our losses."));
        }
        (H::Blistering, W::High, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-4);
            actions.delta_health(-35);
            actions.delta_items(Item::Gold, 500);
            actions.add_dialogue(captain!("Curse the Pirate King and his blasted tomb! The harsh conditions triggered ancient magics. We lost four good souls to the chaos, and the ship's barely afloat. We managed to grab some gold as we fled, but was it worth the price?"));
        }
        _ => {
            actions.delta_items(Item::Gold, 1000);
            actions.delta_food(-30);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("We've braved the tomb and claimed a portion of the Pirate King's hoard. The journey took its toll on our supplies and the crew's health, but the gold will set us up nicely... if we can shake off the curse we all feel lingering."));
        }
    }
}

fn decipher_map(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 300 {
        actions.delta_items(Item::Gold, -300);
        actions.delta_items(Item::Gold, 2000);
        actions.delta_items(Item::Cannon, 1);
        actions.add_dialogue(captain!("Our investment in scholars and supplies paid off! We deciphered the Pirate King's map, revealing a secret entrance. We've claimed a significant portion of his treasure and one of his fabled cannons, all without triggering the main traps!"));
    } else {
        actions.delta_items(Item::Gold, 500);
        actions.delta_health(-20);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("Without proper resources, our attempt to decipher the map was only partially successful. We found a side chamber with some gold, but triggered a trap that claimed one of our own. The ship feels... unsettled."));
    }
}

fn leave_tomb(actions: &mut StoryActions) {
    actions.delta_crew(2);
    actions.delta_food(40);
    actions.delta_items(Item::Gold, 200);
    actions.add_dialogue(captain!("We decided the risk wasn't worth it and left the tomb undisturbed. As we sailed away, we encountered a ship of treasure hunters. They joined our crew, grateful for the warning, and shared their supplies. We also traded some information for a small chest of gold. Sometimes, caution pays unexpected dividends."));
}

pub fn the_pirate_kings_tomb_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! We've found it! The legendary tomb of the Pirate King!"))
        .line(captain!("Are ye certain? Many have searched for this fabled place..."))
        .line(crew2!("Aye, Cap'n! The markings match the old tales. But there's an ominous feeling in the air..."))
        .line(crew3!("I've got a partial map here, Cap'n. With some work and resources, we might find a safer way in."))
        .line(captain!("This could be our greatest haul or our doom, lads. What's our move?"))
        .choice("Enter Tomb", enter_tomb)
        .conditional_choice("Decipher Map", decipher_map, actions.get_item(Item::Gold) >= 300)
        .choice("Leave Tomb", leave_tomb)
        .hint("Squawk! The greatest treasures often come with the deadliest curses!")
}