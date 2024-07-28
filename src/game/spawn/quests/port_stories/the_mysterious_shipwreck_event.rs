use super::port_stories_base;
use crate::game::spawn::quests::prelude::*;

fn investigate_shipwreck(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low | W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 25);
            actions.delta_crew(1);
            actions.add_dialogue(captain!(
                "A successful expedition! We found some treasure and even recruited a survivor."
            ));
        }
        (H::Blistering | H::Warm, _, M::Humid) => {
            actions.delta_crew(-1);
            actions.delta_health(-5);
            actions.add_dialogue(captain!(
                "The heat and humidity were brutal. We lost a good sailor to heatstroke."
            ));
        }
        (_, W::High | W::GaleForce, _) => {
            actions.delta_health(-10);
            actions.add_dialogue(captain!(
                "The strong winds made the expedition treacherous. The ship took some damage."
            ));
        }
        _ => {
            actions.delta_items(Item::Gold, 20);
            actions.add_dialogue(captain!(
                "We found a small amount of salvage, but nothing extraordinary."
            ));
        }
    }
}

fn ignore_shipwreck(actions: &mut StoryActions) {
    actions.delta_crew(-1);
    actions.add_dialogue(crew2!(
        "Cap'n, one of the crew deserted to join another ship investigating the wreck."
    ));
}

pub fn the_mysterious_shipwreck_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!(
            "Cap'n! There's word of a mysterious shipwreck just outside the harbor!"
        ))
        .line(crew3!(
            "Aye, they say it appeared overnight, shrouded in mist."
        ))
        .line(captain!("Interesting... What do you lot think?"))
        .line(crew2!(
            "Could be treasure, could be trouble. The choice is yours, Cap'n."
        ))
        .choice("Investigate", investigate_shipwreck)
        .choice("Ignore", ignore_shipwreck)
        .hint("Squawk! Fair winds make for safe explorations!")
}

