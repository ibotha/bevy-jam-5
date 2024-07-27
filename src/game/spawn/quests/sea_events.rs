use super::prelude::*;

pub fn set_next_port(actions: &mut StoryActions, distance: u32) {
    actions.add_event(FollowingEvent {
        event: port_spotted,
        certainty: Certainty::Possible(10),
        delay: Delay::Distance(distance as i32),
        environment: Environment::Sea(actions.get_current_sea()),
    });
}

pub(super) fn sail(actions: &mut StoryActions) {
    actions.travel(actions.possible_distance().min(actions.get_crew()));
    actions.delta_crew(-actions.danger() / 3);
}

fn rest(actions: &mut StoryActions) {
    actions.delta_crew(1);
}

fn hunker_down(_actions: &mut StoryActions) {}

fn plain_sailing(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(captain!(
            "Nothing on the horizon for today,",
            "Anything I should be aware of sage?"
        ))
        .choice("Sail", sail)
        .choice("Rest", rest)
        .choice("Hunker", hunker_down)
}

fn dock(actions: &mut StoryActions) {
    actions.change_environment(super::Environment::Port(Port::Random));
}

fn sail_on(actions: &mut StoryActions) {
    sail(actions);
    set_next_port(actions, 20);
}

pub fn port_spotted(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("Land HO! There is a port on the horizon."))
        .choice("Sail On", sail_on)
        .choice("Dock", dock)
}

fn explore_island(actions: &mut StoryActions) {
    actions.change_environment(super::Environment::Island(Island::Random));
}

fn island_spotted(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("I see an island captain."))
        .choice("Sail", sail)
        .choice("Explore", explore_island)
}

pub(super) fn select_random_sea_event(actions: &mut StoryActions, sea: Sea) -> EventBuilder {
    let choices = [(island_spotted as EventBuilder, 1), (plain_sailing, 14)];
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

pub(super) fn bounty_hunters(actions: &mut StoryActions) -> DayEvent {
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
