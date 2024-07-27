use super::{sea_stories::*, prelude::*};

pub fn set_next_port(actions: &mut StoryActions, distance: u32) {
    actions.add_event(FollowingEvent {
        event: port_spotted,
        certainty: Certainty::Possible(10),
        delay: Delay::Distance(distance as i32),
        environment: Environment::Sea(actions.get_current_sea()),
    });
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
    let choices = [
        (the_plain_sailing_event as EventBuilder, 14),
        (island_spotted, 1),
    ];
    weighted_random(Some(actions.get_journey_rng()), &choices).clone()
}