use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn confront_leviathan(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 1200);
            actions.delta_health(40);
            actions.delta_crew(4);
            actions.delta_items(Item::Cannon, 6);
            actions.add_dialogue(captain!("Inconceivable! We've defeated the Leviathan! Its hide yields priceless treasures, its essence has empowered our ship, and some of its offspring have joined our crew! We're legends!"));
        }
        (H::Warm | H::Chilly, W::Low | W::High, M::Dry | M::Humid) => {
            actions.delta_items(Item::Gold, 600);
            actions.delta_health(-15);
            actions.delta_crew(-2);
            actions.delta_items(Item::Cannon, 4);
            actions.add_dialogue(captain!("A Pyrrhic victory! We've slain the beast, but at great cost. We've lost two crew members and sustained heavy damage. Still, the Leviathan's remains have yielded incredible treasures and mystical weaponry."));
        }
        (H::Blistering | H::Freezing, W::GaleForce, _) => {
            actions.delta_crew(-5);
            actions.delta_health(-35);
            actions.delta_items(Item::Gold, 300);
            actions.add_dialogue(captain!("Disaster! The Leviathan was too powerful in these conditions. We've lost five crew members and barely escaped with our lives. We managed to salvage some treasures from sunken ships in its wake, but at what cost?"));
        }
        _ => {
            actions.delta_items(Item::Gold, 400);
            actions.delta_health(-10);
            actions.delta_food(150);
            actions.delta_items(Item::Cannon, 2);
            actions.add_dialogue(captain!("We engaged the Leviathan cautiously. While we couldn't defeat it, we managed to drive it off. We've salvaged treasures from its lair, gained some of its scales as incredibly durable armor, and found a bounty of fish in its feeding grounds."));
        }
    }
}

fn attempt_communication(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(5);
        actions.delta_health(100);
        actions.delta_items(Item::Gold, 900);
        actions.delta_items(Item::Cannon, 5);
        actions.add_dialogue(captain!("The Monkey's Paw glowed with primordial energy, allowing us to communicate with the Leviathan! It recognized us as worthy allies, gifting us with its offspring as crew, empowering our ship with its essence, and sharing ancient treasures beyond imagination!"));
    } else {
        actions.delta_food(100);
        actions.delta_health(-40);
        actions.delta_items(Item::Gold, 250);
        actions.delta_crew(-3);
        actions.add_dialogue(captain!("Our attempt to communicate was partially successful, but at a great cost. The Leviathan spared us and granted some of its power, but the psychic backlash claimed three of our crew. We've gained some treasure and knowledge, but the price was high."));
    }
}

fn flee_from_leviathan(actions: &mut StoryActions) {
    actions.delta_food(-50);
    actions.delta_health(-10);
    actions.delta_items(Item::Gold, 100);
    actions.add_dialogue(captain!("We fled from the Leviathan with all haste. The effort strained our ship and depleted our supplies, but we managed to salvage some treasures from shipwrecks we passed in our desperate escape. The crew is shaken but alive."));
}

pub fn the_leviathan_awakening_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! The sea... it's boiling! Something massive is surfacing!"))
        .line(captain!("All hands on deck! What in the seven seas could cause such a disturbance?"))
        .line(crew2!("It's... it's larger than our ship, sir! Scales like islands, eyes like moons!"))
        .line(crew3!("The Leviathan, Cap'n! The ancient one awakens! Legend says it guards treasures from the dawn of time itself!"))
        .line(captain!("The Leviathan... I never thought I'd live to see the day. This could be our greatest triumph or our final voyage. What's our course, crew?"))
        .choice("Confront", confront_leviathan)
        .conditional_choice("Communicate", attempt_communication, actions.get_item(Item::MonkeyPaw) > 0)
        //.choice("Flee", flee_from_leviathan)
        .hint("Squawk! Even the mightiest creatures of the deep can be reasoned with... or defeated!")
}

