use super::port_stories_base;
use crate::game::spawn::quests::prelude::*;

fn inspect_cargo(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind: _,
        moisture,
    } = actions.weather();

    match moisture {
        M::Dry => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("We found a valuable artifact, but one of our crew was overcome by a strange illness."));
        }
        M::Comfortable => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_health(5);
            actions.add_dialogue(captain!("The cargo contained some valuable spices and a curious talisman that seems to bring good fortune."));
        }
        M::Humid => {
            actions.delta_health(-10);
            actions.delta_crew(-2);
            actions.add_dialogue(captain!("The humid air carried a foul miasma from the cargo. Several crew members fell ill before we could seal it back up."));
        }
    }
}

fn hire_expert(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 150 {
        actions.delta_items(Item::Gold, -150);
        actions.delta_items(Item::MonkeyPaw, 1);
        actions.add_dialogue(captain!("The expert identified a rare magical item - a monkey's paw! It could be valuable, but she warned of potential dangers."));
    } else {
        actions.delta_items(Item::Gold, -50);
        actions.delta_crew(1);
        actions.add_dialogue(captain!("We couldn't afford the full consultation, but the expert gave us some advice and decided to join our crew out of curiosity."));
    }
}

fn refuse_cargo(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 25);
    actions.add_dialogue(captain!("We refused to deal with the suspicious cargo. The port authorities rewarded us for our caution."));
}

pub fn the_cursed_cargo_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! There's a strange cargo just arrived at the port. The dockhands are afraid to touch it."))
        .line(captain!("What's so strange about it, Patchy?"))
        .line(crew2!("They say it's covered in weird symbols and seems to glow in the dark, Cap'n."))
        .line(crew3!("I've heard whispers of a curse, but also rumors of great value within."))
        .line(captain!("Interesting... What do you think we should do?"))
        .choice("Inspect", inspect_cargo)
        .conditional_choice("Hire Expert", hire_expert, actions.get_item(Item::Gold) >= 50)
        .choice("Refuse", refuse_cargo)
        .hint("Squawk! Curiosity killed the cat, but satisfaction brought it back!")
}

