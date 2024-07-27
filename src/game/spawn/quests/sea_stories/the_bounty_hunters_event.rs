use crate::game::spawn::quests::prelude::*;

fn hide(actions: &mut StoryActions) {
    let DW {
        heat,
        moisture,
        wind,
    } = actions.weather();

    match (heat, moisture, wind) {
        (H::Blistering | H::Warm, M::Humid, W::High | W::GaleForce) => {
            actions.add_dialogue(crew2!("I told you a storm was coming."));
            actions.add_dialogue(captain!("Looks like we are in the clear."));
        }
        _ => {
            actions.add_dialogue(crew3!("They can sea us clear as day!"));
            actions.add_dialogue(captain!("Damn! They have spotted us!"));
            let favour = actions.battle(5, -1, "the bounty hunters");
            actions.delta_items(Item::Cannon, favour / 3);
            actions.delta_items(Item::Gold, favour * 10);
        }
    }
}

fn fight(actions: &mut StoryActions) {
    let DW {
        heat,
        moisture,
        wind,
    } = actions.weather();

    match (heat, moisture, wind) {
        (H::Blistering | H::Warm, M::Humid, W::High | W::GaleForce) => {
            actions.add_dialogue(crew2!("I can't see where they are!"));
            actions.add_dialogue(captain!("Everyone get down!"));
            let favour = actions.battle(5, -1, "the bounty hunters");
            actions.delta_items(Item::Cannon, favour / 3);
            actions.delta_items(Item::Gold, favour * 10);
        }
        _ => {
            actions.add_dialogue(crew3!("Clear skies, fire away!"));
            actions.add_dialogue(captain!("Nice shot, they are going down!"));
            actions.delta_health((-3 + actions.get_item(Item::Cannon)).min(-1));
            let favour = actions.battle(5, 1, "the bounty hunters");
            actions.delta_items(Item::Cannon, favour / 3);
            actions.delta_items(Item::Gold, favour * 10);
        }
    }
}

fn bribe(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -200);
    actions.add_dialogue(captain!("A steep price."))
}

// If ever failed stealing, we can call this event.
// This will send bounty hunters after the crew.
pub fn the_bounty_hunters_event(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("Ship! Looks like bounty hunters Captian!"))
        .line(crew2!(
            "I think there is a storm brewing! We can hide in that cove."
        ))
        .line(crew3!(
            "A storm!? There isn't a cloud in the sky! We can out-gun them in the open!"
        ))
        .choice("Hide", hide)
        .choice("Fight", fight)
        .conditional_choice("Bribe", bribe, actions.get_item(Item::Gold) > 200)
}