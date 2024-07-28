use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn confront_kraken(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::None, M::Dry) => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_items(Item::Cannon, 2);
            actions.delta_health(20);
            actions.add_dialogue(captain!("By all that's holy! We've defeated the Kraken! Its lair is filled with gold from sunken ships, and we've salvaged two pristine cannons. The crew's morale is through the roof!"));
        }
        (W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_crew(-2);
            actions.delta_health(-15);
            actions.add_dialogue(captain!("We've slain the beast, but at a terrible cost. Two brave souls lost to the depths, and the ship's taken a beating. Still, the Kraken's treasure is ours."));
        }
        (W::High | W::GaleForce, M::Humid) => {
            actions.delta_crew(-4);
            actions.delta_health(-30);
            actions.add_dialogue(captain!("Neptune's wrath! The Kraken was too powerful in this storm. We've lost four good men and the ship's barely afloat. We're lucky to escape with our lives!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 30);
            actions.delta_crew(-1);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("A pyrrhic victory if I ever saw one. We've slain the Kraken and taken some of its hoard, but lost a man and the ship's in dire need of repairs."));
        }
    }
}

fn negotiate_with_kraken(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 30 {
        actions.delta_items(Item::Gold, -30);
        actions.delta_items(Item::Cannon, 3);
        actions.delta_health(30);
        actions.add_dialogue(captain!("I can't believe it worked! We offered the gold as tribute, and the Kraken gave us safe passage. It even allowed us to take some cannons from its graveyard of ships. The crew's calling it a miracle!"));
    } else {
        actions.delta_health(-20);
        actions.delta_crew(-2);
        actions.add_dialogue(captain!("We didn't have enough gold to appease the Kraken. It attacked in a fury, dragging two men to the depths. We barely escaped with our lives."));
    }
}

fn flee_kraken(actions: &mut StoryActions) {
    actions.delta_food(-30);
    actions.delta_health(-10);
    actions.add_dialogue(captain!("We've managed to escape, but the Kraken's chase has led us far off course. Our supplies are dangerously low, and the ship's taken some damage from pushing so hard."));
}

pub fn the_krakens_treasure_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! The sea's churning strangely... and there's a massive shape beneath the waves!"))
        .line(captain!("By the powers... it's the Kraken! I never thought I'd live to see it."))
        .line(crew2!("They say the Kraken guards untold riches from ships it's sunk, Cap'n. But it's more monster than any of us have faced."))
        .line(crew3!("Some legends speak of offering the Kraken tribute to pass safely. But who knows if they're true?"))
        .line(captain!("This could be our greatest triumph or our doom, lads. What's our course?"))
        .choice("Confront Kraken", confront_kraken)
        .conditional_choice("Negotiate", negotiate_with_kraken, actions.get_item(Item::Gold) >= 300)
        .choice("Flee", flee_kraken)
        .hint("Squawk! Only the bravest or the maddest dare to challenge the legends of the deep!")
}