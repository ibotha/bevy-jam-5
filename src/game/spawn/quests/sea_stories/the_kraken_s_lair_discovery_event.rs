use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn explore_lair(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable | H::Warm, W::Low | W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_health(5);
            actions.delta_items(Item::Cannon, 2);
            actions.add_dialogue(captain!("Unbelievable! We've found the Kraken's treasure hoard! Ancient artifacts, gold beyond measure, and even some advanced weaponry. Our ship feels stronger than ever!"));
        }
        (H::Chilly | H::Freezing, W::Medium | W::High, _) => {
            actions.delta_crew(-2);
            actions.delta_items(Item::Gold, 100);
            actions.delta_food(50);
            actions.add_dialogue(captain!("We've lost two brave souls to the Kraken's defenses, but we managed to snag some treasure and a massive haul of rare seafood. The crew is shaken but alive."));
        }
        (H::Blistering, W::High | W::GaleForce, M::Humid) => {
            actions.delta_crew(-4);
            actions.delta_health(-20);
            actions.delta_items(Item::Gold, 75);
            actions.add_dialogue(captain!("Disaster! The Kraken returned while we were exploring! We barely escaped with our lives, losing four crew members and sustaining heavy damage. We grabbed what gold we could in our flight."));
        }
        _ => {
            actions.delta_items(Item::Gold, 75);
            actions.delta_health(15);
            actions.delta_food(15);
            actions.add_dialogue(captain!("We explored cautiously and came away with a decent haul of gold and exotic seafood. The ship seems to have absorbed some of the Kraken's essence, feeling more resilient."));
        }
    }
}

fn attempt_communication(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(1);
        actions.delta_health(30);
        actions.delta_items(Item::Gold, 200);
        actions.add_dialogue(captain!("The Monkey's Paw glowed, allowing us to communicate with the Kraken! It's gifted us with one of its offspring as a crew member, some of its regenerative power, and a portion of its treasure hoard!"));
    } else {
        actions.delta_food(50);
        actions.delta_health(-10);
        actions.delta_items(Item::Gold, 100);
        actions.add_dialogue(captain!("Our attempts at communication were partially successful. The Kraken seemed curious but wary. It's allowed us to take some of its food stock and a small tribute of gold, but the psychic contact has left us drained."));
    }
}

fn leave_quietly(actions: &mut StoryActions) {
    actions.delta_food(20);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We wisely chose to leave without disturbing the lair. The crew is relieved, and we managed to gather some rare fish from the surrounding waters. Sometimes, discretion is the better part of valor."));
}

pub fn the_kraken_s_lair_discovery_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! You won't believe what we've found! A massive underwater cave system!"))
        .line(captain!("Calm down and report properly. What's so special about these caves?"))
        .line(crew2!("Sir, there are giant suction cup marks everywhere. And the water... it's black with ink!"))
        .line(crew3!("It can only be one thing, Cap'n. We've stumbled upon the Kraken's lair!"))
        .line(captain!("Neptune's beard! This could be the discovery of a lifetime... or our watery grave. What's our move, crew?"))
        .choice("Explore Lair", explore_lair)
        .conditional_choice("Communicate", attempt_communication, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Leave Quietly", leave_quietly)
        .hint("Squawk! Even the mightiest beasts have their secrets... and their treasures!")
}