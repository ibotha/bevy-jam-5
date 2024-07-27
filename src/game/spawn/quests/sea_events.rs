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

fn plain_sailing(_actions: &StoryActions) -> DayEvent {
    DayEvent::new()
        .line(captain!(
            "Nothing on the horizon,",
            "should be a good day of smooth sailing."
        ))
        .choice("Sail", sail)
        .choice("Rest", rest)
        .choice("Woah", woah)
}

fn port_spotted(_actions: &StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("Land HO! There is a port on the horizon."))
        .choice("Sail", sail)
        .choice("Dock", dock)
}

fn island_spotted(_actions: &StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("I see an island captain."))
        .choice("Sail", sail)
        .choice("Explore", explore_island)
}

pub(super) fn select_random_sea_event(actions: &mut StoryActions) -> DayEvent {
    let choices = [
        (island_spotted(actions), 1),
        (port_spotted(actions), 1),
        (plain_sailing(actions), 14),
    ];
    weighted_random(Some(actions.get_journey_rng()), &choices).clone()
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
