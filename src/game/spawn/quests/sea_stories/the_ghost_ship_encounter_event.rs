use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn approach_ghost_ship(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::None | W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("Incredible! The ghost ship was filled with ancient treasures. We've come away with gold and even an old cannon!"));
        }
        (W::Medium, M::Comfortable) => {
            actions.delta_crew(-1);
            actions.delta_items(Item::Gold, 100);
            actions.add_dialogue(captain!("We found some treasure aboard, but one of our crew vanished without a trace. The rest are shaken but alive."));
        }
        (W::High | W::GaleForce, M::Humid) => {
            actions.delta_crew(-2);
            actions.delta_health(-20);
            actions.add_dialogue(captain!("It was a trap! Spectral pirates attacked us. We barely escaped, but lost two crew members and took heavy damage."));
        }
        _ => {
            actions.delta_food(-10);
            actions.delta_items(Item::Gold, 50);
            actions.add_dialogue(captain!("The ghost ship vanished as we approached, leaving behind only a small chest of gold and an eerie chill in the air."));
        }
    }
}

fn attempt_communication(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(2);
        actions.delta_health(10);
        actions.add_dialogue(captain!("The Monkey's Paw glowed as we hailed the ghost ship. Two spectral sailors joined our crew, and our ship feels somehow... reinforced."));
    } else {
        actions.delta_food(-15);
        actions.delta_health(-5);
        actions.add_dialogue(captain!("Our attempts at communication were met with an otherworldly silence. The experience left us drained and uneasy."));
    }
}

fn flee_from_ghost_ship(actions: &mut StoryActions) {
    actions.delta_food(-20);
    actions.delta_health(-10);
    actions.add_dialogue(captain!("We turned tail and ran, pushing the ship to its limits. We're safe, but the effort took a toll on our supplies and the ship."));
}

pub fn the_ghost_ship_encounter_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! There's... there's a ghost ship off the port bow!"))
        .line(captain!("A ghost ship? Are you certain?"))
        .line(crew2!("Aye, sir! It's translucent and glowing, with tattered sails. No crew visible."))
        .line(crew3!("I've heard tales of these, Cap'n. They say ghost ships can lead to great fortune or terrible doom."))
        .line(captain!("This could be dangerous... or very rewarding. What's our move, crew?"))
        .choice("Approach", approach_ghost_ship)
        .conditional_choice("Communicate", attempt_communication, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Flee", flee_from_ghost_ship)
        .hint("Squawk! Not all treasures are worth the risk, but some risks bring the greatest rewards!")
}