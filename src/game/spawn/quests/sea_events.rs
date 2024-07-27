use rand::RngCore;

use super::prelude::*;

pub(super) fn sail(actions: &mut StoryActions) {
    actions.travel(10);
}

fn rest(actions: &mut StoryActions) {
    actions.delta_crew(1);
}

fn dock(actions: &mut StoryActions) {
    actions.change_environment(super::Environment::Port);
}

fn explore_island(actions: &mut StoryActions) {
    actions.change_environment(super::Environment::Island);
}

fn woah(actions: &mut StoryActions) {
    actions.delta_crew(10);
    actions.delta_max_crew(10);
    actions.delta_health(10);
    actions.delta_max_health(10);
    actions.delta_food(10);
    actions.delta_max_food(10);
}

fn plain_sailing() -> DayEvent {
    DayEvent::new()
        .line(captain!(
            "Nothing on the horizon,",
            "should be a good day of smooth sailing."
        ))
        .choice("Sail", sail)
        .choice("Rest", rest)
        .choice("Woah", woah)
}

fn port_spotted() -> DayEvent {
    DayEvent::new()
        .line(crew1!("Land HO! There is a port on the horizon."))
        .choice("Sail", sail)
        .choice("Dock", dock)
}

fn island_spotted() -> DayEvent {
    DayEvent::new()
        .line(crew1!("I see an island captain."))
        .choice("Sail", sail)
        .choice("Explore", explore_island)
}

pub(super) fn select_random_sea_event(rng: &mut impl RngCore) -> DayEvent {
    weighted_random(
        Some(rng),
        &[
            (island_spotted(), 1),
            (port_spotted(), 1),
            (plain_sailing(), 14),
        ],
    )
    .clone()
}

// ============= Special Events ==============

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
            actions.delta_health(-10);
            actions.delta_crew(-1);
            actions.delta_items(Item::Cannon, -2);
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
            actions.delta_health(-10);
            actions.delta_crew(-1);
            actions.delta_items(Item::Cannon, -2);
            actions.delta_items(Item::Gold, -200);
        }
        _ => {
            actions.add_dialogue(crew3!("Clear skies, fire away!"));
            actions.add_dialogue(captain!("Nice shot, they are going down!"));
            actions.delta_health((-3 + actions.get_item(Item::Cannon)).min(-1));
            actions.delta_crew(1);
            actions.delta_items(Item::Cannon, 1);
            actions.delta_items(Item::Gold, 200);
        }
    }
}

pub(super) fn bounty_hunters() -> DayEvent {
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
}
