use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn assist_harbormaster(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture: _,
    } = actions.weather();

    match (heat, wind) {
        (H::Comfortable | H::Warm, W::Low | W::Medium) => {
            actions.delta_items(Item::Gold, -50);
            actions.delta_crew(2);
            actions.delta_health(5);
            actions.add_dialogue(captain!("Our assistance was greatly appreciated. We've gained some skilled hands and the harbormaster's favor."));
        }
        (H::Blistering, _) | (_, W::High | W::GaleForce) => {
            actions.delta_items(Item::Gold, -100);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("The task was more challenging than expected due to the conditions. We spent more and took some damage, but the harbormaster is grateful."));
        }
        _ => {
            actions.delta_items(Item::Gold, -25);
            actions.delta_crew(1);
            actions.add_dialogue(captain!("We managed to help out without too much trouble. The harbormaster offered us a new crewmate as thanks."));
        }
    }
}

fn negotiate_fee(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 200 {
        actions.delta_items(Item::Gold, -200);
        actions.delta_items(Item::Cannon, 1);
        actions.add_dialogue(captain!("We've secured a good deal. This new cannon will serve us well."));
    } else {
        actions.delta_items(Item::Gold, -50);
        actions.delta_food(15);
        actions.add_dialogue(captain!("We couldn't afford the cannon, but we negotiated for some extra supplies instead."));
    }
}

fn decline_involvement(actions: &mut StoryActions) {
    actions.delta_crew(-1);
    actions.add_dialogue(captain!("One of our crew was disappointed by our lack of community spirit and left to help the harbormaster."));
}

pub fn the_harbormasters_dilemma_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n, the harbormaster's in a right state! Says there's a problem with the lighthouse."))
        .line(captain!("What sort of problem, Patchy?"))
        .line(crew2!("Heard it's the light mechanism, Cap'n. Been on the fritz, making night docking dangerous."))
        .line(crew3!("The harbormaster's offering a reward for help, but it could be risky work."))
        .line(captain!("Interesting... What are our options?"))
        .choice("Assist", assist_harbormaster)
        .conditional_choice("Negotiate", negotiate_fee, actions.get_item(Item::Gold) >= 50)
        .choice("Decline", decline_involvement)
        .hint("Squawk! A lighthouse in need often leads to treasure indeed!")
}
