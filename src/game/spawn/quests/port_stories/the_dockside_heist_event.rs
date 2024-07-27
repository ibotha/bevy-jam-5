use super::port_stories_base;
use crate::game::spawn::quests::prelude::*;

fn join_heist(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture: _,
    } = actions.weather();

    match (heat, wind) {
        (H::Comfortable | H::Chilly, W::Low | W::None) => {
            actions.delta_items(Item::Gold, 300);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("Perfect conditions for the heist. We made off with a fortune, but lost a crew member in the escape."));
        }
        (H::Warm | H::Comfortable, W::Medium) => {
            actions.delta_items(Item::Gold, 150);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("We managed to steal some gold and a cannon, but it was a close call. The crew's on edge."));
        }
        (H::Blistering, _) | (_, W::High | W::GaleForce) => {
            actions.delta_items(Item::Gold, -100);
            actions.delta_crew(-2);
            actions.add_dialogue(captain!("The heist was a disaster! We lost gold and two crew members. We're lucky to have escaped at all."));
        }
        _ => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_health(-5);
            actions.add_dialogue(captain!(
                "We got away with some gold, but the ship took damage during our hasty departure."
            ));
        }
    }
}

fn warn_merchant(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 100);
    actions.delta_food(15);
    actions.add_dialogue(captain!("The merchant was grateful for the warning. He rewarded us with gold and provisions, and we kept our conscience clear."));
}

fn alert_authorities(actions: &mut StoryActions) {
    if actions.get_crew() > 5 {
        actions.delta_items(Item::Gold, 200);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("The authorities appreciated our help. We received a hefty reward, but one crew member left, disappointed in our 'lack of adventure'."));
    } else {
        actions.delta_items(Item::Gold, 50);
        actions.add_dialogue(captain!("We alerted the authorities, but without enough crew to assist, we only received a small reward for the information."));
    }
}

pub fn the_dockside_heist_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!(
            "Cap'n! I've overheard a group planning a heist on a wealthy merchant's ship tonight."
        ))
        .line(captain!(
            "A heist, you say? That's some valuable information. What else do you know?"
        ))
        .line(crew2!(
            "They're saying the ship's loaded with gold and exotic goods, Cap'n. It's tempting..."
        ))
        .line(crew3!(
            "But it's risky business. We could warn the merchant or alert the authorities instead."
        ))
        .line(captain!(
            "Indeed, we have a decision to make. What are our options?"
        ))
        .choice("Join", join_heist)
        .choice("Warn", warn_merchant)
        .conditional_choice("Alert", alert_authorities, actions.get_crew() > 5)
        .hint("Squawk! Sometimes the greatest treasures are found in a clear conscience!")
}

